# Rust Setup — AI Usage Dock

**Last updated:** 2026-07-19

Required baseline:

```text
Rust stable
rustfmt
clippy
Tauri 2 prerequisites for macOS and Windows
```

Verification:

```bash
rustc --version
cargo --version
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Rust owns tray lifecycle, provider execution, SQLite, native credential storage, scheduling, logging, and graceful shutdown. Frontend access must remain behind narrow typed Tauri commands.
