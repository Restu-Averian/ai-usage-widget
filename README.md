# AI Usage Dock

**AI Usage Dock** is a local-first, cross-platform desktop application designed to monitor your AI provider usage and API quota directly from your macOS Menu Bar or Windows System Tray.

## Project Status

**Current Phase**: Milestone 1 (Repository Foundation) - _In Progress_

## Core Documents

- **[Product Requirements Document (PRD)](docs/AI-Usage-Dock-PRD.md)** - High-level goals, target audience, and scope.
- **[Technical Design](docs/AI-Usage-Dock-Technical-Design.md)** - Core architecture, data flows, and constraints.
- **[Technical Spike](docs/AI-Usage-Dock-Technical-Spike.md)** - Provider integration research and strategies.
- **[UI Specification](docs/ui-specification/README.md)** - Design system, layouts, and states.
- **[Milestones](docs/milestones/README.md)** - Roadmap and task breakdowns.
- **[Agent Instructions](AGENTS.md)** - Mandatory instructions for automated coding agents working in this repository.

## Architecture

- **Frontend**: React + TypeScript
- **Backend**: Rust (via Tauri)
- **Data Storage**: Local SQLite
- **Secret Storage**: Native OS Credential Store (macOS Keychain, Windows Credential Manager)

## Setup and Verification

### Prerequisites

- [Node.js](https://nodejs.org/) (>= 20.0.0)
- [pnpm](https://pnpm.io/) (v11.13.1)
- [Rust](https://www.rust-lang.org/) (Stable toolchain)
- OS-specific build dependencies for [Tauri 2](https://tauri.app/v1/guides/getting-started/prerequisites)

### Commands

Install dependencies:

```bash
pnpm install
```

Run tests and checks:

```bash
pnpm run format:check
pnpm run lint
pnpm run typecheck
pnpm run test
pnpm run rs:fmt
pnpm run rs:check
pnpm run rs:clippy
pnpm run rs:test
```

## Task Workflow

Contributors (including AI agents) must strictly follow this workflow:

1. Open the next active task in `docs/tasks/`.
2. Update the task status to `IN_PROGRESS`.
3. Implement exactly what the task asks, without touching unrelated files.
4. Run all verification checks (`pnpm run format:check`, `lint`, `typecheck`, `test`).
5. Update the task status to `DONE`.
6. Commit changes with a focused commit message.
