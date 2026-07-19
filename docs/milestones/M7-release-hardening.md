# M7 — Release Hardening

**Project:** AI Usage Dock  
**Status:** BACKLOG  
**Last updated:** 2026-07-19

## Objective

Prepare reliable macOS and Windows releases after both providers pass. Focus on packaging, launch at login, graceful shutdown, migration safety, diagnostics, platform smoke tests, and release documentation.

## Dependencies

M6 PASS

## Deliverables

- macOS package/signing path.
- Windows installer path.
- launch at login.
- migration validation.
- redacted diagnostics.
- release checklist.

## Scope Boundaries

- Preserve the security and architecture boundaries in `../../AGENTS.md`.
- Do not add removed provider, chart, appearance, API dashboard, or mobile scope.
- Unknown usage must never become zero.

## Acceptance Criteria

- [ ] macOS and Windows packages pass smoke tests.
- [ ] Launch at login works.
- [ ] Owned children stop on Quit.
- [ ] Migrations are recoverable.
- [ ] Diagnostics are redacted.
- [ ] Release checklist is complete.

## Task Files

See the matching folder under `../tasks/`.
