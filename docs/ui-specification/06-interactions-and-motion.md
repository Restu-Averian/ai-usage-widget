# Interactions and Motion

## 1. Tray Interaction

### Single click

```text
Hidden popup  → show and focus
Visible popup → hide
```

### Right click

Open native tray menu.

If platform conventions make left/right click handling inconsistent, prioritize:

- normal click toggles popup;
- explicit context interaction opens tray menu.

### Double click

No special behavior.

---

## 2. Popup Opening

Sequence:

1. Resolve active monitor and tray position.
2. Constrain popup to work area.
3. Show popup.
4. Focus popup.
5. Render cached data immediately.
6. Trigger refresh only if freshness policy allows it.

Target visible response:

```text
under 150 ms when app is already running
```

Do not delay opening while waiting for provider data.

---

## 3. Popup Closing

Popup closes when:

- tray icon is clicked while visible;
- focus moves outside;
- Escape is pressed from the root view;
- user selects Hide from native menu.

Popup does not close when:

- a modal is open;
- authentication state requires a native prompt;
- a destructive confirmation is active;
- an internal menu/select is open.

---

## 4. Provider Tab Switching

- Immediate switch using cached state.
- Do not refetch solely because a tab was selected if data is fresh.
- If stale, trigger a background refresh after rendering cached data.
- Preserve the selected tab across popup openings.
- Do not animate with horizontal page slides; use a short opacity transition or no transition.

---

## 5. Manual Refresh

On click:

```text
refresh icon rotates briefly
footer shows Refreshing…
cached content remains
```

Rules:

- One refresh per provider at a time.
- Repeat click is ignored or awaits the active refresh.
- Refresh all is available in the native tray menu.
- Refresh errors produce inline feedback, not a blocking dialog.
- Successful unchanged refresh does not show a toast.

---

## 6. Countdown Behavior

- Update once per minute.
- Recalculate immediately after computer wake.
- Show relative time below one week.
- Show exact date when relative copy would be ambiguous.
- When reset time passes:

```text
Reset time passed · Refresh to confirm
```

Do not automatically change usage to `0%`.

---

## 7. Connection Flow

### Local CLI connection

```text
Connect action
    ↓
Detect installation
    ↓
If missing → installation state
If installed and authenticated → fetch usage
If installed and logged out → launch official login
    ↓
Poll authentication state
    ↓
Fetch usage
    ↓
Connected dashboard
```

### API connection

```text
Add API Key
    ↓
Open secure-input dialog
    ↓
Validate and Save
    ↓
Rust validates
    ↓
Save to native credential store
    ↓
Fetch API usage
    ↓
API dashboard
```

### Cancellation

- Canceling login returns to authentication-required state.
- Canceling API dialog does not persist the input.
- Provider-owned external browser windows are not forcibly closed.

---

## 8. Notifications

Click behavior:

- Warning/critical notification → open corresponding provider tab.
- Authentication notification → open provider connection state.
- Reset notification → open provider dashboard.

The app must open the popup before navigating to the relevant internal view.

---

## 9. Motion Tokens

Durations:

```text
motion-instant   0 ms
motion-fast     120 ms
motion-base     180 ms
motion-slow     240 ms
```

Easing:

```text
enter: cubic-bezier(0.16, 1, 0.3, 1)
exit:  cubic-bezier(0.4, 0, 1, 1)
```

Use motion for:

- opacity;
- small position changes;
- progress updates;
- dialog entry;
- tab indicator.

Avoid:

- bouncing;
- large sliding panels;
- continuous pulsing;
- rotating tray icons;
- attention-seeking celebration.

---

## 10. Specific Animations

### Popup

Prefer native window appearance. Internal root may fade from 0 to 1 over 120 ms.

### Usage ring

- Animate from previous value to new value.
- First load may animate from zero only when no cached value exists.
- Duration: 240 ms maximum.
- Disabled with reduced motion.

### Progress bar

- Width transition: 180 ms.
- No looping shimmer after data loads.

### Refresh icon

- One controlled rotation cycle.
- Do not spin indefinitely when a provider command hangs; command timeout must terminate it.

### Skeleton

- Subtle pulse.
- Disabled with reduced motion.

---

## 11. Keyboard Interaction

### Global

| Key             | Action                              |
| --------------- | ----------------------------------- |
| Escape          | Close topmost dialog, then popup    |
| Tab / Shift+Tab | Move focus                          |
| Enter / Space   | Activate focused control            |
| Cmd/Ctrl+,      | Open Settings when popup is focused |
| Cmd/Ctrl+R      | Refresh selected provider           |
| Cmd/Ctrl+1      | Codex                               |
| Cmd/Ctrl+2      | Claude                              |
| Cmd/Ctrl+3      | Antigravity                         |

### Tabs

| Key        | Action                 |
| ---------- | ---------------------- |
| Left/Right | Previous/next provider |
| Home       | First provider         |
| End        | Last provider          |

Do not capture shortcuts while an API-key input is active except Escape.

---

## 12. Tooltips

Show tooltips for:

- icon-only buttons;
- exact reset timestamp;
- reliability labels;
- tray percentage source;
- disabled actions;
- truncated account or plan labels.

Delay:

```text
400–600 ms
```

Tooltips must not contain critical information that is unavailable elsewhere.

---

## 13. Toasts

Use sparingly.

Allowed:

```text
Launch at Login enabled
Usage history cleared
Connection removed
Update downloaded
```

Avoid toasts for:

- every refresh;
- tab switching;
- popup opening;
- normal successful data fetch.

Errors related to the current provider appear inline.

---

## 14. Destructive Confirmation

Confirmation is required for:

- clear usage history;
- reset all connections;
- disconnect an API key when loss could surprise the user.

Confirmation is not required for:

- hiding popup;
- changing theme;
- changing refresh interval;
- reconnecting a provider.

---

## 15. Reduced Motion

When `prefers-reduced-motion` is enabled:

- remove ring/progress interpolation;
- remove skeleton pulsing;
- remove root fade;
- keep necessary state changes immediate;
- never animate layout size.
