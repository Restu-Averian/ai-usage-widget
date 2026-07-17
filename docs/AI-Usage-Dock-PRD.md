# PRD — AI Usage Dock

**Status:** Draft v0.1  
**Primary platforms:** macOS and Windows  
**Future platforms:** Android and iOS  
**Architecture:** Local-first, cross-platform  
**Target user:** Developers who use multiple AI coding tools

---

## 1. Product Overview

AI Usage Dock is a cross-platform application for monitoring usage and remaining quota across multiple AI coding tools from one compact interface.

Initial providers:

1. Codex
2. Claude
3. Antigravity

On macOS, the application runs from the **Menu Bar**. On Windows, it runs from the **System Tray**. Clicking the icon opens a floating panel containing usage percentage, quota windows, reset times, provider status, and usage history.

The application should clearly separate:

- subscription usage;
- API usage;
- officially retrieved values;
- experimental or estimated values.

---

## 2. Problem Statement

Usage information is currently fragmented across different products and interfaces.

Users may need to:

- open multiple applications;
- open different provider dashboards;
- run CLI commands manually;
- remember reset times;
- switch tools only after discovering that one quota has nearly run out.

AI Usage Dock solves this by presenting the status of all supported AI providers in one place.

---

## 3. Product Vision

> One small icon to understand the condition of every connected AI coding quota without opening each provider separately.

Example:

```text
Codex          68% used
Claude         42% used
Antigravity    81% used
```

The user should immediately understand:

- which provider is closest to its limit;
- how much usage remains;
- what quota period applies;
- when the quota resets;
- whether the displayed data is current or stale.

---

## 4. Goals

### Primary goals

- Display Codex, Claude, and Antigravity in one application.
- Run from the macOS Menu Bar and Windows System Tray.
- Show usage using percentages and graphs.
- Support multiple quota windows for one provider.
- Clearly separate subscription usage and API usage.
- Store data locally.
- Never request a provider password.
- Never store API keys in plaintext.
- Preserve a path toward Android and iOS from a shared codebase.

### Secondary goals

- Launch automatically when the computer starts.
- Refresh usage automatically.
- Show warnings when usage is close to a limit.
- Save local usage history.
- Show the latest saved snapshot while offline.
- Allow the Menu Bar to display the most critical percentage.

### Non-goals for MVP

- Payment system.
- AI Usage Dock user accounts.
- Cloud synchronization.
- Browser extensions.
- Browser cookie extraction.
- Native macOS WidgetKit.
- Android or iOS home-screen widgets.
- Linux support.
- Full enterprise usage analytics.

---

## 5. Platform Scope

### MVP platforms

| Platform | MVP implementation                  |
| -------- | ----------------------------------- |
| macOS    | Menu Bar icon and floating panel    |
| Windows  | System Tray icon and floating panel |
| Linux    | Out of scope                        |
| Android  | Future phase                        |
| iOS      | Future phase                        |

### macOS behavior

- Show an icon in the Menu Bar.
- Optionally hide the app from the Dock.
- Open the floating panel when the icon is clicked.
- Close the panel when it loses focus.
- Optionally show a percentage beside the Menu Bar icon.
- Optionally launch when the user signs in.

### Windows behavior

- Show an icon in the System Tray.
- Open the floating panel when the icon is clicked.
- Keep the app running when the panel is closed.
- Optionally launch when Windows starts.
- Show a usage summary in the tray tooltip.

---

## 6. Target User

### Primary persona

A developer who:

- uses Codex, Claude, and Antigravity;
- works on macOS or Windows;
- frequently switches between AI coding tools;
- wants to see quota status without opening each product;
- needs to know when a quota will reset.

### Core user need

> “I want to know which AI quota is nearly exhausted and when it resets without checking each provider manually.”

---

## 7. Core User Flows

### First launch

```text
Open app
   ↓
Welcome screen
   ↓
Select a provider
   ↓
Connect local account or enter API key
   ↓
Validate connection
   ↓
Fetch usage
   ↓
Display dashboard
```

### Daily usage

```text
Click Menu Bar or System Tray icon
   ↓
Floating panel opens
   ↓
View the most critical provider
   ↓
Open a provider tab
   ↓
View quota windows and reset time
```

### Disconnected provider

```text
Claude
────────────────────────

Not connected

[ Connect Claude Code ]
[ Add Anthropic API Key ]

Subscription usage and API usage
will be displayed separately.
```

### Connected provider

```text
Claude
────────────────────────

Weekly usage

        68%
     ███████░░░

32% remaining
Resets in 2d 8h

Session usage
████████████░░░░ 72%

Updated 2 minutes ago
```

---

## 8. Navigation

The floating panel contains three main tabs:

```text
[ Codex ] [ Claude ] [ Antigravity ]
```

Each provider tab supports these states:

| State         | Description                                      |
| ------------- | ------------------------------------------------ |
| Not connected | Connection options are shown                     |
| Connecting    | Login or connection is in progress               |
| Connected     | Current usage is shown                           |
| Loading       | Usage is being refreshed                         |
| Error         | The last request failed                          |
| Stale         | Saved data is shown with a warning               |
| Unsupported   | Only an official dashboard fallback is available |

Footer actions:

```text
Last updated 2 minutes ago       ↻
⚙ Settings
```

---

## 9. Connection Types

The product must distinguish subscription and API connections.

### A. Subscription connection

Used for account-based quota such as:

- Codex through a ChatGPT subscription;
- Claude Code through Claude Pro or Max;
- Antigravity through a Google account.

Example actions:

```text
Connect Codex Account
Connect Claude Code
Connect Antigravity
```

The app must not request provider passwords.

Preferred connection approach:

1. Detect the official CLI.
2. Detect whether the user is authenticated.
3. Open the provider’s official login flow when needed.
4. Read usage from an officially supported local interface.

### B. API connection

Used for usage-based API billing.

Example actions:

```text
Add OpenAI API/Admin Key
Add Anthropic API/Admin Key
Add Google API Key
```

Possible data:

- input tokens;
- output tokens;
- cached tokens;
- request count;
- estimated cost;
- monthly budget percentage.

### Product rule

Subscription and API data must never be merged into one number.

Example:

```text
Subscription
Weekly usage     68%

API
Monthly budget   $12 / $25
```

The source must always be visible:

```text
Source: Codex subscription
Source: OpenAI API
Source: Claude Code subscription
Source: Anthropic API
```

---

## 10. Usage Data Model

```ts
type ProviderId = "codex" | "claude" | "antigravity";

type UsageSource =
  "subscription" | "api" | "local-cli" | "manual" | "estimated";

interface UsageWindow {
  id: string;
  label: string;

  period:
    "session" | "five-hour" | "daily" | "weekly" | "monthly" | "model-specific";

  model?: string;
  usedPercent?: number;
  remainingPercent?: number;
  resetAt?: string;
  startsAt?: string;
}

interface ProviderUsage {
  provider: ProviderId;
  source: UsageSource;

  accountLabel?: string;
  planName?: string;

  windows: UsageWindow[];

  tokens?: {
    input: number;
    output: number;
    cached?: number;
  };

  cost?: {
    used: number;
    budget?: number;
    currency: "USD";
  };

  fetchedAt: string;
  stale: boolean;

  reliability:
    "official-api" | "official-cli" | "experimental-parser" | "manual";
}
```

---

## 11. Multiple Quota Windows

One provider may expose several limits.

Example:

```text
Codex

5-hour limit
██████████░░░░░░ 62%
Resets in 1h 24m

Weekly limit
███████░░░░░░░░░ 43%
Resets Monday, 08:00
```

The product must not hide secondary quota windows.

### Menu Bar percentage

The default Menu Bar value is the highest usage percentage across all connected quota windows.

Example:

```text
Codex weekly        43%
Claude weekly       71%
Antigravity 5-hour  86%

Menu Bar → [icon 86%]
```

Settings:

```text
Menu Bar value:
(•) Highest usage
( ) Codex
( ) Claude
( ) Antigravity
( ) Hide percentage
```

---

## 12. Dashboard Requirements

### Header

- App icon.
- Product name.
- Refresh action.
- Settings action.

### Provider tabs

- Codex.
- Claude.
- Antigravity.

### Primary usage graph

Use a radial progress indicator:

```text
       68%
     Used this week
```

Supporting text:

```text
32% remaining
Reset in 2 days 8 hours
```

### Secondary graph

A compact history chart:

```text
Mon Tue Wed Thu Fri Sat Sun
 ▁   ▂   ▃   ▅   ▆   ▇   ▅
```

### Usage cards

Each quota card contains:

- quota label;
- usage percentage;
- remaining percentage;
- reset timestamp;
- reset countdown;
- model name when applicable;
- data source;
- data reliability.

### Status messages

- Updated just now.
- Updated five minutes ago.
- Offline.
- Data may be outdated.
- Authentication expired.
- CLI is not installed.
- Usage is not available for this connection.

---

## 13. Settings Requirements

```text
General
────────────────────────────
Launch at Login               On
Start Minimized               On
Show Dock Icon                Off
Close panel when unfocused    On

Menu Bar
────────────────────────────
Show percentage               On
Displayed usage     Highest usage
Warning threshold             80%

Refresh
────────────────────────────
Auto-refresh                  On
Refresh interval        5 minutes
Refresh after wake-up         On

Appearance
────────────────────────────
Theme              System / Dark / Light
Compact mode                   Off

Data
────────────────────────────
Usage history              30 days
Clear usage history
Reset all connections
```

---

## 14. Notifications

MVP notification types:

```text
Warning:
Usage reaches 80%

Critical:
Usage reaches 95%

Reset:
Quota becomes available again

Connection:
Authentication expires
```

Anti-spam rules:

- Send only one warning per provider and quota period.
- Do not repeat a notification on every refresh.
- Reset notification state when a new quota period begins.

---

## 15. Technical Stack

### Application shell

```text
Tauri 2
```

Responsibilities:

- macOS Menu Bar.
- Windows System Tray.
- Window management.
- Background process.
- Autostart.
- Native commands.
- Future Android and iOS shell.

### Frontend

```text
React
TypeScript
Vite
Tailwind CSS
shadcn/ui
Recharts
Zustand
TanStack Query
```

### Native layer

```text
Rust
Serde
Tokio
Reqwest
```

### Storage

```text
Tauri Store      → app settings
SQLite           → usage history
Tauri Stronghold → credentials and secrets
```

### MVP architecture

```text
┌─────────────────────────────────┐
│ React Interface                 │
│ Tabs, graph, settings           │
└───────────────┬─────────────────┘
                │ Tauri Commands
┌───────────────▼─────────────────┐
│ Rust Application Layer          │
│                                 │
│ Provider adapters               │
│ Credential manager              │
│ Refresh scheduler               │
│ History service                 │
└───────────────┬─────────────────┘
                │
     ┌──────────┼──────────┐
     ▼          ▼          ▼
  Codex      Claude    Antigravity
  Adapter    Adapter      Adapter
```

The MVP does not require a cloud backend.

---

## 16. Provider Adapter Design

```rust
trait UsageProvider {
    fn provider_id(&self) -> ProviderId;

    async fn detect_installation(
        &self
    ) -> Result<InstallationStatus>;

    async fn check_authentication(
        &self
    ) -> Result<AuthenticationStatus>;

    async fn start_login(
        &self
    ) -> Result<LoginResult>;

    async fn fetch_usage(
        &self
    ) -> Result<ProviderUsage>;

    async fn disconnect(
        &self
    ) -> Result<()>;
}
```

Connector structure:

```text
CodexProvider
├── CodexCliConnector
└── OpenAiApiConnector

ClaudeProvider
├── ClaudeCodeConnector
└── AnthropicApiConnector

AntigravityProvider
├── AntigravityCliConnector
└── GoogleApiConnector
```

---

## 17. Local Database

### `provider_connections`

```sql
id
provider
connection_type
account_label
status
last_success_at
last_error
created_at
updated_at
```

Raw credentials must not be stored here.

### `usage_snapshots`

```sql
id
provider
source
period
model
used_percent
remaining_percent
reset_at
tokens_input
tokens_output
cost_used
captured_at
```

### `notification_states`

```sql
id
provider
usage_window
threshold
period_identifier
notified_at
```

### `app_events`

```sql
id
event_type
provider
message
created_at
```

Logs must not contain API keys or authentication tokens.

---

## 18. Security Requirements

### Required

- Never request provider passwords.
- Never store API keys in `localStorage`.
- Never store API keys in SQLite.
- Never write API keys to logs.
- Access secrets only from the Rust layer.
- Store credentials using Stronghold or the native keychain.
- Send only normalized usage data to the React layer.
- Sanitize all error messages.
- Delete local credentials when disconnecting.
- Do not retain copied credentials in the clipboard.

### Prohibited

```text
Scrape browser cookies
Read browser session tokens
Store passwords
Upload credentials to a personal server
Expose API keys to React DevTools
Hardcode credentials in source code
```

---

## 19. Main Technical Risk

The primary risk is not the Menu Bar, System Tray, graph, or database.

The primary risk is:

> Whether each provider exposes usage in a stable, safe, and programmatically accessible format.

Possible integration outcomes:

### A. Structured official output

```text
Use directly
Reliability: official API, SDK, or CLI JSON
```

### B. Stable plain-text output

```text
Run local process
Parse stdout
Use version-aware fixtures
Reliability: official CLI text
```

### C. Interactive TUI only

```text
Use a pseudo-terminal
Maintain version-specific parser fixtures
Reliability: experimental
```

### D. No safe local access

```text
Show:
Open Official Usage Dashboard
```

Browser cookies and raw stored credentials must not be used.

---

## 20. Development Phases

### Phase 0 — Provider feasibility spike

Goal:

- Prove that real usage can be retrieved.
- Determine whether JSON, text parsing, PTY, or dashboard fallback is required.

Deliverables:

```text
docs/technical-spike-provider-usage.md
spike/fixtures/
spike/parsers/
spike/results/
```

Exit criteria:

- At least one provider returns real usage.
- Every provider has a documented integration recommendation.
- No provider password, browser cookie, or raw stored credential is used.

### Phase 1 — Desktop foundation

- Initialize Tauri 2 with React and TypeScript.
- Add macOS Menu Bar behavior.
- Add Windows System Tray behavior.
- Add floating borderless panel.
- Keep the app alive when the panel closes.
- Add optional macOS Dock visibility.
- Add autostart.

### Phase 2 — UI with mock data

- Header.
- Three provider tabs.
- All connection and error states.
- Radial graph.
- Multiple quota cards.
- History chart.
- Reset countdown.
- Settings.
- Light and dark theme.

### Phase 3 — Core data layer

- Normalized provider interface.
- Tauri commands.
- SQLite migrations.
- Usage snapshot repository.
- Settings store.
- Refresh scheduler.
- Stale data detector.
- Notification threshold service.
- Provider state machine.

### Phase 4 — First real provider

The first provider must be selected from the feasibility result, not assumed in advance.

Complete one vertical slice:

```text
Menu Bar click
→ panel opens
→ usage is fetched
→ data is normalized
→ snapshot is stored
→ graph is updated
→ reset countdown works
```

### Phase 5 — Second provider

- Implement the next most reliable provider.
- Add fixtures and parser tests.
- Preserve all connection modes.

### Phase 6 — Third provider

- Implement the remaining provider.
- Add an official dashboard fallback when direct usage retrieval is unsupported.

### Phase 7 — API usage

- Add API/Admin key modal.
- Validate from the Rust layer.
- Store keys securely.
- Fetch token and cost data.
- Add optional monthly budgets.
- Display API cards separately from subscription quota.

### Phase 8 — Production hardening

- macOS signing and notarization.
- Windows installer and signing.
- Auto updater.
- Migration safety.
- Redacted logs.
- Wake-from-sleep refresh.
- Network recovery.
- Single-instance behavior.
- Multi-monitor testing.
- Display-scaling testing.
- Offline startup testing.

### Phase 9 — Mobile synchronization

Future architecture:

```text
Desktop app
     ↓
Encrypted normalized snapshots
     ↓
Sync backend
     ↓
Android / iOS app
```

Possible backend:

```text
Cloudflare Workers
Hono
D1 or PostgreSQL
```

Raw provider credentials must not be synchronized.

---

## 21. Suggested Project Structure

```text
ai-usage-dock/
├── src/
│   ├── app/
│   ├── components/
│   ├── features/
│   │   ├── providers/
│   │   │   ├── codex/
│   │   │   ├── claude/
│   │   │   └── antigravity/
│   │   ├── usage/
│   │   ├── notifications/
│   │   └── settings/
│   ├── lib/
│   ├── stores/
│   └── types/
│
├── src-tauri/
│   ├── src/
│   │   ├── providers/
│   │   ├── commands/
│   │   ├── database/
│   │   ├── credentials/
│   │   ├── tray/
│   │   ├── scheduler/
│   │   └── lib.rs
│   └── migrations/
│
├── spike/
│   ├── fixtures/
│   ├── parsers/
│   ├── scripts/
│   └── results/
│
├── docs/
│   ├── PRD.md
│   ├── technical-spike-provider-usage.md
│   ├── technical-design.md
│   ├── architecture.md
│   └── security.md
│
├── tests/
├── AGENTS.md
└── README.md
```

---

## 22. MVP Acceptance Criteria

### Desktop shell

- Installable on macOS.
- Installable on Windows.
- Icon appears in the Menu Bar or System Tray.
- Clicking the icon opens the floating panel.
- The app remains active after the panel closes.
- macOS can run without a Dock icon.

### Provider UI

- Tabs for Codex, Claude, and Antigravity.
- Disconnected and connected states.
- Loading, stale, offline, and error states.
- Percentage graph.
- Multiple quota windows.
- Reset countdown.
- Visible data source and reliability.

### Real data

- At least one provider uses real usage data.
- Saved snapshots are available while offline.
- Stale data is clearly marked.

### Security

- No provider password is requested.
- No browser cookie is read.
- API keys do not enter `localStorage`.
- API keys do not appear in logs.
- Disconnect removes local credentials.

### Reliability

- Manual refresh works.
- Auto-refresh works.
- Offline state works.
- Expired authentication is handled.
- Parsers have fixture tests.
- One provider failure does not break the others.

---

## 23. Recommended Execution Order

```text
1. Complete the provider feasibility spike
2. Select the first provider from real test results
3. Create technical-design.md
4. Initialize the Tauri project
5. Implement Menu Bar and System Tray behavior
6. Build the UI with mock data
7. Implement the normalized provider layer
8. Complete one real provider vertical slice
9. Add local history
10. Add the second provider
11. Add the third provider
12. Add API usage
13. Harden and package desktop releases
14. Consider mobile synchronization
```

---

## 24. First Development Task

Do not start by building the full application.

The first task is:

> Determine whether Codex, Claude, and Antigravity expose authenticated subscription usage in a safe, stable, and machine-readable local format.

The output of that task is the Technical Spike document and its test results.

---

## 25. Product Success Definition

AI Usage Dock succeeds when a developer can click one desktop icon and accurately understand:

- which connected AI provider is closest to its quota;
- how much quota remains;
- when each quota resets;
- whether the data is current;
- whether the value represents subscription usage or API usage.

The product must accomplish this without collecting provider passwords, browser cookies, or raw authentication secrets.
