# Screen Specifications

## 1. Screen Inventory

| ID     | Screen                           |
| ------ | -------------------------------- |
| UI-001 | First-run welcome                |
| UI-002 | Provider disconnected            |
| UI-003 | CLI not installed                |
| UI-004 | Connecting/authenticating        |
| UI-005 | Connected subscription usage     |
| UI-006 | Connected API usage              |
| UI-007 | Connected subscription + API     |
| UI-008 | Refreshing                       |
| UI-009 | Stale/offline                    |
| UI-010 | Authentication expired           |
| UI-011 | Provider error                   |
| UI-012 | Unsupported provider integration |
| UI-013 | Settings                         |
| UI-014 | Connection details               |
| UI-015 | API key dialog                   |
| UI-016 | Clear data confirmation          |
| UI-017 | About                            |

---

## 2. UI-001 — First-Run Welcome

### Purpose

Explain the product briefly and guide the user toward connecting the first provider.

### Layout

```text
[App icon]

AI Usage Dock

Track Codex, Claude, and Antigravity
usage from one compact desktop panel.

[ Get Started ]

You can connect providers later in Settings.
```

### Requirements

- Display only when no onboarding completion flag exists.
- One primary action: `Get Started`.
- No carousel.
- No account creation.
- No provider password messaging unless the user starts a connection.
- `Get Started` moves to Codex disconnected state.
- Users may close the popup and return later.
- Onboarding completes after the user visits the dashboard, even without connecting a provider.

### Empty-state illustration

Optional and lightweight:

- simple usage ring;
- three small provider markers;
- no large decorative artwork.

---

## 3. UI-002 — Provider Disconnected

### Purpose

Show available connection methods without implying that all methods provide the same data.

### Layout

```text
[Provider icon]

Codex is not connected

Connect your local Codex session to view
subscription limits, or add an API key to
view API usage and cost.

[ Connect Local Codex ]
[ Add OpenAI API Key ]

Open official usage dashboard
```

### Requirements

- Provider-specific title and copy.
- Connection methods are capability-driven.
- Primary action is the preferred subscription connector.
- Secondary API action is visually distinct.
- Dashboard fallback is a text action.
- Show a short security note:

```text
AI Usage Dock never asks for your provider password.
```

- Do not show unavailable connection methods.

### Provider terminology

| Provider    | Preferred action    |
| ----------- | ------------------- |
| Codex       | Connect Local Codex |
| Claude      | Connect Claude Code |
| Antigravity | Connect Antigravity |

---

## 4. UI-003 — CLI Not Installed

### Layout

```text
Claude Code was not found

Install the official CLI, then retry detection.

[ Open installation guide ]
[ Retry detection ]

Use API connection instead
```

### Requirements

- Display detected platform.
- Do not automatically install a CLI.
- If an executable path was previously approved but is now missing, say:

```text
The previously connected Claude CLI could not be found.
```

- Advanced detail may show the checked executable name.
- A path-selection option is deferred unless technical implementation requires it.

---

## 5. UI-004 — Connecting or Authenticating

### Layout

```text
Connecting Claude Code…

Complete the official sign-in flow in your
browser or terminal. This window will update
automatically when the connection is ready.

[ Cancel ]

Waiting for authentication…
```

### Requirements

- Show the provider icon.
- Show progress without fake percentage.
- Poll authentication using the backend connection state.
- Allow cancellation when the provider supports it.
- A `Reopen sign-in` action may appear after a timeout.
- Keep the current provider tab selected.
- Do not block switching to other providers.
- Preserve connection progress when the popup hides.

---

## 6. UI-005 — Connected Subscription Usage

### Content priority

1. Most critical subscription quota.
2. Reset countdown.
3. Secondary quota windows.
4. Seven-day history.
5. Source and reliability.

### Layout

```text
Claude Max                               Connected

            68%
       Used this week

32% remaining · Resets in 2d 8h

Weekly limit
██████████████░░░░░░ 68%
Resets Jul 20, 08:00

5-hour session
████████████████░░░░ 81%
Resets in 46m

Usage history
▁ ▂ ▃ ▅ ▆ ▇ ▅

Source: Claude Code · Official CLI
```

### Requirements

- The hero uses the most critical known window.
- Hero label reflects the actual period.
- If no percentage exists, replace the ring with a textual availability state.
- Each secondary window remains visible.
- Reset countdown and exact reset time are both available.
- Show plan/account label only when safely available.
- Use a reliability label.
- Show stale state at the page level, not separately on every card.

---

## 7. UI-006 — Connected API Usage

### Layout

```text
OpenAI API                              Connected

Monthly budget

           48%
       $12.00 of $25.00

Input tokens          1.24M
Output tokens          240K
Cached tokens          610K
Requests               8,214

API usage history
▁ ▂ ▃ ▅ ▆ ▇ ▅

Source: OpenAI Usage API
```

### Requirements

- API usage is visually labeled as API.
- Budget percentage is shown only when the user has configured a budget.
- Without a budget, show cost as the hero:

```text
$12.00 this month
```

- Currency is explicit.
- Token values use compact formatting and accessible full-value labels.
- Do not compare API cost percentage with subscription quota percentage as though they are the same metric.

---

## 8. UI-007 — Subscription and API Connected

### Layout

Use a segmented switch inside the provider content:

```text
[ Subscription ] [ API ]
```

Rules:

- Subscription is the default segment.
- The segment is below provider tabs and above the hero.
- The selected segment persists per provider.
- The Menu Bar percentage uses subscription quota by default.
- API budget participates only when the user explicitly selects it in Settings.
- Do not place two full dashboards in one scroll view.

---

## 9. UI-008 — Refreshing

### Behavior

When cached data exists:

- Keep existing values visible.
- Show a small spinner in the refresh button.
- Show `Refreshing…` in the footer.
- Do not replace the entire screen with a skeleton.

When no data exists:

- Use a structured skeleton matching the final layout.
- Do not show `0%`.

Manual refresh:

- Refresh current provider only.
- Disable repeated manual clicks while the same refresh is active.
- Other provider tabs remain usable.

---

## 10. UI-009 — Stale or Offline

### Layout

```text
[ Offline banner ]

Showing the last successful update from
42 minutes ago.

[ Existing usage content remains visible ]

Offline · Updated 42 minutes ago
```

### Requirements

- Preserve readable cached data.
- Use a compact inline banner.
- Do not gray out the entire interface.
- Exact timestamp is available in tooltip or details.
- Countdown may continue from the known reset timestamp.
- Do not claim a reset occurred until a successful refresh confirms it.

Stale but online:

```text
Data may be outdated. Refresh to try again.
```

---

## 11. UI-010 — Authentication Expired

### Layout

```text
Claude needs to reconnect

Your local Claude authentication is no longer
valid. Reconnect to update usage.

[ Reconnect Claude ]

Showing data from 3 hours ago
```

### Requirements

- Cached data remains visible below the notice when available.
- Primary action is reconnect.
- Disconnect appears in connection details, not as a competing primary action.
- The provider tab gets an attention indicator.

---

## 12. UI-011 — Provider Error

### With cached data

```text
Couldn’t refresh Codex

The latest saved usage is still available.

[ Retry ]
[ View details ]
```

### Without cached data

```text
Codex usage is unavailable

We couldn’t retrieve usage from the connected
Codex session.

[ Retry ]
[ Open official dashboard ]
```

### Requirements

- Error title is specific.
- Avoid exposing raw CLI output.
- `View details` shows sanitized code, provider version, and timestamp.
- An error is retryable only when the backend says so.
- Parser-version errors use specialized copy.

---

## 13. UI-012 — Unsupported Integration

### Layout

```text
Automatic usage is not available

This version of Antigravity does not expose a
supported usage format that AI Usage Dock can
read safely.

[ Open official usage dashboard ]
[ Check again ]

Technical status: Unsupported CLI version
```

### Requirements

- Be transparent.
- Do not imply user misconfiguration when the connector is unsupported.
- Do not offer browser-cookie workarounds.
- Preserve future recheck ability.

---

## 14. UI-013 — Settings

### Header

```text
[←] Settings
```

### Sections

```text
General
Menu Bar
Refresh
Appearance
Notifications
Connections
Data
About
```

### MVP settings layout

#### General

- Launch at Login
- Start Minimized
- Show Dock Icon — macOS only
- Close Panel When Unfocused

#### Menu Bar

- Show Percentage
- Displayed Usage
- Warning Threshold
- Critical Threshold

#### Refresh

- Auto-refresh
- Refresh Interval
- Refresh After Wake
- Refresh When Popup Opens

#### Appearance

- Theme
- Compact Mode

#### Notifications

- Warning
- Critical
- Quota Reset
- Authentication Expired

#### Connections

- Codex
- Claude
- Antigravity

Each row shows:

```text
Provider name
Connection summary
Status chevron
```

#### Data

- History Retention
- Clear Usage History
- Reset All Connections

#### About

- Version
- Open-source licenses
- Privacy summary
- Check for updates

---

## 15. UI-014 — Connection Details

### Layout

```text
[←] Claude Connection

Subscription
Connected through Claude Code
Account: r••••@example.com
Plan: Max
CLI version: 1.x.x
Data source: Official CLI

[ Refresh connection ]
[ Disconnect ]

API
Not connected

[ Add Anthropic API Key ]
```

### Requirements

- Subscription and API appear as separate sections.
- Full secrets are never displayed.
- Show approved CLI path only under Advanced details.
- Disconnect requires confirmation only when data loss or secret deletion could surprise the user.
- Historical snapshots remain unless the user also clears them.

---

## 16. UI-015 — API Key Dialog

### Layout

```text
Add OpenAI API Key

The key is stored in your operating system’s
secure credential store and is never shown again.

API key
[ •••••••••••••••••••••••••••• ]

[ Cancel ] [ Validate and Save ]
```

Optional admin-key copy:

```text
An organization-admin key may be required
for usage and cost reporting.
```

### Requirements

- Password input.
- Paste supported.
- No reveal button by default.
- No key appears in logs.
- Validation occurs in Rust.
- Save button shows progress.
- On success, clear input and close dialog.
- On failure, preserve input only while the dialog remains open.
- Never provide a `Copy saved key` action.

---

## 17. UI-016 — Clear Data Confirmation

```text
Clear usage history?

This removes saved usage snapshots from this
device. Provider connections will remain active.

[ Cancel ] [ Clear History ]
```

For reset all connections:

```text
Reset all connections?

Stored API keys and local connection metadata
will be removed. Historical usage will remain
unless you clear it separately.

[ Cancel ] [ Reset Connections ]
```

Destructive actions use explicit labels, not generic `OK`.

---

## 18. UI-017 — About

Content:

```text
AI Usage Dock
Version 0.1.0

A local-first usage monitor for AI coding tools.

Privacy
Provider passwords and browser cookies are never
requested. Connected usage data is stored locally.

[ Check for Updates ]
[ Open Licenses ]
[ Open Project Repository ]
```

Public repository action appears only when a public repository exists.
