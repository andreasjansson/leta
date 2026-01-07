use std::collections::HashSet;
use std::path::PathBuf;

use leta_fs::{path_to_uri, uri_to_path};
use leta_lsp::lsp_types::{CallHierarchyIncomingCall, CallHierarchyItem, CallHierarchyOutgoingCall};
use leta_types::SymbolKind;
use serde_json::{json, Value};

use super::{relative_path, HandlerContext};

pub async fn handle_calls(ctx: &HandlerContext, params: Value) -> Result<Value, String> {
    let workspace_root = PathBuf::from(
        params.get("workspace_root")
            .and_then(|v| v.as_str())
            .ok_or("Missing workspace_root")?
    );
    let mode = params.get("mode")
        .and_then(|v| v.as_str())
        .ok_or("Missing mode")?;
    let max_depth = params.get("max_depth")
        .and_then(|v| v.as_u64())
        .unwrap_or(3) as usize;
    let include_non_workspace = params.get("include_non_workspace")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    match mode {
        "outgoing" => handle_outgoing_calls(ctx, &params, &workspace_root, max_depth, include_non_workspace).await,
        "incoming" => handle_incoming_calls(ctx, &params, &workspace_root, max_depth, include_non_workspace).await,
        "path" => handle_call_path(ctx, &params, &workspace_root, max_depth, include_non_workspace).await,
        _ => Err(format!("Unknown calls mode: {}", mode)),
    }
}

async fn handle_outgoing_calls(
    ctx: &HandlerContext,
    params: &Value,
    workspace_root: &PathBuf,
    max_depth: usize,
    include_non_workspace: bool,
) -> Result<Value, String> {
    let path = PathBuf::from(
        params.get("from_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing from_path")?
    );
    let line = params.get("from_line")
        .and_then(|v| v.as_u64())
        .ok_or("Missing from_line")? as u32;
    let column = params.get("from_column")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    ctx.session.get_or_create_workspace(&path, workspace_root).await?;
    let _ = ctx.session.ensure_document_open(&path, workspace_root).await?;
    let client = ctx.session.get_workspace_client(&path, workspace_root).await
        .ok_or("Failed to get LSP client")?;

    let uri = path_to_uri(&path);
    let prepare_params = json!({
        "textDocument": {"uri": uri},
        "position": {"line": line - 1, "character": column}
    });

    let prepare_result: Result<Vec<CallHierarchyItem>, _> = client
        .send_request("textDocument/prepareCallHierarchy", prepare_params)
        .await;

    let items = match prepare_result {
        Ok(items) if !items.is_empty() => items,
        Ok(_) => return Ok(json!({"message": "No call hierarchy found at this position"})),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("-32601") || msg.contains("not supported") {
                return Err("Call hierarchy not supported by this language server".to_string());
            }
            return Err(format!("LSP error: {}", e));
        }
    };

    let root_item = &items[0];
    let mut visited = HashSet::new();
    let calls = collect_outgoing_calls(ctx, &client, workspace_root, root_item, max_depth, 0, include_non_workspace, &mut visited).await;

    let root = format_call_node(root_item, workspace_root, Some(calls), None);
    Ok(json!({"root": root}))
}

async fn handle_incoming_calls(
    ctx: &HandlerContext,
    params: &Value,
    workspace_root: &PathBuf,
    max_depth: usize,
    include_non_workspace: bool,
) -> Result<Value, String> {
    let path = PathBuf::from(
        params.get("to_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing to_path")?
    );
    let line = params.get("to_line")
        .and_then(|v| v.as_u64())
        .ok_or("Missing to_line")? as u32;
    let column = params.get("to_column")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    ctx.session.get_or_create_workspace(&path, workspace_root).await?;
    let _ = ctx.session.ensure_document_open(&path, workspace_root).await?;
    let client = ctx.session.get_workspace_client(&path, workspace_root).await
        .ok_or("Failed to get LSP client")?;

    let uri = path_to_uri(&path);
    let prepare_params = json!({
        "textDocument": {"uri": uri},
        "position": {"line": line - 1, "character": column}
    });

    let prepare_result: Result<Vec<CallHierarchyItem>, _> = client
        .send_request("textDocument/prepareCallHierarchy", prepare_params)
        .await;

    let items = match prepare_result {
        Ok(items) if !items.is_empty() => items,
        Ok(_) => return Ok(json!({"message": "No call hierarchy found at this position"})),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("-32601") || msg.contains("not supported") {
                return Err("Call hierarchy not supported by this language server".to_string());
            }
            return Err(format!("LSP error: {}", e));
        }
    };

    let root_item = &items[0];
    let mut visited = HashSet::new();
    let called_by = collect_incoming_calls(ctx, &client, workspace_root, root_item, max_depth, 0, include_non_workspace, &mut visited).await;

    let root = format_call_node(root_item, workspace_root, None, Some(called_by));
    Ok(json!({"root": root}))
}

async fn handle_call_path(
    ctx: &HandlerContext,
    params: &Value,
    workspace_root: &PathBuf,
    max_depth: usize,
    include_non_workspace: bool,
) -> Result<Value, String> {
    let from_path = PathBuf::from(
        params.get("from_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing from_path")?
    );
    let from_line = params.get("from_line")
        .and_then(|v| v.as_u64())
        .ok_or("Missing from_line")? as u32;
    let from_column = params.get("from_column")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    let to_path = PathBuf::from(
        params.get("to_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing to_path")?
    );
    let to_line = params.get("to_line")
        .and_then(|v| v.as_u64())
        .ok_or("Missing to_line")? as u32;

    ctx.session.get_or_create_workspace(&from_path, workspace_root).await?;
    let _ = ctx.session.ensure_document_open(&from_path, workspace_root).await?;
    let client = ctx.session.get_workspace_client(&from_path, workspace_root).await
        .ok_or("Failed to get LSP client")?;

    let uri = path_to_uri(&from_path);
    let prepare_params = json!({
        "textDocument": {"uri": uri},
        "position": {"line": from_line - 1, "character": from_column}
    });

    let prepare_result: Result<Vec<CallHierarchyItem>, _> = client
        .send_request("textDocument/prepareCallHierarchy", prepare_params)
        .await;

    let items = match prepare_result {
        Ok(items) if !items.is_empty() => items,
        Ok(_) => return Ok(json!({"message": "No call hierarchy found at from position"})),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("-32601") || msg.contains("not supported") {
                return Err("Call hierarchy not supported by this language server".to_string());
            }
            return Err(format!("LSP error: {}", e));
        }
    };

    let from_item = &items[0];
    let to_rel_path = relative_path(&to_path, workspace_root);
    
    let mut visited = HashSet::new();
    let path = find_call_path(ctx, &client, workspace_root, from_item, &to_rel_path, to_line, max_depth, 0, include_non_workspace, &mut visited).await;

    if let Some(path) = path {
        let path_nodes: Vec<Value> = path.iter().map(|item| format_call_node(item, workspace_root, None, None)).collect();
        Ok(json!({"path": path_nodes}))
    } else {
        let from_symbol = params.get("from_symbol").and_then(|v| v.as_str()).unwrap_or("source");
        let to_symbol = params.get("to_symbol").and_then(|v| v.as_str()).unwrap_or("target");
        Ok(json!({"message": format!("No path found from {} to {} within depth {}", from_symbol, to_symbol, max_depth)}))
    }
}

async fn collect_outgoing_calls(
    ctx: &HandlerContext,
    client: &std::sync::Arc<leta_lsp::LspClient>,
    workspace_root: &PathBuf,
    item: &CallHierarchyItem,
    max_depth: usize,
    current_depth: usize,
    include_non_workspace: bool,
    visited: &mut HashSet<String>,
) -> Vec<Value> {
    if current_depth >= max_depth {
        return vec![];
    }

    let key = format!("{}:{}:{}", item.uri, item.range.start.line, item.name);
    if visited.contains(&key) {
        return vec![];
    }
    visited.insert(key);

    let params = json!({"item": item});
    let result: Result<Vec<CallHierarchyOutgoingCall>, _> = client
        .send_request("callHierarchy/outgoingCalls", params)
        .await;

    let calls = match result {
        Ok(calls) => calls,
        Err(_) => return vec![],
    };

    let mut result_nodes = Vec::new();
    for call in calls {
        let call_item = &call.to;
        
        if !include_non_workspace && is_stdlib_path(&call_item.uri) {
            continue;
        }

        let child_calls = Box::pin(collect_outgoing_calls(
            ctx, client, workspace_root, call_item,
            max_depth, current_depth + 1, include_non_workspace, visited
        )).await;

        let node = format_call_node(call_item, workspace_root, Some(child_calls), None);
        result_nodes.push(node);
    }

    result_nodes
}

async fn collect_incoming_calls(
    ctx: &HandlerContext,
    client: &std::sync::Arc<leta_lsp::LspClient>,
    workspace_root: &PathBuf,
    item: &CallHierarchyItem,
    max_depth: usize,
    current_depth: usize,
    include_non_workspace: bool,
    visited: &mut HashSet<String>,
) -> Vec<Value> {
    if current_depth >= max_depth {
        return vec![];
    }

    let key = format!("{}:{}:{}", item.uri, item.range.start.line, item.name);
    if visited.contains(&key) {
        return vec![];
    }
    visited.insert(key);

    let params = json!({"item": item});
    let result: Result<Vec<CallHierarchyIncomingCall>, _> = client
        .send_request("callHierarchy/incomingCalls", params)
        .await;

    let calls = match result {
        Ok(calls) => calls,
        Err(_) => return vec![],
    };

    let mut result_nodes = Vec::new();
    for call in calls {
        let call_item = &call.from;
        
        if !include_non_workspace && is_stdlib_path(&call_item.uri) {
            continue;
        }

        let child_calls = Box::pin(collect_incoming_calls(
            ctx, client, workspace_root, call_item,
            max_depth, current_depth + 1, include_non_workspace, visited
        )).await;

        let node = format_call_node(call_item, workspace_root, None, Some(child_calls));
        result_nodes.push(node);
    }

    result_nodes
}

async fn find_call_path(
    ctx: &HandlerContext,
    client: &std::sync::Arc<leta_lsp::LspClient>,
    workspace_root: &PathBuf,
    item: &CallHierarchyItem,
    target_path: &str,
    target_line: u32,
    max_depth: usize,
    current_depth: usize,
    include_non_workspace: bool,
    visited: &mut HashSet<String>,
) -> Option<Vec<CallHierarchyItem>> {
    if current_depth >= max_depth {
        return None;
    }

    let key = format!("{}:{}:{}", item.uri, item.range.start.line, item.name);
    if visited.contains(&key) {
        return None;
    }
    visited.insert(key.clone());

    let file_path = uri_to_path(&item.uri);
    let rel_path = relative_path(&file_path, workspace_root);
    let item_line = item.selection_range.start.line + 1;
    
    if rel_path == target_path && item_line == target_line {
        return Some(vec![item.clone()]);
    }

    let params = json!({"item": item});
    let result: Result<Vec<CallHierarchyOutgoingCall>, _> = client
        .send_request("callHierarchy/outgoingCalls", params)
        .await;

    let calls = match result {
        Ok(calls) => calls,
        Err(_) => return None,
    };

    for call in calls {
        let call_item = &call.to;
        
        if !include_non_workspace && is_stdlib_path(&call_item.uri) {
            continue;
        }

        if let Some(mut path) = Box::pin(find_call_path(
            ctx, client, workspace_root, call_item,
            target_path, target_line, max_depth, current_depth + 1, include_non_workspace, visited
        )).await {
            path.insert(0, item.clone());
            return Some(path);
        }
    }

    visited.remove(&key);
    None
}

fn format_call_node(
    item: &CallHierarchyItem,
    workspace_root: &PathBuf,
    calls: Option<Vec<Value>>,
    called_by: Option<Vec<Value>>,
) -> Value {
    let file_path = uri_to_path(&item.uri);
    let rel_path = relative_path(&file_path, workspace_root);
    
    let mut node = json!({
        "name": item.name,
        "kind": SymbolKind::from_lsp_kind(item.kind).to_string(),
        "path": rel_path,
        "line": item.selection_range.start.line + 1,
        "column": item.selection_range.start.character,
    });

    if let Some(detail) = &item.detail {
        node["detail"] = json!(detail);
    }

    if let Some(calls) = calls {
        node["calls"] = json!(calls);
    }

    if let Some(called_by) = called_by {
        node["called_by"] = json!(called_by);
    }

    node
}

fn is_stdlib_path(uri: &str) -> bool {
    uri.contains("/typeshed-fallback/stdlib/")
        || uri.contains("/typeshed/stdlib/")
        || (uri.contains("/libexec/src/") && !uri.contains("/mod/"))
        || (uri.ends_with(".d.ts") && uri.split('/').last().map(|f| f.starts_with("lib.")).unwrap_or(false))
        || uri.contains("/rustlib/src/rust/library/")
}
