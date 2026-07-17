# Platform and Responsive Behavior

## 1. Shared Product Behavior

The same core UI runs on macOS and Windows.

Shared:

- provider tabs;
- connected and disconnected states;
- settings;
- charts;
- local history;
- connection management;
- keyboard navigation.

Platform-specific:

- tray location;
- popup anchoring;
- native menu appearance;
- Dock/taskbar behavior;
- launch-at-login implementation;
- system typography rendering;
- notification presentation.

---

## 2. macOS

### Menu Bar

- Use a monochrome template icon.
- Optional percentage title beside the icon.
- Follow light/dark Menu Bar automatically.
- Default to no Dock icon.
- Allow `Show Dock Icon` in Settings.

### Popup placement

Preferred:

```text
directly below the Menu Bar item
```

Fallback:

```text
upper-right of active screen work area
```

### Keyboard labels

Display:

```text
⌘,
⌘R
⌘1
⌘2
⌘3
```

### Window behavior

- Hide on focus loss.
- Reopening should feel instant.
- Full-screen apps and multiple Spaces must be tested.
- Avoid opening behind the active application.

---

## 3. Windows

### System Tray

- Icon near notification area.
- Usage summary in tooltip.
- Percentage text is not expected beside the icon.
- Tray icon may be hidden by Windows overflow; document pinning only if needed.

### Popup placement

Preferred:

```text
above the taskbar notification area
```

Fallback:

```text
lower-right of active work area
```

If the taskbar is at the top or side, position relative to its work area.

### Keyboard labels

Display:

```text
Ctrl+,
Ctrl+R
Ctrl+1
Ctrl+2
Ctrl+3
```

### Window behavior

- Do not show in taskbar.
- Closing hides.
- Test display scale at 100%, 125%, 150%, 175%, and 200%.

---

## 4. Width Adaptation

### 392–440 px

- Provider icons shown in tabs.
- Standard 16 px horizontal padding.
- Full history chart visible.
- Hero uses medium ring.

### 360–391 px

- Provider icons may be hidden from tabs.
- Horizontal padding may reduce to 12 px.
- Button labels remain complete.
- Long supporting text wraps.
- Hero remains centered.

No supported desktop layout below 360 px.

---

## 5. Height Adaptation

When available height is limited:

- content region scrolls;
- header, tabs, and footer remain visible;
- welcome art is reduced or removed;
- history chart moves below quota cards;
- settings sections remain vertically scrollable.

Do not shrink typography to fit height.

---

## 6. Compact Mode

Compact mode is a deliberate density variant.

Changes:

- width target: 360 px;
- hero ring: small or medium;
- card padding: 12 px;
- progress bars: 6 px;
- history defaults collapsed;
- provider icons removed from tabs;
- nonessential descriptions shortened.

Unchanged:

- interactive target sizes;
- body readability;
- accessibility labels;
- error actions;
- security notes in connection flows.

---

## 7. Long Content

Account labels:

```text
r••••••••@example.com
```

Technical paths:

```text
/Users/…/bin/claude
```

Rules:

- truncate middle for file paths;
- truncate end for account labels only after masking;
- full sanitized value in tooltip;
- do not let labels push status/action controls off screen.

---

## 8. Localization Readiness

Although MVP is English:

- no fixed-width copy containers;
- allow 30–40% text expansion;
- avoid embedding text in images;
- use locale formatting for dates and numbers;
- keep provider names untranslated;
- externalize UI strings.

---

## 9. Future Mobile

The desktop popup design should not simply be stretched into a mobile screen.

Future mobile information architecture:

```text
Overview
Providers
History
Settings
```

Reusable:

- provider identity;
- usage hero;
- quota cards;
- history graph;
- data models;
- state banners;
- copy keys.

Desktop-only:

- Menu Bar/System Tray controls;
- popup auto-hide;
- CLI installation;
- executable trust;
- launch at login.

Mobile home-screen widgets are native platform implementations and display synchronized snapshots only.
