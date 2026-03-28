# Research: Rust Implementations of LSP Servers

Goal: Determine whether we can bundle Rust-native LSP server crates into leta to eliminate the need for users to install language servers separately.

## Summary

Two languages have production-quality, full-featured Rust LSP servers: **Rust** (rust-analyzer) and **Python** (ty). TypeScript/JavaScript has Rust-based linter/formatter LSPs but no full semantic server yet. The rest have no viable Rust options.

## Per-Language Analysis

### Rust → rust-analyzer ✅ PRODUCTION, EMBEDDABLE

- **Written in:** Rust
- **Crate:** Split across many crates (`ide`, `hir`, `syntax`, etc.)
- **Embeddability:** Excellent. The `ide` crate provides the core analysis API without any LSP protocol concerns. The LSP layer is a thin wrapper.
- **Status:** Production quality, maintained by the Rust project.
- **Approach:** Depend on the `ide` crate directly and call its API. Skip LSP protocol entirely.
- **Catch:** Massive dependency tree (~400 crates). Significant compile time and binary size. Requires Rust sysroot/stdlib sources to be available at runtime.
- **Repo:** github.com/rust-lang/rust-analyzer

### Python → ty ✅ PRODUCTION, LIKELY EMBEDDABLE

- **Written in:** Rust (by Astral, the ruff/uv team)
- **Status:** Beta (released Dec 2025), but rapidly maturing. Already used with PyCharm, VS Code, Neovim, Zed. Releasing weekly with major features.
- **LSP features:** Full — go-to-definition, find-references, completions with auto-import, rename refactoring, inlay hints, hover, signature help, call hierarchy, diagnostics. Jupyter notebook support.
- **Performance:** 10-60x faster than mypy/Pyright without caching. 80x faster than Pyright for incremental updates (4.7ms vs 386ms on PyTorch).
- **Embeddability:** Likely good — Astral's crates (ruff, uv) are well-structured Rust libraries. ty is built on a Salsa-based incremental computation framework, similar to rust-analyzer's architecture. Would need investigation into whether the LSP server can be driven as a library.
- **Catch:** Still beta. Some type system features may be incomplete compared to Pyright. Astral was acquired by OpenAI in early 2026 — unclear impact on open-source direction.
- **Repo:** github.com/astral-sh/ty

### TypeScript/JavaScript → Partial ⚠️ LINTER/FORMATTER ONLY

No Rust project provides full semantic TypeScript/JavaScript LSP (go-to-definition, find-references, type-aware call hierarchy). The Rust ecosystem has focused on linting and formatting:

- **Biome** (biomejs.dev): Rust-based LSP for JS/TS/CSS/JSON. Provides linting, formatting, code actions. Has `biome_lsp` crate. Does NOT provide go-to-definition, find-references, call hierarchy, or type-aware navigation. It parses JS/TS but has no full type checker.
- **OXC / Oxlint** (oxc.rs): Rust-based JS/TS toolchain. `oxc_language_server` crate provides LSP for linting and formatting. Same limitation — no semantic navigation. Oxlint 1.0 stable, Oxfmt beta (Feb 2026). OXC is a foundational toolkit (parser, resolver, transformer) but not a type checker.
- **typescript-language-server (crates.io):** Very early (v0.1.0) Rust reimplementation using tree-sitter. Proof-of-concept, not production ready.

**Why no full Rust TS/JS LSP exists:** TypeScript's type system is enormously complex (structural typing, conditional types, mapped types, template literals, etc.). Microsoft is porting TypeScript itself to **Go** (TypeScript 7 / "tsgo"), not Rust. This Go-based TypeScript will include a native LSP server and is expected to supersede the current Node.js-based approach.

**Verdict:** For leta's needs (semantic navigation), we still need typescript-language-server (Node.js) or the upcoming tsgo. Biome/OXC could supplement for linting but can't replace the semantic features.

### Go → No Rust option ❌

- **Current:** gopls (written in Go, maintained by the Go team)
- **Rust candidates:** None found.
- gopls is deeply integrated with Go's toolchain (`go/packages`, `go/types`). No Rust reimplementation has been attempted.

### Java → No Rust option ❌

- **Current:** jdtls (Eclipse JDT Language Server, written in Java)
- **Rust candidates:** None found.
- Java's type system, classpath resolution, and build tool integration (Maven, Gradle) build on decades of Eclipse JDT work.

### C/C++ → No Rust option ❌

- **Current:** clangd (written in C++, built on LLVM/Clang)
- **Rust candidates:** None found.
- clangd depends on Clang's compiler frontend for all semantic analysis. Would need to reimplement or FFI-bind a C/C++ compiler frontend.

### Ruby → No Rust option ❌

- **Current:** ruby-lsp (written in Ruby, by Shopify)
- **Rust candidates:** None found.
- Ruby's extreme dynamism makes static analysis very hard.

### PHP → Barely ⚠️

- **Current:** intelephense (TypeScript/Node.js, proprietary)
- **Rust candidates:**
  - **phpls-rs** (github.com/sawmurai/phpls-rs): PHP language server written in Rust. Appears abandoned (last commit years ago). Very incomplete.
- **Verdict:** Not viable.

### Lua → No Rust option ❌

- **Current:** lua-language-server (written in C++ and Lua)
- **Rust candidates:** None found.

### OCaml → No Rust option ❌

- **Current:** ocamllsp (written in OCaml, uses Merlin)
- **Rust candidates:** None found.

### Zig → No Rust option ❌

- **Current:** zls (written in Zig)
- **Rust candidates:** None found.

## Scorecard

| Language | Rust LSP? | Maturity | Semantic features? | Embeddable? |
|----------|-----------|----------|-------------------|-------------|
| Rust | rust-analyzer | Production | Full | Yes (ide crate) |
| Python | ty | Beta | Full | Likely |
| TypeScript/JS | Biome/OXC | Production | Lint/format only | Yes, but not useful for leta |
| Go | — | — | — | — |
| Java | — | — | — | — |
| C/C++ | — | — | — | — |
| Ruby | — | — | — | — |
| PHP | phpls-rs | Abandoned | Incomplete | — |
| Lua | — | — | — | — |
| OCaml | — | — | — | — |
| Zig | — | — | — | — |

## Recommended Approach

### Short-term: Auto-download LSP servers on first use

Like Mason (Neovim plugin), automatically download and manage LSP server installations. On first `leta workspace add`, detect languages and install the necessary servers. This solves the UX problem immediately for all languages.

### Medium-term: Embed ty for Python

ty is the strongest candidate for embedding. It's written in Rust by a team (Astral) known for building well-structured, embeddable Rust libraries (ruff, uv). Python is also the #1 language for AI coding agents (leta's primary users), making this the highest-impact integration. Steps:
1. Investigate ty's crate structure and whether the LSP server can be driven as a library
2. Benchmark IPC overhead vs. direct library calls to quantify the speedup
3. If viable, add ty as an optional cargo feature behind a flag

### Long-term: Embed rust-analyzer for Rust

rust-analyzer's `ide` crate is explicitly designed for library use. This is the easiest embedding technically, but benefits only Rust projects. Worth doing once the Python integration proves the pattern works.

### Watch: TypeScript's Go port

Microsoft's TypeScript 7 (tsgo) will include a native Go-based LSP server. When it ships, it will be significantly faster than the current Node.js-based typescript-language-server. Not embeddable in Rust, but the performance improvement from Go's compiled nature may reduce the incentive to embed.

## Key Insight

The landscape has shifted significantly since 2024. The Astral team (ruff, uv, ty) has demonstrated that Rust-based Python tooling can be production-quality and extremely fast. ty in particular is a game-changer — it's the first serious Rust-based full-featured LSP server for a language other than Rust itself. If this trend continues, we may see more languages get Rust LSP servers over time, but for now, Python and Rust are the only viable embedding targets.