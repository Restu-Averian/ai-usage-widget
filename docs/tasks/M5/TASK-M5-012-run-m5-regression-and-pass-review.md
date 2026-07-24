# TASK-M5-012 — Run M5 Regression and PASS Review

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** IN_PROGRESS
**Priority:** P0  
**Last updated:** 2026-07-24

## Objective

Run automated and manual acceptance checks and mark M5 PASS only when every gate succeeds.

## Dependencies

- Milestone prerequisites

## In Scope

- Run automated and manual acceptance checks and mark M5 PASS only when every gate succeeds.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [ ] Tray, popup, Quit, Codex singleton, remaining semantics, cache restart, and production cleanup pass.
- [x] Failures and limitations are documented honestly.
- [ ] M6 remains locked until this review passes.

## Verification

- 2026-07-24 automated checks passed:
  - `pnpm lint`
  - `pnpm typecheck`
  - `pnpm test` (21 tests)
  - `pnpm build`
  - `cd src-tauri && cargo fmt --all --check`
  - `cd src-tauri && cargo check`
  - `cd src-tauri && cargo test` (45 tests)
  - `cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings`
  - `cd src-tauri && cargo check --release`
  - `pnpm tauri build`
- Production bundle scan found no `Developer Mock State`, `set_fake_provider_scenario`, `Usage history`, `Claude`, `Compact Mode`, or `theme selector` strings in `dist`.
- Packaged macOS restart loop ran 3 times against `src-tauri/target/release/bundle/macos/AI Usage Widget.app`: each launch produced exactly one app process and one owned Codex `app-server` child, and app quit cleaned both processes.
- Live app-data SQLite verification after migration showed one `codex` snapshot, one `antigravity` snapshot, no `claude` snapshot, no `api_usage_totals` table, and schema migrations 1-3 applied.
- Source review confirmed popup quota uses `src/lib/usage-display.ts` and tray quota uses equivalent remaining-percent formatting in `src-tauri/src/tray.rs`; unknown remains `null`/`—`.
- Source review confirmed tray-first setup creates the tray before asynchronous database/provider initialization.

## Regression Fixes Applied

- Native tray menu now includes Codex status, Refresh, Open/Hide, and Quit. The Codex status item is managed so live quota updates reach the menu.
- Tray Refresh now reuses the existing Codex refresh path and updates the Codex menu item.
- Release builds reject fake provider scenario mutation with a typed invalid-input response; debug/test mock scenarios remain available.
- Obsolete `api_usage_totals` persistence was removed from fresh schema and dropped for existing local databases.
- Existing local snapshot history is pruned to one latest row per active provider and removes stale non-active providers such as Claude.

## Remaining Blocker

Native macOS status-bar left-click/right-click behavior was not fully verified by this agent. AppleScript could launch and quit the packaged app and inspect the app process, but it did not expose a reliable status-bar item target for left-click/right-click/menu-item HITL checks. M5 must stay `IN_PROGRESS` until a human or a trusted desktop automation environment verifies:

- exactly one visible tray icon;
- left-click hidden -> show/focus;
- left-click visible -> hide;
- right-click native menu;
- tray menu Open;
- tray menu Refresh;
- tray menu Quit.

## Completion Report

```text
files changed
implementation summary
commands run
passed and failed checks
manual verification
acceptance-criteria status
security considerations
known limitations
recommended next task
```
