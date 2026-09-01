---
title: Default Module
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
code_clipboard: true
highlight_theme: darkula
headingLevel: 2
generator: "@tarslib/widdershins v4.0.30"


---

# Default Module

Base URLs:

# Authentication

- HTTP Authentication, scheme: bearer

# Compatibility

The existing Langfuse-compatible and OTLP endpoints are frozen contracts. Optimizations
are additive by default: new optional query parameters and response fields are allowed,
but existing field names, pagination defaults, response envelopes, query parameter names,
and status-code semantics must not change without a versioned migration plan.

The supported OTLP paths remain:

- `POST /api/public/otel/v1/traces`
- `POST /api/public/otel/v1/metrics`

Standard OTLP JSON, protobuf, and gzip requests continue to use these paths. Domain
correlation data belongs in metadata or OTEL attributes rather than new top-level
Langfuse fields. The gateway-side OTLP exporter is **not wired yet**; this repository
currently accepts and queries OTLP data but does not provide that producer integration.

Recommended correlation and semantic attribute names are:

- `gen_ai.request.model`
- `gen_ai.usage.input_tokens`
- `gen_ai.usage.output_tokens`
- `xrouter.request_id`
- `xrouter.route`
- `xrouter.provider`
- `xrouter.attempt_index`
- `xrouter.is_fallback`
- `xrouter.queue_wait_ms`

These names are conventions, not new database columns. Keep high-cardinality identifiers
such as request IDs and user IDs out of metric labels; use trace metadata or attributes
for correlation.

# Xinference/Model Monitoring

## GET Traces Endpoint

GET /api/public/traces

### Request Parameters

| Name          | Location | Type          | Required | Description              |
| ------------- | -------- | ------------- | -------- | ------------------------ |
| page          | query    | integer       | No       | Page number              |
| limit         | query    | integer       | No       | Limit of returned items  |
| userId        | query    | string        | No       | Recorded user ID         |
| name          | query    | string        | No       | Recorded name            |
| sessionId     | query    | string        | No       | Recorded session_id      |
| externalId    | query    | string        | No       | External correlation ID for trace reconciliation |
| requestId     | query    | string        | No       | Filter by request ID stored in trace metadata |
| fromTimestamp | query    | string        | No       | ISO 8601 format          |
| toTimestamp   | query    | string        | No       | ISO 8601 format          |
| orderBy       | query    | string        | No       | Sort order               |
| tags          | query    | array[string] | No       | Tags                     |

> Response Example

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "data": [
      {
        "id": "2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
        "timestamp": "2025-06-26T06:16:19.504000Z",
        "name": null,
        "input": null,
        "output": null,
        "sessionId": null,
        "release": null,
        "version": null,
        "userId": "administrator",
        "metadata": null,
        "tags": [],
        "public": false,
        "htmlPath": "/project/20250101/traces/2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
        "latency": 12.771000146865845,
        "totalCost": 0,
        "observations": [
          "96e16fda-f796-414d-8cfb-61f9a4343be0"
        ],
        "scores": [],
        "externalId": null,
        "bookmarked": false,
        "projectId": "20250101",
        "createdAt": "2025-06-26T06:16:27.893Z",
        "updatedAt": "2025-06-26T06:16:27.893Z"
      }
    ],
    "meta": {
      "page": 1,
      "limit": 50,
      "totalItems": 1,
      "totalPages": 1
    }
  }
}
```

### Response

| Status Code | Meaning                                                       | Description | Data Models |
| ----------- | ------------------------------------------------------------- | ----------- | ----------- |
| 200         | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)       | none        | Inline      |

### Response Data Structure

Status Code **200**

| Name             | Type          | Required | Constraints | Display Name | Description |
| ---------------- | ------------- | -------- | ----------- | ------------ | ----------- |
| » message        | string        | true     | none        |              | none        |
| » data           | object        | false    | none        |              | none        |
| »» data          | [object]      | false    | none        |              | none        |
| »»» id           | string        | false    | none        |              | none        |
| »»» timestamp    | string¦null   | false    | none        |              | none        |
| »»» name         | string¦null   | false    | none        |              | none        |
| »»» input        | string¦null   | false    | none        |              | none        |
| »»» output       | string¦null   | false    | none        |              | none        |
| »»» sessionId    | string¦null   | false    | none        |              | none        |
| »»» release      | string¦null   | false    | none        |              | none        |
| »»» version      | string¦null   | false    | none        |              | none        |
| »»» userId       | string¦null   | false    | none        |              | none        |
| »»» metadata     | object¦null   | false    | none        |              | none        |
| »»» tags         | [string]¦null | false    | none        |              | none        |
| »»» public       | boolean       | false    | none        |              | none        |
| »»» htmlPath     | string¦null   | false    | none        |              | none        |
| »»» latency      | string        | false    | none        |              | none        |
| »»» totalCost    | string        | false    | none        |              | none        |
| »»» observations | [string]      | false    | none        |              | none        |
| »»» scores       | [string]¦null | false    | none        |              | none        |
| »»» externalId   | string¦null   | false    | none        |              | none        |
| »»» bookmarked   | boolean       | false    | none        |              | none        |
| »»» projectId    | string        | false    | none        |              | none        |
| »»» createdAt    | string        | false    | none        |              | none        |
| »»» updatedAt    | string        | false    | none        |              | none        |
| »» meta          | object        | false    | none        |              | none        |
| »»» page         | integer       | false    | none        |              | none        |
| »»» limit        | integer       | false    | none        |              | none        |
| »»» totalItems   | integer       | false    | none        |              | none        |
| »»» totalPages   | integer       | false    | none        |              | none        |

## GET Trace Detail Endpoint

GET /api/public/traces/{trace_id}

### Request Parameters

| Name     | Location | Type   | Required | Description                                   |
| -------- | -------- | ------ | -------- | --------------------------------------------- |
| trace_id | path     | string | Yes      | The trace ID obtained from the traces endpoint |

> Response Example

> 200 Response

```json
{
  "message": "Request Successful.",
  "data": {
    "id": "2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
    "timestamp": "2025-06-26T06:16:19.504000Z",
    "name": null,
    "input": null,
    "output": null,
    "sessionId": null,
    "release": null,
    "version": null,
    "userId": "administrator",
    "metadata": null,
    "tags": [],
    "public": false,
    "htmlPath": "/project/20250101/traces/2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
    "latency": 12.77100014686584,
    "totalCost": 0,
    "observations": [
      {
        "id": "96e16fda-f796-414d-8cfb-61f9a4343be0",
        "traceId": "2b19f7aa-3c9e-4102-b31f-fdc461a9991d",
        "type": "GENERATION",
        "name": "chat",
        "startTime": "2025-06-26T06:16:19.504000Z",
        "endTime": "2025-06-26T06:16:32.275000Z",
        "completionStartTime": "2025-06-26T06:16:20.716000Z",
        "model": "qwen3",
        "modelParameters": null,
        "input": [
          {
            "role": "user",
            "content": "test"
          }
        ],
        "version": null,
        "metadata": {
          "stream": true,
          "stream_options": {
            "include_usage": true
          }
        },
        "output": "test",
        "usage": {
          "input": 13,
          "output": 353,
          "total": 366,
          "unit": "TOKENS"
        },
        "level": "DEFAULT",
        "statusMessage": null,
        "parentObservationId": null,
        "promptId": null,
        "promptName": null,
        "promptVersion": null,
        "modelId": null,
        "inputPrice": null,
        "outputPrice": null,
        "totalPrice": null,
        "calculatedInputCost": null,
        "calculatedOutputCost": null,
        "calculatedTotalCost": null,
        "latency": 12.771,
        "timeToFirstToken": 1.212,
        "completionTokens": 353,
        "unit": "TOKENS",
        "totalTokens": 366,
        "projectId": "20250101",
        "createdAt": "2025-06-26T06:16:28.040Z",
        "promptTokens": 13,
        "updatedAt": "2025-06-26T06:16:32.306Z"
      }
    ],
    "scores": [],
    "externalId": null,
    "bookmarked": false,
    "projectId": "20250101",
    "createdAt": "2025-06-26T06:16:27.893Z",
    "updatedAt": "2025-06-26T06:16:27.893Z"
  }
}
```

### Response

| Status Code | Meaning                                                       | Description | Data Models |
| ----------- | ------------------------------------------------------------- | ----------- | ----------- |
| 200         | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)       | none        | Inline      |

### Response Data Structure

Status Code **200**

| Name                     | Type          | Required | Constraints | Display Name | Description |
| ------------------------ | ------------- | -------- | ----------- | ------------ | ----------- |
| » message                | string        | true     | none        |              | none        |
| » data                   | object        | false    | none        |              | none        |
| »» id                    | string        | false    | none        |              | ID          |
| »» timestamp             | string        | false    | none        |              | none        |
| »» name                  | string¦null   | false    | none        |              | none        |
| »» input                 | string¦null   | false    | none        |              | none        |
| »» output                | string¦null   | false    | none        |              | none        |
| »» sessionId             | string¦null   | false    | none        |              | none        |
| »» release               | string¦null   | false    | none        |              | none        |
| »» version               | string¦null   | false    | none        |              | none        |
| »» userId                | string¦null   | false    | none        |              | none        |
| »» metadata              | object¦null   | false    | none        |              | none        |
| »» tags                  | [string]¦null | false    | none        |              | none        |
| »» public                | boolean       | false    | none        |              | none        |
| »» htmlPath              | string¦null   | false    | none        |              | none        |
| »» latency               | number        | false    | none        |              | none        |
| »» totalCost             | number        | false    | none        |              | none        |
| »» observations          | [object]      | false    | none        |              | none        |
| »»» id                   | string        | false    | none        |              | none        |
| »»» traceId              | string        | false    | none        |              | none        |
| »»» type                 | string        | false    | none        |              | none        |
| »»» name                 | string        | false    | none        |              | none        |
| »»» startTime            | string        | false    | none        |              | none        |
| »»» endTime              | string        | false    | none        |              | none        |
| »»» completionStartTime  | string        | false    | none        |              | none        |
| »»» model                | string        | false    | none        |              | none        |
| »»» modelParameters      | object¦null   | false    | none        |              | none        |
| »»» input                | [object]      | false    | none        |              | none        |
| »»»» role                | string        | false    | none        |              | none        |
| »»»» content             | string        | false    | none        |              | none        |
| »»» version              | string¦null   | false    | none        |              | none        |
| »»» metadata             | object        | false    | none        |              | none        |
| »»»» stream              | boolean       | false    | none        |              | none        |
| »»»» stream_options      | object        | false    | none        |              | none        |
| »»»»» include_usage      | boolean       | false    | none        |              | none        |
| »»» output               | string        | false    | none        |              | none        |
| »»» usage                | object        | false    | none        |              | none        |
| »»»» input               | integer       | false    | none        |              | none        |
| »»»» output              | integer       | false    | none        |              | none        |
| »»»» total               | integer       | false    | none        |              | none        |
| »»»» unit                | string        | false    | none        |              | none        |
| »»» level                | string        | false    | none        |              | none        |
| »»» statusMessage        | string¦null   | false    | none        |              | none        |
| »»» parentObservationId  | string¦null   | false    | none        |              | none        |
| »»» promptId             | string¦null   | false    | none        |              | none        |
| »»» promptName           | string¦null   | false    | none        |              | none        |
| »»» promptVersion        | string¦null   | false    | none        |              | none        |
| »»» modelId              | string¦null   | false    | none        |              | none        |
| »»» inputPrice           | string¦null   | false    | none        |              | none        |
| »»» outputPrice          | string¦null   | false    | none        |              | none        |
| »»» totalPrice           | string¦null   | false    | none        |              | none        |
| »»» calculatedInputCost  | string¦null   | false    | none        |              | none        |
| »»» calculatedOutputCost | string¦null   | false    | none        |              | none        |
| »»» calculatedTotalCost  | string¦null   | false    | none        |              | none        |
| »»» latency              | number        | false    | none        |              | none        |
| »»» timeToFirstToken     | number        | false    | none        |              | none        |
| »»» completionTokens     | integer       | false    | none        |              | none        |
| »»» unit                 | string        | false    | none        |              | none        |
| »»» totalTokens          | integer       | false    | none        |              | none        |
| »»» projectId            | string        | false    | none        |              | none        |
| »»» createdAt            | string        | false    | none        |              | none        |
| »»» promptTokens         | integer       | false    | none        |              | none        |
| »»» updatedAt            | string        | false    | none        |              | none        |
| »» scores                | [string]      | false    | none        |              | none        |
| »» externalId            | string¦null   | false    | none        |              | none        |
| »» bookmarked            | boolean       | false    | none        |              | none        |
| »» projectId             | string        | false    | none        |              | none        |
| »» createdAt             | string        | false    | none        |              | none        |
| »» updatedAt             | string        | false    | none        |              | none        |

# Data Models

All endpoints below are called via **HTTP REST API**.
Independent of Langfuse SDK, suitable for direct server requests, gateway forwarding, or unified monitoring scenarios.

### Endpoint List

- `GET /api/public/metrics`
  Returns high-level overview metrics (trace count, latency avg/p95/p99) or observation metrics (distinct trace IDs with error status) over a specified time window.
  Primary use: Dashboard overview widgets compatibility for integrations (e.g., Xinference's `/v1/l/metric/overview`).
  This is an overview endpoint, not a generic metrics timeseries query.

- `GET /api/public/metrics/query`
  Generic label-based metrics timeseries query over raw sample points.
  Query params: `name`, optional `from` / `to`, optional `labels` JSON filter, optional `step` (`1m`, `5m`, `1h`, `1d`), optional `agg` (`avg`, `max`, `min`, `sum`, `last`, `count`, `p50`, `p90`, `p99`), and optional `group_by` as a comma-separated list of label keys.
  Response shape: `{ data: [ { labels, values: [ { timestamp, value } ] } ], meta: { latest_ts, series_count, truncated } }`.
  Note: `p50` / `p90` / `p99` are true quantiles over raw sample points. Histogram-derived `{name}_count` / `{name}_sum` points from OTLP ingest are compatibility shims, not a source of true quantiles.
  `rate` / `increase` are intentionally not supported yet because xtrace stores raw samples, not cumulative counters.

- `GET /api/public/metrics/names`
  Returns the distinct metric names currently stored for the default project/environment.
  Primary use: autocomplete / discovery for the query endpoint.

- `GET /api/public/metrics/daily`
  Returns daily aggregated model invocation usage and cost statistics.
  Primary use: daily invocation volume, token usage, and cost analytics.

- `GET /api/public/traces`
  Query trace list endpoint for filtering and paginated retrieval of trace metadata.
  Typically used as the trace query entry point and trace ID retrieval endpoint.

- `GET /api/public/traces/{trace_id}`
  Query a single trace's detailed information, returning complete trace and observation data.
  Primary use: debugging, auditing, and performance analysis of single model invocations.

### Endpoint Relationship

| Endpoint                        | Method | Granularity      | Primary Use          |
| ------------------------------- | ------ | ---------------- | -------------------- |
| `/api/public/metrics`           | HTTP   | Overview metrics | Dashboard overview compatibility |
| `/api/public/metrics/query`     | HTTP   | Time series      | Label-based metric query |
| `/api/public/metrics/names`     | HTTP   | Discovery        | Metric name autocomplete |
| `/api/public/metrics/daily`     | HTTP   | Daily aggregation| Usage / cost analytics   |
| `/api/public/traces`            | HTTP   | Trace list       | Trace query and filtering|
| `/api/public/traces/{trace_id}` | HTTP   | Single trace detail | Trace debugging and analysis |

Metric naming and labels should follow OTel GenAI semantic conventions where applicable, for example `gen_ai.request.model` and `gen_ai.usage.input_tokens`. Keep high-cardinality values such as `user_id` and `request_id` out of metric labels; those belong on trace metadata and correlation fields instead.

https://api.reference.langfuse.com/#tag/trace/GET/api/public/traces/{traceId}
