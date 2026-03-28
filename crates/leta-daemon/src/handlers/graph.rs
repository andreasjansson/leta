use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use fastrace::trace;
use leta_fs::uri_to_path;
use leta_lsp::lsp_types::{
    CallHierarchyItem, CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams,
    CallHierarchyPrepareParams, Position, TextDocumentIdentifier, TextDocumentPositionParams,
};
use leta_lsp::LspClient;
use leta_types::{
    CallGraphEdge, CallGraphFileEdges, CallGraphSymbol, GraphParams, GraphResult, SymbolInfo,
    SymbolKind,
};
use regex::Regex;
use tokio::sync::Semaphore;
use tracing::{debug, info};

use super::{relative_path, HandlerContext};

const CALLABLE_KINDS: &[&str] = &["Function", "Method", "Constructor"];

fn is_callable(sym: &SymbolInfo) -> bool {
    CALLABLE_KINDS.iter().any(|k| sym.kind == *k)
}

fn cache_key(file_path: &Path, mtime: &str) -> String {
    format!("callgraph2:{}:{}", file_path.display(), mtime)
}

fn is_path_in_workspace(uri: &str, workspace_root: &Path) -> bool {
    let file_path = uri_to_path(uri);
    match file_path.strip_prefix(workspace_root) {
        Ok(rel_path) => {
            let excluded_dirs = [
                ".venv",
                "venv",
                "node_modules",
                "vendor",
                ".git",
                "__pycache__",
                "target",
                "build",
                "dist",
            ];
            !rel_path
                .iter()
                .any(|part| excluded_dirs.iter().any(|d| part.to_str() == Some(*d)))
        }
        Err(_) => false,
    }
}

fn sym_to_graph_symbol(sym: &SymbolInfo) -> CallGraphSymbol {
    CallGraphSymbol {
        name: sym.name.clone(),
        kind: sym.kind.clone(),
        path: sym.path.clone(),
        line: sym.line,
        column: sym.column,
        detail: sym.detail.clone(),
    }
}

fn call_item_to_graph_symbol(item: &CallHierarchyItem, workspace_root: &Path) -> CallGraphSymbol {
    let file_path = uri_to_path(item.uri.as_str());
    let rel_path = relative_path(&file_path, workspace_root);
    let kind = SymbolKind::from_lsp(item.kind);

    CallGraphSymbol {
        name: item.name.clone(),
        kind: kind.to_string(),
        path: rel_path,
        line: item.selection_range.start.line + 1,
        column: item.selection_range.start.character,
        detail: item.detail.clone(),
    }
}

#[trace]
pub async fn handle_graph(
    ctx: &HandlerContext,
    params: GraphParams,
) -> Result<GraphResult, String> {
    let workspace_root = PathBuf::from(&params.workspace_root);
    let start = std::time::Instant::now();

    let exclude_regexes: Vec<Regex> = params
        .exclude_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    let include_regexes: Vec<Regex> = params
        .include_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    let path_matches = |path: &str| -> bool {
        if !include_regexes.is_empty() && !include_regexes.iter().any(|re| re.is_match(path)) {
            return false;
        }
        if exclude_regexes.iter().any(|re| re.is_match(path)) {
            return false;
        }
        true
    };

    let all_symbols = super::collect_all_workspace_symbols(ctx, &workspace_root).await?;

    let callable_symbols: Vec<&SymbolInfo> = all_symbols
        .iter()
        .filter(|s| is_callable(s))
        .filter(|s| path_matches(&s.path))
        .collect();
    info!(
        "Building call graph: {} callable symbols out of {} total",
        callable_symbols.len(),
        all_symbols.len()
    );

    let mut symbols_by_file: HashMap<&str, Vec<&SymbolInfo>> = HashMap::new();
    for sym in &callable_symbols {
        symbols_by_file.entry(&sym.path).or_default().push(sym);
    }

    let mut all_edges: Vec<CallGraphEdge> = Vec::new();
    let mut files_from_cache = 0u32;
    let mut files_computed = 0u32;
    let mut servers_waited: HashSet<String> = HashSet::new();
    let mut unsupported_servers: Vec<String> = Vec::new();

    for (rel_path, file_syms) in &symbols_by_file {
        let file_path = workspace_root.join(rel_path);
        let mtime = leta_fs::file_mtime(&file_path);
        let key = cache_key(&file_path, &mtime);

        if let Some(cached) = ctx.hover_cache.get::<CallGraphFileEdges>(&key) {
            all_edges.extend(cached.edges);
            files_from_cache += 1;
            continue;
        }

        let first_sym = file_syms[0];
        let file_abs = workspace_root.join(&first_sym.path);
        let workspace = match ctx
            .session
            .get_or_create_workspace(&file_abs, &workspace_root)
            .await
        {
            Ok(ws) => ws,
            Err(e) => {
                debug!("Skipping {}: {}", rel_path, e);
                continue;
            }
        };

        let client = match workspace.client().await {
            Some(c) => c,
            None => continue,
        };

        let server_name = client.server_name().to_string();
        if servers_waited.insert(server_name.clone()) {
            client.wait_for_indexing(30).await;
        }

        if !client.supports_call_hierarchy().await {
            debug!(
                "Skipping {} - server doesn't support call hierarchy",
                rel_path
            );
            if !unsupported_servers.contains(&server_name) {
                unsupported_servers.push(server_name);
            }
            ctx.hover_cache
                .set(&key, &CallGraphFileEdges { edges: vec![] });
            continue;
        }

        workspace.ensure_document_open(&file_abs).await.ok();

        let edges = collect_file_edges(&client, &workspace_root, file_syms).await;

        ctx.hover_cache
            .set(&key, &CallGraphFileEdges { edges: edges.clone() });
        all_edges.extend(edges);
        files_computed += 1;
    }

    // Filter edges after caching (cache always stores all edges)
    if !params.include_non_workspace {
        all_edges.retain(|e| e.in_workspace);
    }
    if !include_regexes.is_empty() || !exclude_regexes.is_empty() {
        all_edges.retain(|e| path_matches(&e.caller.path) && path_matches(&e.callee.path));
    }

    let elapsed = start.elapsed();
    info!(
        "Call graph built in {:?}: {} edges, {} files from cache, {} files computed",
        elapsed,
        all_edges.len(),
        files_from_cache,
        files_computed,
    );

    let mut node_set: HashSet<CallGraphSymbol> = HashSet::new();
    for sym in &all_symbols {
        if path_matches(&sym.path) {
            node_set.insert(sym_to_graph_symbol(sym));
        }
    }
    for edge in &all_edges {
        node_set.insert(edge.callee.clone());
    }

    let mut nodes: Vec<CallGraphSymbol> = node_set.into_iter().collect();
    nodes.sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));

    all_edges.sort_by(|a, b| {
        (&a.caller.path, a.caller.line, &a.callee.name)
            .cmp(&(&b.caller.path, b.caller.line, &b.callee.name))
    });

    let error = if all_edges.is_empty() && !unsupported_servers.is_empty() {
        unsupported_servers.sort();
        Some(format!(
            "Call hierarchy is not supported by {}",
            unsupported_servers.join(", ")
        ))
    } else {
        None
    };

    Ok(GraphResult {
        nodes,
        edges: all_edges,
        indexing_time_ms: Some(elapsed.as_millis() as u64),
        error,
    })
}

async fn collect_file_edges(
    client: &Arc<LspClient>,
    workspace_root: &Path,
    symbols: &[&SymbolInfo],
) -> Vec<CallGraphEdge> {
    let semaphore = Arc::new(Semaphore::new(10));
    let mut handles = Vec::new();

    for sym in symbols {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let client = Arc::clone(client);
        let workspace_root = workspace_root.to_path_buf();
        let sym = (*sym).clone();

        let handle = tokio::spawn(async move {
            let result = collect_symbol_edges(&client, &workspace_root, &sym).await;
            drop(permit);
            result
        });
        handles.push(handle);
    }

    let mut edges = Vec::new();
    for handle in handles {
        if let Ok(Ok(file_edges)) = handle.await {
            edges.extend(file_edges);
        }
    }
    edges
}

async fn collect_symbol_edges(
    client: &Arc<LspClient>,
    workspace_root: &Path,
    sym: &SymbolInfo,
) -> Result<Vec<CallGraphEdge>, String> {
    let file_path = workspace_root.join(&sym.path);
    let uri = leta_fs::path_to_uri(&file_path);

    let items: Option<Vec<CallHierarchyItem>> = client
        .send_request(
            "textDocument/prepareCallHierarchy",
            CallHierarchyPrepareParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier {
                        uri: uri.parse().unwrap(),
                    },
                    position: Position {
                        line: sym.line - 1,
                        character: sym.column,
                    },
                },
                work_done_progress_params: Default::default(),
            },
        )
        .await
        .map_err(|e| format!("{}", e))?;

    let items = match items {
        Some(items) if !items.is_empty() => items,
        _ => return Ok(vec![]),
    };

    let outgoing: Option<Vec<CallHierarchyOutgoingCall>> = client
        .send_request(
            "callHierarchy/outgoingCalls",
            CallHierarchyOutgoingCallsParams {
                item: items[0].clone(),
                work_done_progress_params: Default::default(),
                partial_result_params: Default::default(),
            },
        )
        .await
        .map_err(|e| format!("{}", e))?;

    let calls = match outgoing {
        Some(calls) => calls,
        None => return Ok(vec![]),
    };

    let caller = sym_to_graph_symbol(sym);
    let mut edges = Vec::new();

    for call in calls {
        let in_workspace = is_path_in_workspace(call.to.uri.as_str(), workspace_root);
        let callee = call_item_to_graph_symbol(&call.to, workspace_root);
        let call_site_line = call
            .from_ranges
            .first()
            .map(|r| r.start.line + 1);

        edges.push(CallGraphEdge {
            caller: caller.clone(),
            callee,
            in_workspace,
            call_site_line,
        });
    }

    Ok(edges)
}
