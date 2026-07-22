# PRD — AI Usage Widget

**Document status:** Approved product scope  
**Last updated:** 2026-07-19

## 1. Product Overview

AI Usage Widget is a local-first desktop utility that shows remaining AI subscription quota from the macOS Menu Bar or Windows System Tray.

The product is intentionally small. It answers four questions quickly:

1. How much quota remains?
2. When does it reset?
3. Is the provider connected?
4. Is the displayed data fresh?

## 2. Active Scope

Providers:

- Codex;
- Antigravity.

Platforms:

- macOS Menu Bar;
- Windows System Tray.

Technology:

- Tauri 2;
- React;
- TypeScript;
- Vite;
- Rust;
- SQLite;
- TanStack Query;
- Zustand.

## 3. Product Principles

- Tray-first: the desktop shell must appear before provider work begins.
- Local-first: provider data and settings remain on the device.
- Remaining-first: every user-facing percentage describes quota remaining.
- Honest unknowns: unavailable data renders as `—`, never `0%`.
- Minimal interface: current status matters more than analytics.
- Safe boundaries: provider execution, persistence, and secrets stay in Rust.

## 4. Goals

### Primary goals

- Show current remaining quota for Codex and Antigravity.
- Show the quota period and reset time when available.
- Keep the tray usable when a provider fails.
- Refresh manually and on a configurable interval.
- Restore the latest cached snapshot after restart.
- Support launch at login.

### Non-goals

- A third provider.
- Historical usage charts or screens.
- Compact mode.
- Appearance or theme selection.
- Light or system appearance modes.
- API token/cost dashboards.
- Cloud sync or mobile apps.
- Browser-cookie scraping or credential-file inspection.

## 5. Primary User Flow

### Startup

```text
launch app
→ tray icon appears immediately
→ cached snapshot hydrates when available
→ provider initialization runs independently
→ popup shows connected, disconnected, stale, or error state
```

### Daily check

```text
click tray icon
→ popup toggles
→ active provider displays remaining quota
→ user may switch provider or refresh
```

### Tray menu

```text
Codex — 55% remaining
Antigravity — Not connected
──────────────────────────
Refresh
Open AI Usage Widget
Quit
```

Provider failures must update labels without removing or duplicating the tray.

## 6. Main Popup

The popup contains:

- provider tabs for Codex and Antigravity;
- provider and plan label when known;
- connection/freshness status;
- remaining-quota ring;
- quota period;
- reset time;
- last refreshed time;
- refresh action;
- settings entry.

Example:

```text
55%
Remaining weekly
Resets Jul 23, 10:00
Updated 2 min ago
```

Unknown example:

```text
—
Remaining weekly
Reset unavailable
```

## 7. Remaining-Quota Semantics

Provider-source values may use consumed quota:

```ts
usedPercent: number | null;
```

Presentation derives:

```ts
remainingPercent =
  usedPercent == null ? null : clamp(100 - usedPercent, 0, 100);
```

Rules:

- calculate remaining in one shared presentation mapper;
- use the same result in popup and tray menu;
- clamp only at the display boundary;
- do not persist fabricated values;
- do not convert unknown to zero;
- label the percentage with `Remaining` and the quota period.

## 8. Settings

Production settings are limited to useful controls:

- refresh interval;
- launch at login;
- provider connect/reconnect/disconnect controls;
- provider-specific dashboard fallback when required.

Development-only mock-state controls must be excluded from production builds.

## 9. Reliability Requirements

- Exactly one tray icon exists.
- Tray creation does not wait for SQLite, keyring, Codex app-server, provider refresh, or scheduler initialization.
- Left-click toggles the popup.
- Right-click opens the native menu.
- Quit terminates owned background work safely.
- Codex app-server has at most one owned process.
- Concurrent refreshes are single-flight per provider.
- Provider errors do not terminate the application shell.
- Cached data remains distinguishable from fresh data.

## 10. Security Requirements

Frontend must not:

- execute generic shell commands;
- access SQLite;
- read credential files or browser cookies;
- retrieve secrets;
- store secrets in localStorage, Zustand, or TanStack Query cache.

Rust must expose narrow typed commands and use the native credential store for secrets.

## 11. Local Persistence

Persist only:

- latest snapshot per provider;
- provider connection metadata;
- refresh metadata;
- useful app settings;
- executable trust records when needed.

A historical usage table is not part of the product. If an old migration created one solely for the removed chart, remove or supersede it safely before release.

## 12. Roadmap

```text
M0 Provider Feasibility                         PASS
M1 Repository Foundation                       PASS
M2 Desktop Shell                               PASS, regression tracked in M5
M3 Mock UI                                     PASS, final scope corrected
M4 Core Application Layer                      PASS, persistence scope corrected
M5 Codex Stabilization and Simplification      IN PROGRESS
M6 Antigravity Provider Integration            LOCKED until M5 PASS
M7 Release Hardening                           BACKLOG
```

## 13. MVP Acceptance Criteria

- Tray appears immediately and reliably on macOS.
- Windows tray behavior passes before release.
- Only one tray instance exists.
- Popup and tray show the same remaining percentage.
- A source value of `45% used` renders as `55% remaining`.
- Unknown renders as `—`.
- Codex works end-to-end without duplicate app-server processes.
- Antigravity works end-to-end after M6.
- Latest snapshots survive restart.
- Removed features and removed provider scope are absent from production.
- Security boundaries in `AGENTS.md` are preserved.
