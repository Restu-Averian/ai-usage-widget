# AI Usage Dock — Small Tasks

**Document status:** Draft v0.1  
**Last updated:** 2026-07-16

This directory contains executable small-task files for milestones M0 through M12.

## Structure

```text
docs/tasks/
├── README.md
├── M0/
├── M1/
├── ...
└── M12/
```

Each milestone folder contains a `README.md` and one file per task.

## Execution

Only the active milestone should normally have tasks moved to `READY` or `IN_PROGRESS`. Start with M0 and M1. Do not execute all task files in one prompt. Future tasks are planning drafts and may be refined after earlier milestone findings.

## Statuses

```text
BACKLOG
READY
IN_PROGRESS
BLOCKED
REVIEW
DONE
CANCELLED
```

Always follow root `AGENTS.md`.
