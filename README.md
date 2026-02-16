<div align="center">

# Semantic Delta Protocol (SDP)

**A universal standard for structural code persistence and semantic versioning.**

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

---

## Origin & Future

SDP was created specifically for **Mnemosyne** — a local history tool for developers. It exists because we needed a way to track code changes that survives refactoring, not just line-by-line diffs.

Currently, SDP has one purpose: powering Mnemosyne. But we hope it could be useful for more:

- **IDE integrations** — Semantic-aware version control built into editors
- **AI coding assistants** — Understanding code evolution for better context
- **Refactoring tools** — Track how code changes across renames and moves
- **Code review** — Semantic diffs that ignore formatting noise

If you're interested in using SDP for something else, we'd love to hear about it. Open an issue or reach out!

---

## The Problem We Solve

Every developer has been here:

> **You spend 3 hours refactoring. You rename `UserService` to `AccountService`. You move functions around. You clean up whitespace. Then disaster strikes.** You need to revert, but Git shows 47 files changed with 2,000 lines diff. Everything looks like it was deleted and rewritten—even though your logic is mostly the same.

**This is the fundamental limitation of line-based versioning.**

Traditional tools track **what changed**. SDP tracks **what mattered**.

---

## Why Semantic Matters

Git tells you *which lines* changed. SDP tells you *what your code actually did*.

| Scenario | Git Shows | SDP Shows |
|----------|-----------|-----------|
| Rename function | `- fn old()` + `fn new()` | Function "old" renamed to "new" |
| Reformat code | 500 lines changed | No changes (same semantic content) |
| Move file | File deleted + new file | Symbol "Foo" moved to new location |
| Add field to struct | 3 lines added | Field "email" added to struct "User" |

**This is the difference between:**
- Knowing *that* code changed
- Understanding *what* changed

---

## Core Concepts

### Structural Hash

A fingerprint of code **structure**, not content:

```rust
// Same structural hash regardless of:
// - Whitespace
// - Comments
// - Variable names (in some contexts)

fn calculate_total(items: Vec<f64>) -> f64 {
    items.iter().sum()
}
```

The structural hash survives refactoring because it captures **what the code does**, not how it's written.

### Semantic Delta

The actual change that matters:

```json
{
  "type": "modified",
  "symbol": "User::validate_email",
  "old_hash": "abc123",
  "new_hash": "def456",
  "changes": [
    { "type": "added", "node": "if let Some(email)" },
    { "type": "removed", "node": "regex::is_valid(&email)" }
  ]
}
```

### Content Addressable Storage

Every unique chunk of code is stored once, referenced by BLAKE3 hash:

```
cas/
├── ab/           # Hash prefix "ab..."
│   └── abc123... # Actual content
├── cd/
│   └── def456...
```

This means 100 versions of a file with 95% similarity = ~1.05x storage, not 100x.

---

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Editor    │────▶│  SDP Engine │────▶│  Storage    │
└─────────────┘     └─────────────┘     └─────────────┘
                          │
                          ▼
                   ┌─────────────┐
                   │ Tree-Sitter │
                   └─────────────┘
```

### Save Pipeline

1. **Parse** — Tree-sitter builds AST
2. **Extract** — Identify symbols (functions, classes, etc.)
3. **Hash** — Generate structural fingerprints
4. **Compare** — Find semantic deltas vs previous state
5. **Store** — Commit to CAS + update registry

---

## Protocol (JSON-RPC)

SDP uses JSON-RPC 2.0 over Unix Domain Sockets (Unix) or Named Pipes (Windows):

```json
// Get symbol history
{
  "jsonrpc": "2.0",
  "method": "sdp/getSymbolHistory",
  "params": {
    "path": "/src/main.rs",
    "symbol": "main"
  },
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "symbol": "main",
    "history": [
      {
        "version": 5,
        "hash": "abc123",
        "delta": { "type": "modified", "changes": [...] },
        "timestamp": "2024-01-15T10:30:00Z"
      }
    ]
  },
  "id": 1
}
```

### Methods

| Method | Description |
|--------|-------------|
| `sdp/initialize` | Initialize session with client capabilities |
| `sdp/save` | Save current file state with semantic analysis |
| `sdp/getHistory` | Get version history for file |
| `sdp/getSymbolHistory` | Get evolutionary history for specific symbol |
| `sdp/restore` | Restore file to version |
| `sdp/search` | Search across all semantic history |

---

## Storage Format

```
project/
├── .sdp/                  # Protocol data (or .mnemosyne)
│   ├── db/                # redb B-tree registry
│   │   └── symbols.db     # Symbol history & structural hashes
│   └── cas/               # Content Addressable Storage
│       └── {hash_prefix}/  # Deduplicated chunks (BLAKE3)
```

**Why redb?** Pure Rust, Copy-on-Write, embedded, no migration headaches.

---

## Supported Languages

SDP works with any Tree-sitter supported language:

- **Systems**: Rust, Go, C, C++
- **Scripting**: Python, JavaScript, TypeScript, Ruby, PHP
- **Enterprise**: Java, C#, Swift, Kotlin
- **Web**: HTML, CSS, JSON, YAML, Markdown
- **...and 100+ more**

---

## Implementations

| Project | Description |
|---------|-------------|
| [Mnemosyne](https://github.com/alessandrobrunoh/Mnemosyne) | Local history CLI using SDP |
| Zed Editor | Built-in semantic editing |

---

## Getting Started

```rust
use semantic_delta_protocol::{Engine, Config};

let config = Config::default();
let engine = Engine::new(config)?;

engine.save("/src/main.rs", content)?;

// Get evolutionary history of a function
let history = engine.get_symbol_history("/src/main.rs", "calculate_total")?;
for version in history.versions {
    println!("{}: {:?}", version.delta.delta_type, version.delta.changes);
}
```

---

## Why This Matters

### For IDEs

Build semantic-aware features that understand **what** code does, not just **where** it is:
- "When did this function change?"
- "Show me all versions of this class"
- "What was this variable called before?"

### For Developers

Never lose context during refactoring:
- Rename tracking
- Move detection  
- Semantic diffs (what actually changed, not just lines)

### For AI Coding Assistants

Semantic history enables intelligent context:
- "Continue from where the user left off"
- "Understand the evolution of this function"
- "Don't suggest already-tried approaches"

---

## License

APACHE 2.0 — See [LICENSE](LICENSE)
