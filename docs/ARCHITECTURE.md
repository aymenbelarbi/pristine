# Pristine Architecture

## System Architecture Overview

Pristine is a code context compiler that transforms repositories into optimized context artifacts. The system follows a pipeline architecture where each stage processes data and passes it to the next stage.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Pristine System Architecture                       │
└─────────────────────────────────────────────────────────────────────────────┘

┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Source  │───▶│ Acquire  │───▶│Inventory │───▶│ Classify │───▶│  Select  │
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘
                                                                      │
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐         │
│ Deliver  │◀───│  Render  │◀───│ Assemble │◀───│ Compress │◀────────┘
└──────────┘    └──────────┘    └──────────┘    └──────────┘
```

## Crate Dependency Diagram

```
                    ┌─────────────────────┐
                    │   pristine-core     │
                    │   (Domain Types)    │
                    └─────────┬───────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼───────┐   ┌────────▼────────┐   ┌───────▼───────┐
│pristine-      │   │pristine-domain  │   │pristine-      │
│acquire        │   │(Engine/Config)  │   │observability  │
└───────┬───────┘   └────────┬────────┘   └───────────────┘
        │                    │
        │    ┌───────────────┼───────────────┐
        │    │               │               │
┌───────▼────▼───┐  ┌───────▼───────┐  ┌───▼────────────┐
│pristine-       │  │pristine-      │  │pristine-       │
│inventory       │  │select         │  │compress        │
└────────────────┘  └───────────────┘  └────────────────┘
        │                   │                   │
        │    ┌──────────────┼───────────────┐   │
        │    │              │               │   │
┌───────▼────▼───┐  ┌──────▼──────┐  ┌─────▼──▼─────┐
│pristine-       │  │pristine-    │  │pristine-     │
│cache           │  │safety       │  │extract       │
└────────────────┘  └─────────────┘  └──────────────┘
                            │
                    ┌───────▼───────┐
                    │pristine-mcp   │
                    └───────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                     Adapter Layer                                │
├─────────────────┬─────────────────┬─────────────────────────────┤
│ pristine-cli    │ pristine-api    │ pristine-mcp                │
│ (CLI)           │ (HTTP API)      │ (MCP Server)                │
└─────────────────┴─────────────────┴─────────────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │   Web Frontend    │
                    │ (React/TypeScript)│
                    └───────────────────┘
```

## Crate Descriptions

### Core Crates

| Crate | Description | Dependencies |
|-------|-------------|--------------|
| `pristine-core` | Core types, traits, and error definitions | None (workspace only) |
| `pristine-domain` | Business logic, engine, and pipeline orchestration | `pristine-core` |
| `pristine-observability` | Metrics, logging, and tracing infrastructure | `pristine-core` |

### Pipeline Crates

| Crate | Description | Dependencies |
|-------|-------------|--------------|
| `pristine-acquire` | Repository acquisition from local/remote sources | `pristine-core` |
| `pristine-inventory` | File system traversal and catalog creation | `pristine-core` |
| `pristine-select` | File selection engine with scoring and budgeting | `pristine-core` |
| `pristine-compress` | Content compression (light, structural, summary) | `pristine-core` |
| `pristine-extract` | Content extraction and encoding detection | `pristine-core` |
| `pristine-cache` | Caching layer for snapshots and artifacts | `pristine-core` |
| `pristine-safety` | Secret scanning and policy enforcement | `pristine-core` |

### Adapter Crates

| Crate | Description | Dependencies |
|-------|-------------|--------------|
| `pristine-cli` | Command-line interface | All pipeline crates |
| `pristine-api` | HTTP API server | All pipeline crates |
| `pristine-mcp` | Model Context Protocol server | All pipeline crates |

## Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Data Flow                                       │
└─────────────────────────────────────────────────────────────────────────────┘

User Input (CLI/API/MCP/Web)
    │
    ▼
┌─────────────────┐
│  ArtifactRequest │  ← Source, Profile, Query, Policy, Budget, Output Config
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   SourceRef     │  ← Parsed source (local path, GitHub URL, etc.)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  RepoSnapshot   │  ← Acquired repository with snapshot ID
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  FileCatalog    │  ← List of all files with metadata
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ClassifiedCatalog│  ← Files tagged with semantic roles
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ SelectionPlan   │  ← Files selected with inclusion levels and reasons
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│CompressedCatalog│  ← Files compressed according to their level
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ ContextArtifact │  ← Final structured artifact with metadata
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│RenderedArtifact │  ← Formatted output (JSON/Markdown/XML/Text)
└────────┬────────┘
         │
         ▼
    User Output
```

## Design Decisions and Rationale

### 1. Workspace Architecture

**Decision:** Split functionality into multiple crates within a Cargo workspace.

**Rationale:**
- **Modularity:** Each crate has a single responsibility, making the codebase easier to understand and maintain.
- **Compile times:** Changes to one crate don't trigger recompilation of unrelated crates.
- **Testability:** Each crate can be tested independently.
- **Reusability:** Crates like `pristine-core` can be used by multiple adapters.

### 2. Trait-Based Pipeline

**Decision:** Define pipeline stages as traits in `pristine-core`.

**Rationale:**
- **Flexibility:** Different implementations can be swapped in (e.g., different compression strategies).
- **Testability:** Mock implementations can be used for testing.
- **Extensibility:** New pipeline stages can be added without modifying existing code.

### 3. Layered Fidelity

**Decision:** Files are included at different fidelity levels (full, compressed, summary, tree-only) rather than binary include/exclude.

**Rationale:**
- **Budget efficiency:** Token budgets are met by lowering fidelity, not cutting files.
- **Context preservation:** Even excluded files contribute to the directory structure.
- **Explainability:** Each file's inclusion level and reasons are tracked.

### 4. Async Throughout

**Decision:** All I/O operations are async using Tokio.

**Rationale:**
- **Performance:** Multiple files can be processed concurrently.
- **Scalability:** The API server can handle multiple requests simultaneously.
- **Resource efficiency:** Async I/O uses fewer threads than blocking I/O.

### 5. Caching Strategy

**Decision:** Cache snapshots and artifacts by content-addressed fingerprint.

**Rationale:**
- **Performance:** Repeated runs on the same repository are near-instant.
- **Consistency:** Same input always produces same output.
- **Storage efficiency:** Content-addressed deduplication reduces storage needs.

### 6. Safety First

**Decision:** Secret scanning runs before delivery with configurable policy modes.

**Rationale:**
- **Security:** Prevents accidental exposure of secrets.
- **Compliance:** Organizations can enforce strict policies.
- **Trust:** Users can safely share context artifacts externally.

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Language | Rust 1.75+ | Core implementation |
| Async Runtime | Tokio | Async I/O and concurrency |
| HTTP Server | Axum | API server framework |
| CLI Framework | Clap | Command-line parsing |
| Git Operations | git2 (libgit2) | Repository operations |
| File Traversal | ignore (ripgrep) | Fast file walking with gitignore support |
| Pattern Matching | globset | Glob pattern matching |
| Syntax Parsing | tree-sitter | Structural compression |
| Token Counting | tiktoken-rs | Token estimation |
| Caching | moka | In-memory LRU cache |
| Metrics | metrics + prometheus | Observability |
| Error Handling | thiserror + anyhow | Error management |
| Serialization | serde | JSON/YAML serialization |
| Frontend | React 18 + Vite | Web UI |
| UI Components | shadcn UI | Component library |
| Styling | Tailwind CSS | CSS framework |
| State Management | Zustand | Frontend state |
| Data Fetching | TanStack Query | Server state management |

## Performance Characteristics

| Operation | Target | Notes |
|-----------|--------|-------|
| Overview Pack (cache miss) | < 2s | Medium repository (~1000 files) |
| Task Pack (cache miss) | < 5s | With query processing |
| Repeated run (cache hit) | < 50ms | Cached artifact |
| p95 across all profiles | < 15s | Worst case |
| Binary startup | < 10ms | Static binary |

## Scalability Considerations

1. **Horizontal Scaling:** The API server is stateless and can be load balanced.
2. **Cache Distribution:** Redis can replace in-memory cache for multi-instance deployments.
3. **Storage:** Snapshots can be stored in S3-compatible object storage.
4. **Rate Limiting:** Built-in rate limiting prevents abuse.
