# TASK-M5-007 — Implement Shared Remaining Quota Mapper

**Milestone:** [M5 — Codex Stabilization and Simplification](../../milestones/M5-codex-stabilization-and-simplification.md)  
**Status:** DONE
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

- [x] 45 used maps to 55 remaining.
- [x] Null maps to null and renders as —.
- [x] Clamping happens only at the display boundary.
- [x] Popup and tray values match.

## Verification

- `pnpm vitest run src/lib/usage-display.test.ts` passed.
- `cd src-tauri && cargo test tray::tests::formats_known_unknown_and_auth_required_codex_tray_values -- --nocapture` passed.
- Popup uses `src/lib/usage-display.ts`; tray uses the equivalent tray formatter in `src-tauri/src/tray.rs`.

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
