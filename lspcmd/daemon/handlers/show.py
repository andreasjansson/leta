"""Handler for show command."""

from pathlib import Path

from ..rpc import ShowParams, ShowResult
from ...utils.text import read_file_content, get_lines_around
from .base import HandlerContext, find_symbol_at_line, expand_variable_range


async def handle_show(ctx: HandlerContext, params: ShowParams) -> ShowResult:
    body = params.body

    if params.range_start_line is not None:
        return await _handle_direct_definition(ctx, params, body)

    if body:
        return await _handle_definition_body(ctx, params)

    return await _handle_location_only(ctx, params)


async def _handle_direct_definition(
    ctx: HandlerContext, params: ShowParams, body: bool
) -> ShowResult:
    path = Path(params.path).resolve()
    workspace_root = Path(params.workspace_root).resolve()
    line = params.line
    context = params.context
    head = params.head or 200
    symbol_name = params.symbol_name
    symbol_kind = params.symbol_kind

    rel_path = ctx.relative_path(path, workspace_root)
    content = read_file_content(path)
    lines = content.splitlines()

    if body:
        range_start = params.range_start_line
        range_end = params.range_end_line

        if range_start is not None and range_end is not None:
            start = range_start - 1
            end = range_end - 1

            if start == end and symbol_kind in ("Constant", "Variable"):
                end = expand_variable_range(lines, start)
        else:
            workspace = await ctx.session.get_or_create_workspace(path, workspace_root)
            doc = await workspace.ensure_document_open(path)
            result = await workspace.client.send_request(
                "textDocument/documentSymbol",
                {"textDocument": {"uri": doc.uri}},
            )
            if result:
                symbol = find_symbol_at_line(result, line - 1)
                if symbol and "range" in symbol:
                    start = symbol["range"]["start"]["line"]
                    end = symbol["range"]["end"]["line"]
                else:
                    start = end = line - 1
            else:
                start = end = line - 1

        if context > 0:
            start = max(0, start - context)
            end = min(len(lines) - 1, end + context)

        total_lines = end - start + 1
        truncated = total_lines > head
        if truncated:
            end = start + head - 1

        return ShowResult(
            path=rel_path,
            start_line=start + 1,
            end_line=end + 1,
            content="\n".join(lines[start : end + 1]),
            symbol=symbol_name,
            truncated=truncated,
            total_lines=total_lines,
        )
    else:
        if context > 0 and path.exists():
            ctx_lines, start, _ = get_lines_around(content, line - 1, context)
            return ShowResult(
                path=rel_path,
                start_line=start + 1,
                end_line=start + len(ctx_lines),
                content="\n".join(ctx_lines),
                symbol=symbol_name,
            )
        return ShowResult(
            path=rel_path,
            start_line=line,
            end_line=line,
            content=lines[line - 1] if line <= len(lines) else "",
            symbol=symbol_name,
        )


async def _handle_definition_body(ctx: HandlerContext, params: ShowParams) -> ShowResult:
    workspace, doc, path = await ctx.get_workspace_and_document({
        "path": params.path,
        "workspace_root": params.workspace_root,
    })
    line, column = ctx.parse_position({"line": params.line, "column": params.column})
    context = params.context
    head = params.head or 200
    symbol_name = params.symbol_name

    workspace_root = Path(params.workspace_root).resolve()

    result = await workspace.client.send_request(
        "textDocument/definition",
        {
            "textDocument": {"uri": doc.uri},
            "position": {"line": line, "character": column},
        },
    )

    locations = ctx.format_locations(result, workspace.root, 0)
    if not locations:
        raise ValueError("Definition not found")

    loc = locations[0]
    rel_path = loc["path"]
    file_path = workspace_root / rel_path
    target_line = loc["line"] - 1

    workspace2, doc2, _ = await ctx.get_workspace_and_document({
        "path": str(file_path),
        "workspace_root": params.workspace_root,
    })

    symbols_result = await workspace2.client.send_request(
        "textDocument/documentSymbol",
        {"textDocument": {"uri": doc2.uri}},
    )

    content = read_file_content(file_path)
    lines = content.splitlines()

    if symbols_result:
        symbol = find_symbol_at_line(symbols_result, target_line)
        if symbol and "range" in symbol:
            start = symbol["range"]["start"]["line"]
            end = symbol["range"]["end"]["line"]
            if context > 0:
                start = max(0, start - context)
                end = min(len(lines) - 1, end + context)

            total_lines = end - start + 1
            truncated = total_lines > head
            if truncated:
                end = start + head - 1

            return ShowResult(
                path=rel_path,
                start_line=start + 1,
                end_line=end + 1,
                content="\n".join(lines[start : end + 1]),
                symbol=symbol_name,
                truncated=truncated,
                total_lines=total_lines,
            )
        else:
            raise ValueError("Language server does not provide symbol ranges")

    return ShowResult(
        path=rel_path,
        start_line=loc["line"],
        end_line=loc["line"],
        content=lines[target_line] if target_line < len(lines) else "",
        symbol=symbol_name,
    )


async def _handle_location_only(ctx: HandlerContext, params: ShowParams) -> ShowResult:
    workspace, doc, path = await ctx.get_workspace_and_document({
        "path": params.path,
        "workspace_root": params.workspace_root,
    })
    line, column = ctx.parse_position({"line": params.line, "column": params.column})
    context = params.context

    result = await workspace.client.send_request(
        "textDocument/definition",
        {
            "textDocument": {"uri": doc.uri},
            "position": {"line": line, "character": column},
        },
    )

    locations = ctx.format_locations(result, workspace.root, context)
    if not locations:
        raise ValueError("Definition not found")

    loc = locations[0]
    workspace_root = Path(params.workspace_root).resolve()
    file_path = workspace_root / loc["path"]
    content = read_file_content(file_path)
    lines = content.splitlines()

    if "context_lines" in loc:
        return ShowResult(
            path=loc["path"],
            start_line=loc["context_start"],
            end_line=loc["context_start"] + len(loc["context_lines"]) - 1,
            content="\n".join(loc["context_lines"]),
        )

    return ShowResult(
        path=loc["path"],
        start_line=loc["line"],
        end_line=loc["line"],
        content=lines[loc["line"] - 1] if loc["line"] <= len(lines) else "",
    )
