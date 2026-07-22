# ADR 0001 — Use Tauri 2 for the Desktop Shell

**Status:** Accepted

AI Usage Widget uses Tauri 2 with a React/TypeScript popup and Rust-owned desktop lifecycle.

This keeps tray, process, SQLite, secret-store, and platform behavior outside the WebView while supporting macOS Menu Bar and Windows System Tray delivery.
