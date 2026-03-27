use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use tokio::sync::OnceCell;

use leta_cache::LmdbCache;
use leta_config::Config;
use leta_daemon::handlers::{handle_add_workspace, handle_remove_workspace};
use leta_daemon::handlers::{
    handle_calls, handle_declaration, handle_files, handle_grep, handle_implementations,
    handle_move_file, handle_references, handle_rename, handle_resolve_symbol, handle_show,
    handle_subtypes, handle_supertypes, HandlerContext,
};
use leta_daemon::session::Session;
use leta_output::*;
use leta_types::*;

struct State {
    ctx: HandlerContext,
    config: Config,
}

static STATE: OnceCell<State> = OnceCell::const_new();

async fn get_state() -> Result<&'static State> {
    STATE
        .get_or_try_init(|| async {
            let mut config = Config::load().map_err(|e| anyhow!("{}", e))?;
            config.cleanup_stale_workspace_roots();

            let cache_dir = leta_config::get_cache_dir();
            std::fs::create_dir_all(&cache_dir)?;

            let hover_cache = LmdbCache::new(
                &cache_dir.join("hover_cache.lmdb"),
                config.daemon.hover_cache_size,
            )
            .map_err(|e| anyhow!("{}", e))?;
            let symbol_cache = LmdbCache::new(
                &cache_dir.join("symbol_cache.lmdb"),
                config.daemon.symbol_cache_size,
            )
            .map_err(|e| anyhow!("{}", e))?;

            let session = Arc::new(Session::new(config.clone()));
            let ctx = HandlerContext::new(session, Arc::new(hover_cache), Arc::new(symbol_cache));

            Ok(State { ctx, config })
        })
        .await
}

fn get_workspace_root(config: &Config, working_dir: &Path) -> Result<PathBuf> {
    config
        .get_best_workspace_root(working_dir, Some(working_dir))
        .ok_or_else(|| {
            anyhow!(
                "No workspace found for {}\nRun: leta workspace add",
                working_dir.display()
            )
        })
}

fn get_workspace_root_for_path(
    config: &Config,
    path: &Path,
    working_dir: &Path,
) -> Result<PathBuf> {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    config
        .get_best_workspace_root(&path, Some(working_dir))
        .ok_or_else(|| {
            anyhow!(
                "No workspace found for {}\nRun: leta workspace add",
                path.display()
            )
        })
}

async fn resolve(symbol: &str, workspace_root: &Path) -> Result<ResolveSymbolResult> {
    let state = get_state().await?;
    let params = ResolveSymbolParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        symbol_path: symbol.to_string(),
    };
    let result = handle_resolve_symbol(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    if let Some(error) = &result.error {
        let mut msg = error.clone();
        if let Some(matches) = &result.matches {
            for m in matches {
                let container = m
                    .container
                    .as_ref()
                    .map(|c| format!(" in {}", c))
                    .unwrap_or_default();
                let kind = format!("[{}] ", m.kind);
                let detail = m
                    .detail
                    .as_ref()
                    .map(|d| format!(" ({})", d))
                    .unwrap_or_default();
                let ref_str = m.reference.as_deref().unwrap_or("");
                msg.push_str(&format!("\n  {}", ref_str));
                msg.push_str(&format!(
                    "\n    {}:{} {}{}{}{}",
                    m.path, m.line, kind, m.name, detail, container
                ));
            }
            if let Some(total) = result.total_matches {
                let shown = matches.len() as u32;
                if total > shown {
                    msg.push_str(&format!("\n  ... and {} more", total - shown));
                }
            }
        }
        return Err(anyhow!("{}", msg));
    }

    Ok(result)
}

pub async fn show(working_dir: &Path, symbol: &str, context: u32, head: u32) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let resolved = resolve(symbol, &workspace_root).await?;

    let params = ShowParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        path: resolved.path.unwrap_or_default(),
        line: resolved.line.unwrap_or(0),
        column: resolved.column.unwrap_or(0),
        context,
        head: if head > 0 { Some(head) } else { None },
        symbol_name: Some(symbol.to_string()),
        symbol_kind: resolved.kind,
        range_start_line: resolved.range_start_line,
        range_end_line: resolved.range_end_line,
        direct_location: true,
    };
    let result = handle_show(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    Ok(format_show_result(&result, head))
}

pub async fn refs(working_dir: &Path, symbol: &str, context: u32, head: u32) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let resolved = resolve(symbol, &workspace_root).await?;

    let params = ReferencesParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        path: resolved.path.unwrap_or_default(),
        line: resolved.line.unwrap_or(0),
        column: resolved.column.unwrap_or(0),
        context,
        head,
    };
    let result = handle_references(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = format!("leta refs \"{}\"", symbol);
    Ok(format_references_result(&result, head, &command_base))
}

pub struct GrepOptions {
    pub path_pattern: Option<String>,
    pub kinds: Option<Vec<String>>,
    pub case_sensitive: bool,
    pub exclude_patterns: Vec<String>,
    pub head: u32,
}

pub async fn grep(working_dir: &Path, pattern: &str, options: GrepOptions) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;

    let head = options.head;
    let params = GrepParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        pattern: pattern.to_string(),
        kinds: options.kinds,
        case_sensitive: options.case_sensitive,
        path_pattern: options.path_pattern,
        exclude_patterns: options.exclude_patterns,
        limit: head,
    };
    let result = handle_grep(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = format!("leta grep \"{}\"", pattern);
    Ok(format_grep_result(&result, head, &command_base))
}

pub async fn files(
    working_dir: &Path,
    subpath: Option<&Path>,
    exclude_patterns: Vec<String>,
    include_patterns: Vec<String>,
    filter_pattern: Option<&str>,
    head: u32,
) -> Result<String> {
    let state = get_state().await?;
    let (workspace_root, resolved_subpath) = if let Some(path) = subpath {
        let target = path.canonicalize()?;
        let wr = get_workspace_root_for_path(&state.config, &target, working_dir)?;
        (wr, Some(target.to_string_lossy().to_string()))
    } else {
        (get_workspace_root(&state.config, working_dir)?, None)
    };

    let params = FilesParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        subpath: resolved_subpath,
        exclude_patterns,
        include_patterns,
        filter_pattern: filter_pattern.map(String::from),
        head,
    };
    let result = handle_files(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = "leta files".to_string();
    Ok(format_files_result(&result, head, &command_base))
}

pub async fn calls(
    working_dir: &Path,
    from: Option<&str>,
    to: Option<&str>,
    max_depth: u32,
    include_non_workspace: bool,
    head: u32,
) -> Result<String> {
    if from.is_none() && to.is_none() {
        return Err(anyhow!("At least one of --from or --to must be specified"));
    }

    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let ws = workspace_root.to_string_lossy().to_string();

    let mut params = CallsParams {
        workspace_root: ws,
        mode: CallsMode::Outgoing,
        from_path: None,
        from_line: None,
        from_column: None,
        from_symbol: None,
        to_path: None,
        to_line: None,
        to_column: None,
        to_symbol: None,
        max_depth,
        include_non_workspace,
        head,
    };

    if let (Some(from_sym), Some(to_sym)) = (from, to) {
        let from_r = resolve(from_sym, &workspace_root).await?;
        let to_r = resolve(to_sym, &workspace_root).await?;
        params.mode = CallsMode::Path;
        params.from_path = from_r.path;
        params.from_line = from_r.line;
        params.from_column = from_r.column;
        params.from_symbol = Some(from_sym.to_string());
        params.to_path = to_r.path;
        params.to_line = to_r.line;
        params.to_column = to_r.column;
        params.to_symbol = Some(to_sym.to_string());
    } else if let Some(from_sym) = from {
        let r = resolve(from_sym, &workspace_root).await?;
        params.mode = CallsMode::Outgoing;
        params.from_path = r.path;
        params.from_line = r.line;
        params.from_column = r.column;
        params.from_symbol = Some(from_sym.to_string());
    } else if let Some(to_sym) = to {
        let r = resolve(to_sym, &workspace_root).await?;
        params.mode = CallsMode::Incoming;
        params.to_path = r.path;
        params.to_line = r.line;
        params.to_column = r.column;
        params.to_symbol = Some(to_sym.to_string());
    }

    let result = handle_calls(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = "leta calls".to_string();
    Ok(format_calls_result(&result, head, &command_base))
}

pub async fn declaration(
    working_dir: &Path,
    symbol: &str,
    context: u32,
    head: u32,
) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let resolved = resolve(symbol, &workspace_root).await?;

    let params = DeclarationParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        path: resolved.path.unwrap_or_default(),
        line: resolved.line.unwrap_or(0),
        column: resolved.column.unwrap_or(0),
        context,
        head,
    };
    let result = handle_declaration(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = format!("leta declaration \"{}\"", symbol);
    Ok(format_declaration_result(&result, head, &command_base))
}

pub async fn implementations(
    working_dir: &Path,
    symbol: &str,
    context: u32,
    head: u32,
) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let resolved = resolve(symbol, &workspace_root).await?;

    let params = ImplementationsParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        path: resolved.path.unwrap_or_default(),
        line: resolved.line.unwrap_or(0),
        column: resolved.column.unwrap_or(0),
        context,
        head,
    };
    let result = handle_implementations(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = format!("leta implementations \"{}\"", symbol);
    Ok(format_implementations_result(&result, head, &command_base))
}

pub async fn subtypes(working_dir: &Path, symbol: &str, context: u32, head: u32) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let resolved = resolve(symbol, &workspace_root).await?;

    let params = SubtypesParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        path: resolved.path.unwrap_or_default(),
        line: resolved.line.unwrap_or(0),
        column: resolved.column.unwrap_or(0),
        context,
        head,
    };
    let result = handle_subtypes(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = format!("leta subtypes \"{}\"", symbol);
    Ok(format_subtypes_result(&result, head, &command_base))
}

pub async fn supertypes(
    working_dir: &Path,
    symbol: &str,
    context: u32,
    head: u32,
) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let resolved = resolve(symbol, &workspace_root).await?;

    let params = SupertypesParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        path: resolved.path.unwrap_or_default(),
        line: resolved.line.unwrap_or(0),
        column: resolved.column.unwrap_or(0),
        context,
        head,
    };
    let result = handle_supertypes(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    let command_base = format!("leta supertypes \"{}\"", symbol);
    Ok(format_supertypes_result(&result, head, &command_base))
}

pub async fn rename(working_dir: &Path, symbol: &str, new_name: &str) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = get_workspace_root(&state.config, working_dir)?;
    let resolved = resolve(symbol, &workspace_root).await?;

    let params = RenameParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        path: resolved.path.unwrap_or_default(),
        line: resolved.line.unwrap_or(0),
        column: resolved.column.unwrap_or(0),
        new_name: new_name.to_string(),
    };
    let result = handle_rename(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    Ok(format_rename_result(&result))
}

pub async fn mv(working_dir: &Path, old_path: &str, new_path: &str) -> Result<String> {
    let state = get_state().await?;
    let old = PathBuf::from(old_path).canonicalize()?;
    let new = working_dir.join(new_path);
    let workspace_root = get_workspace_root_for_path(&state.config, &old, working_dir)?;

    let params = MoveFileParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
        old_path: old.to_string_lossy().to_string(),
        new_path: new.to_string_lossy().to_string(),
    };
    let result = handle_move_file(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    Ok(format_move_file_result(&result))
}

pub async fn workspace_add(path: &Path) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = path.canonicalize()?;

    let params = AddWorkspaceParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
    };
    let result = handle_add_workspace(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    if result.added {
        Ok(format!("Added workspace: {}", result.workspace_root))
    } else {
        Ok(format!(
            "Workspace already added: {}",
            result.workspace_root
        ))
    }
}

pub async fn workspace_remove(working_dir: &Path, path: Option<&Path>) -> Result<String> {
    let state = get_state().await?;
    let workspace_root = if let Some(p) = path {
        p.canonicalize()?
    } else {
        get_workspace_root(&state.config, working_dir)?
    };

    let params = RemoveWorkspaceParams {
        workspace_root: workspace_root.to_string_lossy().to_string(),
    };
    let _result = handle_remove_workspace(&state.ctx, params)
        .await
        .map_err(|e| anyhow!("{}", e))?;

    Ok(format!("Removed workspace: {}", workspace_root.display()))
}
