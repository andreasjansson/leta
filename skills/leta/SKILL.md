---
name: leta
description: "Fast semantic code navigation via LSP. Load FIRST before ANY code task - even 'simple' ones. Trigger scenarios: (1) fixing lint/type/pyright/mypy warnings or errors, (2) fixing reportAny/reportUnknownType/Any type errors, (3) adding type annotations, (4) refactoring or modifying code, (5) finding where a function/class/symbol is defined, (6) finding where a symbol is used/referenced/imported, (7) understanding what a function calls or what calls it, (8) exploring unfamiliar code or understanding architecture, (9) renaming symbols across codebase, (10) finding interface/protocol implementations, (11) ANY task where you'd use ripgrep to find code or read-file to view a function. Use `leta show SYMBOL` instead of read-file, `leta refs SYMBOL` instead of ripgrep for usages, `leta grep PATTERN` instead of ripgrep for definitions, `leta graph` for full project architecture overview, `leta files` instead of list-directory."
---

# Leta - LSP Enabled Tools for Agents

Leta provides fast semantic code navigation using Language Server Protocol. Unlike text-based search tools, Leta understands code structure and can find symbol definitions, references, implementations, and more.

## ⚠️ STOP AND THINK - Default to leta

After loading this skill, **leta should be your DEFAULT tool for code exploration**, not ripgrep-like tools or file reading.

**Before you act, check this list:**

| If you're about to... | STOP! Instead use... |
|----------------------|---------------------|
| Use `read-file` to view a function/class you know the name of | `leta show <symbol_name>` |
| Use `read-file` with specific start and end line ranges in order to view a specific function | `leta show <symbol_name>` |
| Use `read-file` to "browse" or "understand" a file | `leta grep ".*" "path/to/file" -k function,method` to list functions, or `leta show <symbol>` |
| Use ripgrep-like tools to find where a function is defined | `leta grep "<function_name>" -k function,method` |
| Use ripgrep-like tools to find usages/references of a symbol | `leta refs <symbol_name>` |
| Use ripgrep-like tools to see where a function is imported/wired up | `leta refs <symbol_name>` |
| Use ripgrep-like tools to find code related to a concept (e.g. "billing") | `leta grep "<concept>" -k function,method` |
| Use `list-directory` to explore project structure | `leta files` |
| Manually search for interface implementations | `leta implementations <interface>` |
| Grep for function calls to trace code flow | `leta calls --to/--from <function>` |
| Read a function's implementation to understand what it depends on | `leta calls --from <function>` first for overview |
| Read multiple files to understand how functions connect | `leta graph` for the full picture, or `leta calls --from <function>` for a specific function |
| Trying to understand an unfamiliar codebase | `leta graph` first for architecture overview |

**The Golden Rule:** If you know the symbol name, **always** use leta. Only use ripgrep when searching for things that aren't symbols (string literals, comments, config values).

**DON'T fall back to old habits.** If you know a symbol name, use leta.

### ⚠️ Anti-pattern: "Browsing" Files

**Don't** read a whole file just "to understand it" or "see the context." This is a common mistake.

If you're tempted to do this, ask yourself: *What symbol am I actually looking for?* Then use:
- `leta show <symbol>` if you know the symbol name
- `leta grep ".*" path/to/file -k function,method` to see what functions exist in a file
- `leta refs <symbol>` to find where something is used

### ⚠️ Anti-pattern: "I want to see where this is imported/wired up"

When you think "I want to see where this function is imported" or "how is this view wired into URL routes" - that's just finding references! Don't use ripgrep to search for the function name. Use:
- `leta refs <function_name>` - shows imports, URL configs, everywhere it's used

## When to Use leta vs ripgrep-like tools

**Use leta for:**
- Finding where a function/class/method is DEFINED
- Finding all USAGES of a symbol
- Understanding call hierarchies (what calls what)
- Getting a full architectural overview of a project
- Finding interface implementations
- Semantic refactoring (rename symbol across codebase)
- Exploring project structure with symbol information
- Viewing a symbol's implementation when you know its name

**Use ripgrep-like tools for:**
- Searching for **literal strings in comments, docs, or config files** (not code)
- Multi-word phrase search in non-code content
- Searching for library/external symbols not defined in your code
- Pattern matching in string literals or configuration
- Searching in file types leta doesn't understand (markdown, yaml, etc.)

**Don't use ripgrep-like tools for:**
- Finding where a function/class is defined → use `leta grep`
- Finding where a symbol is used → use `leta refs`
- Finding code related to a concept (e.g. "billing", "auth") → use `leta grep "<concept>" -k function,method`

## Quick Start

Before using leta on a new project, add it as a workspace:

```bash
leta workspace add /path/to/project
```

## Core Commands

### `leta graph` - Full Workspace Call Graph ⭐ START HERE

**Run this first when exploring an unfamiliar project.** Shows the complete call graph as trees rooted at entry points (functions nothing else calls). This gives you a token-efficient architectural overview of the entire codebase — which functions exist, how they connect, and where the entry points are.

```bash
# Full call graph (excludes test files by default)
leta graph

# Include stdlib/dependency calls
leta graph --include-non-workspace

# Only show Go files
leta graph -i '\.go$'

# Exclude generated code
leta graph -x generated -x proto

# Include test files
leta graph --include-tests
```

The output shows trees from entry points, with `↑` marking nodes whose subtree was already shown earlier, and `↻` marking recursive calls. For multi-language workspaces, results are grouped by language with the largest subgraph first.

**When to use `graph`:**
- You're starting work on an unfamiliar codebase — run this first
- You want to understand the overall architecture and data flow
- You need to see how all the pieces fit together
- You want to find entry points and dead code

### `leta show` - View Symbol Definition ⭐ USE THIS INSTEAD OF READ-FILE

**This is the killer feature you should use constantly.** Print the full body of a function, class, or method. ALWAYS use this instead of `read-file` when you know the symbol name.

```bash
# Show a function
leta show handle_request

# Show a method on a class
leta show UserRepository.add_user

# Show with surrounding context
leta show parse_config -n 5

# Limit output length
leta show COUNTRY_CODES --head 50
```

**Symbol formats:**
- `SymbolName` - Find by name
- `Parent.Symbol` - Qualified name (Class.method)
- `path:Symbol` - Filter by file path
- `path:Parent.Symbol` - Path + qualified name

### `leta grep` - Find Symbol Definitions

Search for symbols matching a regex pattern. Only searches symbol NAMES, not file contents. Use this instead of ripgrep-like tools when looking for where something is defined.

```bash
leta grep PATTERN [PATH_REGEX] [OPTIONS]
```

The optional PATH_REGEX argument filters files by matching a regex against the relative file path.

```bash
# Find all functions/methods starting with "test"
leta grep "^test" -k function,method

# Find a class
leta grep "UserRepository" -k class

# Find all methods in a specific file
leta grep ".*" "server\.py" -k method

# Find symbols in Python files only
leta grep "validate" '\.py$' -k function

# Find symbols in a specific directory
leta grep "User" "models/"

# Find symbols in test files
leta grep "test" "test/"

# Find public Go functions (capitalized)
leta grep "^[A-Z]" '\.go$' -k function -C
```

**Options:**
- `-k, --kind TEXT` - Filter by kind: class, function, method, variable, constant, interface, struct, enum, property, field, constructor, module, namespace, package, typeparameter
- `-x, --exclude TEXT` - Exclude files matching regex (repeatable)
- `-C, --case-sensitive` - Case-sensitive matching. Note that `leta grep` is case-insensitive by default
- `--head N` - Maximum results to return (default: 200)

### `leta files` - Project File Tree

Show source file tree with line counts. Use for understanding the file layout when `leta graph` gives you the architecture. **Always prefer `leta files` over `list-directory`-like tools** since it prints not just the filenames, but a full tree of files (excluding `.git`, `__pycache__`, etc.), and their sizes and line counts.

```bash
# Overview of entire project
leta files

# Only show src/ directory
leta files src/

# Exclude test directories (regex pattern)
leta files -x test -x vendor

# Only show Python files
leta files -f '\.py$'
```

### `leta refs` - Find All References ⭐ USE THIS INSTEAD OF RIPGREP FOR USAGES

**This is the correct way to find where a symbol is used.** Don't use ripgrep to search for a function name - use `leta refs` instead. It understands code structure and won't give you false positives from comments or similarly-named symbols.

```bash
# Find all usages of a function
leta refs validate_email

# Find with context lines
leta refs UserRepository.save -n 2

# Find where a class is instantiated or referenced
leta refs UserRepository
```

### `leta calls` - Call Hierarchy

**Use this to trace a specific function's dependencies or callers.** For a full project overview, use `leta graph` instead.

```bash
# What does main() call? (understand dependencies before reading code)
leta calls --from main

# What calls validate_email()? (find all callers)
leta calls --to validate_email

# Find path from one function to another
leta calls --from main --to save_to_db

# Include stdlib/dependency calls
leta calls --from process_request --include-non-workspace
```

**When to use `calls` vs `graph`:**
- Use `graph` for: full project architecture, finding entry points, overall understanding
- Use `calls --from` for: tracing what a specific function depends on
- Use `calls --to` for: finding all callers of a specific function
- Use `calls --from X --to Y` for: finding the call path between two functions

### `leta implementations` - Find Implementations

Find all implementations of an interface or abstract method.

```bash
# Find all classes implementing Storage interface
leta implementations Storage

# Find implementations of a specific method
leta implementations Validator.validate
```

### `leta supertypes` / `leta subtypes` - Type Hierarchy

Navigate class inheritance.

```bash
# What does this class extend/implement?
leta supertypes MyDatabaseStorage

# What classes extend this one?
leta subtypes BaseHandler
```

### `leta declaration` - Find Declaration

Find where a symbol is declared (useful for variables, parameters).

```bash
leta declaration config_path
```

## Refactoring Commands

### `leta rename` - Rename Symbol

Rename a symbol across the entire workspace. Updates all references.

```bash
# Rename a function
leta rename old_function_name new_function_name

# Rename a method
leta rename UserRepository.add_user create_user
```

### `leta mv` - Move File and Update Imports

Move/rename a file and update all import statements.

```bash
leta mv src/user.ts src/models/user.ts
```

## Common Workflows

### Exploring Unfamiliar Code

```bash
# 1. Get the full call graph — this is the fastest way to understand a project
leta graph

# 2. If the project is large, focus on specific areas
leta graph -i 'src/handlers'

# 3. Dive into specific functions you see in the graph
leta show handle_request

# 4. If you need the file layout
leta files
```

### Understanding a Function

```bash
# 1. Get the high-level overview: what does it call?
leta calls --from process_request

# 2. Who calls this function?
leta calls --to process_request

# 3. NOW read the implementation (with context from steps 1-2)
leta show process_request

# 4. Find all usages/references
leta refs process_request
```

### Finding Interface Implementations

```bash
# 1. Find the interface
leta grep "Storage" -k interface

# 2. Find all implementations
leta implementations Storage

# 3. Look at a specific implementation
leta show FileStorage
```

## Tips

1. **Start with `leta graph`** to understand project architecture — it's the most token-efficient way to get a holistic view of how the codebase fits together.

2. **Combine with ripgrep-like tools** - use leta for "where is X defined/used?" and ripgrep-like tools for "where does string Y appear?"

3. **Symbol formats are flexible** - if `SymbolName` is ambiguous, qualify it with `path:Symbol` or `Parent.Symbol`.

4. **Check workspace first** - if commands fail, ensure you've run `leta workspace add`.

5. **Don't redirect stderr** (e.g., `2>/dev/null`) - when a symbol is ambiguous, leta outputs disambiguation options to stderr showing how to qualify the symbol name. You need to see this to know how to fix the command.
