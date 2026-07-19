# TASK-M4-008 — Implement Native Secret Store

**Milestone:** [M4 — Core Application Layer](../../milestones/M4-core-application-layer.md)  
**Status:** DONE  
**Priority:** P2  
**Last updated:** 2026-07-19

## Objective

Keep provider secrets in the OS credential store and outside frontend state and SQLite.

## Dependencies

- Milestone prerequisites

## In Scope

- Keep provider secrets in the OS credential store and outside frontend state and SQLite.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [x] The core boundary is available.
- [x] Final scope contains no historical usage or removed provider requirement.
- [x] Frontend cannot access system internals directly.

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
