# M6 — Antigravity Provider Integration

**Project:** AI Usage Dock  
**Status:** LOCKED until M5 PASS  
**Last updated:** 2026-07-19

## Objective

Implement Antigravity as the second and final provider using the established safe provider boundary. Do not redesign the application or add new product surfaces.

## Dependencies

M5 PASS

## Deliverables

- detection and connection.
- safe fetch and parsing.
- normalized snapshot.
- remaining quota UI/tray integration.
- cross-provider refresh isolation.

## Scope Boundaries

- Preserve the security and architecture boundaries in `../../AGENTS.md`.
- Do not add removed provider, chart, appearance, API dashboard, or mobile scope.
- Unknown usage must never become zero.

## Acceptance Criteria

- [ ] Antigravity works end-to-end.
- [ ] Provider failure is isolated.
- [ ] Popup and tray use remaining semantics.
- [ ] No new product complexity is introduced.

## Task Files

See the matching folder under `../tasks/`.
