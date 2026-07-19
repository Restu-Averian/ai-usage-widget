# TASK-M5-006 — Harden Codex Fetch and Parser

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** BACKLOG  
**Priority:** P1  
**Last updated:** 2026-07-19

## Objective

Fetch real Codex quota through app-server and fail closed on unsupported data.

## Dependencies

- Milestone prerequisites

## In Scope

- Fetch real Codex quota through app-server and fail closed on unsupported data.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [ ] Supported payloads have sanitized fixtures.
- [ ] Unknown output is not accepted as zero.
- [ ] Reset and used-percent values are validated.

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
