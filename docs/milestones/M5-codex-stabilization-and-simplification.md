# M5 — Codex Stabilization and Simplification

**Project:** AI Usage Dock  
**Status:** IN PROGRESS  
**Last updated:** 2026-07-19

## Objective

Stabilize the tray and Codex app-server lifecycle, convert all user-facing quota to remaining semantics, remove obsolete product features, simplify persistence, and pass regression gates before any Antigravity work.

## Dependencies

M0–M4

## Deliverables

- tray-first startup.
- exactly one tray icon.
- single Codex app-server owner.
- remaining mapper shared by popup and tray.
- fixed-dark simplified UI.
- latest-cache persistence.
- production mock isolation.

## Scope Boundaries

- Preserve the security and architecture boundaries in `../../AGENTS.md`.
- Do not add removed provider, chart, appearance, API dashboard, or mobile scope.
- Unknown usage must never become zero.

## Acceptance Criteria

- [ ] Tray appears before all fallible initialization.
- [ ] Exactly one tray icon exists.
- [ ] Left-click, right-click, and Quit work.
- [ ] Codex failure leaves tray operational.
- [ ] No duplicate Codex app-server exists.
- [x] 45% used renders as 55% remaining everywhere.
- [x] Unknown renders as —.
- [x] Removed features are absent from production.
- [ ] Latest cached snapshot survives restart.
- [ ] Relevant automated and macOS manual checks pass.

## Task Files

See the matching folder under `../tasks/`.
