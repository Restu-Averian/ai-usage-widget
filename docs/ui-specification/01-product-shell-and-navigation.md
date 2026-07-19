# 01 — Product Shell and Navigation

The tray is the primary entry point. The application owns one popup window and one tray icon.

## Tray behavior

- Left-click toggles the popup.
- Right-click opens the native menu.
- Quit performs graceful shutdown.
- Provider initialization never blocks tray creation.

## Popup navigation

The only top-level provider tabs are:

```text
Codex | Antigravity
```

Settings opens as a small dialog or secondary view and contains only refresh interval, launch at login, and provider connection controls.
