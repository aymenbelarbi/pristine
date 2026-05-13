# Pristine API Reference

## Base URL

```
http://localhost:8080
```

## Authentication

The API currently does not require authentication for local deployments. For production deployments, configure authentication via environment variables or reverse proxy.

## Content Types

- **Request:** `application/json`
- **Response:** `application/json`

## Endpoints

### Health Check

#### `GET /health`

Check the health status of the API server.

**Response:**

```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

---

### Generate Overview Artifact

#### `POST /v1/artifacts/overview`

Generate an overview artifact for a repository. This provides architecture, entrypoints, key patterns, and tech stack information.

**Request Body:**

```json
{
  "source": {
    "kind": "github",
    "locator": "https://github.com/user/repo",
    "revision": "main",
    "subpath": "src/"
  },
  "profile": "overview",
  "policy": {
    "mode": "redact",
    "scan_secrets": true
  },
  "budget": {
    "max_tokens": 50000,
    "compression_preference": "light"
  },
  "output": {
    "format": "json",
    "include_tree": true,
    "include_stats": true,
    "include_reasons": true
  }
}
```

**Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source.kind` | string | Yes | Source type: `local`, `github`, `gitlab` |
| `source.locator` | string | Yes | Repository URL or local path |
| `source.revision` | string | No | Git branch, tag, or commit SHA |
| `source.subpath` | string | No | Subdirectory path within repository |
| `policy.mode` | string | No | Policy mode: `allow`, `redact`, `fail` |
| `policy.scan_secrets` | boolean | No | Enable secret scanning (default: true) |
| `budget.max_tokens` | integer | No | Maximum token budget (default: 50000) |
| `budget.compression_preference` | string | No | Compression: `none`, `light`, `structural`, `summary` |
| `output.format` | string | No | Output format: `json`, `markdown`, `xml`, `text` |
| `output.include_tree` | boolean | No | Include directory tree (default: true) |
| `output.include_stats` | boolean | No | Include statistics (default: true) |
| `output.include_reasons` | boolean | No | Include selection reasons (default: true) |

**Response:**

```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "completed",
  "artifact": {
    "artifact_type": "overview",
    "metadata": {
      "artifact_version": "1.0.0",
      "generator_version": "0.1.0",
      "source": {
        "kind": "github",
        "locator": "https://github.com/user/repo",
        "revision": "main"
      },
      "revision": "abc123def456",
      "profile": "overview",
      "policy_mode": "redact",
      "generated_at": "2024-01-15T10:30:00Z",
      "fingerprint": "sha256:abc123..."
    },
    "file_units": [
      {
        "path": "README.md",
        "content": "# Project Name\n\n...",
        "inclusion": "full",
        "language": "markdown",
        "line_count": 150,
        "token_count": 2500,
        "reasons": ["entrypoint", "high_score"],
        "compression_mode": "none"
      }
    ],
    "stats": {
      "total_files": 150,
      "included_files": 45,
      "full_files": 20,
      "compressed_files": 15,
      "summary_files": 10,
      "tree_only_files": 0,
      "excluded_files": 105,
      "total_bytes": 1048576,
      "total_tokens": 45000,
      "compression_ratio": 0.65
    },
    "warnings": []
  }
}
```

---

### Generate Task Pack

#### `POST /v1/artifacts/pack`

Generate a task-focused context pack. Files are ranked by relevance to the query.

**Request Body:**

```json
{
  "source": {
    "kind": "local",
    "locator": "/path/to/project"
  },
  "profile": "pack",
  "query": "JWT authentication flow",
  "policy": {
    "mode": "redact",
    "scan_secrets": true
  },
  "budget": {
    "max_tokens": 30000,
    "compression_preference": "structural"
  },
  "output": {
    "format": "json",
    "include_reasons": true
  }
}
```

**Additional Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | string | Yes | Task or query describing needed context |

---

### Generate Review Pack

#### `POST /v1/artifacts/review-diff`

Generate a review pack for a diff between two revisions.

**Request Body:**

```json
{
  "source": {
    "kind": "github",
    "locator": "https://github.com/user/repo"
  },
  "profile": "review_diff",
  "diff": {
    "base": "main",
    "head": "feature/auth-refactor"
  },
  "policy": {
    "mode": "redact",
    "scan_secrets": true
  },
  "budget": {
    "max_tokens": 40000
  },
  "output": {
    "format": "json",
    "include_reasons": true
  }
}
```

**Additional Parameters:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `diff.base` | string | Yes | Base revision (branch, tag, or commit) |
| `diff.head` | string | Yes | Head revision (branch, tag, or commit) |

---

### Generate Agent Pack

#### `POST /v1/artifacts/agent`

Generate a structured agent pack for programmatic consumption.

**Request Body:**

```json
{
  "source": {
    "kind": "local",
    "locator": "/path/to/project"
  },
  "profile": "agent",
  "query": "database connection pooling",
  "policy": {
    "mode": "redact",
    "scan_secrets": true
  },
  "budget": {
    "max_tokens": 20000
  },
  "output": {
    "format": "json"
  }
}
```

---

### Generate Safe Share Pack

#### `POST /v1/artifacts/safe-share`

Generate a safe-to-share artifact with strict policy enforcement.

**Request Body:**

```json
{
  "source": {
    "kind": "local",
    "locator": "/path/to/project"
  },
  "profile": "safe_share",
  "policy": {
    "mode": "redact",
    "scan_secrets": true,
    "block_patterns": ["vendor/**", "node_modules/**", ".env*"]
  },
  "budget": {
    "max_tokens": 25000,
    "compression_preference": "structural"
  },
  "output": {
    "format": "markdown"
  }
}
```

---

### Get Job Status

#### `GET /v1/jobs/{id}`

Check the status of an async job.

**Response:**

```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "processing",
  "progress": 0.65,
  "stage": "compress"
}
```

**Status Values:**

| Status | Description |
|--------|-------------|
| `pending` | Job is queued |
| `processing` | Job is being processed |
| `completed` | Job completed successfully |
| `failed` | Job failed with an error |

---

### Get Job Result

#### `GET /v1/jobs/{id}/result`

Retrieve the result of a completed job.

**Response:** Same as the artifact response from the generation endpoints.

---

### Metrics

#### `GET /metrics`

Prometheus-compatible metrics endpoint.

**Response:**

```
# HELP pristine_requests_total Total number of requests
# TYPE pristine_requests_total counter
pristine_requests_total{profile="overview"} 150
pristine_requests_total{profile="pack"} 230

# HELP pristine_request_duration_seconds Request duration in seconds
# TYPE pristine_request_duration_seconds histogram
pristine_request_duration_seconds_bucket{le="0.1"} 100
pristine_request_duration_seconds_bucket{le="0.5"} 200
pristine_request_duration_seconds_bucket{le="1.0"} 280
```

---

### Swagger UI

#### `GET /docs`

Interactive API documentation via Swagger UI.

---

## Error Responses

All endpoints return errors in a consistent format:

```json
{
  "error": {
    "code": "INVALID_SOURCE",
    "message": "Source not found: https://github.com/nonexistent/repo",
    "details": {}
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_REQUEST` | 400 | Malformed request body |
| `INVALID_SOURCE` | 400 | Invalid source format |
| `SOURCE_NOT_FOUND` | 404 | Repository not found |
| `GIT_ERROR` | 502 | Git operation failed |
| `POLICY_VIOLATION` | 403 | Policy violation detected |
| `SECRET_DETECTED` | 403 | Secret detected with fail policy |
| `BUDGET_EXCEEDED` | 400 | Token budget exceeded |
| `INTERNAL_ERROR` | 500 | Internal server error |

---

## Rate Limiting

The API implements rate limiting to prevent abuse. Default limits:

- **Requests per minute:** 60
- **Burst size:** 10

Rate limit headers are included in responses:

```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1705312800
```

---

## Examples

### cURL Examples

**Generate an overview:**
```bash
curl -X POST http://localhost:8080/v1/artifacts/overview \
  -H "Content-Type: application/json" \
  -d '{
    "source": {
      "kind": "github",
      "locator": "https://github.com/rust-lang/rustfmt"
    },
    "output": { "format": "markdown" }
  }'
```

**Generate a task pack:**
```bash
curl -X POST http://localhost:8080/v1/artifacts/pack \
  -H "Content-Type: application/json" \
  -d '{
    "source": {
      "kind": "local",
      "locator": "./my-project"
    },
    "query": "error handling strategy",
    "budget": { "max_tokens": 30000 }
  }'
```

**Generate a review pack:**
```bash
curl -X POST http://localhost:8080/v1/artifacts/review-diff \
  -H "Content-Type: application/json" \
  -d '{
    "source": {
      "kind": "github",
      "locator": "https://github.com/user/repo"
    },
    "diff": {
      "base": "main",
      "head": "feature/new-feature"
    }
  }'
```

### JavaScript/TypeScript Examples

```typescript
const response = await fetch('http://localhost:8080/v1/artifacts/overview', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    source: {
      kind: 'github',
      locator: 'https://github.com/user/repo',
      revision: 'main'
    },
    output: { format: 'json' }
  })
});

const data = await response.json();
console.log(data.artifact.stats);
```

### Python Examples

```python
import requests

response = requests.post(
    'http://localhost:8080/v1/artifacts/pack',
    json={
        'source': {
            'kind': 'local',
            'locator': '/path/to/project'
        },
        'query': 'authentication flow',
        'budget': {'max_tokens': 30000}
    }
)

data = response.json()
print(f"Total tokens: {data['artifact']['stats']['total_tokens']}")
```
