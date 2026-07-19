# TASK-M5-007 — Implement Shared Remaining Quota Mapper

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** BACKLOG  
**Priority:** P0  
**Last updated:** 2026-07-19

## Objective

Derive display-only remainingPercent from provider usedPercent and reuse it across popup and tray.

## Dependencies

- Milestone prerequisites

## In Scope

- Derive display-only remainingPercent from provider usedPercent and reuse it across popup and tray.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [ ] 45 used maps to 55 remaining.
- [ ] Null maps to null and renders as —.
- [ ] Clamping happens only at the display boundary.
- [ ] Popup and tray values match.

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
