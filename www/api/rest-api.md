# REST API Reference

All endpoints except `/healthz` require authentication:

```
Authorization: Bearer <API_BEARER_TOKEN>
```

## Ingest

### POST /v1/l/batch

Batch ingest traces and observations.

**Request Body:**

```json
{
  "trace": {
    "id": "uuid",
    "timestamp": "2026-01-01T00:00:00Z",
    "name": "chat",
    "userId": "alice",
    "session_id": "sess-001",
    "release": "v1.0",
    "version": "v2.1",
    "tags": ["prod", "gpt-4"],
    "metadata": {"custom_key": "value"},
    "environment": "default"
  },
  "observations": [
    {
      "id": "uuid",
      "traceId": "uuid",
      "type": "GENERATION",
      "name": "llm-call",
      "startTime": "2026-01-01T00:00:00Z",
      "endTime": "2026-01-01T00:00:01Z",
      "model": "gpt-4o-mini",
      "input": {"role": "user", "content": "Hello"},
      "output": {"role": "assistant", "content": "Hi!"},
      "promptTokens": 5,
      "completionTokens": 12,
      "totalTokens": 17,
      "parentObservationId": null,
      "metadata": {"agent_role": "Planner"}
    }
  ]
}
```

**Response:** `200 OK`

```json
{
  "message": "Request Successful."
}
```

::: tip Nested Spans
Use `parentObservationId` to create span trees. For agent workflows, nest Plan → Execute → Reflect spans under a parent cycle span.
:::

## Query

### GET /api/public/traces

Paginated trace list with filtering.

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default 1) |
| `limit` | integer | Items per page (1–200, default 50) |
| `userId` | string | Filter by user ID |
| `sourceIp` | string | Filter by `metadata.sourceIp` (normalized). Omit to skip. `__empty__` matches missing or blank IP |
| `name` | string | Filter by trace name |
| `sessionId` | string | Filter by session ID |
| `fromTimestamp` | ISO 8601 | Start time (inclusive) |
| `toTimestamp` | ISO 8601 | End time (inclusive) |
| `orderBy` | string | Sort: `timestamp.desc`, `latency.desc`, `totalCost.asc`, etc. |
| `tags` | string[] | Filter by tags (all-of semantics) |
| `version` | string | Filter by version |
| `release` | string | Filter by release |
| `environment` | string[] | Filter by environment |
| `fields` | string | Comma-separated: `io`, `scores`, `observations`, `metrics` |

**Response:**

```json
{
  "data": [
    {
      "id": "2b19f7aa-...",
      "timestamp": "2026-01-01T00:00:00Z",
      "name": "chat",
      "sessionId": "sess-001",
      "version": "v2.1",
      "release": "v1.0",
      "userId": "alice",
      "tags": ["prod"],
      "latency": 1.234,
      "totalCost": 0.005,
      "observations": ["96e16fda-..."],
      "scores": []
    }
  ],
  "meta": {
    "page": 1,
    "limit": 50,
    "totalItems": 1,
    "totalPages": 1
  }
}
```

List and detail rows always include `metadata.sourceIp` (empty string when unknown). IP comparison trims, strips `:port`, maps `::ffff:x.x.x.x` to IPv4, and is case-insensitive.

### GET /api/public/traces/facets

Dropdown facets for `sourceIp` or `userId` over a time window.

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `field` | string | Required: `sourceIp` or `userId` |
| `fromTimestamp` | ISO 8601 | Required start time (inclusive) |
| `toTimestamp` | ISO 8601 | Required end time (inclusive) |
| `tags` | string[] | Same all-of filter as the list |
| `limit` | integer | Default 200, max 1000 |

**Response:**

```json
{
  "data": [
    { "value": "192.168.1.10", "count": 14 },
    { "value": "", "count": 3 }
  ]
}
```

### GET /api/public/traces/:traceId

Get a single trace with all its observations.

**Response:**

```json
{
  "id": "2b19f7aa-...",
  "timestamp": "2026-01-01T00:00:00Z",
  "name": "chat",
  "observations": [
    {
      "id": "96e16fda-...",
      "traceId": "2b19f7aa-...",
      "type": "GENERATION",
      "name": "llm-call",
      "model": "gpt-4o-mini",
      "startTime": "2026-01-01T00:00:00Z",
      "endTime": "2026-01-01T00:00:01Z",
      "parentObservationId": null,
      "usage": {
        "input": 5,
        "output": 12,
        "total": 17
      },
      "latency": 1.234,
      "metadata": {"agent_role": "Planner"}
    }
  ]
}
```

### GET /api/public/metrics/daily

Daily aggregated metrics with per-model token usage breakdown.

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number |
| `limit` | integer | Items per page |
| `traceName` | string | Filter by trace name |
| `userId` | string | Filter by user ID |
| `tags` | string[] | Filter by tags |
| `fromTimestamp` | ISO 8601 | Start time |
| `toTimestamp` | ISO 8601 | End time |
| `version` | string | Filter by trace version |
| `release` | string | Filter by trace release |

**Response:**

```json
{
  "data": [
    {
      "date": "2026-01-15",
      "countTraces": 150,
      "countObservations": 430,
      "totalCost": 12.50,
      "usage": [
        {
          "model": "gpt-4o-mini",
          "inputUsage": 50000,
          "outputUsage": 120000,
          "totalUsage": 170000,
          "countTraces": 100,
          "countObservations": 280,
          "totalCost": 8.50
        }
      ]
    }
  ],
  "meta": { "page": 1, "limit": 50, "totalItems": 30, "totalPages": 1 }
}
```

## Error Responses

All errors follow a consistent format:

```json
{
  "message": "Human-readable error message",
  "code": "MACHINE_READABLE_CODE",
  "data": null
}
```

| HTTP Status | Code | Retryable |
|------------|------|-----------|
| 400 | `BAD_REQUEST` | No |
| 401 | `UNAUTHORIZED` | No |
| 404 | `NOT_FOUND` | No |
| 429 | `TOO_MANY_REQUESTS` | Yes |
| 500 | `INTERNAL_ERROR` | Yes |
| 503 | `SERVICE_UNAVAILABLE` | Yes |
