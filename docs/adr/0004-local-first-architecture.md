# 4. Local-First Architecture

Date: 2026-07-16

## Status

Accepted

## Context

Users want absolute privacy over their API keys and usage patterns. If we use a cloud synchronization service to persist their keys, fetch usage, and push it back to the client, we inherit massive security, compliance, and infrastructure liabilities, and we break user trust.

## Decision

The application will use a strict **Local-First Architecture**.

- No central cloud backend will be developed or maintained for the MVP.
- All application data (settings, usage history) will be stored in a local SQLite database file.
- All secrets (API keys) will be stored in the native operating system credential store (macOS Keychain, Windows Credential Manager).
- Provider fetch requests are executed locally directly from the user's desktop to the provider.

## Consequences

- **Pros**:
  - Uncompromised user privacy and security.
  - Zero server hosting costs or backend maintenance for the MVP.
  - Functions completely offline for reviewing history.
- **Cons**:
  - No cross-device synchronization in the MVP.
  - Fetching behavior depends on the user's local network conditions and proxies.
