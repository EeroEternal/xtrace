---
name: admin-ui-change
description: Change xtrace frontend UI with progressive reading of docs/design.md. Use for dashboard pages, components, dialogs, lists, settings, tokens, or any user-visible UI work.
---

# Frontend UI change

## Rule
[`docs/design.md`](../../../docs/design.md) is the **only UI design entry**. Do not invent a second visual dialect. Prefer primitives under `frontend/src/components/`.

## Always load
1. `docs/design.md` (whole file)
2. `AGENTS.md` §始终生效 → UI
3. [`docs/ai/agents/ui-entry.md`](../../../docs/ai/agents/ui-entry.md)

## Verify
1. Re-check PR checklist in `docs/design.md`.
2. `cd frontend && npm run lint`
3. Dialogs stay within `max-h-[85vh]` with internal scroll.
