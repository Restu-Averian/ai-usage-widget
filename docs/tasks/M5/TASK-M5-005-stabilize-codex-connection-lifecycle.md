# TASK-M5-005 — Stabilize Codex Connection Lifecycle

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** BACKLOG  
**Priority:** P1  
**Last updated:** 2026-07-19

## Objective

Make detection, connection, authentication state, retry, and disconnect behavior deterministic.

## Dependencies

- Milestone prerequisites

## In Scope

- Make detection, connection, authentication state, retry, and disconnect behavior deterministic.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [ ] Plan and connection state are accurate.
- [ ] Reconnect and failure states are recoverable.
- [ ] No credential file or browser cookie is read.

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
