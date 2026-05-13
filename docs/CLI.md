# Pristine CLI Reference

## Installation

### Homebrew (macOS/Linux)

```bash
brew install pristine
```

### Cargo

```bash
cargo install pristine
```

### Download Binary

```bash
curl -sSf https://pristine.dev/install.sh | sh
```

### Docker

```bash
docker pull ghcr.io/aymenbelarbi/pristine:latest
```

## Global Options

These options are available for all commands:

| Flag | Description |
|------|-------------|
| `--config <PATH>` | Path to configuration file |
| `--verbose` | Enable verbose output |
| `--help` | Print help information |
| `--version` | Print version information |

## Commands

### `pristine overview`

Generate an overview artifact for a repository. This provides architecture, entrypoints, key patterns, and tech stack information.

**Usage:**

```bash
pristine overview <SOURCE> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `SOURCE` | Yes | Repository URL or local path |

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--revision <REV>` | `main` | Git branch, tag, or commit SHA |
| `--subpath <PATH>` | | Subdirectory path |
| `--format <FMT>` | `markdown` | Output format: `json`, `markdown`, `xml`, `text` |
| `--output <FILE>` | stdout | Output file path |
| `--explain` | false | Show selection reasons per file |
| `--max-tokens <N>` | 50000 | Maximum token budget |
| `--policy <MODE>` | `allow` | Policy mode: `allow`, `redact`, `fail` |

**Examples:**

```bash
# Overview of a GitHub repository
pristine overview https://github.com/rust-lang/rustfmt

# Overview with specific revision
pristine overview https://github.com/user/repo --revision v1.0.0

# Overview of a subdirectory
pristine overview ./my-project --subpath src/

# Save to file
pristine overview https://github.com/user/repo --output overview.md

# JSON output with explanations
pristine overview ./my-project --format json --explain --output overview.json
```

---

### `pristine pack`

Generate a task-focused context pack. Files are ranked by relevance to the query.

**Usage:**

```bash
pristine pack <SOURCE> --query <QUERY> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `SOURCE` | Yes | Repository URL or local path |

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--query <QUERY>` | | Task or query describing needed context |
| `--revision <REV>` | `main` | Git branch, tag, or commit SHA |
| `--subpath <PATH>` | | Subdirectory path |
| `--max-tokens <N>` | 50000 | Maximum token budget |
| `--format <FMT>` | `markdown` | Output format: `json`, `markdown`, `xml`, `text` |
| `--output <FILE>` | stdout | Output file path |
| `--explain` | false | Show selection reasons per file |
| `--policy <MODE>` | `allow` | Policy mode: `allow`, `redact`, `fail` |
| `--compression <MODE>` | `light` | Compression: `none`, `light`, `structural`, `summary` |

**Examples:**

```bash
# Pack context for authentication task
pristine pack ./my-project --query "JWT authentication flow"

# Pack with token budget
pristine pack https://github.com/user/repo --query "error handling" --max-tokens 30000

# Pack with structural compression
pristine pack ./my-project --query "database models" --compression structural

# Save as JSON
pristine pack ./my-project --query "API endpoints" --format json --output api-context.json
```

---

### `pristine review-diff`

Generate a review pack for a diff between two revisions.

**Usage:**

```bash
pristine review-diff <SOURCE> --base <BASE> --head <HEAD> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `SOURCE` | Yes | Repository URL or local path |

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--base <REV>` | Yes | Base revision (branch, tag, or commit) |
| `--head <REV>` | Yes | Head revision (branch, tag, or commit) |
| `--format <FMT>` | `markdown` | Output format: `json`, `markdown`, `xml`, `text` |
| `--output <FILE>` | stdout | Output file path |
| `--explain` | false | Show selection reasons per file |
| `--policy <MODE>` | `allow` | Policy mode: `allow`, `redact`, `fail` |

**Examples:**

```bash
# Review a feature branch
pristine review-diff ./my-project --base main --head feature/auth

# Review with PR reference
pristine review-diff https://github.com/user/repo --base main --head pull/123/head

# Save review as JSON
pristine review-diff ./my-project --base v1.0.0 --head v2.0.0 --format json --output review.json

# Review with explanations
pristine review-diff ./my-project --base main --head HEAD --explain
```

---

### `pristine agent`

Generate a structured agent pack for programmatic consumption.

**Usage:**

```bash
pristine agent <SOURCE> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `SOURCE` | Yes | Repository URL or local path |

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--revision <REV>` | `main` | Git branch, tag, or commit SHA |
| `--query <QUERY>` | | Optional query for task-focused context |
| `--format <FMT>` | `json` | Output format: `json`, `markdown`, `xml`, `text` |
| `--output <FILE>` | stdout | Output file path |
| `--max-tokens <N>` | 50000 | Maximum token budget |
| `--policy <MODE>` | `allow` | Policy mode: `allow`, `redact`, `fail` |

**Examples:**

```bash
# Agent pack for a repository
pristine agent ./my-project

# Agent pack with query
pristine agent https://github.com/user/repo --query "database connection pooling"

# Save agent pack
pristine agent ./my-project --output agent-context.json
```

---

### `pristine safe-share`

Generate a safe-to-share artifact with strict policy enforcement.

**Usage:**

```bash
pristine safe-share <SOURCE> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `SOURCE` | Yes | Repository URL or local path |

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--revision <REV>` | `main` | Git branch, tag, or commit SHA |
| `--policy <MODE>` | `redact` | Policy mode: `allow`, `redact`, `fail` |
| `--format <FMT>` | `markdown` | Output format: `json`, `markdown`, `xml`, `text` |
| `--output <FILE>` | stdout | Output file path |
| `--compression <MODE>` | `structural` | Compression: `none`, `light`, `structural`, `summary` |

**Examples:**

```bash
# Safe share with defaults (redact mode)
pristine safe-share ./my-project

# Safe share as JSON
pristine safe-share ./my-project --format json --output safe-context.json

# Safe share with fail policy
pristine safe-share ./my-project --policy fail
```

---

### `pristine config`

Configuration management commands.

**Subcommands:**

#### `pristine config init`

Initialize a new configuration file in the current directory.

```bash
pristine config init
```

Creates a `.pristine.yaml` file with default settings.

#### `pristine config validate`

Validate the configuration file.

```bash
pristine config validate
```

#### `pristine config show`

Show the current configuration.

```bash
pristine config show
```

---

### `pristine-server`

Start the HTTP API server.

**Usage:**

```bash
pristine-server [OPTIONS]
```

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--host <HOST>` | `0.0.0.0` | Server host |
| `--port <PORT>` | `8080` | Server port |
| `--config <PATH>` | | Path to configuration file |

**Examples:**

```bash
# Start server with defaults
pristine-server

# Start on custom port
pristine-server --port 3000

# Start with config file
pristine-server --config pristine.yaml
```

---

### `pristine mcp`

Start the MCP server for AI assistant integration.

**Usage:**

```bash
pristine mcp [OPTIONS]
```

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--config <PATH>` | | Path to configuration file |

---

## Exit Codes

| Code | Meaning |
|------|---------|
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

## Configuration File

Pristine looks for configuration in the following locations (in order of precedence):

1. `--config <path>` flag
2. `.pristine.yaml` in current directory
3. `.pristine.yml` in current directory
4. `.pristine.json` in current directory
5. `~/.config/pristine/config.yaml` (global config)

**Example `.pristine.yaml`:**

```yaml
default_profile: overview

output:
  format: markdown
  include_tree: true
  include_stats: true
  include_reasons: true

budget:
  max_tokens: 50000
  max_bytes: 10485760
  max_files: 100
  compression_preference: light

policy:
  mode: redact
  scan_secrets: true
  block_patterns:
    - "vendor/**"
    - "node_modules/**"
    - "*.lock"

source:
  default_revision: main
  shallow_clone: true

cache:
  enabled: true
  directory: .pristine/cache
  max_size: 1073741824
  ttl: 86400

logging:
  level: info
  format: pretty
```

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `PRISTINE_GITHUB_TOKEN` | GitHub personal access token |
| `PRISTINE_CONFIG` | Path to config file |
| `PRISTINE_DEFAULT_PROFILE` | Default profile |
| `PRISTINE_CACHE_DIR` | Cache directory path |
| `PRISTINE_LOG_LEVEL` | Log level |
| `PRISTINE_SERVER_PORT` | Server port |

---

## Common Use Cases

### Quick Repository Overview

```bash
pristine overview https://github.com/user/repo --output overview.md
```

### Task-Focused Context for LLM

```bash
pristine pack ./my-project \
  --query "implement user authentication" \
  --max-tokens 30000 \
  --format markdown \
  --output context.md
```

### PR Review Context

```bash
pristine review-diff ./my-project \
  --base main \
  --head feature/new-feature \
  --explain \
  --output review.md
```

### Safe External Sharing

```bash
pristine safe-share ./my-project \
  --policy redact \
  --compression structural \
  --output safe-context.md
```

### CI/CD Integration

```bash
# In GitHub Actions
- name: Generate context
  run: |
    pristine review-diff . \
      --base ${{ github.event.pull_request.base.sha }} \
      --head ${{ github.event.pull_request.head.sha }} \
      --format json \
      --output context.json
```

### Agent Integration

```bash
pristine agent ./my-project \
  --query "database schema" \
  --format json \
  --max-tokens 20000 \
  --output agent-context.json
```
