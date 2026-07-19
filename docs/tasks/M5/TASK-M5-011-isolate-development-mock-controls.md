# TASK-M5-011 — Isolate Development Mock Controls

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** DONE
**Priority:** P1  
**Last updated:** 2026-07-19

## Objective

Keep deterministic mock states available for development and tests but absent from production.

## Dependencies

- Milestone prerequisites

## In Scope

- Keep deterministic mock states available for development and tests but absent from production.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [x] Production UI has no mock selector.
- [x] Production command surface cannot force fake state.
- [x] No fake value is used as missing-data fallback.

## Verification

- Mock controls remain behind `import.meta.env.DEV`.
- Production Antigravity uses the backend state instead of fake scenario mutation.
- Production `dist` scan found no developer mock control strings.

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
