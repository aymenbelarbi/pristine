# Pristine Configuration Guide

## Configuration File Locations

Pristine looks for configuration files in the following locations (in order of precedence):

1. `--config <path>` flag (highest precedence)
2. `.pristine.yaml` in current directory
3. `.pristine.yml` in current directory
4. `.pristine.json` in current directory
5. `~/.config/pristine/config.yaml` (global config, lowest precedence)

## Configuration File Format

Configuration files can be in YAML (recommended) or JSON format.

### YAML Example

```yaml
# .pristine.yaml

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
  allow_patterns: ["*.rs", "*.py"]
  block_patterns: ["vendor/**", "node_modules/**"]
  redact_patterns: ["*secret*", "*key*", "*token*"]
  max_file_size: 1048576

source:
  default_revision: main
  shallow_clone: true
  auth:
    github_token: ${GITHUB_TOKEN}

cache:
  enabled: true
  directory: .pristine/cache
  max_size: 1073741824
  ttl: 86400

server:
  host: 0.0.0.0
  port: 8080
  cors_origins: ["http://localhost:3000"]
  rate_limit:
    requests_per_minute: 60
    burst_size: 10

logging:
  level: info
  format: pretty

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

### JSON Example

```json
{
  "default_profile": "overview",
  "output": {
    "format": "markdown",
    "include_tree": true,
    "include_stats": true,
    "include_reasons": true
  },
  "budget": {
    "max_tokens": 50000,
    "max_bytes": 10485760,
    "max_files": 100,
    "compression_preference": "light"
  },
  "policy": {
    "mode": "redact",
    "scan_secrets": true,
    "block_patterns": ["vendor/**", "node_modules/**"]
  }
}
```

## Configuration Options

### `default_profile`

**Type:** string  
**Default:** `overview`  
**Description:** The default profile to use when none is specified.

**Allowed values:**
- `overview` - Architecture, onboarding, codebase orientation
- `pack` - Task-focused implementation context
- `review_diff` - PR review, change analysis
- `agent` - Structured JSON context for agents
- `safe_share` - External sharing with policy enforcement

---

### `output`

Output configuration options.

#### `output.format`

**Type:** string  
**Default:** `markdown`  
**Description:** Default output format.

**Allowed values:**
- `json` - JSON format for programmatic consumption
- `markdown` - Markdown format for human reading
- `xml` - XML format for model-friendly structured output
- `text` - Plain text format

#### `output.include_tree`

**Type:** boolean  
**Default:** `true`  
**Description:** Include directory tree in output.

#### `output.include_stats`

**Type:** boolean  
**Default:** `true`  
**Description:** Include statistics (file counts, token counts, etc.) in output.

#### `output.include_reasons`

**Type:** boolean  
**Default:** `true`  
**Description:** Include selection reasons for each file.

#### `output.theme`

**Type:** string  
**Default:** `null`  
**Description:** Theme for rendered output (used by some renderers).

---

### `budget`

Budget configuration options.

#### `budget.max_tokens`

**Type:** integer  
**Default:** `50000`  
**Description:** Maximum token budget for the artifact.

#### `budget.max_bytes`

**Type:** integer  
**Default:** `10485760` (10 MB)  
**Description:** Maximum total bytes for included files.

#### `budget.max_files`

**Type:** integer  
**Default:** `100`  
**Description:** Maximum number of files to include.

#### `budget.compression_preference`

**Type:** string  
**Default:** `light`  
**Description:** Default compression mode when budget needs to be reduced.

**Allowed values:**
- `none` - No compression, full file content
- `light` - Remove boilerplate, collapse blank lines
- `structural` - Retain imports, signatures, type defs
- `summary` - Emit symbol inventory only

---

### `policy`

Policy and safety configuration.

#### `policy.mode`

**Type:** string  
**Default:** `allow`  
**Description:** Policy mode for handling secrets.

**Allowed values:**
- `allow` - Emit detected secrets with a warning
- `redact` - Replace secrets with `[REDACTED: <type>]` markers
- `fail` - Exit with code 5 if any secret is detected

#### `policy.scan_secrets`

**Type:** boolean  
**Default:** `true`  
**Description:** Enable secret scanning.

#### `policy.allow_patterns`

**Type:** array of strings  
**Default:** `[]`  
**Description:** Glob patterns for files to always include.

#### `policy.block_patterns`

**Type:** array of strings  
**Default:** `[]`  
**Description:** Glob patterns for files to always exclude.

#### `policy.redact_patterns`

**Type:** array of strings  
**Default:** `[]`  
**Description:** Glob patterns for files to always redact.

#### `policy.max_file_size`

**Type:** integer  
**Default:** `null`  
**Description:** Maximum file size in bytes. Larger files are excluded.

---

### `source`

Source repository configuration.

#### `source.default_revision`

**Type:** string  
**Default:** `main`  
**Description:** Default git revision to use.

#### `source.shallow_clone`

**Type:** boolean  
**Default:** `true`  
**Description:** Use shallow clones when possible for faster acquisition.

#### `source.auth.github_token`

**Type:** string  
**Default:** `${GITHUB_TOKEN}`  
**Description:** GitHub personal access token. Supports environment variable substitution.

---

### `cache`

Cache configuration.

#### `cache.enabled`

**Type:** boolean  
**Default:** `true`  
**Description:** Enable caching of snapshots and artifacts.

#### `cache.directory`

**Type:** string  
**Default:** `.pristine/cache`  
**Description:** Directory for cache storage.

#### `cache.max_size`

**Type:** integer  
**Default:** `1073741824` (1 GB)  
**Description:** Maximum cache size in bytes.

#### `cache.ttl`

**Type:** integer  
**Default:** `86400` (24 hours)  
**Description:** Cache time-to-live in seconds.

---

### `server`

Server configuration (for `pristine-server`).

#### `server.host`

**Type:** string  
**Default:** `0.0.0.0`  
**Description:** Server bind address.

#### `server.port`

**Type:** integer  
**Default:** `8080`  
**Description:** Server port.

#### `server.cors_origins`

**Type:** array of strings  
**Default:** `[]`  
**Description:** Allowed CORS origins.

#### `server.rate_limit.requests_per_minute`

**Type:** integer  
**Default:** `60`  
**Description:** Maximum requests per minute per client.

#### `server.rate_limit.burst_size`

**Type:** integer  
**Default:** `10`  
**Description:** Maximum burst size for rate limiting.

---

### `logging`

Logging configuration.

#### `logging.level`

**Type:** string  
**Default:** `info`  
**Description:** Log level.

**Allowed values:**
- `trace` - Most verbose
- `debug` - Debug information
- `info` - General information
- `warn` - Warnings only
- `error` - Errors only

#### `logging.format`

**Type:** string  
**Default:** `pretty`  
**Description:** Log format.

**Allowed values:**
- `json` - JSON format for machine parsing
- `pretty` - Human-readable format

---

### `ignore`

File ignore configuration.

#### `ignore.patterns`

**Type:** array of strings  
**Default:** `[]`  
**Description:** Additional glob patterns to ignore.

#### `ignore.use_gitignore`

**Type:** boolean  
**Default:** `true`  
**Description:** Respect `.gitignore` files.

#### `ignore.use_pristineignore`

**Type:** boolean  
**Default:** `true`  
**Description:** Respect `.pristineignore` files.

---

### `compression`

Compression configuration.

#### `compression.default_mode`

**Type:** string  
**Default:** `light`  
**Description:** Default compression mode.

#### `compression.tree_sitter.enabled`

**Type:** boolean  
**Default:** `true`  
**Description:** Enable Tree-sitter for structural compression.

#### `compression.tree_sitter.languages`

**Type:** array of strings  
**Default:** `[rust, python, typescript, javascript, go]`  
**Description:** Languages to use Tree-sitter for.

---

## Environment Variables

All configuration options can be set via environment variables. Environment variables take precedence over config file values but are overridden by CLI flags.

| Variable | Configuration Path | Description |
|----------|-------------------|-------------|
| `PRISTINE_GITHUB_TOKEN` | `source.auth.github_token` | GitHub token |
| `PRISTINE_CONFIG` | - | Path to config file |
| `PRISTINE_DEFAULT_PROFILE` | `default_profile` | Default profile |
| `PRISTINE_CACHE_DIR` | `cache.directory` | Cache directory |
| `PRISTINE_LOG_LEVEL` | `logging.level` | Log level |
| `PRISTINE_SERVER_PORT` | `server.port` | Server port |
| `PRISTINE_POLICY_MODE` | `policy.mode` | Policy mode |
| `PRISTINE_MAX_TOKENS` | `budget.max_tokens` | Max tokens |

---

## Configuration Examples

### Minimal Configuration

```yaml
# .pristine.yaml
default_profile: overview
budget:
  max_tokens: 30000
```

### Development Configuration

```yaml
# .pristine.yaml
default_profile: pack

output:
  format: markdown
  include_reasons: true

budget:
  max_tokens: 50000
  compression_preference: light

policy:
  mode: allow
  scan_secrets: false

logging:
  level: debug
  format: pretty
```

### Production Configuration

```yaml
# pristine.yaml
default_profile: overview

output:
  format: json
  include_tree: true
  include_stats: true

budget:
  max_tokens: 100000
  compression_preference: structural

policy:
  mode: redact
  scan_secrets: true
  block_patterns:
    - "vendor/**"
    - "node_modules/**"
    - ".env*"
    - "*secret*"
    - "*key*"

cache:
  enabled: true
  max_size: 2147483648  # 2 GB
  ttl: 172800  # 48 hours

server:
  host: 0.0.0.0
  port: 8080
  cors_origins: ["https://app.example.com"]
  rate_limit:
    requests_per_minute: 120
    burst_size: 20

logging:
  level: info
  format: json
```

### CI/CD Configuration

```yaml
# .pristine.yaml
default_profile: review_diff

output:
  format: json
  include_reasons: true

budget:
  max_tokens: 40000
  compression_preference: structural

policy:
  mode: redact
  scan_secrets: true

cache:
  enabled: true
  directory: /tmp/pristine-cache

logging:
  level: warn
  format: json
```

### Safe Share Configuration

```yaml
# .pristine.yaml
default_profile: safe_share

output:
  format: markdown
  include_tree: true
  include_stats: true

budget:
  max_tokens: 25000
  compression_preference: structural

policy:
  mode: redact
  scan_secrets: true
  block_patterns:
    - ".env*"
    - "*secret*"
    - "*key*"
    - "*credential*"
    - "vendor/**"
    - "node_modules/**"
    - "*.lock"

compression:
  default_mode: structural
```

---

## Configuration Validation

Validate your configuration file:

```bash
pristine config validate
```

Show the current effective configuration:

```bash
pristine config show
```

---

## `.pristineignore` File

Create a `.pristineignore` file in your repository root to specify additional files to ignore:

```
# Dependencies
vendor/
node_modules/

# Build artifacts
target/
dist/
build/

# Environment files
.env
.env.local
.env.*.local

# IDE
.idea/
.vscode/

# OS
.DS_Store
Thumbs.db

# Logs
*.log

# Cache
.pristine/cache/
```

---

## Configuration Precedence

Configuration values are resolved in the following order (highest to lowest precedence):

1. CLI flags
2. Environment variables
3. Project config file (`.pristine.yaml`)
4. Global config file (`~/.config/pristine/config.yaml`)
5. Default values

This allows you to set sensible defaults in a project config file while still allowing overrides via environment variables or CLI flags.
