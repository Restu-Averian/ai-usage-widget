# Technical Design — AI Usage Dock

**Document status:** Active design baseline  
**Last updated:** 2026-07-19

## 1. Purpose

Define the smallest reliable architecture for a local-first tray application that displays remaining subscription quota for Codex and Antigravity.

## 2. System Context

```text
React + TypeScript popup
        ↓ typed Tauri IPC
Rust application core
 ├── desktop shell and tray
 ├── provider registry
 │    ├── Codex connector
 │    └── Antigravity connector
 ├── refresh coordinator
 ├── SQLite repositories
 └── native credential store
```

The active registry contains exactly `codex` and `antigravity`.

## 3. Runtime Ownership

### Frontend

- renders the popup;
- owns view-only state;
- validates IPC responses;
- derives remaining quota through one shared mapper;
- never executes providers, reads SQLite, or retrieves secrets.

### Rust

- owns the application and tray lifecycle;
- owns provider processes and network transports;
- parses provider output;
- stores latest snapshots and refresh metadata;
- owns scheduler, logging, and graceful shutdown.

## 4. Tray-First Startup

Required startup sequence:

```text
1. construct Tauri application state required by the shell
2. create one tray using a stable tray identifier
3. register left-click, right-click, menu, and Quit behavior
4. mark shell ready
5. initialize SQLite and hydrate latest cached snapshots
6. initialize native credential storage
7. initialize Codex app-server owner
8. initialize provider refresh services
9. start scheduler
```

Steps 5–9 are fallible. Their failures are logged and represented as states. They must not unwind shell setup or exit the process.

### Startup state model

```rust
enum SubsystemState {
    Pending,
    Ready,
    Degraded { message: String },
}
```

Tray creation should not depend on a fully initialized provider registry. Initial native labels may use `Loading…` until cached or live state arrives.

## 5. Tray and Window Lifecycle

- Use one constant tray ID.
- Guard tray construction against repeated setup calls.
- Left-click toggles the existing popup; it does not create duplicate windows.
- Right-click opens the native menu.
- Hiding the popup does not quit the process.
- Quit stops scheduler tasks, terminates owned provider children, flushes safe state, and exits.
- Provider or database failure cannot remove the tray.

## 6. Codex App-Server Lifecycle

The Codex integration uses one Rust-owned process manager.

```rust
struct CodexServerManager {
    child: Mutex<Option<OwnedChild>>,
    init: Mutex<InitState>,
}
```

Required behavior:

- `ensure_started()` is idempotent;
- concurrent callers share one initialization attempt;
- a live child is reused;
- a dead child is reaped before restart;
- at most one child is owned by the application;
- shutdown is explicit and bounded;
- stdout/stderr are redacted and size-limited;
- failed startup produces a Codex error state, not an application exit.

## 7. Domain Contracts

### Provider identity

```ts
type ProviderId = 'codex' | 'antigravity';
```

### Source snapshot

```ts
type ProviderUsageWindow = {
  period: 'daily' | 'weekly' | 'monthly' | 'other';
  usedPercent: number | null;
  resetsAt: string | null;
};

type ProviderSnapshot = {
  providerId: ProviderId;
  plan: string | null;
  connectionStatus:
    | 'not-connected'
    | 'connecting'
    | 'connected'
    | 'authentication-required'
    | 'authentication-expired'
    | 'unavailable'
    | 'error';
  windows: ProviderUsageWindow[];
  fetchedAt: string | null;
  source: string;
  isCached: boolean;
  errorCode: string | null;
};
```

Provider contracts preserve `usedPercent` because it reflects source semantics.

### Presentation contract

```ts
type UsageDisplay = {
  remainingPercent: number | null;
  periodLabel: string;
  resetLabel: string;
};

function toRemainingPercent(usedPercent: number | null): number | null {
  if (usedPercent == null || Number.isNaN(usedPercent)) return null;
  return Math.min(100, Math.max(0, 100 - usedPercent));
}
```

Clamping happens only here. Persistence and provider parsing retain raw normalized values or reject invalid data.

## 8. IPC Surface

Narrow commands only:

```text
get_app_bootstrap
list_provider_snapshots
refresh_provider(providerId)
refresh_all_providers
connect_provider(providerId)
disconnect_provider(providerId)
get_settings
update_settings
quit_application
```

Do not expose generic shell, filesystem, SQL, process, or secret commands.

Backend events:

```text
app://bootstrap-updated
provider://snapshot-updated
provider://refresh-state
provider://connection-state
```

## 9. Provider Registry

```rust
ProviderRegistry {
    Codex,
    Antigravity,
}
```

A trait may keep connectors isolated, but avoid plugin systems, dynamic loading, or generic capability frameworks not needed by the two active providers.

Each connector owns:

- detection;
- connection state;
- safe fetch;
- parsing;
- normalization;
- error mapping.

## 10. Refresh Coordination

- Manual refresh and scheduled refresh share the same path.
- Use single-flight per provider.
- A refresh-all operation runs providers independently.
- One provider failure does not cancel the other.
- Scheduler starts only after the tray is usable.
- Backoff applies after failures.
- Last successful snapshot remains available and is marked cached/stale.

Default interval is configurable. The exact supported interval options belong to the settings contract rather than frontend constants.

## 11. SQLite Design

SQLite stores current product state, not analytics.

Recommended schema:

```sql
CREATE TABLE provider_snapshots (
  provider_id TEXT PRIMARY KEY,
  payload_json TEXT NOT NULL,
  fetched_at TEXT,
  updated_at TEXT NOT NULL
);

CREATE TABLE refresh_metadata (
  provider_id TEXT PRIMARY KEY,
  last_attempt_at TEXT,
  last_success_at TEXT,
  next_refresh_at TEXT,
  consecutive_failures INTEGER NOT NULL DEFAULT 0,
  last_error_code TEXT
);

CREATE TABLE provider_connections (
  provider_id TEXT PRIMARY KEY,
  status TEXT NOT NULL,
  metadata_json TEXT,
  updated_at TEXT NOT NULL
);

CREATE TABLE app_settings (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE provider_executable_approvals (
  provider_id TEXT PRIMARY KEY,
  executable_path TEXT NOT NULL,
  executable_fingerprint TEXT,
  approved_at TEXT NOT NULL
);
```

No historical usage table is required. If a development migration already created one exclusively for the removed chart, introduce a safe cleanup migration or rebuild the unreleased development database as documented by M5.

Secrets are never stored in these tables.

## 12. Settings Model

```ts
type AppSettings = {
  refreshIntervalMinutes: number;
  launchAtLogin: boolean;
};
```

Provider connection actions are commands, not theme or layout preferences.

Removed settings:

- compact mode;
- theme;
- appearance mode.

## 13. Frontend State

### TanStack Query

Use for:

- provider snapshots;
- refresh state;
- settings;
- connection actions.

Do not place secrets in variables, query data, or mutation context.

### Zustand

Use only for transient UI state such as active provider tab and dialog visibility. Do not persist secrets or duplicate provider snapshot cache.

## 14. UI Architecture

Production component tree:

```text
AppShell
├── ProviderTabs
├── ProviderStatus
├── RemainingQuotaRing
├── QuotaDetails
├── RefreshStatus
└── SettingsDialog
```

There is no chart component, historical screen, compact variant, appearance selector, or third-provider tab.

## 15. Error and Freshness Model

Important states:

```text
loading
not-connected
authentication-required
connected-fresh
refreshing
connected-cached
stale
authentication-expired
provider-unavailable
output-unrecognized
version-unsupported
internal-error
```

Parser failure never becomes zero usage. Unknown quota renders as `—`.

## 16. Security Design

- Tauri capabilities allow only required commands and window/tray behavior.
- Provider processes use explicit executable paths and argument arrays.
- No shell-string interpolation.
- Output is redacted, bounded, and treated as untrusted.
- Provider versions and parsers are tested with sanitized fixtures.
- Native credential-store access remains Rust-only.
- Logs exclude tokens, cookies, credentials, and raw sensitive payloads.

## 17. Development Mock Boundary

Mock scenarios may be compiled or enabled only in development builds.

Production requirements:

- no visible mock-state selector;
- no command that forces fake provider state;
- no mock provider in the active registry;
- no fake percentage used as fallback for missing real data.

## 18. Testing Strategy

### M5 gates

- startup with SQLite failure still shows one tray;
- startup with Codex failure still shows one tray;
- repeated initialization produces one tray and one Codex child;
- left-click toggles popup;
- right-click opens menu;
- Quit terminates the child;
- `45 used` maps to `55 remaining` in popup and tray;
- null maps to `—`;
- cached snapshot survives restart;
- removed controls are absent in production build.

### M6 gates

- Antigravity states normalize independently;
- refresh-all isolates failures;
- both tray labels use remaining semantics.

### M7 gates

- macOS and Windows packaging smoke tests pass;
- launch-at-login works;
- migration path is validated;
- redacted diagnostics are safe;
- release checklist passes.

## 19. Repository Structure

```text
src/
├── app/
├── components/
├── features/providers/
├── features/settings/
├── ipc/
├── presentation/
└── test-support/          # development and tests only

src-tauri/src/
├── app/
├── desktop/
├── providers/
│   ├── codex/
│   └── antigravity/
├── persistence/
├── scheduler/
├── secrets/
└── commands/
```

## 20. Implementation Sequence

```text
M5.1 tray-first startup and single tray
M5.2 Codex app-server singleton lifecycle
M5.3 remaining-quota contract and presentation
M5.4 UI/settings simplification
M5.5 SQLite and production-mock cleanup
M5.6 regression verification and PASS review
→ only then M6 Antigravity
→ M7 packaging and release hardening
```
