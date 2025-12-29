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
├── __main__.py              # Entry point: python -m lspcmd
├── cli.py                   # Click CLI definitions
├── daemon/
│   ├── __init__.py
│   ├── server.py            # Daemon server (Unix socket or TCP)
│   ├── manager.py           # Language server lifecycle management
│   ├── session.py           # Session state (open docs, workspaces)
│   └── pidfile.py           # PID file management
├── lsp/
│   ├── __init__.py
│   ├── protocol.py          # LSP base protocol (JSON-RPC over stdio)
│   ├── client.py            # LSP client implementation
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

## Core Components

### 1. CLI Layer (`cli.py`)

Uses Click to implement commands. Each command:
1. Ensures the daemon is running (spawns if needed)
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
def find_definition(path, position, context):
    """Find definition of symbol at position."""
    ...
```

### 2. Daemon (`daemon/`)

The daemon runs as a background process:
- **PID file**: Stored in `~/.cache/lspcmd/lspcmd.pid`
- **Socket**: Unix socket at `~/.cache/lspcmd/lspcmd.sock`
- **Protocol**: JSON-RPC over the socket for CLI-to-daemon communication

The daemon:
- Spawns and manages language server processes
- Keeps servers running between CLI invocations (performance)
- Maintains document state (open documents, versions)
- Handles workspace folder management
- Auto-shuts down after configurable idle timeout

#### Daemon Lifecycle
1. First CLI command checks for pidfile
2. If no daemon or stale pid, spawn new daemon (daemonize)
3. CLI connects via Unix socket
4. CLI sends request, waits for response
5. Daemon keeps running, managing servers

### 3. LSP Client (`lsp/`)

Implements the LSP base protocol:

```python
class LSPClient:
    """Low-level LSP client over stdio."""
    
    def __init__(self, process: subprocess.Popen):
        self.process = process
        self.request_id = 0
        self._pending_requests = {}
    
    def send_request(self, method: str, params: dict) -> dict:
        """Send request and wait for response."""
        ...
    
    def send_notification(self, method: str, params: dict) -> None:
        """Send notification (no response expected)."""
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
All commands use `LINE,COLUMN` format (1-based, matching editor conventions).
Internally converted to 0-based for LSP.

### Command Details

#### `lspcmd describe-session`
Shows current daemon state: workspaces, servers, open documents.

#### `lspcmd describe-thing-at-point PATH LINE,COLUMN`
- Opens document if needed (didOpen)
- Sends `textDocument/hover` request
- Returns hover info (type, documentation)

#### `lspcmd list-code-actions PATH LINE,COLUMN`
- Sends `textDocument/codeAction` request
- Lists available actions with their titles

#### `lspcmd execute-code-action PATH LINE,COLUMN CODE_ACTION_NAME`
- Gets code actions, finds matching one
- If action has `edit`, applies it
- If action has `command`, executes via `workspace/executeCommand`

#### `lspcmd find-declaration PATH LINE,COLUMN [--context=N]`
- Sends `textDocument/declaration` request
- Outputs `filename:line` (or with context lines)

#### `lspcmd find-definition PATH LINE,COLUMN [--context=N]`
- Sends `textDocument/definition` request
- Outputs `filename:line` (or with context lines)

#### `lspcmd find-implementation PATH LINE,COLUMN [--context=N]`
- Sends `textDocument/implementation` request
- Outputs `filename:line` (or with context lines)

#### `lspcmd print-definition PATH LINE,COLUMN`
- Gets definition location
- Reads the file
- For functions: finds the full function body (uses document symbols)
- For variables: prints the line

#### `lspcmd find-references PATH LINE,COLUMN`
- Sends `textDocument/references` request
- Lists all reference locations

#### `lspcmd find-type-definition PATH LINE,COLUMN [--context=N]`
- Sends `textDocument/typeDefinition` request
- Outputs type location with optional context

#### `lspcmd format-buffer PATH`
- Sends `textDocument/formatting` request
- Applies text edits to file
- Writes formatted file back

#### `lspcmd organize-imports PATH`
- Sends `textDocument/codeAction` with kind `source.organizeImports`
- Executes the action

#### `lspcmd rename PATH LINE,COLUMN NEW_NAME`
- Sends `textDocument/rename` request
- Applies workspace edit (may affect multiple files)

#### `lspcmd restart-workspace`
- Shuts down all servers in workspace
- Re-initializes them

#### `lspcmd list-signatures [PATH]`
- If PATH: sends `textDocument/documentSymbol`, filters to functions
- If no PATH: sends `workspace/symbol` with empty query, filters

#### `lspcmd list-symbols [PATH]`
- If PATH: sends `textDocument/documentSymbol`
- If no PATH: sends `workspace/symbol`
- Outputs `filename:line: kind name`

#### `lspcmd search-symbol REGEX [PATH]`
- Gets symbols (document or workspace)
- Filters by regex
- Outputs matching symbols

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
    def install(self, server: ServerConfig) -> bool:
        """Install server if not present."""
        if self.is_installed(server):
            return True
        
        # Run installation command
        if server.install_type == 'npm':
            subprocess.run(['npm', 'install', '-g', server.package])
        elif server.install_type == 'pip':
            subprocess.run(['pip', 'install', server.package])
        # etc.
```

Installation directory: `~/.local/share/lspcmd/servers/`

## Configuration

Config file: `~/.config/lspcmd/config.toml`

```toml
[daemon]
idle_timeout = 300  # seconds
socket_path = "~/.cache/lspcmd/lspcmd.sock"

[servers.python]
preferred = "pyright"  # or "pylsp", "ruff-lsp"
auto_install = true

[servers.rust]
preferred = "rust-analyzer"

[formatting]
tab_size = 4
insert_spaces = true
```

## Error Handling

- Server not installed → prompt to install or error with instructions
- Server crashes → restart, report error
- Request timeout → configurable, default 30s
- File not found → clear error message
- Position out of range → error with file info

## Dependencies

```toml
[project]
dependencies = [
    "click>=8.0",        # CLI framework
    "pydantic>=2.0",     # Data validation
]
```

## Testing Strategy

1. **Unit tests**: Protocol encoding/decoding, URI handling
2. **Integration tests**: Start real servers, make requests
3. **Mock tests**: Mock server responses for edge cases

## Implementation Order

### Phase 1: Core Infrastructure
1. LSP protocol implementation (`lsp/protocol.py`, `lsp/types.py`)
2. Basic LSP client (`lsp/client.py`)
3. PID file and daemon basics (`daemon/pidfile.py`, `daemon/server.py`)

### Phase 2: Server Management
1. Server registry and detection (`servers/registry.py`)
2. Server installation (`servers/installer.py`)
3. Session management (`daemon/session.py`)

### Phase 3: CLI Commands
1. Basic CLI structure with Click
2. Implement `describe-session`
3. Implement `find-definition` (simplest location command)
4. Implement remaining location commands
5. Implement `format-buffer`, `rename`
6. Implement code actions
7. Implement symbol listing

### Phase 4: Polish
1. Configuration system
2. Better error messages
3. Documentation
4. Tests

## Open Questions

1. **Multi-root workspaces**: How to detect workspace root? Use git root, or specific project files?
   - Proposal: Check for `.git`, `pyproject.toml`, `Cargo.toml`, `package.json`, etc.

2. **Concurrent requests**: Should daemon handle multiple CLI connections simultaneously?
   - Proposal: Yes, using asyncio

3. **Log output**: Where to log daemon activity?
   - Proposal: `~/.cache/lspcmd/daemon.log`

4. **Server output**: What to do with server stderr/stdout?
   - Proposal: Log to per-server log files in `~/.cache/lspcmd/servers/`
