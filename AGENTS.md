# AGENTS.md — AI Usage Dock

This file defines mandatory instructions for every coding agent working in this repository.

**Last updated:** 2026-07-19

## 1. Product Scope

AI Usage Dock is a local-first desktop application for checking remaining AI subscription quota from the system tray.

Active providers:

- Codex;
- Antigravity.

Primary platforms:

- macOS Menu Bar;
- Windows System Tray.

Current roadmap:

```text
M0–M4  completed foundation
M5     Codex stabilization and product simplification
M6     Antigravity integration
M7     release hardening
```

Do not implement Antigravity work until M5 is explicitly marked PASS.

## 2. Required Reading Order

Before implementing a task, read:

1. `AGENTS.md`;
2. the active task file;
3. `docs/milestones/README.md`;
4. the linked milestone;
5. only the relevant PRD, technical-design, and UI sections.

Source-of-truth priority:

```text
current user instruction
→ active task acceptance criteria
→ AGENTS.md
→ Technical Design
→ UI Specification
→ PRD
→ Milestones
→ existing implementation
```

## 3. Scope Discipline

Implement only the active task. Do not:

- start a later milestone early;
- restore removed providers or removed features;
- add analytics, historical charts, compact layouts, or theme selection;
- introduce cloud services or mobile synchronization;
- refactor unrelated modules;
- add abstractions only for hypothetical future providers.

The application uses one fixed dark visual theme.

## 4. Mandatory Architecture Boundaries

```text
React + TypeScript presentation
        ↓ narrow typed Tauri IPC
Rust application layer
        ↓
provider adapters / SQLite / native credential store
```

Frontend may own:

- rendering and interactions;
- TanStack Query server-state views;
- Zustand UI-only state;
- formatting and the remaining-quota presentation mapper;
- development-only mock scenarios.

Frontend must not:

- execute a generic shell command;
- access SQLite directly;
- read credential files or browser cookies;
- retrieve stored secrets;
- store secrets in localStorage, Zustand, or Query Cache;
- turn unknown quota into zero.

Rust owns:

- tray and popup lifecycle;
- provider detection and provider processes;
- Codex app-server lifecycle;
- provider parsing and normalized snapshots;
- SQLite and migrations;
- secret storage;
- refresh scheduling;
- logging and graceful shutdown.

Never expose generic IPC commands such as `shell.execute`, `sql.query`, `filesystem.read`, or `secret.read`.

## 5. Tray-First Startup Rule

Tray creation is the first runtime responsibility.

Required startup order:

```text
create application shell
→ create exactly one tray icon
→ register tray menu and click handlers
→ make shell operational
→ initialize SQLite and hydrate cache
→ initialize keyring
→ initialize Codex app-server
→ start provider refresh and scheduler
```

Database, keyring, provider, parser, or network failure must not prevent the tray from appearing. Initialization failures become recoverable provider/application states.

## 6. Provider Rules

Active provider structure:

```text
src-tauri/src/providers/
├── codex/
└── antigravity/
```

The registry may remain technically extensible, but the active registry, UI, tests, and roadmap contain only these two providers.

Never:

- scrape browser cookies;
- read raw provider credential files;
- reverse-engineer private IDE traffic;
- request a provider password;
- fabricate quota or reset information;
- silently parse unrecognized output as valid data.

Provider DTOs may preserve `usedPercent` when that is the source value. User-facing surfaces must show remaining quota:

```ts
remainingPercent = usedPercent == null
  ? null
  : clamp(100 - usedPercent, 0, 100)
```

Clamping occurs only at the display boundary. Unknown remains `null` and renders as `—`.

## 7. Persistence Rules

Persist only data needed by the current product:

- latest cached snapshot per provider;
- refresh metadata;
- provider connection metadata;
- executable trust records when required;
- useful settings such as refresh interval and launch at login.

Do not create or retain a historical usage pipeline solely for a removed chart.

Secrets belong in the native credential store, never SQLite or frontend state.

## 8. UI Rules

The production UI contains:

- Codex and Antigravity provider tabs;
- connection state;
- remaining quota ring;
- quota period and reset time;
- last refresh state;
- manual refresh;
- minimal settings.

Removed product features must not reappear:

- third provider;
- historical usage chart or screen;
- compact mode;
- theme selector;
- light or system appearance modes;
- API-usage dashboard;
- mobile sync.

Developer mock controls may exist only behind a development build guard.

## 9. Codex Process Lifecycle

Codex app-server must have a single-owner lifecycle in Rust:

- at most one child process;
- concurrent initialization joins the same attempt;
- stale child state is detected;
- shutdown terminates the owned child safely;
- provider failure does not terminate the tray process;
- duplicate refreshes use single-flight behavior.

## 10. Quality Gates

For each task, run the smallest relevant checks and report honest results:

```text
pnpm lint
pnpm typecheck
pnpm test
pnpm build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
pnpm tauri dev  # manual desktop checks when required
```

Do not claim a check passed unless it was run.

## 11. Completion Report

Every completed coding task reports:

```text
files changed
implementation summary
commands run
passed and failed checks
manual verification
acceptance-criteria status
security considerations
known limitations
recommended next task
```
