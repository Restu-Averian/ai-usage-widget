# Product Shell and Navigation

## 1. Main Desktop Surfaces

AI Usage Dock has two primary desktop surfaces:

1. Native Menu Bar or System Tray item.
2. Floating usage popup.

The application may also show:

- native notifications;
- native credential prompts;
- provider-owned browser login flows;
- operating-system confirmation dialogs.

---

## 2. Menu Bar and System Tray Item

### 2.1 Default state

macOS:

```text
[App icon] [optional percentage]
```

Windows:

```text
[App icon]
```

Windows usage detail appears in the tooltip because tray title behavior differs from macOS.

### 2.2 Percentage source

Default:

```text
Highest known used percentage
across all connected quota windows
```

Settings may select:

- highest usage;
- Codex;
- Claude;
- Antigravity;
- hidden.

### 2.3 Tray item states

| State                 | Icon/title behavior                                        |
| --------------------- | ---------------------------------------------------------- |
| No provider connected | neutral icon, no percentage                                |
| Normal usage          | normal icon and percentage                                 |
| Warning               | warning indicator plus percentage                          |
| Critical              | critical indicator plus percentage                         |
| Stale                 | subtle stale marker                                        |
| Refreshing            | do not animate continuously; optional brief activity state |
| Global error          | error marker only when no usable cached data exists        |

Do not rely only on color.

### 2.4 Native tray menu

```text
Open AI Usage Dock
Refresh All
────────────────────────
Codex          68%
Claude         42%
Antigravity    81%
────────────────────────
Settings
Launch at Login       ✓
Quit
```

Rules:

- Provider rows are informational.
- `Refresh All` is disabled while all providers are refreshing.
- `Open AI Usage Dock` changes to `Hide AI Usage Dock` while visible.
- `Quit` always terminates the background process.

---

## 3. Usage Popup

### 3.1 Dimensions

Default logical dimensions:

```text
Width: 392 px
Height: 600 px
Minimum supported width: 360 px
Maximum supported width: 440 px
```

Compact mode target:

```text
Width: 360 px
Height: content-based, maximum 520 px
```

### 3.2 Window characteristics

- Borderless.
- Non-resizable for MVP.
- Hidden on startup.
- Reused rather than destroyed.
- Constrained inside the active monitor work area.
- Not shown in the Windows taskbar.
- Optional macOS Dock icon controlled by Settings.
- Scroll is allowed inside the content region only.

### 3.3 Layout regions

```text
┌──────────────────────────────────┐
│ App Header                       │  52 px
├──────────────────────────────────┤
│ Provider Tabs                    │  44 px
├──────────────────────────────────┤
│                                  │
│ Scrollable Provider Content      │  flexible
│                                  │
├──────────────────────────────────┤
│ Status Footer                    │  40 px
└──────────────────────────────────┘
```

Settings replaces tabs and provider content with:

```text
┌──────────────────────────────────┐
│ Back + Settings Header           │  52 px
├──────────────────────────────────┤
│                                  │
│ Scrollable Settings Content      │  flexible
│                                  │
└──────────────────────────────────┘
```

---

## 4. App Header

Default dashboard header:

```text
[App icon] AI Usage Dock          [↻] [⚙]
```

Requirements:

- App icon: 20 px.
- Title: `AI Usage Dock`.
- Refresh action: refresh current provider.
- Settings action: open Settings route.
- Icon buttons: minimum 32 × 32 px visual target, 40 × 40 px interactive area when space allows.
- Refresh displays progress only for the selected provider.
- Button tooltip appears after a short hover delay.

Compact mode may display:

```text
AI Usage Dock                   [↻] [⚙]
```

without the app icon.

---

## 5. Provider Tabs

Canonical order:

```text
Codex | Claude | Antigravity
```

Requirements:

- Equal-width tabs.
- Selected tab is visually dominant.
- Provider icon may appear at widths above 380 px.
- At narrower widths, label-only tabs are permitted.
- Tab selection persists between popup openings.
- Keyboard arrows move between tabs.
- `Home` selects Codex.
- `End` selects Antigravity.
- Tabs use correct ARIA tab semantics.

Provider badge behavior:

- Optional warning dot when a non-selected provider becomes critical.
- Optional error dot when attention is required.
- No numeric badge inside the tab for MVP.

---

## 6. Navigation Model

Routes:

```text
/dashboard/:provider
/settings
/settings/connections
/settings/data
/about
```

For MVP these may be internal view states rather than a router dependency.

Recommended behavior:

- Opening the app returns to the last selected provider.
- Opening Settings preserves the previous provider.
- Back from Settings returns to the previous provider.
- Provider login browser flows do not change the current internal route.
- A provider notification click opens the corresponding provider tab.

---

## 7. Status Footer

Default:

```text
Updated 2 minutes ago                    ● Online
```

Possible variants:

```text
Refreshing…
Offline · Updated 42 minutes ago
Data may be outdated
Authentication required
Usage unavailable
```

Footer requirements:

- Remains visible while content scrolls.
- Uses concise copy.
- Provides tooltip or accessible detail for reliability and freshness.
- Manual refresh remains in the header, not the footer.
- Do not show both `Online` and a redundant green badge inside the content.

---

## 8. Scrolling

- Header, tabs, and footer remain fixed.
- Provider content scrolls independently.
- Do not use nested scroll areas.
- Preserve selected provider scroll position only during the current popup session.
- Switching providers resets that provider to its last in-session position.
- Opening the popup again starts content at the top unless a blocking process is active.

---

## 9. Focus Management

When popup opens:

1. Preserve focus if reopened immediately.
2. Otherwise focus the popup container without showing an intrusive focus ring.
3. Keyboard navigation starts at the first actionable element.

When Settings opens:

- Move focus to the Settings heading.
- Back returns focus to the Settings button.

When a dialog closes:

- Return focus to the element that opened it.

---

## 10. Auto-Hide Guard

Clicking outside closes the popup except while:

- an API-key dialog is open;
- a native credential prompt is open;
- provider authentication is in progress;
- a confirmation dialog is open;
- a file or system picker is open.

The popup may close while an external browser login page remains open. Reopening it must show the current connection progress.
