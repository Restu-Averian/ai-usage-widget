# M2 — Desktop Shell

**Project:** AI Usage Dock  
**Document status:** Draft v0.1  
**Last updated:** 2026-07-16

Related documents:

- `../AI-Usage-Dock-PRD.md`
- `../AI-Usage-Dock-Technical-Spike.md`
- `../AI-Usage-Dock-Technical-Design.md`
- `../ui-specification/README.md`
- `../../AGENTS.md`

---

## 1. Objective

Build the cross-platform background application shell with macOS Menu Bar and Windows System Tray behavior.

## 2. Intended Outcome

A user can launch the application, click its desktop status icon, open and hide a floating popup, and explicitly quit the process.

No real provider data is required.

## 3. Dependencies

- M1 repository foundation.
- Tauri 2 development prerequisites.
- React and TypeScript scaffold.
- Product shell specification.

## 4. In Scope

- Initialize Tauri 2 application.
- Create React/TypeScript WebView.
- Start with popup hidden.
- Create tray/Menu Bar icon.
- Toggle popup on tray click.
- Position popup near tray.
- Constrain popup to active monitor work area.
- Hide on focus loss.
- Keep app alive after popup closes.
- Add explicit Quit.
- Add single-instance behavior.
- Add launch-at-login boundary.
- Add optional macOS Dock icon boundary.
- Add Windows taskbar exclusion.
- Add a temporary shell placeholder UI.

## 5. Out of Scope

- Real provider connectors.
- Final visual design.
- SQLite usage history.
- API key storage.
- Notifications.
- Production updater.

## 6. Demonstration Flow

```text
Launch
→ status icon appears
→ click icon
→ popup opens
→ click elsewhere
→ popup hides
→ click icon again
→ same popup reopens quickly
→ select Quit
→ application terminates
```

## 7. Suggested Task Groups

### M2-A — Tauri scaffold

- Initialize Tauri 2.
- Configure application identifier.
- Configure popup window.
- Add platform icons.

### M2-B — Tray lifecycle

- Create tray.
- Add native menu.
- Add Open/Hide.
- Add Quit.
- Sync native menu state.

### M2-C — Popup behavior

- Toggle visibility.
- Focus on open.
- Hide on focus loss.
- Prevent duplicate window creation.
- Constrain placement.

### M2-D — Platform behavior

- macOS Menu Bar template icon.
- macOS Dock icon preference boundary.
- Windows tray icon.
- Windows taskbar exclusion.
- Display scaling smoke test.

### M2-E — Lifecycle hardening

- Single-instance behavior.
- Startup hidden.
- Explicit quit.
- Basic launch-at-login toggle boundary.

## 8. Acceptance Criteria

### Shared

- Exactly one background instance runs.
- Popup starts hidden.
- Tray click toggles popup.
- Popup reopens without recreating application state.
- Closing popup does not terminate process.
- Explicit Quit works.
- No generic shell permission is exposed to frontend.
- Popup remains inside visible work area.

### macOS

- Icon appears in Menu Bar.
- Template icon works in light/dark appearance.
- App can operate without Dock icon.
- Popup opens on active display.

### Windows

- Icon appears in System Tray.
- Popup is not shown in taskbar.
- Popup placement respects taskbar location.
- Basic scaling works at 100%, 125%, 150%, and 200%.

## 9. Verification

```text
pnpm lint
pnpm typecheck
cargo fmt --check
cargo clippy
cargo test
cargo check
manual macOS tray test
manual Windows tray test or documented unverified status
```

## 10. Risks

- Window positioning differs by platform.
- Focus-loss events may fire during native dialogs.
- Notch and multiple-monitor behavior may be inconsistent.
- Launch-at-login behavior may require platform-specific handling.
- Tauri capability configuration may accidentally over-permit the WebView.

## 11. Review Questions

- Does popup behavior feel immediate?
- Is only one window created?
- Are lifecycle controls explicit?
- Are platform-specific behaviors isolated?
- Has provider logic remained absent?

## 12. Exit and Handoff

M3 may use the working shell to render complete mock UI.

M4 may rely on:

- stable window label;
- tray lifecycle;
- backend event mechanism;
- application state initialization.
