# Rust Tooling and Prerequisites

## Toolchain

This repository uses the `stable` Rust toolchain as defined in `rust-toolchain.toml`.

## Platform Prerequisites

To compile the Tauri application, you must install the platform-specific dependencies:

### macOS

1. Install Xcode Command Line Tools: `xcode-select --install`
2. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Windows

1. Install Visual Studio C++ Build Tools.
2. Install Rust: Download `rustup-init.exe` from the official website.

## Commands

The following scripts are configured for checking Rust quality (accessible via `pnpm`):

- `pnpm rs:fmt` - Formats all Rust code
- `pnpm rs:check` - Checks for compilation errors
- `pnpm rs:clippy` - Runs the clippy linter
- `pnpm rs:test` - Runs all Rust unit tests
