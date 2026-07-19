# TASK-M5-009 — Simplify Settings and Fix Dark Mode

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** DONE
**Priority:** P1  
**Last updated:** 2026-07-19

## Objective

Remove compact and appearance controls and keep only useful operational settings.

## Dependencies

- Milestone prerequisites

## In Scope

- Remove compact and appearance controls and keep only useful operational settings.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [x] No compact-mode control remains.
- [x] No theme selector or light/system mode remains.
- [x] Refresh interval and launch at login remain.

## Verification

- Removed compact-mode state and fixed the root app to dark mode.
- Removed the theme selector and deleted the theme preference helper/test.
- Settings now expose refresh interval and launch-at-login controls; provider connection controls remain in provider-state UI.

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
