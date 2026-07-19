# TASK-M5-010 — Remove Obsolete Provider and Persistence Paths

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** BACKLOG  
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

- [ ] The active registry contains only Codex and Antigravity.
- [ ] No historical writer/query/retention path remains.
- [ ] Migration cleanup is safe for the unreleased app.

## Verification

- Run focused checks and record honest results.

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
