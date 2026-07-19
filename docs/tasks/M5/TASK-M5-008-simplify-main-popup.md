# TASK-M5-008 — Simplify Main Popup

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** DONE
**Priority:** P1  
**Last updated:** 2026-07-19

## Objective

Remove historical analytics and retain only current provider status, remaining quota, reset, freshness, and refresh controls.

## Dependencies

- Milestone prerequisites

## In Scope

- Remove historical analytics and retain only current provider status, remaining quota, reset, freshness, and refresh controls.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [x] No historical chart or screen remains.
- [x] Codex and Antigravity are the only tabs.
- [x] The quota ring represents remaining quota.

## Verification

- Removed `HistoryChart` from the popup and deleted the component/CSS.
- Production `dist` scan found no `Claude`, `Usage history`, `Compact Mode`, theme-selector, or mock-control strings.
- `pnpm lint`, `pnpm typecheck`, `pnpm test`, and `pnpm build` passed.

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
