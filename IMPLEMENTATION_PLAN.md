# lspcmd Implementation Plan

A command-line wrapper around LSP language servers, inspired by lsp-mode.el.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                           lspcmd CLI                                │
│  (User-facing commands: find-definition, rename, format-buffer...)  │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         lspcmd Daemon                               │
│  - Manages language server processes                                │
│  - Maintains open documents state                                   │
│  - Handles LSP initialization/shutdown                              │
│  - Caches workspace state                                           │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                    ┌─────────────┴─────────────┐
                    ▼                           ▼
           ┌───────────────┐           ┌───────────────┐
           │  pyright LSP  │           │  rust-analyzer│
           │    Server     │           │    Server     │
           └───────────────┘           └───────────────┘
```

## Project Structure

```
lspcmd/
├── __init__.py
├── cli.py                   # Click CLI definitions for `lspcmd`
├── daemon_cli.py            # Click CLI for `lspcmd-daemon`
├── daemon/
│   ├── __init__.py
│   ├── server.py            # Async daemon server (Unix socket)
│   ├── manager.py           # Language server lifecycle management
│   ├── session.py           # Session state (open docs, workspaces)
│   └── pidfile.py           # PID file management
├── lsp/
│   ├── __init__.py
│   ├── protocol.py          # LSP base protocol (JSON-RPC over stdio)
│   ├── client.py            # Async LSP client implementation
│   ├── types.py             # LSP type definitions (Position, Range, etc.)
│   ├── capabilities.py      # Client/Server capabilities
│   └── requests.py          # Higher-level LSP request helpers
├── servers/
│   ├── __init__.py
│   ├── registry.py          # Server registry & detection
│   ├── installer.py         # Automatic server installation
│   └── configs/             # Server configurations
│       ├── __init__.py
│       ├── python.py        # pyright, pylsp, ruff-lsp
│       ├── typescript.py    # typescript-language-server
│       ├── rust.py          # rust-analyzer
│       ├── go.py            # gopls
│       └── ...
├── utils/
│   ├── __init__.py
│   ├── uri.py               # File URI handling
│   ├── text.py              # Text document utilities
│   └── config.py            # Configuration management
└── output/
    ├── __init__.py
    └── formatters.py        # Output formatting (plain, json, etc.)
```

## Entry Points

Defined in `pyproject.toml` (or `setup.py`):

```toml
[project.scripts]
lspcmd = "lspcmd.cli:cli"
lspcmd-daemon = "lspcmd.daemon_cli:main"
```

## Core Components

### 1. CLI Layer (`cli.py`)

Uses Click to implement commands. Each command:
1. Ensures the daemon is running (spawns `lspcmd-daemon` if needed)
2. Connects to daemon via Unix socket
3. Sends request and receives response
4. Formats and outputs result

```python
import click

@click.group()
def cli():
    """LSP command-line interface."""
    pass

@cli.command()
@click.argument('path')
@click.argument('position')  # LINE,COLUMN format
@click.option('--context', '-n', default=0, help='Lines of context')
async def find_definition(path, position, context):
    """Find definition of symbol at position."""
    ...

@cli.command()
async def shutdown():
    """Shutdown the lspcmd daemon."""
    ...

@cli.command()
async def config():
    """Print config file location and contents."""
    ...
```

### 2. Daemon (`daemon/`)

The daemon runs as a background process:
- **PID file**: Stored in `~/.cache/lspcmd/lspcmd.pid`
- **Socket**: Unix socket at `~/.cache/lspcmd/lspcmd.sock`
- **Protocol**: JSON-RPC over the socket for CLI-to-daemon communication
- **Logs**: `~/.cache/lspcmd/log/daemon.log`

The daemon:
- Spawns and manages language server processes
- Keeps servers running between CLI invocations (performance)
- Maintains document state (open documents, versions)
- Handles workspace folder management
- Runs forever until explicitly stopped via `lspcmd shutdown`

#### Daemon Lifecycle
1. First CLI command checks for pidfile
2. If no daemon or stale pid, spawn new daemon (`lspcmd-daemon`)
3. CLI connects via Unix socket
4. CLI sends request, waits for response
5. Daemon keeps running indefinitely, managing servers
6. User runs `lspcmd shutdown` to stop daemon

### 3. LSP Client (`lsp/`)

Implements the LSP base protocol using asyncio:

```python
class LSPClient:
    """Async LSP client over stdio."""
    
    def __init__(self, process: asyncio.subprocess.Process):
        self.process = process
        self.request_id = 0
        self._pending_requests: dict[int, asyncio.Future] = {}
    
    async def send_request(self, method: str, params: dict) -> dict:
        """Send request and wait for response."""
        ...
    
    async def send_notification(self, method: str, params: dict) -> None:
        """Send notification (no response expected)."""
        ...
    
    async def _read_loop(self) -> None:
        """Background task to read responses and notifications."""
        ...
```

Key protocol details (from LSP spec):
- Messages have `Content-Length` header
- Body is JSON-RPC 2.0
- Requests have `id`, notifications don't
- Server can send notifications/requests to client

### 4. Server Registry (`servers/`)

Maps languages to their LSP servers:

```python
SERVERS = {
    'python': [
        ServerConfig(
            name='pyright',
            command=['pyright-langserver', '--stdio'],
            install_cmd='npm install -g pyright',
            languages=['python'],
            file_patterns=['*.py'],
        ),
        ServerConfig(
            name='pylsp',
            command=['pylsp'],
            install_cmd='pip install python-lsp-server',
            languages=['python'],
            file_patterns=['*.py'],
        ),
    ],
    'rust': [
        ServerConfig(
            name='rust-analyzer',
            command=['rust-analyzer'],
            install_cmd='rustup component add rust-analyzer',
            languages=['rust'],
            file_patterns=['*.rs'],
        ),
    ],
    # ... more
}
```

Server detection:
1. Check if server binary exists in PATH
2. If not and auto-install enabled, run install command
3. Use configuration to pick preferred server for language

### 5. Session State (`daemon/session.py`)

Tracks:
- Open workspaces (project roots)
- Open documents (with version numbers)
- Server capabilities per workspace
- Cached symbols/diagnostics

```python
@dataclass
class Session:
    workspaces: dict[Path, Workspace]  # root -> Workspace
    
@dataclass  
class Workspace:
    root: Path
    server: LSPClient
    open_documents: dict[str, OpenDocument]  # uri -> doc
    capabilities: ServerCapabilities
```

## Commands Implementation

### Position Format

All commands use `LINE,COLUMN` format where:
- **LINE** is 1-based (like Emacs `line-number-at-pos`)
- **COLUMN** is 0-based (like Emacs `current-column`)

Internally converted for LSP (both 0-based).

### Command Details

#### `lspcmd describe-session`
*Inspired by [lsp-mode.el:9150](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L9150)*

Shows current daemon state: workspaces, servers, open documents.

#### `lspcmd describe-thing-at-point PATH LINE,COLUMN`
*Inspired by [lsp-mode.el:5526](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L5526)*

- Opens document if needed (didOpen)
- Sends `textDocument/hover` request
- Returns hover info (type, documentation)

#### `lspcmd list-code-actions PATH LINE,COLUMN`
*Inspired by [lsp-mode.el:6157](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6157)*

- Sends `textDocument/codeAction` request
- Lists available actions with their titles

#### `lspcmd execute-code-action PATH LINE,COLUMN CODE_ACTION_NAME`
*Inspired by [lsp-mode.el:6188](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6188)*

- Gets code actions, finds matching one
- If action has `edit`, applies it
- If action has `command`, executes via `workspace/executeCommand`

#### `lspcmd find-declaration PATH LINE,COLUMN [--context=N]`
*Inspired by [lsp-mode.el:6751](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6751)*

- Sends `textDocument/declaration` request
- Outputs `filename:line` (or with context lines)

#### `lspcmd find-definition PATH LINE,COLUMN [--context=N]`
*Inspired by [lsp-mode.el:6756](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6756)*

- Sends `textDocument/definition` request
- Outputs `filename:line` (or with context lines)

#### `lspcmd find-implementation PATH LINE,COLUMN [--context=N]`
*Inspired by [lsp-mode.el:6771](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6771)*

- Sends `textDocument/implementation` request
- Outputs `filename:line` (or with context lines)

#### `lspcmd print-definition PATH LINE,COLUMN`
*Inspired by [lsp-mode.el:6756](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6756)*

- Gets definition location
- Reads the file
- For functions: finds the full function body (uses document symbols)
- For variables: prints the line

#### `lspcmd find-references PATH LINE,COLUMN`
*Inspired by [lsp-mode.el:6779](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6779)*

- Sends `textDocument/references` request
- Lists all reference locations

#### `lspcmd find-type-definition PATH LINE,COLUMN [--context=N]`
*Inspired by [lsp-mode.el:6787](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6787)*

- Sends `textDocument/typeDefinition` request
- Outputs type location with optional context

#### `lspcmd format-buffer PATH`
*Inspired by [lsp-mode.el:6330](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6330)*

- Sends `textDocument/formatting` request
- Applies text edits to file
- Writes formatted file back

#### `lspcmd organize-imports PATH`
*Inspired by [lsp-mode.el:6373](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6373)*

- Sends `textDocument/codeAction` with kind `source.organizeImports`
- Executes the action

#### `lspcmd rename PATH LINE,COLUMN NEW_NAME`
*Inspired by [lsp-mode.el:6684](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L6684)*

- Sends `textDocument/rename` request
- Applies workspace edit (may affect multiple files)

#### `lspcmd restart-workspace`
*Inspired by [lsp-mode.el:9478](https://github.com/emacs-lsp/lsp-mode/blob/master/lsp-mode.el#L9478)*

- Shuts down all servers in workspace
- Re-initializes them

#### `lspcmd list-signatures [PATH]`
- If PATH: sends `textDocument/documentSymbol`, filters to functions
- If no PATH: sends `workspace/symbol` with empty query, filters
- Outputs `filename:line: signature`

#### `lspcmd list-symbols [PATH]`
- If PATH: sends `textDocument/documentSymbol`
- If no PATH: sends `workspace/symbol`
- Outputs `filename:line: kind name`

#### `lspcmd search-symbol REGEX [PATH]`
- Gets symbols (document or workspace)
- Filters by regex
- Outputs matching symbols

#### `lspcmd shutdown`
- Connects to daemon
- Sends shutdown request
- Daemon terminates gracefully (closes all servers)

#### `lspcmd config`
- Prints path to config file
- Dumps config file contents to stdout

## Document Synchronization

The daemon must keep documents in sync with disk:
1. On first access to a file, send `textDocument/didOpen`
2. Track document versions
3. Before operations, check if file changed and send `textDocument/didChange` if needed
4. Use incremental sync if supported, full sync otherwise

For CLI use (files not actively edited):
- Simple approach: always read file from disk, send full content
- Server sees current disk state

## Server Installation

Following lsp-mode pattern:

```python
class ServerInstaller:
    async def install(self, server: ServerConfig) -> bool:
        """Install server if not present."""
        if await self.is_installed(server):
            return True
        
        # Run installation command
        if server.install_type == 'npm':
            await asyncio.create_subprocess_exec('npm', 'install', '-g', server.package)
        elif server.install_type == 'pip':
            await asyncio.create_subprocess_exec('pip', 'install', server.package)
        # etc.
```

Installation directory: `~/.local/share/lspcmd/servers/`

## Configuration

Config file: `~/.config/lspcmd/config.toml`

```toml
[daemon]
log_level = "info"  # debug, info, warning, error

[workspaces]
# Saved workspace roots (auto-populated when user confirms)
roots = [
    "/home/user/projects/myproject",
    "/home/user/projects/another",
]

[servers.python]
preferred = "pyright"  # or "pylsp", "ruff-lsp"
auto_install = true

[servers.rust]
preferred = "rust-analyzer"

[formatting]
tab_size = 4
insert_spaces = true
```

### Workspace Root Detection

When a command targets a file, lspcmd determines the workspace root:

1. Check if file is under a known workspace root (from config)
2. If not, detect using heuristics:
   - Look for `.git`, `pyproject.toml`, `Cargo.toml`, `package.json`, `go.mod`, etc.
   - Walk up directory tree until found or hit filesystem root
3. Prompt user to confirm detected root (with default)
4. Save confirmed root to config file for future use

```
$ lspcmd find-definition src/foo.py 42,10
Workspace root not configured for this file.
Detected: /home/user/projects/myproject (contains .git, pyproject.toml)
Use this root? [Y/n]: 
Saved to ~/.config/lspcmd/config.toml
```

## Logging

### Daemon Logs
- Location: `~/.cache/lspcmd/log/daemon.log`
- Verbosity controlled by `daemon.log_level` in config
- Levels: debug, info, warning, error

### Server Logs
- Location: `~/.cache/lspcmd/log/{server_name}/stdout.log` and `stderr.log`
- Example: `~/.cache/lspcmd/log/pyright/stdout.log`
- Captures all output from language server processes

## Error Handling

- Server not installed → prompt to install or error with instructions
- Server crashes → restart, report error
- Request timeout → configurable, default 30s
- File not found → clear error message
- Position out of range → error with file info
- Daemon not running → auto-start or clear error

## Dependencies

```toml
[project]
dependencies = [
    "click>=8.0",        # CLI framework
    "pydantic>=2.0",     # Data validation
    "toml>=0.10",        # Config file parsing
]
```

## Async Architecture

Almost all functions should be async to support:
- Concurrent CLI connections to daemon
- Parallel requests to multiple language servers
- Non-blocking I/O for server communication
- Responsive daemon even during long operations

```python
# Example async flow
async def handle_find_definition(path: str, line: int, col: int) -> list[Location]:
    workspace = await session.get_or_create_workspace(path)
    await workspace.ensure_document_open(path)
    
    result = await workspace.server.send_request(
        "textDocument/definition",
        {
            "textDocument": {"uri": path_to_uri(path)},
            "position": {"line": line - 1, "character": col},
        }
    )
    
    return parse_locations(result)
```

## Testing Strategy

1. **Unit tests**: Protocol encoding/decoding, URI handling
2. **Integration tests**: Start real servers, make requests
3. **Mock tests**: Mock server responses for edge cases

## Implementation Order

### Phase 1: Core Infrastructure
1. LSP protocol implementation (`lsp/protocol.py`, `lsp/types.py`)
2. Async LSP client (`lsp/client.py`)
3. PID file and daemon basics (`daemon/pidfile.py`, `daemon/server.py`)

### Phase 2: Server Management
1. Server registry and detection (`servers/registry.py`)
2. Server installation (`servers/installer.py`)
3. Session management (`daemon/session.py`)

### Phase 3: CLI Commands
1. Basic CLI structure with Click
2. Implement `config` and `shutdown`
3. Implement `describe-session`
4. Implement `find-definition` (simplest location command)
5. Implement remaining location commands
6. Implement `format-buffer`, `rename`
7. Implement code actions
8. Implement symbol listing

### Phase 4: Polish
1. Configuration system with workspace root prompting
2. Better error messages
3. Documentation
4. Tests
