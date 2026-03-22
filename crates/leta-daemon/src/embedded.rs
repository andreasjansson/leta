use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};
use tokio::sync::OnceCell;

use crate::handlers::HandlerContext;
use crate::session::Session;
use leta_cache::LmdbCache;
use leta_config::Config;

struct EmbeddedState {
    ctx: HandlerContext,
}

static STATE: OnceCell<EmbeddedState> = OnceCell::const_new();

async fn get_state() -> anyhow::Result<&'static EmbeddedState> {
    STATE
        .get_or_try_init(|| async {
            let config = Config::load().map_err(|e| anyhow::anyhow!("{}", e))?;
            let cache_dir = leta_config::get_cache_dir();
            std::fs::create_dir_all(&cache_dir)?;

            let hover_cache_size = config.daemon.hover_cache_size;
            let symbol_cache_size = config.daemon.symbol_cache_size;

            let hover_cache =
                LmdbCache::new(&cache_dir.join("hover_cache.lmdb"), hover_cache_size)
                    .map_err(|e| anyhow::anyhow!("{}", e))?;
            let symbol_cache =
                LmdbCache::new(&cache_dir.join("symbol_cache.lmdb"), symbol_cache_size)
                    .map_err(|e| anyhow::anyhow!("{}", e))?;

            let session = Arc::new(Session::new(config));

            let ctx = HandlerContext::new(session, Arc::new(hover_cache), Arc::new(symbol_cache));
            Ok(EmbeddedState { ctx })
        })
        .await
}

/// Dispatch a method call directly to leta handlers, bypassing the socket layer.
/// Returns the same JSON structure as the daemon server's dispatch.
pub async fn dispatch(method: &str, params: Value) -> Value {
    let state = match get_state().await {
        Ok(s) => s,
        Err(e) => return json!({"error": format!("Failed to initialize leta: {}", e)}),
    };

    let ctx = &state.ctx;

    macro_rules! handle {
        ($params_ty:ty, $handler:expr) => {{
            match serde_json::from_value::<$params_ty>(params) {
                Ok(p) => match $handler(ctx, p).await {
                    Ok(result) => json!({"result": result}),
                    Err(e) => json!({"error": e}),
                },
                Err(e) => json!({"error": format!("Invalid params: {}", e)}),
            }
        }};
    }

    use crate::handlers::*;
    use leta_types::*;

    match method {
        "grep" => handle!(GrepParams, handle_grep),
        "show" => handle!(ShowParams, handle_show),
        "references" => handle!(ReferencesParams, handle_references),
        "declaration" => handle!(DeclarationParams, handle_declaration),
        "implementations" => handle!(ImplementationsParams, handle_implementations),
        "subtypes" => handle!(SubtypesParams, handle_subtypes),
        "supertypes" => handle!(SupertypesParams, handle_supertypes),
        "calls" => handle!(CallsParams, handle_calls),
        "rename" => handle!(RenameParams, handle_rename),
        "move-file" => handle!(MoveFileParams, handle_move_file),
        "files" => handle!(FilesParams, handle_files),
        "resolve-symbol" => handle!(ResolveSymbolParams, handle_resolve_symbol),
        "add-workspace" => handle!(AddWorkspaceParams, handle_add_workspace),
        "describe-session" => handle!(DescribeSessionParams, handle_describe_session),
        "restart-workspace" => handle!(RestartWorkspaceParams, handle_restart_workspace),
        "remove-workspace" => handle!(RemoveWorkspaceParams, handle_remove_workspace),
        _ => json!({"error": format!("Unknown method: {}", method)}),
    }
}

/// Resolve a symbol name to a file path + line + column.
/// This is the same resolve-symbol call the CLI uses before show/refs/etc.
pub async fn resolve_symbol(symbol: &str, workspace_root: &Path) -> Result<Value, String> {
    let result = dispatch(
        "resolve-symbol",
        json!({
            "workspace_root": workspace_root.to_string_lossy(),
            "symbol_path": symbol,
        }),
    )
    .await;

    if let Some(error) = result.get("error").and_then(|e| e.as_str()) {
        return Err(error.to_string());
    }

    Ok(result
        .get("result")
        .cloned()
        .unwrap_or(Value::Null))
}
