# Technical Spike — Provider Usage Feasibility

**Project:** AI Usage Dock  
**Document status:** Ready to execute  
**Date:** 2026-07-16  
**Spike duration:** 2–4 focused working days  
**Primary environment:** macOS  
**Secondary verification:** Windows  
**Owner:** Restu

---

## 1. Purpose

Determine whether AI Usage Dock can safely and reliably retrieve **subscription usage/quota** from:

1. OpenAI Codex
2. Anthropic Claude / Claude Code
3. Google Antigravity

The spike must answer this before the production application, database, charts, notifications, or mobile synchronization are implemented.

The preferred result is a local, machine-readable integration that does not require the user to manually open provider dashboards.

---

## 2. Product Question

Can a local desktop application retrieve the following data for each provider?

- authenticated account state;
- account or plan label, when safely available;
- usage percentage;
- remaining percentage;
- quota window type;
- reset timestamp;
- model-specific quota, when applicable;
- stale/error state;
- data source reliability.

Example normalized result:

```json
{
  "provider": "codex",
  "connectionType": "subscription",
  "accountLabel": "masked@example.com",
  "windows": [
    {
      "id": "five-hour",
      "label": "5-hour limit",
      "usedPercent": 68,
      "remainingPercent": 32,
      "resetAt": "2026-07-16T14:00:00+07:00"
    },
    {
      "id": "weekly",
      "label": "Weekly limit",
      "usedPercent": 43,
      "remainingPercent": 57,
      "resetAt": "2026-07-20T08:00:00+07:00"
    }
  ],
  "fetchedAt": "2026-07-16T08:30:00+07:00",
  "reliability": "official-cli",
  "stale": false
}
```

---

## 3. Why This Spike Is Required

The desktop shell is not the primary technical risk. Tauri can provide a macOS Menu Bar item and a Windows System Tray item.

The main risk is the availability and stability of subscription usage data.

Current official product behavior establishes that:

- Codex exposes remaining limits in an active CLI session through `/status`.
- Claude exposes subscription consumption in its Usage settings, including session and weekly limits.
- Antigravity CLI supports Google sign-in and exposes model usage through `/usage`.

However, those interfaces may be interactive and may not provide a stable JSON command intended for third-party applications.

This spike must determine whether the integration should use:

1. an official structured command or SDK;
2. plain-text process output;
3. a pseudo-terminal parser;
4. an official dashboard fallback;
5. manual usage input as a last resort.

---

## 4. Scope

### In scope

- Detect installed CLI executables.
- Record CLI versions.
- Inspect available commands and flags.
- Test authenticated and unauthenticated states.
- Test interactive usage commands.
- Test non-interactive execution.
- Capture sanitized stdout, stderr, and terminal output.
- Identify ANSI/TUI behavior.
- Identify usage fields and reset timestamps.
- Build throwaway parser prototypes.
- Create parser fixtures and automated tests.
- Test offline and expired-auth behavior.
- Recommend one integration strategy per provider.
- Verify the selected approach on macOS.
- Perform a basic Windows parity check for the selected approach.

### Out of scope

- Production Tauri application.
- Final UI and graphs.
- Production SQLite schema.
- Notifications.
- Cloud synchronization.
- Android or iOS application.
- API token and cost reporting.
- Browser cookie extraction.
- Browser session scraping.
- Reading raw provider credential files.
- Reverse-engineering encrypted credentials.
- Circumventing provider restrictions.
- Shipping a PTY parser before it has fixtures and version guards.

---

## 5. Safety and Security Rules

The spike must never:

- ask for an OpenAI, Anthropic, or Google password;
- copy browser cookies;
- copy OAuth refresh tokens;
- print API keys;
- inspect credential storage files for secret values;
- upload captured output to a public repository;
- commit raw terminal recordings before sanitization;
- expose complete email addresses when a masked value is enough;
- send credentials or raw provider responses to an external server.

Permitted behavior:

- launch the provider's official CLI;
- use its official sign-in flow;
- execute documented commands;
- inspect documented help output;
- capture terminal output locally;
- redact personal and secret values;
- parse user-visible usage information.

All raw captures must remain inside:

```text
.private/
```

The `.private/` directory must be ignored by Git.

Only sanitized fixtures may be committed.

---

## 6. Key Hypotheses

### H1 — Codex

`/status` contains usable subscription-limit information, but it may only be available inside the interactive TUI.

Potential outcomes:

- Best: a documented JSON/status command exists.
- Acceptable: usage is emitted as stable plain text.
- Experimental: usage can only be captured through a PTY.
- Rejected: usage requires browser-cookie or credential extraction.

### H2 — Claude

Claude's subscription usage is available to the user, but the local CLI may not expose a stable machine-readable command for remaining consumer quota.

Potential outcomes:

- Best: a supported CLI command returns structured usage.
- Acceptable: an interactive usage/status command emits stable text.
- Alternative: a local provider-supported telemetry channel gives sufficient data.
- Fallback: open the official Usage settings page.

Important distinction:

- OpenTelemetry or organization reporting may describe consumed tokens and costs.
- The product requires **remaining subscription quota and reset windows**.
- Those are not automatically equivalent.

### H3 — Antigravity

Antigravity CLI's `/usage` output contains model-level quota data but may be an interactive TUI view.

Potential outcomes:

- Best: SDK or CLI exposes structured quota data.
- Acceptable: `/usage` emits parseable text.
- Experimental: PTY parser with version-specific fixtures.
- Fallback: open the official usage surface.

---

## 7. Research Questions

For every provider, answer:

1. What is the executable name?
2. How is installation detected?
3. What version command is available?
4. What command detects authentication without exposing credentials?
5. Does an official usage command exist?
6. Can it run non-interactively?
7. Does it support JSON?
8. Does it require a TTY?
9. Does output change based on terminal width?
10. Does output contain ANSI colors or cursor control?
11. Are percentages explicit or derived?
12. Are reset timestamps explicit?
13. Are timestamps local time or UTC?
14. Can several quota windows appear simultaneously?
15. Can quota be model-specific?
16. What happens when the account is logged out?
17. What happens when the network is offline?
18. What happens when usage is exhausted?
19. What happens after the CLI is upgraded?
20. Does the selected method work on both macOS and Windows?

---

## 8. Repository Structure

```text
ai-usage-dock/
├── docs/
│   ├── PRD.md
│   └── technical-spike-provider-usage.md
│
├── spike/
│   ├── README.md
│   ├── package.json
│   │
│   ├── scripts/
│   │   ├── detect-tools.ts
│   │   ├── capture-command.ts
│   │   ├── redact-output.ts
│   │   └── inspect-ansi.ts
│   │
│   ├── parsers/
│   │   ├── shared.ts
│   │   ├── codex.ts
│   │   ├── claude.ts
│   │   └── antigravity.ts
│   │
│   ├── fixtures/
│   │   ├── codex/
│   │   ├── claude/
│   │   └── antigravity/
│   │
│   ├── tests/
│   │   ├── codex.test.ts
│   │   ├── claude.test.ts
│   │   └── antigravity.test.ts
│   │
│   └── results/
│       ├── codex.md
│       ├── claude.md
│       ├── antigravity.md
│       └── decision.md
│
├── .private/
│   ├── raw-captures/
│   └── screenshots/
│
└── .gitignore
```

Add to `.gitignore`:

```gitignore
.private/
*.raw.log
*.cast
.env
.env.*
```

---

## 9. Common Detection Script

The first implementation should only detect the CLIs and versions.

Expected output:

```json
{
  "platform": "darwin",
  "tools": {
    "codex": {
      "installed": true,
      "path": "/opt/homebrew/bin/codex",
      "version": "..."
    },
    "claude": {
      "installed": true,
      "path": "/Users/user/.local/bin/claude",
      "version": "..."
    },
    "antigravity": {
      "installed": true,
      "path": "/Users/user/.local/bin/agy",
      "version": "..."
    }
  }
}
```

Detection rules:

### macOS/Linux

```bash
command -v codex
command -v claude
command -v agy
```

### Windows PowerShell

```powershell
Get-Command codex -ErrorAction SilentlyContinue
Get-Command claude -ErrorAction SilentlyContinue
Get-Command agy -ErrorAction SilentlyContinue
```

Version probes must have:

- a five-second timeout;
- stdout and stderr capture;
- no shell interpolation from user input;
- redacted results;
- preserved exit codes.

Try these candidates without assuming all are supported:

```bash
codex --version
claude --version
agy --version
```

Then inspect:

```bash
codex --help
claude --help
agy --help
```

---

## 10. Capture Strategy

Use three capture levels.

### Level A — Normal process capture

Use when the command works without a terminal.

Capture:

- command;
- arguments;
- exit code;
- duration;
- stdout;
- stderr;
- CLI version;
- operating system.

### Level B — TTY recording

Use when the command behaves differently in a terminal.

On macOS, a disposable recording can be created with:

```bash
script -q .private/raw-captures/<provider>-interactive.raw.log <command>
```

Do not commit the raw file.

### Level C — Programmatic PTY prototype

Use only after Level A fails and the interactive output has been inspected.

Recommended spike library:

```text
node-pty
```

This is for experimentation only. The production application may later replace it with a Rust PTY implementation.

PTY test dimensions:

- terminal width: 80, 100, 120;
- terminal height: 24, 40;
- color enabled and disabled;
- light and dark terminal;
- first run;
- authenticated;
- logged out;
- offline;
- exhausted quota.

---

## 11. Provider Experiment — Codex

### 11.1 Baseline

Run:

```bash
command -v codex
codex --version
codex --help
```

Capture the documented diagnostic output:

```bash
codex doctor --json
```

Purpose:

- confirm installation;
- inspect redacted authentication health;
- determine whether the JSON includes usage or rate-limit information;
- never assume diagnostic JSON is a quota API.

### 11.2 Interactive usage

Start:

```bash
codex
```

Inside the session, run:

```text
/status
```

Record:

- visible quota labels;
- percentages;
- remaining values;
- reset times;
- account/plan label;
- whether output stays in scrollback;
- whether it is rendered as a modal/TUI component;
- whether terminal resizing changes the content.

Also inspect whether rate-limit data can be placed in the status line:

```text
/statusline
```

### 11.3 Non-interactive probes

Inspect official help for commands or flags containing:

```text
status
usage
limit
rate
json
account
login
auth
```

Do not assume that `codex exec "/status"` is equivalent to entering `/status` in the interactive TUI. Test it only as a disposable experiment and document the result.

### 11.4 Codex fixture states

Create sanitized fixtures for:

```text
authenticated-normal
authenticated-near-limit
authenticated-exhausted
logged-out
offline
command-error
narrow-terminal
wide-terminal
```

### 11.5 Codex success criteria

Codex is considered feasible when:

- usage can be retrieved without reading credential files;
- at least one quota window can be normalized;
- reset information is captured or explicitly marked unavailable;
- parser tests pass for all saved fixtures;
- the method survives two terminal widths;
- malformed output returns a typed error instead of false data.

---

## 12. Provider Experiment — Claude

### 12.1 Baseline

Run:

```bash
command -v claude
claude --version
claude --help
```

Search help and interactive commands for:

```text
usage
status
account
login
logout
doctor
json
telemetry
```

### 12.2 Authentication distinction

Before testing usage, record whether Claude Code is using:

- a Claude subscription account; or
- `ANTHROPIC_API_KEY`.

Do not print the key.

Only record:

```json
{
  "authenticationMode": "subscription"
}
```

or:

```json
{
  "authenticationMode": "api-key"
}
```

The spike must not mix API billing usage with subscription quota.

### 12.3 Interactive usage probe

Start:

```bash
claude
```

Inspect the built-in command list and test the current usage-related command exposed by the installed version.

Candidate commands to verify, not assume:

```text
/usage
/status
/doctor
```

Capture:

- five-hour/session percentage;
- weekly percentage;
- model-specific weekly limit;
- reset times;
- plan name;
- usage-credit state;
- subscription versus API authentication;
- output format and TTY dependency.

### 12.4 Telemetry experiment

Claude Code supports OpenTelemetry for usage/cost/activity monitoring.

Run a disposable local console-exporter experiment only to answer:

- Does telemetry expose remaining subscription quota?
- Does it expose reset timestamps?
- Or does it only expose consumed tokens, cost, and activity?

If it does not expose remaining subscription quota, record it as unsuitable for the core widget. It may still be useful for a future API/organization analytics feature.

### 12.5 Claude fixture states

Create sanitized fixtures for:

```text
subscription-normal
subscription-near-limit
subscription-exhausted
api-key-mode
logged-out
offline
usage-credits-enabled
command-error
```

### 12.6 Claude success criteria

Claude is considered feasible when:

- the app can distinguish subscription and API-key modes;
- subscription usage can be retrieved without browser-cookie extraction;
- session and/or weekly quota can be normalized;
- unavailable fields remain `null`, not estimated silently;
- parser fixtures cover at least one session and one weekly state;
- authentication changes produce an explicit connection-state change.

---

## 13. Provider Experiment — Antigravity

### 13.1 Baseline

Run:

```bash
command -v agy
agy --version
agy --help
```

Record:

- executable path;
- CLI version;
- login mode;
- whether Google OAuth, API key, or Google Cloud project is active.

Do not print the API key or OAuth tokens.

### 13.2 Interactive usage

Start:

```bash
agy
```

Inside the session, run:

```text
/usage
```

Also inspect:

```text
/model
/logout
```

Capture:

- quota per model;
- used or remaining percentage;
- reset timestamp;
- active model;
- account mode;
- whether output is a table, modal, or normal scrollback;
- whether exhausted models remain visible;
- whether output changes across terminal sizes.

### 13.3 Structured interface investigation

Inspect `agy --help` and installed documentation for:

```text
usage
quota
status
json
account
auth
sdk
```

If the official Antigravity SDK exposes a supported quota/status interface, prototype it before committing to PTY parsing.

Do not reverse-engineer private network calls from the IDE.

### 13.4 Antigravity fixture states

Create sanitized fixtures for:

```text
oauth-normal
api-key-mode
cloud-project-mode
one-model-exhausted
all-models-exhausted
logged-out
offline
command-error
```

### 13.5 Antigravity success criteria

Antigravity is considered feasible when:

- model-level quota can be identified;
- Google OAuth and API-key modes can be distinguished;
- at least one quota value can be normalized;
- exhausted-model state is represented correctly;
- parser tests do not depend on terminal color;
- unsupported output results in `parser-version-unsupported`.

---

## 14. Normalized Spike Types

```ts
export type ProviderId = "codex" | "claude" | "antigravity";

export type ConnectionMode =
  "subscription" | "oauth" | "api-key" | "cloud-project" | "unknown";

export type Reliability =
  | "official-api"
  | "official-sdk"
  | "official-cli-json"
  | "official-cli-text"
  | "experimental-pty"
  | "dashboard-only"
  | "unsupported";

export interface SpikeUsageWindow {
  id: string;
  label: string;
  period:
    | "five-hour"
    | "session"
    | "daily"
    | "weekly"
    | "monthly"
    | "model-specific"
    | "unknown";
  model?: string;
  usedPercent?: number;
  remainingPercent?: number;
  resetAt?: string;
}

export interface SpikeProviderResult {
  provider: ProviderId;
  cliVersion: string;
  connectionMode: ConnectionMode;
  authenticated: boolean;
  accountLabel?: string;
  planName?: string;
  windows: SpikeUsageWindow[];
  reliability: Reliability;
  fetchedAt: string;
  warnings: string[];
}

export type SpikeErrorCode =
  | "cli-not-installed"
  | "not-authenticated"
  | "network-unavailable"
  | "command-timeout"
  | "command-failed"
  | "tty-required"
  | "usage-unavailable"
  | "parser-version-unsupported"
  | "output-unrecognized";
```

---

## 15. Parser Rules

Every parser must:

- accept raw text plus CLI version;
- remove ANSI sequences;
- normalize line endings;
- tolerate extra whitespace;
- avoid locale-dependent number parsing where possible;
- never invent missing percentages;
- never convert a vague indicator into a precise number;
- validate percentages are within `0..100`;
- validate reset timestamps before returning them;
- report unrecognized output;
- preserve unknown fields in debug metadata only;
- include the fixture filename in test failures.

Bad behavior:

```ts
return { usedPercent: 0 };
```

when parsing fails.

Required behavior:

```ts
throw new ParserError("output-unrecognized");
```

---

## 16. Redaction Rules

Sanitized fixtures must replace:

```text
Full email       → user@example.com
User name        → USER_NAME
Home directory   → /Users/USER
Windows profile  → C:\Users\USER
Project name     → SAMPLE_PROJECT
Organization     → SAMPLE_ORG
Access token     → [REDACTED_TOKEN]
API key          → [REDACTED_API_KEY]
Session ID       → [REDACTED_SESSION_ID]
```

The redactor must detect common secret patterns and fail closed.

If a possible secret is detected, the file must not be copied to `fixtures/`.

---

## 17. Experiment Matrix

| Provider    | State           | Normal Process | Interactive TTY | PTY Prototype |    macOS |    Windows |
| ----------- | --------------- | -------------: | --------------: | ------------: | -------: | ---------: |
| Codex       | Logged in       |       Required |        Required |     If needed | Required | Smoke test |
| Codex       | Logged out      |       Required |        Required |     If needed | Required | Smoke test |
| Codex       | Offline         |       Required |        Required |     If needed | Required |   Optional |
| Claude      | Subscription    |       Required |        Required |     If needed | Required | Smoke test |
| Claude      | API key         |       Required |        Required |     If needed | Required | Smoke test |
| Claude      | Logged out      |       Required |        Required |     If needed | Required |   Optional |
| Antigravity | Google OAuth    |       Required |        Required |     If needed | Required | Smoke test |
| Antigravity | API key         |       Required |        Required |     If needed | Required |   Optional |
| Antigravity | Exhausted model |       Required |        Required |     If needed | Required |   Optional |

---

## 18. Decision Matrix

Score every provider approach from 1 to 5.

| Criterion              | Weight |
| ---------------------- | -----: |
| Officially supported   |    25% |
| Machine-readable       |    20% |
| Stable across versions |    15% |
| Cross-platform         |    10% |
| No credential exposure |    15% |
| Low maintenance        |    10% |
| Accurate reset data    |     5% |

Candidate approaches:

1. Official API
2. Official SDK
3. Official CLI JSON
4. Official CLI plain text
5. Interactive PTY parser
6. Official dashboard fallback
7. Manual input
8. Unsupported

Decision rule:

- Prefer any supported structured interface over parsing.
- Accept plain text only with versioned fixtures.
- Accept PTY parsing only for a local-only beta marked `experimental`.
- Reject any method requiring browser cookies or raw credential access.

---

## 19. Results Template

Create one file per provider.

```md
# <Provider> Spike Result

## Environment

- OS:
- Architecture:
- CLI version:
- Authentication mode:
- Test date:

## Commands Inspected

- ...

## Usage Data Found

- ...

## Output Type

- JSON / plain text / interactive TUI / unavailable

## TTY Required

- Yes / No

## Fields Available

- Used percentage:
- Remaining percentage:
- Reset time:
- Plan:
- Account label:
- Model quota:

## Security Notes

- ...

## Failure States

- Logged out:
- Offline:
- Exhausted:
- Command timeout:

## Parser Strategy

- ...

## Reliability

- ...

## Recommendation

- Ship / experimental / dashboard fallback / unsupported

## Open Questions

- ...
```

---

## 20. Final Decision Document

Create:

```text
spike/results/decision.md
```

It must contain a concise result table:

| Provider    | Recommended integration | Reliability | MVP status | Main risk |
| ----------- | ----------------------- | ----------- | ---------- | --------- |
| Codex       | TBD                     | TBD         | TBD        | TBD       |
| Claude      | TBD                     | TBD         | TBD        | TBD       |
| Antigravity | TBD                     | TBD         | TBD        | TBD       |

And one of these project decisions:

### Decision A — Proceed with all three

At least one stable or acceptable path exists for every provider.

### Decision B — Proceed with partial MVP

Ship the providers with stable integrations. Show an official-dashboard fallback for unsupported providers.

### Decision C — Reframe the product

Focus the MVP on API usage/cost rather than subscription quota.

### Decision D — Stop

No safe and maintainable method exists for the core product requirement.

---

## 21. Execution Order

### Day 1

1. Create spike repository structure.
2. Add `.private/` protection.
3. Implement CLI detection.
4. Capture versions and help output.
5. Test Codex installation and `/status`.
6. Test Antigravity installation and `/usage`.

### Day 2

1. Test Claude authentication modes.
2. Test current Claude usage-related command.
3. Capture authenticated and logged-out states.
4. Investigate structured/non-interactive options.
5. Sanitize initial fixtures.

### Day 3

1. Build throwaway parsers.
2. Add parser fixture tests.
3. Test terminal widths.
4. Test offline states.
5. Score integration strategies.

### Day 4, if required

1. Windows smoke test.
2. Investigate official SDK/API alternatives.
3. Complete the decision document.
4. Produce the implementation recommendation.

---

## 22. Exit Criteria

The spike is complete only when:

- all three CLIs have been detected or explicitly marked unavailable;
- installed versions have been recorded;
- authenticated usage has been manually inspected for every available provider;
- logged-out behavior has been captured;
- raw captures have been sanitized;
- at least one fixture exists per tested state;
- every parser has tests;
- no fixture contains secrets;
- every provider has a recommended integration strategy;
- Windows parity risk is documented;
- the project has a clear Proceed, Partial MVP, Reframe, or Stop decision.

---

## 23. Deliverables

```text
docs/technical-spike-provider-usage.md
spike/scripts/detect-tools.ts
spike/scripts/capture-command.ts
spike/scripts/redact-output.ts
spike/parsers/*.ts
spike/fixtures/**/*
spike/tests/*.test.ts
spike/results/codex.md
spike/results/claude.md
spike/results/antigravity.md
spike/results/decision.md
```

---

## 24. Prompt for the Coding Agent

```text
Read:
- docs/PRD.md
- docs/technical-spike-provider-usage.md

Execute the provider-usage technical spike only.

Rules:
- Do not create the production Tauri application.
- Do not implement the final UI.
- Do not read browser cookies.
- Do not inspect or print raw credential values.
- Do not upload any captured output.
- Store raw captures only in .private/, which must be gitignored.
- Commit only sanitized fixtures.
- Never treat API usage as subscription usage.
- Never fabricate missing percentages or reset times.
- Add timeouts to every child process.
- Preserve command exit codes and stderr.
- Use version-aware parser fixtures.
- Stop and report if any step would expose credentials.

Tasks:
1. Scaffold the spike directory defined in the document.
2. Implement cross-platform CLI detection for codex, claude, and agy.
3. Record executable paths, versions, and help capabilities.
4. Create safe command and PTY capture utilities.
5. Guide me through interactive authenticated tests one provider at a time.
6. Sanitize captures before creating fixtures.
7. Build parser prototypes only after inspecting real output.
8. Add automated tests for every fixture.
9. Complete the three provider result files.
10. Complete spike/results/decision.md.

Do not claim success for a provider unless real authenticated usage was captured
and normalized without accessing browser cookies or stored secrets.

At the end, report:
- files changed;
- commands run;
- tests passed and failed;
- provider feasibility table;
- security concerns;
- recommended first provider for the MVP.
```
