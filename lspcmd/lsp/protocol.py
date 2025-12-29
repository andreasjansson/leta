import json
import asyncio
from dataclasses import dataclass
from typing import Any


@dataclass
class JsonRpcRequest:
    method: str
    params: dict[str, Any] | list[Any] | None
    id: int | str | None = None


@dataclass
class JsonRpcResponse:
    id: int | str | None
    result: Any = None
    error: dict[str, Any] | None = None


@dataclass
class JsonRpcNotification:
    method: str
    params: dict[str, Any] | list[Any] | None = None


class LSPProtocolError(Exception):
    pass


class LSPResponseError(Exception):
    def __init__(self, code: int, message: str, data: Any = None):
        self.code = code
        self.message = message
        self.data = data
        super().__init__(f"LSP Error {code}: {message}")


class LanguageServerNotFound(Exception):
    def __init__(self, server_name: str, language: str, install_cmd: str | None = None):
        self.server_name = server_name
        self.language = language
        self.install_cmd = install_cmd
        if install_cmd:
            msg = f"Language server '{server_name}' for {language} not found. Install with: {install_cmd}"
        else:
            msg = f"Language server '{server_name}' for {language} not found"
        super().__init__(msg)


class LanguageServerStartupError(Exception):
    def __init__(self, server_name: str, language: str, workspace_root: str, original_error: Exception):
        self.server_name = server_name
        self.language = language
        self.workspace_root = workspace_root
        self.original_error = original_error
        
        msg = (
            f"Language server '{server_name}' failed to start for {language} files in {workspace_root}\n"
            f"\n"
            f"Error: {original_error}\n"
            f"\n"
            f"Possible causes:\n"
            f"  - The project may not be a valid {language} project (missing config files)\n"
            f"  - The language server may have crashed or timed out\n"
            f"  - Try running '{server_name}' directly in that directory to see detailed errors\n"
            f"\n"
            f"To exclude these files, use: lspcmd grep PATTERN 'your/path/*.ext' -x 'path/to/exclude/*'"
        )
        super().__init__(msg)


def encode_message(obj: dict[str, Any]) -> bytes:
    content = json.dumps(obj).encode("utf-8")
    header = f"Content-Length: {len(content)}\r\n\r\n".encode("ascii")
    return header + content


async def read_message(reader: asyncio.StreamReader) -> dict[str, Any]:
    headers: dict[str, str] = {}

    while True:
        line = await reader.readline()
        if not line:
            raise LSPProtocolError("Connection closed")

        line_str = line.decode("ascii").strip()
        if not line_str:
            break

        if ":" in line_str:
            key, value = line_str.split(":", 1)
            headers[key.strip()] = value.strip()

    if "Content-Length" not in headers:
        raise LSPProtocolError("Missing Content-Length header")

    content_length = int(headers["Content-Length"])
    content = await reader.readexactly(content_length)

    return json.loads(content.decode("utf-8"))
