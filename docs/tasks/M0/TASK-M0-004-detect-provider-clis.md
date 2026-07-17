# TASK-M0-004 — Detect Provider CLIs

**Milestone:** [M0 — Provider Feasibility](../../milestones/M0-provider-feasibility.md)  
**Status:** BACKLOG  
**Priority:** P0  
**Last updated:** 2026-07-16

---

## 1. Objective

Detect Provider CLIs as a small, independently reviewable step for Provider Feasibility.

## 2. Dependencies

- Milestone prerequisites from `../../milestones/M0-provider-feasibility.md`
- Recommended previous task: TASK-M0-003

## 3. In Scope

- Detect Codex, Claude, and Antigravity executables.
- Record absolute paths, versions, help output, OS, and architecture.
- Report missing tools without failing the whole scan.

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
