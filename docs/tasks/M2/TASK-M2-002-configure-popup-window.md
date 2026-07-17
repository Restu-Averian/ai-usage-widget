# TASK-M2-002 — Configure Popup Window

**Milestone:** [M2 — Desktop Shell](../../milestones/M2-desktop-shell.md)  
**Status:** BACKLOG  
**Priority:** P2  
**Last updated:** 2026-07-16

---

## 1. Objective

Configure Popup Window as a small, independently reviewable step for Desktop Shell.

## 2. Dependencies

- Milestone prerequisites from `../../milestones/M2-desktop-shell.md`
- Recommended previous task: TASK-M2-001

## 3. In Scope

- Create borderless reusable popup with specified dimensions.
- Hide it on startup and preserve state across hide/show.
- Exclude it from Windows taskbar where supported.

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
