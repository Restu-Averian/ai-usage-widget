# Content and Copy

## 1. Language

MVP UI language:

```text
English
```

The product must be coded with externalized copy so localization can be added later.

Avoid storing complete sentences directly across many components.

---

## 2. Tone

Copy should be:

- concise;
- calm;
- factual;
- transparent;
- non-accusatory;
- security-aware;
- understandable without provider-specific jargon.

Avoid:

- “Oops!”
- “Something went terribly wrong”
- “Unlimited”
- “Guaranteed”
- “Real-time” unless the data is truly real-time
- blaming the user for provider limitations

---

## 3. Core Terminology

Use:

```text
usage
remaining
quota
limit
reset
subscription
API
connection
provider
updated
stale
offline
```

Use `quota window` internally and in advanced details. In primary UI, use the provider’s familiar label where known:

```text
5-hour limit
Weekly limit
Session usage
Monthly budget
```

---

## 4. Percentage Direction

Primary UI uses:

```text
used percentage
```

Example:

```text
68% used
32% remaining
```

Do not mix a remaining percentage hero on one provider with a used percentage hero on another.

---

## 5. Connection Copy

### Codex

```text
Codex is not connected

Connect your local Codex session to view
subscription limits, or add an OpenAI API key
to view API usage and cost.

Connect Local Codex
Add OpenAI API Key
Open official usage dashboard
```

### Claude

```text
Claude is not connected

Connect Claude Code to view subscription usage,
or add an Anthropic API key for API usage.

Connect Claude Code
Add Anthropic API Key
Open official usage page
```

### Antigravity

```text
Antigravity is not connected

Connect the local Antigravity CLI to view
available model quota.

Connect Antigravity
Add Google API Key
Open official usage page
```

Connection methods must be hidden when unsupported.

---

## 6. Security Copy

Canonical short note:

```text
AI Usage Dock never asks for your provider password.
```

API-key dialog:

```text
The key is stored in your operating system’s
secure credential store and is never shown again.
```

Privacy summary:

```text
Usage data and connection metadata are stored
locally on this device.
```

Do not claim zero data collection if future crash reporting or updates collect metadata. Update copy before enabling such systems.

---

## 7. Freshness Copy

```text
Updated just now
Updated 2 minutes ago
Updated 1 hour ago
Refreshing…
Data may be outdated
Offline · Updated 42 minutes ago
Never updated
```

Exact timestamp tooltip:

```text
Last successful update: Jul 16, 2026 at 09:42
```

---

## 8. Reset Copy

```text
Resets in 46m
Resets in 2h 14m
Resets in 2d 8h
Resets Jul 20, 08:00
Reset time passed · Refresh to confirm
Reset time unavailable
```

Do not display seconds.

---

## 9. Reliability Copy

```text
Official API
Official SDK
Official CLI
CLI parser
Experimental
Dashboard only
```

Experimental tooltip:

```text
This integration depends on a provider CLI format
that may change in future versions.
```

Dashboard-only copy:

```text
Automatic usage is not available for this
provider configuration.
```

---

## 10. Error Copy

### Network

```text
You’re offline

Showing the last successful usage update.
```

### Timeout

```text
The provider took too long to respond.

Retry the usage check.
```

### CLI missing

```text
Claude Code was not found

Install the official CLI, then retry detection.
```

### Authentication expired

```text
Claude needs to reconnect

Your local Claude authentication is no longer valid.
```

### Unsupported parser

```text
Automatic usage is not available

This provider version does not expose a supported
usage format that AI Usage Dock can read safely.
```

### Invalid API key

```text
The API key could not be validated.

Check the key and required permissions, then try again.
```

### Secret store unavailable

```text
Secure storage is unavailable

AI Usage Dock could not access the operating system’s
credential store.
```

### Database

```text
Local data could not be opened

Restart the app. Your existing database will not be
deleted automatically.
```

---

## 11. Button Labels

Preferred:

```text
Get Started
Connect Local Codex
Connect Claude Code
Connect Antigravity
Add API Key
Validate and Save
Retry
Retry Detection
Reconnect
Open Installation Guide
Open Official Dashboard
View Details
Disconnect
Clear History
Reset Connections
Cancel
Back
Done
Quit
```

Avoid generic labels:

```text
Submit
Proceed
Confirm
OK
Yes
No
```

Destructive confirmation uses explicit action names.

---

## 12. Settings Copy

### General

```text
Launch at Login
Start AI Usage Dock when you sign in.

Start Minimized
Run in the Menu Bar or System Tray without opening the popup.

Show Dock Icon
Display AI Usage Dock in the macOS Dock.

Close Panel When Unfocused
Hide the popup when you click elsewhere.
```

### Menu Bar

```text
Show Percentage
Display a usage percentage beside the macOS Menu Bar icon.

Displayed Usage
Choose which provider or quota determines the displayed percentage.

Warning Threshold
Notify when usage reaches this percentage.

Critical Threshold
Use the critical state at this percentage.
```

### Refresh

```text
Auto-refresh
Update connected providers on a schedule.

Refresh Interval
How often AI Usage Dock checks for updated usage.

Refresh After Wake
Check usage after the computer wakes from sleep.

Refresh When Popup Opens
Update stale providers when the panel opens.
```

### Data

```text
History Retention
Choose how long local usage snapshots are kept.

Clear Usage History
Remove saved usage snapshots from this device.

Reset All Connections
Remove stored API keys and local connection metadata.
```

---

## 13. Number Formatting

Examples:

```text
1,240
12.4K
1.24M
$12.00
68%
```

Rules:

- Use user locale for separators.
- Use compact formatting in cards.
- Provide full values in accessible labels/tooltips.
- Currency code appears when ambiguity exists.
- Do not use more than two decimal places for displayed cost.

---

## 14. Date and Time

- Display local device time.
- Store timestamps as UTC.
- Use locale-aware formatting.
- Relative reset copy is preferred.
- Exact date is always available.
- Avoid ambiguous date-only formats such as `07/08/26`.
