# M5 — Codex Stabilization and Simplification

**Project:** AI Usage Widget  
**Status:** IN PROGRESS  
**Last updated:** 2026-07-24

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

- [x] Tray appears before all fallible initialization.
- [ ] Exactly one tray icon exists.
- [ ] Left-click, right-click, and Quit work.
- [x] Codex failure leaves tray operational.
- [x] No duplicate Codex app-server exists.
- [x] 45% used renders as 55% remaining everywhere.
- [x] Unknown renders as —.
- [x] Removed features are absent from production.
- [x] Latest cached snapshot survives restart.
- [ ] Relevant automated and macOS manual checks pass.

## 2026-07-24 Regression Review

Automated validation passed: frontend lint, typecheck, tests, production build, Rust fmt/check/test/clippy/release check, and Tauri production build.

Packaged restart/process validation passed three times with exactly one app process, one owned Codex app-server child, and clean process shutdown after app quit.

Live app-data SQLite verification confirmed latest-only active-provider persistence after migration: one Codex snapshot, one Antigravity snapshot, no Claude snapshot, no obsolete API-usage table.

M5 is not PASS yet because native macOS status-bar left-click/right-click/menu item behavior still needs direct HITL verification in an environment that can target the status-bar icon.

## Task Files

See the matching folder under `../tasks/`.
