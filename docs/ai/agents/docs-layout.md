# Documentation Layout & Lifecycle

This document describes the structure and lifecycle of the `docs/` tree.

## Directory Structure

- `docs/architecture.md`: Thin architecture entry.
- `docs/architecture/product-positioning.md`: Commercial + OTEL + Xrouter boundary.
- `docs/architecture/module-boundaries.md`: Ports, facts, no producer-table joins.
- `docs/design.md`: UI hard rules for `frontend/`.
- `docs/ai/agents/`: Engineering, commit style, loop charter.
- `docs/project_status.md`: Biweekly snapshot (human-refreshed; not auto-written).

## Document Lifecycle Discipline

1. **No Phantom Capabilities**: Never document skeleton-only or hypothetical features as ready.
2. **Deterministic Verification**: SQL schemas and code snippets inside documentation must be executable and verified.
