"""Handler for references command."""

from ..rpc import ReferencesParams, ReferencesResult, LocationInfo
from .base import HandlerContext


async def handle_references(
    ctx: HandlerContext, params: ReferencesParams
) -> ReferencesResult:
    workspace, doc, path = await ctx.get_workspace_and_document({
        "path": params.path,
        "workspace_root": params.workspace_root,
    })
    line, column = ctx.parse_position({"line": params.line, "column": params.column})
    context = params.context

    result = await workspace.client.send_request(
        "textDocument/references",
        {
            "textDocument": {"uri": doc.uri},
            "position": {"line": line, "character": column},
            "context": {"includeDeclaration": True},
        },
    )

    locations = ctx.format_locations(result, workspace.root, context)
    return ReferencesResult(
        locations=[LocationInfo(**loc) for loc in locations]
    )
