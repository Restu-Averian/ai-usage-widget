# TASK-M5-003 — Guarantee Single Tray Lifecycle

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** BACKLOG  
**Priority:** P0  
**Last updated:** 2026-07-19

## Objective

Prevent duplicate tray creation and stabilize left-click, right-click, popup toggle, and Quit behavior.

## Dependencies

- Milestone prerequisites

## In Scope

- Prevent duplicate tray creation and stabilize left-click, right-click, popup toggle, and Quit behavior.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [ ] Exactly one tray icon exists across repeated initialization paths.
- [ ] Left-click toggles one popup.
- [ ] Right-click opens the native menu.
- [ ] Quit works.

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
