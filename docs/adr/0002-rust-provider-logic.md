# 2. Rust Provider Logic

Date: 2026-07-16

## Status

Accepted

## Context

AI Usage Dock needs to interact with various AI provider tools (CLIs, HTTP endpoints, system keyrings) to monitor usage. If this logic lived in the frontend webview, we would need to expose generic system-level APIs (e.g., shell command execution, file system read/write, environment variable access) to the JavaScript context, introducing significant security risks.

## Decision

We will implement **all provider detection, parsing, process execution, and HTTP fetching logic in the Rust backend layer**.

- The frontend will not execute generic commands or raw CLI processes.
- The Rust layer will expose narrow, strictly-typed Tauri IPC endpoints to retrieve normalized usage data.

## Consequences

- **Pros**:
  - Eliminates the risk of arbitrary code execution from the webview.
  - Ensures secrets are only handled by the secure Rust context.
  - Better performance and tighter control over process lifecycle management.
- **Cons**:
  - Development requires adding new IPC endpoints and Rust domain structs for new features rather than quickly mocking them in JS.
