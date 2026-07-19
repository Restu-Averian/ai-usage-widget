# AI Usage Dock — Small Tasks

**Last updated:** 2026-07-19

Executable task folders exist only for M0 through M7.

```text
docs/tasks/
├── M0/  # completed
├── M1/  # completed
├── M2/  # completed
├── M3/  # completed, corrected scope
├── M4/  # completed, corrected persistence scope
├── M5/  # active
├── M6/  # locked until M5 PASS
└── M7/  # backlog
```

Statuses:

```text
BACKLOG
READY
IN_PROGRESS
BLOCKED
REVIEW
DONE
CANCELLED
```

Only one task should normally be `IN_PROGRESS`. Follow root `AGENTS.md` and the linked milestone.
