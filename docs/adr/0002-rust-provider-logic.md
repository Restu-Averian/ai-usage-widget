# ADR 0002 — Keep Provider Logic in Rust

**Status:** Accepted

Provider detection, authentication state, process ownership, parsing, normalization, and errors are implemented in Rust.

The frontend receives typed snapshots only. It cannot execute generic commands, read SQLite, inspect credential files, read browser cookies, or retrieve saved secrets.
