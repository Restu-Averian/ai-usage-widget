# 04 — Component Specifications

## ProviderTabs

Exactly two tabs: Codex and Antigravity.

## RemainingQuotaRing

- Center value: integer percentage or `—`.
- Label: `Remaining weekly`, `Remaining daily`, or equivalent.
- Source `usedPercent` is converted through the shared mapper.
- The arc visually represents remaining quota.

## ProviderStatus

Shows connection and freshness without exposing internal errors or secrets.

## QuotaDetails

Shows reset time and plan when available.

## RefreshControl

Triggers the typed provider refresh command and reflects single-flight state.

## SettingsDialog

Contains refresh interval, launch at login, and provider connection controls only.
