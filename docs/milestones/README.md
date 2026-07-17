# AI Usage Dock — Milestone Roadmap

**Document status:** Draft v0.1  
**Last updated:** 2026-07-16

This directory contains one implementation document per milestone.

The milestone files define outcomes and review gates. They are not substitutes for small-task files. Before executing a milestone, create detailed tasks under:

```text
docs/tasks/<milestone-id>/
```

Example:

```text
docs/tasks/M0/
├── TASK-M0-001-scaffold-spike.md
├── TASK-M0-002-detect-provider-clis.md
└── ...
```

---

## Delivery Strategy

The project begins with two coordinated tracks.

```text
Track A — Risk validation         Track B — Product foundation
─────────────────────────         ────────────────────────────
M0 Provider Feasibility           M1 Repository Foundation
                                  M2 Desktop Shell
                                  M3 Mock UI
              ↓                           ↓
              └──────────────┬────────────┘
                             ↓
                  M4 Core Application Layer
                             ↓
                  M5 First Provider Slice
```

Real production provider connectors must not be implemented until M0 has selected the safe strategy for each provider.

---

## Milestone Index

| ID                                        | Milestone                     | Main outcome                                              | Depends on                   |
| ----------------------------------------- | ----------------------------- | --------------------------------------------------------- | ---------------------------- |
| [M0](M0-provider-feasibility.md)          | Provider Feasibility          | Evidence-based integration decision for all providers     | Documentation                |
| [M1](M1-repository-foundation.md)         | Repository Foundation         | Stable project structure and engineering workflow         | Documentation                |
| [M2](M2-desktop-shell.md)                 | Desktop Shell                 | Working Menu Bar/System Tray application shell            | M1                           |
| [M3](M3-mock-ui.md)                       | Mock UI                       | Complete UI using normalized fake data                    | M1, partial M2               |
| [M4](M4-core-application-layer.md)        | Core Application Layer        | Typed IPC, storage, scheduler, secrets, provider registry | M1–M3                        |
| [M5](M5-first-provider-vertical-slice.md) | First Provider Vertical Slice | One real provider works end-to-end                        | M0, M4                       |
| [M6](M6-second-provider-integration.md)   | Second Provider Integration   | Two independent real provider integrations                | M5                           |
| [M7](M7-third-provider-or-fallback.md)    | Third Provider or Fallback    | Initial three-provider promise completed honestly         | M5, M6                       |
| [M8](M8-api-usage.md)                     | API Usage                     | Token, cost, and budget reporting                         | M4, provider API feasibility |
| [M9](M9-notifications-and-history.md)     | Notifications and History     | Useful local monitoring and alerts                        | M5+                          |
| [M10](M10-desktop-hardening.md)           | Desktop Hardening             | Signed and resilient release artifacts                    | M5–M9                        |
| [M11](M11-public-beta.md)                 | Public Beta                   | Feedback-ready limited release                            | M10                          |
| [M12](M12-mobile-sync.md)                 | Mobile Sync                   | Future Android/iOS snapshot viewing                       | Stable desktop product       |

---

## Milestone Execution Workflow

```text
Read milestone
↓
Create small-task files
↓
Review task dependencies
↓
Move one task to IN_PROGRESS
↓
Implement only that task
↓
Run verification
↓
Move task to REVIEW
↓
Review against acceptance criteria
↓
Mark task DONE
↓
Close milestone when every exit criterion passes
```

---

## Task Statuses

```text
BACKLOG
READY
IN_PROGRESS
BLOCKED
REVIEW
DONE
CANCELLED
```

Only one task should normally be `IN_PROGRESS` for one coding agent.

---

## Rules

- Do not ask an agent to build the entire application in one prompt.
- Detail only the current milestone and the next immediate milestone.
- Do not implement production provider parsing before M0 review.
- Every milestone must produce something demonstrable.
- A milestone is not complete merely because code was written.
- Required tests and acceptance criteria must pass.
- Architecture deviations must be documented.
- Newly discovered work belongs in follow-up tasks.
- Security boundaries in `AGENTS.md` are mandatory.

---

## Current Recommended Order

```text
M0 and M1 may begin in parallel
↓
M2 and M3 may begin after M1
↓
M4 begins after shell/UI contracts stabilize
↓
M5 begins only after M0 selects the first provider
↓
M6 and M7 add providers one at a time
↓
M8 adds API usage separately
↓
M9 adds monitoring value
↓
M10 prepares release
↓
M11 validates product demand
↓
M12 remains future work
```

---

## Milestone Definition of Done

A milestone is complete only when:

- all required deliverables exist;
- acceptance criteria pass;
- relevant automated checks pass;
- manual verification is documented;
- no known critical security issue remains;
- incomplete work is represented by explicit follow-up tasks;
- documentation matches implementation;
- the result can be demonstrated;
- the next milestone can safely depend on it.
