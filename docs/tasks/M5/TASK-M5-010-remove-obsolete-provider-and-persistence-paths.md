# TASK-M5-010 — Remove Obsolete Provider and Persistence Paths

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** PARTIAL
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
- [ ] Migration/table naming cleanup is still deferred; existing unreleased tables are reused for latest-cache storage.

## Verification

- Removed Claude from active provider unions, production registry, tray menu, frontend schema, tabs, and fixtures.
- Removed frontend/Rust usage-history command/query paths.
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
