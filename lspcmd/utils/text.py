from pathlib import Path

LANGUAGE_IDS = {
    ".py": "python",
    ".pyi": "python",
    ".js": "javascript",
    ".jsx": "javascriptreact",
    ".ts": "typescript",
    ".tsx": "typescriptreact",
    ".rs": "rust",
    ".go": "go",
    ".c": "c",
    ".h": "c",
    ".cpp": "cpp",
    ".cc": "cpp",
    ".cxx": "cpp",
    ".hpp": "cpp",
    ".hxx": "cpp",
    ".java": "java",
    ".rb": "ruby",
    ".php": "php",
    ".cs": "csharp",
    ".fs": "fsharp",
    ".swift": "swift",
    ".kt": "kotlin",
    ".kts": "kotlin",
    ".scala": "scala",
    ".lua": "lua",
    ".sh": "shellscript",
    ".bash": "shellscript",
    ".zsh": "shellscript",
    ".json": "json",
    ".yaml": "yaml",
    ".yml": "yaml",
    ".toml": "toml",
    ".xml": "xml",
    ".html": "html",
    ".htm": "html",
    ".css": "css",
    ".scss": "scss",
    ".less": "less",
    ".md": "markdown",
    ".markdown": "markdown",
    ".sql": "sql",
    ".r": "r",
    ".R": "r",
    ".el": "emacs-lisp",
    ".clj": "clojure",
    ".cljs": "clojurescript",
    ".ex": "elixir",
    ".exs": "elixir",
    ".erl": "erlang",
    ".hrl": "erlang",
    ".hs": "haskell",
    ".ml": "ocaml",
    ".mli": "ocaml",
    ".vim": "vim",
    ".zig": "zig",
    ".nim": "nim",
    ".d": "d",
    ".dart": "dart",
    ".v": "v",
    ".vue": "vue",
    ".svelte": "svelte",
}


def get_language_id(path: str | Path) -> str:
    path = Path(path)
    return LANGUAGE_IDS.get(path.suffix, "plaintext")


def read_file_content(path: str | Path) -> str:
    return Path(path).read_text()


def get_line_at(content: str, line: int) -> str:
    lines = content.splitlines()
    if 0 <= line < len(lines):
        return lines[line]
    return ""


def get_lines_around(content: str, line: int, context: int) -> tuple[list[str], int, int]:
    lines = content.splitlines()
    start = max(0, line - context)
    end = min(len(lines), line + context + 1)
    return lines[start:end], start, end


def position_to_offset(content: str, line: int, character: int) -> int:
    lines = content.splitlines(keepends=True)
    offset = 0
    for i, ln in enumerate(lines):
        if i == line:
            return offset + character
        offset += len(ln)
    return offset


def offset_to_position(content: str, offset: int) -> tuple[int, int]:
    lines = content.splitlines(keepends=True)
    current = 0
    for i, ln in enumerate(lines):
        if current + len(ln) > offset:
            return i, offset - current
        current += len(ln)
    return len(lines), 0
