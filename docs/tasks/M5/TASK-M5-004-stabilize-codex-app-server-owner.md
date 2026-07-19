# TASK-M5-004 — Stabilize Codex App-Server Owner

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** PARTIAL
**Priority:** P0  
**Last updated:** 2026-07-19

## Objective

Implement one idempotent Rust owner for the Codex app-server process.

## Dependencies

- Milestone prerequisites

## In Scope

- Implement one idempotent Rust owner for the Codex app-server process.

## Out of Scope

- Work from later milestones.
- Unrelated refactors or restored removed features.
- Unsafe credential, cookie, generic shell, generic SQL, or secret-read access.

## Acceptance Criteria

- [ ] Concurrent starts produce one owned process.
- [ ] Dead children are reaped safely.
- [x] Quit terminates the owned process through the provider shutdown hook.
- [ ] Codex failure leaves the tray operational.

## Verification

- Added explicit provider shutdown and wired Quit to stop scheduler and owned provider children.
- `cd src-tauri && cargo test` and `cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings` passed.
- Manual duplicate-process and desktop Quit behavior still need live tray QA before this task is DONE.

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
