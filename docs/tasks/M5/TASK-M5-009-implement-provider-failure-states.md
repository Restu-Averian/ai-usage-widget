# TASK-M5-009 — Implement Provider Failure States

**Milestone:** [M5 — First Provider Vertical Slice](../../milestones/M5-first-provider-vertical-slice.md)  
**Status:** BACKLOG  
**Priority:** P2  
**Last updated:** 2026-07-16

---

## 1. Objective

Implement Provider Failure States as a small, independently reviewable step for First Provider Vertical Slice.

## 2. Dependencies

- Milestone prerequisites from `../../milestones/M5-first-provider-vertical-slice.md`
- Recommended previous task: TASK-M5-008

## 3. In Scope

- Handle offline cache, stale, auth expiry, timeout, parser error, unsupported version, and dashboard fallback.

## 4. Out of Scope

- Work belonging to later tasks or milestones.
- Unrelated refactors or visual redesigns.
- Unsafe credential access, browser-cookie extraction, or fabricated provider data.

## 5. Expected Files

Identify exact affected files before implementation. Typical locations may include:

```text
docs/
spike/
src/
src-tauri/
tests/
```

## 6. Acceptance Criteria

- [ ] The task scope is implemented without unrelated changes.
- [ ] Relevant automated tests or deterministic verification exist.
- [ ] Applicable repository checks from `AGENTS.md` pass.
- [ ] No credential, cookie, token, or raw secret is logged or committed.
- [ ] Documentation is updated when behavior or architecture changes.

## 7. Verification

- Run focused tests for the changed module.
- Run applicable lint, typecheck, Rust, build, or manual checks from `AGENTS.md`.
- Record commands and honest results in the completion report.

## 8. Security Checklist

- [ ] No provider password is requested.
- [ ] No browser cookie or raw provider credential file is read.
- [ ] No saved secret is exposed to the frontend.
- [ ] No generic shell, SQL, filesystem, or secret-read capability is added.
- [ ] Unknown values are not silently converted to zero.
- [ ] Subscription and API usage remain separate where relevant.

Mark non-applicable items explicitly during review.

## 9. Completion Report

```text
- files changed;
- implementation summary;
- commands run;
- tests passed;
- tests failed;
- acceptance criteria status;
- security considerations;
- architecture deviations;
- known limitations;
- recommended next task.
```
