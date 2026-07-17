# 1. Tauri 2 Architecture

Date: 2026-07-16

## Status

Accepted

## Context

AI Usage Dock needs to be a cross-platform desktop application that runs with minimal footprint. It requires native capabilities like the macOS menu bar or Windows system tray, local storage, process execution, and system notifications. An Electron application would require shipping a heavy chromium and node bundle, increasing memory usage and package size.

## Decision

We will use **Tauri 2** as the core framework for building the desktop application.

- **Frontend**: React + TypeScript (for UI rendering).
- **Backend**: Rust (for system interactions, provider fetching, data storage).

## Consequences

- **Pros**:
  - Extremely small bundle size.
  - Very low memory usage compared to Electron.
  - Native integration with OS features (menu bar, tray).
  - High performance backend with Rust.
- **Cons**:
  - Requires developers to manage both Rust and TypeScript ecosystems.
  - IPC boundaries must be strictly typed and managed between the webview and the Rust backend.
