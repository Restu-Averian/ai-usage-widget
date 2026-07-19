# Product Scope and Path Migration — 2026-07-19

This document records the roadmap simplification applied after the Codex real-provider slice exposed tray regressions and unnecessary product complexity.

## Product Decisions

- Active providers are Codex and Antigravity only.
- User-facing quota means remaining quota.
- The application uses a fixed dark theme.
- Historical usage UI and storage are removed.
- Compact mode and appearance selection are removed.
- Tray creation is independent from all provider initialization.
- M5 must pass before Antigravity work starts.

## Milestone Path Changes

| Previous path | Current path | Action |
| --- | --- | --- |
| `docs/milestones/M5-first-provider-vertical-slice.md` | `docs/milestones/M5-codex-stabilization-and-simplification.md` | Replaced |
| `docs/milestones/M6-second-provider-integration.md` | `docs/milestones/M6-antigravity-provider-integration.md` | Replaced |
| `docs/milestones/M7-third-provider-or-fallback.md` | `docs/milestones/M7-release-hardening.md` | Replaced |
| `docs/milestones/M8-api-usage.md` | — | Removed from scope |
| `docs/milestones/M9-notifications-and-history.md` | — | Removed from scope |
| `docs/milestones/M10-desktop-hardening.md` | `docs/milestones/M7-release-hardening.md` | Relevant work merged |
| `docs/milestones/M11-public-beta.md` | `docs/milestones/M7-release-hardening.md` | Essential release work merged |
| `docs/milestones/M12-mobile-sync.md` | — | Removed from roadmap |

Task folders now exist only for `M0` through `M7`. Obsolete task folders were removed rather than retained as misleading active work.

## Removed UI Task Paths

The former chart, multi-appearance, third-provider, API dashboard, historical storage, notification, and mobile-sync tasks were removed. Development-only provider-state fixtures remain, but production mock controls are prohibited.
