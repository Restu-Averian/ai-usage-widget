# TASK-M5-010 — Remove Obsolete Provider and Persistence Paths

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** DONE
**Priority:** P1  
**Last updated:** 2026-07-19

## Objective

Remove obsolete provider registry entries, fixtures, tray labels, tests, and historical storage used only by removed features.

## Dependencies

- Milestone prerequisites

## In Scope

- Remove obsolete provider registry entries, fixtures, tray labels, tests, and historical storage used only by removed features.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [x] The active registry contains only Codex and Antigravity.
- [x] No historical query/retention path remains.
- [x] Snapshot writes now keep only the latest snapshot per provider.
- [x] Obsolete API-usage persistence created for removed dashboard/history scope is dropped.

## Verification

- Removed Claude from active provider unions, production registry, tray menu, frontend schema, tabs, and fixtures.
- Removed frontend/Rust usage-history command/query paths.
- Removed `api_usage_totals` from fresh SQLite schema and added migration `m5_drop_obsolete_api_usage_totals` for existing local databases.
- Added migration `m5_prune_obsolete_provider_snapshots` so existing local databases remove stale non-active providers such as Claude and retain only the latest snapshot per active provider.
- `cd src-tauri && cargo test` passed, including latest-only persistence coverage.

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
