# 5. Pseudo-Terminal (PTY) Gate Decision

Date: 2026-07-16

## Status

Accepted

## Context

Some AI tools (e.g., Anthropic's Claude Code CLI or GitHub Copilot CLI) might require pseudo-terminal (PTY) environments to output formatting or execute sub-commands cleanly. Adding PTY dependencies introduces significant cross-platform compilation complexity and increases binary size, which could compromise the lightweight nature of the Rust backend.

## Decision

We will **gate any inclusion of PTY dependencies behind a strict approval process (Spike)**.

- Priority is always given to official structured interfaces (APIs, JSON output from CLIs).
- Only when all fallback strategies (plain-text parsing, dashboard fallback) fail, and a version-aware parser requires a PTY, will it be considered.
- Until a provider spike proves a PTY is mandatory and safe, no PTY dependencies will be added to the project.

## Consequences

- **Pros**:
  - Keeps the initial build simple, small, and fast.
  - Forces developers to find standard, robust integration strategies rather than relying on brittle terminal automation.
- **Cons**:
  - If a crucial provider genuinely only supports PTY interactions, their integration might be delayed until the spike is approved.
