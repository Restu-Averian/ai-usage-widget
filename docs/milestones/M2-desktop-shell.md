# M2 — Desktop Shell

**Project:** AI Usage Dock  
**Status:** PASS — regression remediation assigned to M5  
**Last updated:** 2026-07-19

## Objective

Provide the Tauri popup, macOS Menu Bar behavior, Windows tray boundary, single-instance behavior, and native menu. Later work exposed a tray-ordering regression; M5 owns the final tray-first correction.

## Dependencies

M1

## Deliverables

- one popup window.
- native tray menu.
- left-click toggle.
- right-click menu.
- Quit behavior.

## Scope Boundaries

- Preserve the security and architecture boundaries in `../../AGENTS.md`.
- Do not add removed provider, chart, appearance, API dashboard, or mobile scope.
- Unknown usage must never become zero.

## Acceptance Criteria

- [x] Tray and popup were demonstrated.
- [x] Regression is explicitly tracked by M5 rather than hidden.

## Task Files

See the matching folder under `../tasks/`.
