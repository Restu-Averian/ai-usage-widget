# M3 — Mock UI

**Project:** AI Usage Widget  
**Status:** PASS — final product scope corrected  
**Last updated:** 2026-07-19

## Objective

Define the production UI shape with fixed dark styling, Codex and Antigravity tabs, remaining-quota semantics, connection states, and minimal settings. Removed chart and appearance work is not part of the final product.

## Dependencies

M1 and partial M2

## Deliverables

- two provider tabs.
- remaining quota ring.
- freshness and error states.
- minimal settings.
- development-only scenarios.

## Scope Boundaries

- Preserve the security and architecture boundaries in `../../AGENTS.md`.
- Do not add removed provider, chart, appearance, API dashboard, or mobile scope.
- Unknown usage must never become zero.

## Acceptance Criteria

- [x] Only Codex and Antigravity remain.
- [x] Remaining quota labels are canonical.
- [x] Removed controls are absent from final spec.

## Task Files

See the matching folder under `../tasks/`.
