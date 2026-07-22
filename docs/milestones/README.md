# AI Usage Widget — Milestone Roadmap

**Document status:** Active  
**Last updated:** 2026-07-19

## Delivery Strategy

```text
M0–M4 completed foundation
        ↓
M5 Codex stabilization and simplification
        ↓ PASS gate
M6 Antigravity provider integration
        ↓
M7 release hardening
```

## Milestone Index

| ID                                                 | Milestone                              | Status                             | Main outcome                              |
| -------------------------------------------------- | -------------------------------------- | ---------------------------------- | ----------------------------------------- |
| [M0](M0-provider-feasibility.md)                   | Provider Feasibility                   | PASS                               | Safe paths for Codex and Antigravity      |
| [M1](M1-repository-foundation.md)                  | Repository Foundation                  | PASS                               | Stable project and engineering baseline   |
| [M2](M2-desktop-shell.md)                          | Desktop Shell                          | PASS with regression tracked in M5 | Tray and popup shell                      |
| [M3](M3-mock-ui.md)                                | Mock UI                                | PASS with final scope correction   | Fixed-dark, two-provider UI contract      |
| [M4](M4-core-application-layer.md)                 | Core Application Layer                 | PASS with persistence correction   | IPC, registry, latest snapshot, scheduler |
| [M5](M5-codex-stabilization-and-simplification.md) | Codex Stabilization and Simplification | IN PROGRESS                        | Stable tray, remaining quota, reduced UI  |
| [M6](M6-antigravity-provider-integration.md)       | Antigravity Provider Integration       | LOCKED                             | Second and final provider                 |
| [M7](M7-release-hardening.md)                      | Release Hardening                      | BACKLOG                            | Packaging, platform stability, release    |

## Execution Rules

- M5 is the only active implementation milestone.
- Do not begin M6 until every M5 exit criterion passes.
- Do not restore removed scope.
- Keep one task `IN_PROGRESS` at a time.
- A milestone passes only after automated and manual acceptance evidence is recorded.

Task files live under `docs/tasks/<milestone-id>/`.
