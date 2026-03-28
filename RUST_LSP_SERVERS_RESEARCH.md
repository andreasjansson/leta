# Research: Rust Implementations of LSP Servers

Goal: Determine whether we can bundle Rust-native LSP server crates into leta to eliminate the need for users to install language servers separately.

## Summary

**Verdict: Not feasible for most languages. Only Rust has a viable embeddable option today.**

Most production-quality LSP servers are written in the language they serve (Go for Go, TypeScript for TypeScript, etc.) or in C++. The few Rust implementations that exist are either linters-only (not full LSP), too immature, or architecturally not designed for embedding as a library.

## Per-Language Analysis

### Rust → rust-analyzer ✅ EMBEDDABLE

- **Written in:** Rust
- **Crate:** `rust-analyzer` (split across many crates: `ide`, `hir`, `syntax`, etc.)
- **Embeddability:** Excellent. rust-analyzer is explicitly designed as a set of libraries. The `ide` crate provides the core analysis API without any LSP concerns. The LSP layer (`crates/rust-analyzer`) is a thin wrapper.
- **Status:** Production quality, maintained by the Rust project itself.
- **Approach:** Depend on the `ide` crate directly and call its API for goto-definition, find-references, etc. Skip the LSP protocol entirely.
- **Catch:** Massive dependency tree. Would add significant compile time and binary size. Also requires the Rust sysroot/stdlib sources to be available.

### Python → No viable Rust option ⚠️

- **Current:** basedpyright (TypeScript/Node.js)
- **Rust candidates:**
  - **pylyzer** (github.com/mtshiba/pylyzer): Rust-based Python type checker and LSP. Claims 100x faster than pyright. Built on top of the Erg language's type system. However, it's a one-person project, significantly less mature than pyright, and its type inference is not compatible with Python's type system in many edge cases.
  - **ruff** (astral.sh): Has a Rust-based LSP server (`ruff server`), but it's a linter/formatter only — no go-to-definition, find-references, call hierarchy, or any of the semantic navigation features leta needs.
  - **ZubanLS** (zubanls.com): Commercial, closed-source Python LSP written in Rust. Not embeddable.
- **Verdict:** No viable option. pylyzer is too immature for production use, ruff doesn't provide semantic features. basedpyright (TypeScript) remains the best Python LSP by far.

### TypeScript/JavaScript → No viable Rust option ⚠️

- **Current:** typescript-language-server (TypeScript/Node.js wrapper around tsserver)
- **Rust candidates:**
  - **Biome** (biomejs.dev): Rust-based toolchain for JS/TS with an LSP server (`biome_lsp` crate). However, like ruff, it's primarily a linter and formatter. It does NOT provide go-to-definition, find-references, call hierarchy, or type-aware navigation. It parses JS/TS but doesn't have a type checker.
  - **typescript-language-server (crates.io)**: A very new (v0.1.0) Rust crate. Uses tree-sitter for parsing. Appears to be a proof-of-concept/hobby project — not production ready. No type checking.
- **Note:** Microsoft is porting TypeScript itself to Go (TypeScript 7 / "tsgo"), which will include a native LSP server. This will supersede the current Node.js-based server but won't be in Rust.
- **Verdict:** No viable option. TypeScript's type system is enormously complex. No Rust project has attempted a full reimplementation. The closest thing (Biome) explicitly scopes out type-aware features.

### Go → No Rust option ❌

- **Current:** gopls (written in Go)
- **Rust candidates:** None found.
- **Notes:** gopls is maintained by the Go team and deeply integrated with Go's toolchain (`go/packages`, `go/types`). Reimplementing Go's type system in Rust would be a multi-year effort with no clear benefit since gopls is already very fast.
- **Verdict:** Not feasible. gopls is the only real option.

### Java → No Rust option ❌

- **Current:** jdtls (Eclipse JDT Language Server, written in Java)
- **Rust candidates:** None found.
- **Notes:** Java's type system, classpath resolution, and build tool integration (Maven, Gradle) are enormously complex. jdtls builds on decades of Eclipse JDT work. No one has attempted this in Rust.
- **Verdict:** Not feasible.

### C/C++ → No Rust option ❌

- **Current:** clangd (written in C++)
- **Rust candidates:** None found.
- **Notes:** clangd is built on LLVM/Clang's frontend, which provides the actual C/C++ parsing, type checking, and semantic analysis. You'd need to reimplement (or bind to) a C/C++ compiler frontend in Rust. The `tree-sitter-cpp` grammar can parse but has no semantic understanding.
- **Verdict:** Not feasible. Could potentially use Clang as a C library via FFI, but that defeats the purpose of a pure-Rust solution and still requires LLVM to be installed.

### Ruby → No Rust option ❌

- **Current:** ruby-lsp (written in Ruby)
- **Rust candidates:** None found.
- **Notes:** ruby-lsp was recently rewritten by Shopify and is the official Ruby LSP. Ruby's dynamic nature makes static analysis extremely hard. No Rust implementations exist.
- **Verdict:** Not feasible.

### PHP → No Rust option ❌

- **Current:** intelephense (written in TypeScript/Node.js, proprietary)
- **Rust candidates:** None found.
- **Verdict:** Not feasible.

### Lua → No Rust option ❌

- **Current:** lua-language-server (written in C++ and Lua)
- **Rust candidates:** None found.
- **Verdict:** Not feasible.

### OCaml → No Rust option ❌

- **Current:** ocamllsp (written in OCaml)
- **Rust candidates:** None found.
- **Notes:** ocamllsp uses OCaml's own compiler infrastructure (Merlin) for semantic analysis. Not replicable in Rust.
- **Verdict:** Not feasible.

### Zig → No Rust option ❌

- **Current:** zls (written in Zig)
- **Rust candidates:** None found.
- **Verdict:** Not feasible.

## Alternative Approaches

### 1. Bundle pre-built LSP server binaries

Instead of embedding Rust crates, bundle pre-compiled binaries of each LSP server with leta's distribution. This solves the "users don't have to install LSP servers" problem without requiring Rust reimplementations.

**Pros:**
- Actually works for all languages
- No maintenance burden of keeping Rust reimplementations in sync with upstream
- Users get the same battle-tested servers they'd install manually

**Cons:**
- Massive distribution size (hundreds of MB)
- Version management and updates become leta's problem
- Some servers (jdtls) require a JVM, intelephense requires Node.js — can't truly eliminate runtime deps
- Licensing complexity (intelephense is proprietary)

### 2. Auto-download LSP servers on first use

Like Mason (Neovim plugin), automatically download and manage LSP server installations. On first `leta workspace add`, detect the languages in the project and install the necessary servers.

**Pros:**
- Zero-config for users
- Servers are always up-to-date
- Small initial binary size

**Cons:**
- First run is slow (downloading)
- Requires network access
- Still external processes, not embedded

### 3. Embed rust-analyzer only, keep external for everything else

Since rust-analyzer is the one LSP server actually written in Rust with a library API, embed just that one. Keep external server processes for all other languages. This gives Rust projects a zero-dependency experience while keeping the proven approach for everything else.

**Pros:**
- Genuine speedup for Rust projects (no IPC overhead)
- No need to install rust-analyzer separately
- Minimal additional complexity

**Cons:**
- Only benefits one language
- rust-analyzer's dependency tree is very large (~400 crates)
- Significantly increases compile time and binary size

## Recommendation

**Option 2 (auto-download) is the most practical path** for solving the user experience problem. It gives users zero-config setup without the impossible task of reimplementing a dozen language servers in Rust.

Option 3 (embed rust-analyzer) is an interesting optimization that could be layered on later for the Rust-specific case, but the compile time and binary size costs need to be measured first.

The dream of a single monolithic Rust binary that understands all languages is, unfortunately, not achievable with today's ecosystem. The problem isn't Rust — it's that each language's semantic analysis requires deep knowledge of that language's type system, module resolution, build tooling, and standard library, which takes years of effort per language to build correctly.
