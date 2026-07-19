# ADR 0007 — Create the Tray Before Fallible Initialization

**Status:** Accepted  
**Date:** 2026-07-19

The tray is the application shell and must be created before SQLite hydration, keyring initialization, Codex app-server startup, provider refresh, or scheduler startup.

Subsystem failures become degraded states and must not prevent one stable tray icon from appearing.
