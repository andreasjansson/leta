"""Handler for remove-workspace command."""

from pathlib import Path

from ..rpc import RemoveWorkspaceParams, RemoveWorkspaceResult
from .base import HandlerContext


async def handle_remove_workspace(
    ctx: HandlerContext, params: RemoveWorkspaceParams
) -> RemoveWorkspaceResult:
    workspace_root = Path(params.workspace_root).resolve()
    servers = ctx.session.workspaces.get(workspace_root, {})

    if not servers:
        return RemoveWorkspaceResult(servers_stopped=[])

    stopped: list[str] = []
    for server_name, workspace in list(servers.items()):
        if workspace.client is not None:
            await workspace.stop_server()
        stopped.append(server_name)

    del ctx.session.workspaces[workspace_root]
    return RemoveWorkspaceResult(servers_stopped=stopped)
