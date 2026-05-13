<div align="center">

![Pristine Banner](https://raw.githubusercontent.com/aymenbelarbi/pristine/main/pristine.png)

# Pristine

**The code context compiler for humans, agents, and CI.**

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org)
[![Status: Alpha](https://img.shields.io/badge/status-alpha-yellow.svg)](#roadmap)

</div>

---

## What is Pristine?

Pristine is a **code context compiler**. You give it a repository, a scope, and a task. It gives back the **smallest trustworthy context artifact** for that job — not a raw dump of everything.

Where other tools flatten your entire codebase into one text file and hand it to an LLM, Pristine selects, ranks, compresses, and structures context around what actually matters for the task at hand. It decides which files belong at full fidelity, which belong as compressed signatures, and which belong only as directory structure — then tells you why.

Pristine is written in Rust. It ships as a single static binary, an HTTP API, an MCP server, and a web UI built on shadcn UI. One engine powers all four.

---

## Problem Statement

Every team using LLMs for code tasks runs into the same wall.

You want to ask an LLM to review a PR, explain an auth flow, help implement a feature, or onboard a new engineer. To do that, the model needs code context. So you reach for the simplest tool: export the whole repository into one giant text dump and paste it in.

This breaks down fast:

- **Token budgets explode.** A real codebase has hundreds or thousands of files. Most are irrelevant to the task. Dumping everything wastes 70–90% of your token budget on noise: lockfiles, generated code, vendor directories, notebooks, build artifacts, fixtures.
- **The output is flat and unordered.** A raw file dump treats `README.md` and a 4,000-line generated client the same way. The model has no signal for what matters.
- **Nothing is safe to share.** Repos contain secrets, credentials, and internal configs. A naive export might expose API keys, private keys, or tokens to an external model or third-party tool.
- **The result is not reusable.** Every run generates a fresh dump. There is no caching, no stable artifact format, no way to consume context programmatically from CI or an agent.
- **There is no task alignment.** The same flat dump gets used for architecture review, bug triage, PR review, and onboarding — even though each of those jobs needs a very different slice of the codebase.

Existing repository packers solve the "how do I get code into a prompt" problem. They do not solve the "how do I get the *right* code into a prompt" problem.

---

## Solution Design

Pristine treats context generation as a **compilation problem**, not an export problem.

A compiler takes source code and a target and produces an optimized artifact for that target. Pristine does the same for repository context: it takes a repository, a profile (overview, pack, review, agent, safe-share), and an optional query, and produces a context artifact optimized for that use case.

This means:

- **Task-aware selection.** The engine scores files by relevance signals — entrypoints, changed files, query matches, import adjacency, test proximity — and selects only what is needed.
- **Layered fidelity.** Files are not included or excluded. They are represented at the right level: full text, compressed structure (signatures only), one-line summary, or directory-only. Token budgets are met by lowering fidelity, not cutting files arbitrarily.
- **Explainable decisions.** Every file in the output carries a reason: `entrypoint`, `query_match`, `changed_file`, `adjacent_test`, `budget_downgraded`. You can see exactly why the engine made each choice.
- **Safe by default.** Secret scanning runs before delivery. Sensitive content is redacted or blocked depending on your policy mode. The `safe-share` profile applies strict defaults before any context leaves your environment.
- **Stable machine-readable artifacts.** Every output is a versioned, structured artifact with metadata, provenance, token estimates, and stable file unit IDs — ready for programmatic consumption by CI, agents, or APIs.
- **One engine, many surfaces.** CLI, HTTP API, MCP server, and web UI all call the same pipeline. The output is identical regardless of how you invoke it.

---

## Use Cases

### Understand an unfamiliar codebase

You join a new team or pick up an open-source project. You want architecture, entrypoints, key patterns, and tech stack — not 400 files of implementation detail.

```bash
pristine overview https://github.com/org/repo
```

Pristine generates an **Overview Pack**: executive summary, directory map, tech stack, entrypoints, config files, and high-centrality zones. In under two seconds.

---

### Get implementation context for a specific task

You are implementing a feature and need the auth flow, the relevant models, and the related tests — without the rest of the codebase in the way.

```bash
pristine pack ./my-project --query "JWT authentication flow" --max-tokens 30000
```

Pristine generates a **Task Pack**: files ranked by relevance to your query, neighboring tests and configs included, peripheral files compressed to signatures. You get a focused, budget-aware context artifact ready to drop into any LLM.

---

### Review a pull request

You want an LLM to review a feature branch — changed files, impacted interfaces, touched tests, config deltas, and a risk summary.

```bash
pristine review-diff https://github.com/org/repo --base main --head feature/auth-refactor --explain
```

Pristine generates a **Review Pack**: the diff, every changed file, tests adjacent to changed code, interface boundaries, config changes, and an explanation of what the engine included and why.

---

### Feed context to a coding agent or IDE assistant

Your agent or IDE assistant needs structured, stable context to reason about the codebase. It calls Pristine over MCP and gets back file chunks with stable IDs, metadata, provenance, and selection reasons — not a raw blob.

```
Tool: pack_context
Source: ./my-project
Query: "database connection pooling"
Max tokens: 20000
```

Pristine responds with an **Agent Pack**: chunked file units, hashes, line metadata, and selection reasons in JSON — ready to be grounded, cited, and paginated by the agent.

---

### Share code context safely outside your environment

You want to send context to an external model, a consultant, or a third-party tool — but your repo contains infrastructure config, secrets in test fixtures, and internal API credentials.

```bash
pristine safe-share ./my-project --policy redact --format markdown
```

Pristine generates a **Safe Pack**: secrets detected and redacted, credential paths excluded, policy mode and redaction summary attached, compressed output by default. You can share it without worrying about what you exposed.

---

### Use it in CI

You want every PR to generate a stable, reproducible context artifact that your review bot, security scanner, or architecture linter can consume.

```yaml
# .github/workflows/review.yml
- name: Generate context artifact
  run: |
    pristine review-diff . \
      --base ${{ github.event.pull_request.base.sha }} \
      --head ${{ github.event.pull_request.head.sha }} \
      --format json \
      --output artifact.json
```

Pristine caches by commit SHA, so repeated runs on the same ref are near-instant. The artifact format is versioned and stable across Pristine releases.

---

## Quick Start

### Install

**Homebrew (macOS/Linux):**
```bash
brew install pristine
```

**Cargo:**
```bash
cargo install pristine
```

**Download binary:**
```bash
curl -sSf https://pristine.dev/install.sh | sh
```

**Docker:**
```bash
docker pull ghcr.io/aymenbelarbi/pristine:latest
```

---

### First run

```bash
# Generate an overview of any GitHub repo
pristine overview https://github.com/rust-lang/rustfmt

# Pack context for a task in a local project
pristine pack ./my-project --query "error handling strategy"

# Review a diff
pristine review-diff ./my-project --base main --head HEAD

# Output to a file
pristine overview https://github.com/org/repo --output context.md

# Use JSON for programmatic consumption
pristine pack ./my-project --query "auth flow" --format json --output context.json
```

---

### Start the server

```bash
# Start the API server (default port 8080)
pristine-server

# With custom config
pristine-server --config pristine.yaml

# With Docker Compose
docker compose up
```

The web UI is available at `http://localhost:8080`.  
The API docs are at `http://localhost:8080/docs`.

---

### Configure a project

Drop a `.pristine.yaml` file in your repo root:

```yaml
# .pristine.yaml

default_profile: overview

budget:
  max_tokens: 50000
  compression_preference: light

policy:
  mode: redact
  scan_secrets: true
  block_patterns:
    - "vendor/**"
    - "node_modules/**"
    - "*.lock"

output:
  format: markdown
  include_tree: true
  include_stats: true
  include_reasons: true

ignore:
  patterns:
    - "*.log"
    - ".env*"
    - "target/"
    - "__pycache__/"
  use_gitignore: true
```

---

## How It Works

Pristine runs every request through a nine-stage pipeline. Each stage produces a typed output that can be independently tested, cached, and inspected.

```
Source → Acquire → Inventory → Classify → Select → Compress → Assemble → Render → Deliver
```

### 1. Source

Parse the input into a `SourceRef`. Supports local paths, GitHub URLs, branches, tags, commit SHAs, subpaths, and PR references. GitLab and Bitbucket are planned.

### 2. Acquire

Choose the cheapest safe fetch strategy:

| Mode | When | Strategy |
|---|---|---|
| Metadata | Fast overview | API only, no clone |
| Scoped snapshot | Limited path or overview | Shallow clone + sparse checkout |
| Full snapshot | Review, local analysis | Full checkout with cache reuse |

Pristine does not do a full deep clone unless the task requires it. Snapshots are cached by `(source, revision)` and reused across requests.

### 3. Inventory

Walk the snapshot using the `ignore` crate (the same engine powering `ripgrep`). Apply `.gitignore`, `.git/info/exclude`, engine-level noise rules, and a project-local `.pristineignore` if present. Capture path, size, extension, encoding, text/binary status, and depth for every file.

### 4. Classify

Tag every file with semantic roles: `source`, `config`, `lockfile`, `test`, `docs`, `generated`, `vendor`, `notebook`, `binary`, `fixture`, `migration`, `secrets_risk`, `entrypoint`, `manifest`. Classification uses path patterns, filename patterns, extension registries, and framework markers. No AI needed — disciplined rules cover 95%+ of the real-world distribution.

### 5. Select

This is the core of Pristine. The selection engine scores every file and assigns an inclusion level:

| Level | Meaning |
|---|---|
| `full` | Full file content |
| `compressed` | Structural representation — signatures, imports, docstrings |
| `summary` | Metadata + top symbols + line ranges |
| `tree_only` | Appears in the directory structure only |
| `excluded` | Omitted entirely |

**Scoring signals (v1, weighted heuristics):**

| Signal | Weight |
|---|---|
| Changed file (review mode) | +40 |
| Manifest or entrypoint | +25 |
| Test adjacent to changed file | +20 |
| Direct query hit | +15 |
| Import adjacency | +10 |
| Generated content | −25 |
| Vendor content | −35 |
| Binary or large artifact | −40 |

Budget is enforced by **lowering fidelity first, not cutting files**. If the token budget is exceeded, compressed files become summaries, summaries become tree-only entries, and tree-only entries are dropped — in that order.

Every decision is recorded: each file in the output carries a list of selection reasons (`entrypoint`, `query_match`, `changed_file`, `adjacent_test`, `policy_excluded`, `budget_downgraded`).

### 6. Compress

Compress selected files according to their assigned level:

| Mode | Behavior |
|---|---|
| `none` | Full file |
| `light` | Remove boilerplate, collapse blank lines, strip low-value comments |
| `structural` | Retain imports, signatures, type defs, docstrings; collapse bodies |
| `summary` | Emit symbol inventory, line ranges, file role |

Structural and summary compression use [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) when a grammar is available for the file's language. Supported: Rust, Python, TypeScript/JavaScript, Go, Java. Compression never blocks the pipeline — if a parser is unavailable, Pristine falls back to light text-aware compression.

### 7. Assemble

Build the `ContextArtifact` object with metadata, provenance, file units, statistics, and warnings. Every artifact carries:

- Artifact version and generator version
- Source reference and resolved revision
- Profile and policy mode
- Generation timestamp
- Token estimate
- File counts by inclusion level
- Any security warnings

### 8. Render

Convert the canonical artifact to the requested format. Renderers are pure and deterministic — the same artifact always produces the same output.

| Format | Use case |
|---|---|
| Markdown | Human reading, chat copy-paste |
| JSON | Programmatic consumption, API responses |
| XML | Model-friendly structured format |
| Text | Compatibility mode |

### 9. Deliver

Return the rendered artifact to the CLI as stdout or a file, as an HTTP response body, as an MCP tool response, or serve it through the web UI.

---

## Profiles

| Profile | Command | Best for |
|---|---|---|
| Overview | `pristine overview` | Architecture, onboarding, codebase orientation |
| Pack | `pristine pack --query "..."` | Task-focused implementation context |
| Review Diff | `pristine review-diff --base ... --head ...` | PR review, change analysis |
| Agent | `pristine agent` | Structured JSON context for agents and tools |
| Safe Share | `pristine safe-share` | External sharing with policy enforcement |

---

## Surfaces

### CLI

```bash
pristine overview <source> [options]
pristine pack <source> --query "..." [options]
pristine review-diff <source> --base <ref> --head <ref> [options]
pristine agent <source> --format json [options]
pristine safe-share <source> --policy redact [options]

# Config management
pristine config init
pristine config validate
pristine config show
```

**Common flags:**

| Flag | Description |
|---|---|
| `--revision` | Git branch, tag, or commit SHA |
| `--subpath` | Limit to a subdirectory |
| `--format` | Output format: json, markdown, xml, text |
| `--max-tokens` | Token budget |
| `--policy` | Policy mode: allow, redact, fail |
| `--include` | Glob patterns to include |
| `--exclude` | Glob patterns to exclude |
| `--explain` | Show selection reasons per file |
| `--output` | Write to file (default: stdout) |
| `--cache` | Cache behavior: use, refresh, disable |
| `--config` | Path to config file |

---

### HTTP API

All profiles are available as POST endpoints under `/v1/artifacts/`:

```bash
curl -X POST http://localhost:8080/v1/artifacts/pack \
  -H "Content-Type: application/json" \
  -d '{
    "source": {
      "kind": "github",
      "locator": "https://github.com/user/repo",
      "revision": "main"
    },
    "profile": "pack",
    "query": "authentication flow",
    "policy": { "mode": "redact", "scan_secrets": true },
    "budget": { "max_tokens": 30000 },
    "output": { "format": "json", "include_reasons": true }
  }'
```

**Endpoints:**

| Method | Endpoint | Description |
|---|---|---|
| POST | `/v1/artifacts/overview` | Generate overview artifact |
| POST | `/v1/artifacts/pack` | Generate task pack |
| POST | `/v1/artifacts/review-diff` | Generate review pack |
| POST | `/v1/artifacts/agent` | Generate agent pack |
| POST | `/v1/artifacts/safe-share` | Generate safe pack |
| GET | `/v1/jobs/{id}` | Get job status |
| GET | `/v1/jobs/{id}/result` | Get job result |
| GET | `/health` | Health check |
| GET | `/metrics` | Prometheus metrics |
| GET | `/docs` | Swagger UI |

---

### MCP Server

Pristine ships a native MCP server. Add it to your IDE assistant, agent, or coding tool:

```json
{
  "mcpServers": {
    "pristine": {
      "command": "pristine",
      "args": ["mcp"]
    }
  }
}
```

**Available tools:**

| Tool | Description |
|---|---|
| `overview_repo` | Generate an overview artifact |
| `pack_context` | Generate a task-focused context pack |
| `review_diff` | Generate a review pack for a diff |
| `search_files` | Search for files by query or pattern |
| `read_file` | Read a file with optional compression and policy checks |
| `safe_share` | Generate a safe-to-share artifact |

---

### Web UI

Visit `http://localhost:8080` after starting the server. The web UI is built with React, TypeScript, and shadcn UI. It supports all five profiles, dark mode, file tree exploration, code viewing, stats, selection reason inspection, and raw JSON/Markdown output.

---

## Architecture

### Crate structure

Pristine is a Rust workspace with eleven crates and a separate TypeScript frontend:

```
crates/
├── pristine-domain/       Core types, configs, profiles, schemas
├── pristine-acquire/      Source parsing, auth, git access, snapshots
├── pristine-inventory/    File traversal, ignore rules, catalog creation
├── pristine-classify/     File tagging, framework markers, entrypoints
├── pristine-select/       Relevance rules, budgeting, inclusion levels
├── pristine-compress/     Text and syntax-aware compression
├── pristine-assemble/     Artifact construction
├── pristine-render/       JSON/Markdown/XML/text renderers
├── pristine-cache/        Snapshot and artifact caches
├── pristine-security/     Secret scanning, policy enforcement
├── pristine-cli/          CLI adapter
├── pristine-server/       HTTP API
└── pristine-mcp/          MCP server

web/                       React + TypeScript + shadcn UI frontend
```

The domain crate has no upstream dependencies. Every other crate depends on it. The CLI, server, and MCP crates are thin adapters — they configure and invoke the engine, they do not own domain logic.

### Technology choices

| Need | Choice |
|---|---|
| Async runtime | `tokio` |
| HTTP server | `axum` |
| CLI | `clap` |
| Git operations | `git2` |
| Filesystem traversal | `ignore` (ripgrep's engine) |
| Pattern matching | `globset` |
| Syntax parsing | `tree-sitter` |
| Token estimation | `tiktoken-rs` |
| Caching | `moka` |
| Metrics | `metrics` + `metrics-exporter-prometheus` |
| Error handling | `thiserror` + `anyhow` |
| Frontend | React 18 + Vite + shadcn UI + Tailwind CSS |

### Performance

Pristine is fast because:
- File inventory runs in parallel using bounded `tokio` tasks.
- Compression of independent files runs concurrently.
- Snapshots and artifacts are cached by content-addressed fingerprint.
- Shallow and sparse clones are preferred where the profile allows it.
- The binary is statically linked — startup time is under 10ms.

Target benchmarks:

| Operation | Target |
|---|---|
| Overview Pack (medium repo, cache miss) | < 2s |
| Task Pack (medium repo, cache miss) | < 5s |
| Repeated run (cache hit) | < 50ms |
| p95 across all profiles | < 15s |

---

## Safety and Security

Pristine treats security as a first-class pipeline concern, not an afterthought.

### Secret scanning

Before delivery, Pristine scans all selected file content for common secret patterns: AWS access keys, GitHub tokens, private keys, generic API keys, and more. Scanning happens in two passes: before selection (on candidates) and after compression (on rendered content).

### Policy modes

| Mode | Behavior |
|---|---|
| `allow` | Emit detected secrets with a warning |
| `redact` | Replace secrets with `[REDACTED: <type>]` markers |
| `fail` | Exit with code 5 if any secret is detected |

The `safe-share` profile defaults to `redact` mode and excludes common credential paths (`.env*`, `*secret*`, `*key*`, `credentials*`) by default.

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Source not found |
| 4 | Policy violation |
| 5 | Secret detected (with `--policy fail`) |
| 6 | Budget exceeded |
| 7 | Network error |
| 8 | Git error |

---

## Configuration Reference

### Config file locations (in order of precedence)

1. `--config <path>` flag
2. `.pristine.yaml` in current directory
3. `.pristine.yml` in current directory
4. `.pristine.json` in current directory
5. `~/.config/pristine/config.yaml` (global config)

### Full configuration schema

```yaml
# .pristine.yaml

default_profile: overview         # overview | pack | review_diff | agent | safe_share

output:
  format: markdown                 # json | markdown | xml | text
  include_tree: true
  include_stats: true
  include_reasons: true

budget:
  max_tokens: 50000
  max_bytes: 10485760              # 10 MB
  max_files: 100
  compression_preference: light   # none | light | structural | summary

policy:
  mode: allow                      # allow | redact | fail
  scan_secrets: true
  allow_patterns: ["*.rs", "*.py"]
  block_patterns: ["vendor/**", "node_modules/**"]
  redact_patterns: ["*secret*", "*key*", "*token*"]
  max_file_size: 1048576           # 1 MB

source:
  default_revision: main
  shallow_clone: true
  auth:
    github_token: ${GITHUB_TOKEN}  # or use PRISTINE_GITHUB_TOKEN env var

cache:
  enabled: true
  directory: .pristine/cache
  max_size: 1073741824             # 1 GB
  ttl: 86400                       # 24 hours

server:
  host: 0.0.0.0
  port: 8080
  cors_origins: ["http://localhost:3000"]
  rate_limit:
    requests_per_minute: 60
    burst_size: 10

logging:
  level: info                      # trace | debug | info | warn | error
  format: json                     # json | pretty

ignore:
  patterns: ["*.log", "*.tmp", ".env*", "target/", "__pycache__/"]
  use_gitignore: true
  use_pristineignore: true

compression:
  default_mode: light
  tree_sitter:
    enabled: true
    languages: [rust, python, typescript, javascript, go]
```

### Environment variables

| Variable | Description |
|---|---|
| `PRISTINE_GITHUB_TOKEN` | GitHub personal access token |
| `PRISTINE_CONFIG` | Path to config file |
| `PRISTINE_DEFAULT_PROFILE` | Default profile |
| `PRISTINE_CACHE_DIR` | Cache directory path |
| `PRISTINE_LOG_LEVEL` | Log level |
| `PRISTINE_SERVER_PORT` | Server port |

---

## Self-Hosting

### Docker Compose

```yaml
# compose.yml
services:
  pristine:
    image: ghcr.io/aymenbelarbi/pristine:latest
    ports:
      - "8080:8080"
    environment:
      PRISTINE_GITHUB_TOKEN: ${GITHUB_TOKEN}
      PRISTINE_LOG_LEVEL: info
    volumes:
      - pristine-cache:/app/.cache
      - ./pristine.yaml:/app/pristine.yaml
    command: ["pristine-server", "--config", "/app/pristine.yaml"]

volumes:
  pristine-cache:
```

### Kubernetes

Helm chart available at `helm/pristine`. Quick install:

```bash
helm repo add pristine https://charts.pristine.dev
helm install pristine pristine/pristine \
  --set env.PRISTINE_GITHUB_TOKEN=$GITHUB_TOKEN \
  --set server.port=8080
```

---

## CI/CD Integration

### GitHub Actions — generate a review artifact on every PR

```yaml
name: Pristine Review

on:
  pull_request:
    branches: [main]

jobs:
  context:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Install Pristine
        run: curl -sSf https://pristine.dev/install.sh | sh
      - name: Generate review artifact
        run: |
          pristine review-diff . \
            --base ${{ github.event.pull_request.base.sha }} \
            --head ${{ github.event.pull_request.head.sha }} \
            --format json \
            --policy redact \
            --output review.json
      - uses: actions/upload-artifact@v4
        with:
          name: pristine-review
          path: review.json
```

---

## Roadmap

### Alpha (current)
- Local repo and GitHub support
- Overview Pack and Task Pack
- Markdown and JSON output
- Layered inclusion levels
- Secret scanning and policy modes
- CLI + lightweight HTTP API + MCP server

### Private Beta
- Review Pack (diff-centric workflows)
- Artifact caching
- Tree-sitter compression (Rust, Python, TypeScript)
- Web UI (shadcn UI)
- Safe Pack with strict defaults

### Public Beta
- Stable artifact schema versions
- Full MCP tool coverage
- Go + Java Tree-sitter grammars
- Performance benchmarks
- Docker + Kubernetes

### v1.0
- GitLab support
- Agent Pack optimizations
- Enterprise policy controls
- Audit logs
- SSO

---

## Contributing

Pristine is open source under the MIT OR Apache-2.0 license.

```bash
git clone https://github.com/aymenbelarbi/pristine
cd pristine
cargo build
cargo test

# Run the CLI in dev mode
cargo run --bin pristine -- overview ./

# Run the web UI in dev mode
cd web && npm install && npm run dev
```

Before contributing, please read `CONTRIBUTING.md` and open an issue to discuss significant changes.

---

## License

Pristine is licensed under the [MIT License](LICENSE-MIT)

---

<div align="center">

Built with Rust. Designed for engineers who care about what goes into a prompt.

[GitHub](https://github.com/aymenbelarbi/pristine) · [Docs](https://pristine.dev/docs) · [API Reference](https://pristine.dev/api) · [Discord](https://discord.gg/pristine)

</div>
