# TASK-M5-002 — Enforce Tray-First Startup

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** BACKLOG  
**Priority:** P0  
**Last updated:** 2026-07-19

## Objective

Create and register the tray before database, keyring, Codex, provider, or scheduler initialization.

## Dependencies

- Milestone prerequisites

## In Scope

- Create and register the tray before database, keyring, Codex, provider, or scheduler initialization.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [ ] Tray appears when downstream initialization fails.
- [ ] Startup errors become degraded states instead of process exit.
- [ ] Tray initialization has deterministic logs.

## Verification

- Run macOS startup with normal dependencies.
- Inject or simulate SQLite, keyring, and Codex startup failures.

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
