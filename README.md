<div align="center">

# 🌲 Semantic Delta Protocol (SDP)

**A universal standard for structural code persistence and semantic versioning.**

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

---

## What is SDP?

SDP is a protocol and framework for **understanding code at the semantic level** — not just lines, but functions, classes, and symbols. It tracks how your code evolves structurally.

### Core Concepts

| Concept | Description |
|--------|-------------|
| **Structural Hash** | A hash of code structure, not content. Survives whitespace changes and refactors. |
| **Semantic Delta** | The difference between two versions of a symbol (function, class, etc.) |
| **Content Addressable Storage** | Deduplicated storage using BLAKE3 hashes |
| **AST Analysis** | Tree-sitter powered parsing for accurate symbol extraction |

---

## Why Semantic?

Traditional versioning tracks **lines**. SDP tracks **symbols**:

```
Traditional (Git):
  - Line 5 changed from "fn foo()"
  - Line 10 added "let x = 1"

SDP (This Protocol):
  - Function "foo" renamed to "bar" 
  - New variable "x" added in "main"
  - Struct "User" field "email" removed
```

This means:
- **Rename refactoring** → tracked as rename, not delete+add
- **Whitespace changes** → ignored (same semantic content)
- **Move to different file** → tracked if symbol identity preserved

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

1. **Parse** → Tree-sitter builds AST
2. **Extract** → Identify symbols (functions, classes, etc.)
3. **Hash** → Generate structural fingerprints
4. **Compare** → Find semantic deltas vs previous state
5. **Store** → Commit to CAS + update registry

---

## Protocol (JSON-RPC)

SDP uses JSON-RPC 2.0 for communication:

```json
// Get file history
{"jsonrpc": "2.0", "method": "sdp/getFileHistory", "params": {"path": "/src/main.rs"}, "id": 1}

// Response
{"jsonrpc": "2.0", "result": {"versions": [{"hash": "abc123", "symbols": [...], "timestamp": "..."}]}, "id": 1}
```

### Methods

| Method | Description |
|--------|-------------|
| `sdp/save` | Save current file state |
| `sdp/getHistory` | Get version history for file |
| `sdp/getSymbolHistory` | Get history for specific symbol |
| `sdp/restore` | Restore file to version |
| `sdp/search` | Search across all history |

---

## Storage Format

```
project/
├── .sdp/                  # SDP data (or project's .mnemosyne)
│   ├── db/                # SQLite registry
│   │   └── symbols.db     # Symbol history
│   └── cas/               # Content Addressable Storage
│       └── {hash_prefix}/  # Content chunks
```

---

## Supported Languages

SDP works with any Tree-sitter supported language:

- Rust, Go, C, C++, Python, JavaScript, TypeScript
- Java, C#, Ruby, PHP, Swift, Kotlin
- HTML, CSS, JSON, YAML, Markdown
- ...and 100+ more

---

## Implementations

| Project | Description |
|--------|-------------|
| [Mnemosyne](https://github.com/alessandrobrunoh/Mnemosyne) | Local history CLI using SDP |
| Zed Editor | Built-in semantic editing |

---

## Getting Started

```rust
use semantic_delta_protocol::{Engine, Config};

let config = Config::default();
let engine = Engine::new(config)?;

engine.save("/src/main.rs", content)?;

// Get history
let history = engine.get_history("/src/main.rs")?;
for version in history.versions {
    println!("{} - {} symbols", version.hash, version.symbols.len());
}
```

---

## License

MIT — See [LICENSE](LICENSE)
