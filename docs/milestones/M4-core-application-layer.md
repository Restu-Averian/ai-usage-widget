# M4 — Core Application Layer

**Project:** AI Usage Widget  
**Status:** PASS — persistence scope corrected  
**Last updated:** 2026-07-19

## Objective

Provide typed IPC, Rust domain contracts, provider registry, process/HTTP boundaries, SQLite repositories, native secrets, events, and refresh coordination. Final persistence stores latest state only.

## Dependencies

M1–M3

## Deliverables

- typed commands.
- two-provider registry.
- latest-snapshot repository.
- refresh metadata.
- secret-store boundary.

## Scope Boundaries

- Preserve the security and architecture boundaries in `../../AGENTS.md`.
- Do not add removed provider, chart, appearance, API dashboard, or mobile scope.
- Unknown usage must never become zero.

## Acceptance Criteria

- [x] Frontend has no direct system/data access.
- [x] Registry is limited to active providers.
- [x] Persistence is latest-state only.

## Task Files

See the matching folder under `../tasks/`.
