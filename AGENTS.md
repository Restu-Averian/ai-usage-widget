# AGENTS.md — AI Usage Dock

This file defines mandatory instructions for every coding agent working in this repository.

These rules apply to Codex, Claude Code, Antigravity, and other automated coding agents.

---

## 1. Project Summary

AI Usage Dock is a local-first cross-platform desktop application for monitoring AI provider usage and quota.

Initial providers:

- Codex;
- Claude;
- Antigravity.

Primary platforms:

- macOS Menu Bar;
- Windows System Tray.

Future platforms:

- Android;
- iOS.

Core architecture:

```text
React + TypeScript
        ↓
Typed Tauri IPC
        ↓
Rust application layer
        ↓
Provider connectors / SQLite / native credential store
```

---

## 2. Required Reading Order

Before implementing any task, read:

1. `AGENTS.md`
2. The active task file
3. `docs/milestones/README.md`
4. Relevant architecture and UI documents

Core documents:

```text
docs/AI-Usage-Dock-PRD.md
docs/AI-Usage-Dock-Technical-Spike.md
docs/AI-Usage-Dock-Technical-Design.md
docs/ui-specification/README.md
```

For UI tasks, also read:

```text
docs/ui-specification/05-design-system.md
```

Then read only the screen/component documents relevant to the task.

Do not load and rewrite unrelated documents.

---

## 3. Source of Truth Priority

When documents conflict, use this priority:

```text
1. Explicit current user instruction
2. Active task acceptance criteria
3. AGENTS.md
4. Technical Design
5. UI Specification
6. PRD
7. Milestones
8. Existing implementation
```

If a conflict affects architecture or security, stop and report it before proceeding.

Do not silently choose one interpretation.

---

## 4. Scope Discipline

Implement only the active task.

Do not:

- build future milestones early;
- add unrelated features;
- redesign unrelated screens;
- refactor unrelated modules;
- change existing data fields without approval;
- introduce a cloud backend during the local-first MVP;
- implement all providers in one task;
- replace established dependencies without documenting why.

When useful follow-up work is discovered:

1. record it as a follow-up task;
2. do not implement it unless required by the active task.

---

## 5. Mandatory Architecture Boundaries

### Frontend responsibilities

Frontend may contain:

- UI rendering;
- view state;
- keyboard interaction;
- charts;
- formatting;
- typed IPC clients;
- runtime response validation;
- mock UI scenarios.

Frontend must not contain:

- provider CLI process execution;
- provider output parsing;
- raw database queries;
- stored-secret retrieval;
- API requests containing provider secrets;
- browser-cookie access;
- arbitrary shell commands.

### Rust responsibilities

Rust owns:

- provider detection;
- authentication-state detection;
- provider login launching;
- CLI execution;
- PTY execution when approved;
- provider parsing;
- provider HTTP requests;
- normalized usage;
- SQLite;
- secret storage;
- scheduler;
- notifications;
- logging;
- desktop lifecycle.

### IPC rule

Expose narrow typed commands only.

Never expose generic:

```text
shell.execute
filesystem.read
filesystem.write
sql.query
secret.read
```

to the frontend.

---

## 6. Provider Integration Rules

Provider implementations must be isolated.

Preferred structure:

```text
src-tauri/src/providers/
├── codex/
├── claude/
└── antigravity/
```

Every provider must expose normalized data rather than provider-specific raw output.

Do not implement a production connector until the provider spike determines its strategy.

Allowed strategies:

```text
official-api
official-sdk
official-cli-json
official-cli-text
experimental-pty
dashboard-only
unsupported
```

Priority:

```text
Official structured interface
↓
Official plain-text interface
↓
Version-aware PTY parser
↓
Official dashboard fallback
```

Never:

- scrape browser cookies;
- read raw provider credential files;
- reverse-engineer private IDE network calls;
- ask for provider passwords;
- label API usage as subscription usage;
- fabricate unavailable percentages;
- fabricate reset timestamps.

---

## 7. Subscription and API Separation

Subscription usage and API usage are different data sources.

They must remain separate in:

- domain types;
- database records;
- UI;
- history;
- refresh state;
- errors;
- settings;
- notifications.

Do not merge them into a single percentage.

Do not let an API budget replace subscription quota in the Menu Bar by default.

---

## 8. Security Rules

### Secrets

Secrets must never be stored in:

```text
localStorage
sessionStorage
Zustand
TanStack Query
SQLite
plain JSON
logs
fixtures
source code
committed environment files
```

Secrets are stored through the native operating-system credential store or the approved secure-storage abstraction.

Stored secrets must never be returned to the frontend.

There must be no command equivalent to:

```text
get_saved_api_key
```

### API key input

A newly entered key may exist briefly in the input component and IPC request.

Required behavior:

- use a masked input;
- avoid global frontend state;
- send directly to Rust;
- validate in Rust;
- save securely;
- clear input after success;
- never log request arguments.

### Process execution

- Resolve an absolute executable path.
- Use direct process spawning.
- Pass arguments as an array.
- Do not build one shell command string.
- Use allowlisted commands and arguments.
- Add timeouts.
- Limit output size.
- Preserve exit status.
- Kill timed-out child processes where possible.
- Do not log process environments.
- Treat output as untrusted.

### Logging

Never log:

- API keys;
- bearer tokens;
- OAuth tokens;
- raw credential files;
- complete process environment;
- authorization headers;
- unsanitized provider output.

Use typed, sanitized error messages.

---

## 9. Data Correctness Rules

Unknown data remains unknown.

Never convert parse failure into:

```ts
{
  usedPercent: 0;
}
```

Use a typed error instead.

Percentage rules:

- must be between 0 and 100;
- may be derived only from an explicit complementary value;
- derived values must be identifiable internally;
- stored precision must not be replaced by display rounding.

Reset rules:

- store timestamps in UTC;
- display in local device time;
- do not declare a reset after the timestamp passes without a successful provider refresh;
- show `Reset time passed · Refresh to confirm`.

Stale rules:

- preserve the last successful snapshot;
- label it clearly;
- do not gray out all data;
- never present stale data as current.

---

## 10. TypeScript Rules

- Use TypeScript for all frontend source.
- Avoid `any`.
- Prefer `unknown` plus validation.
- Use discriminated unions for states.
- Use Zod for runtime IPC validation.
- Keep domain types centralized.
- Avoid duplicate backend-owned state in Zustand.
- Use TanStack Query for backend-owned data.
- Use Zustand only for transient UI state.
- Prefer named exports for reusable modules.
- Keep components focused.
- Do not place provider parsing in TypeScript.

Example state style:

```ts
type ProviderViewState =
  | { type: "disconnected" }
  | { type: "connecting" }
  | { type: "connected"; usage: ProviderUsage }
  | { type: "stale"; usage: ProviderUsage; error: AppError }
  | { type: "error"; error: AppError };
```

---

## 11. React Rules

- Use function components.
- Keep side effects inside explicit hooks.
- Do not fetch in arbitrary render paths.
- Avoid large components that handle every provider state.
- Use shared components from the UI Specification.
- Preserve semantic HTML.
- Use ARIA only when native semantics are insufficient.
- Do not hardcode complete UI copy across many components.
- Respect reduced motion.
- Do not use provider brand colors for usage status.
- Do not render unknown percentage as an empty 0% ring.

For UI work, implement and verify mock states before real provider data.

---

## 12. Rust Rules

- Use typed errors.
- Avoid panics in application paths.
- Do not use `unwrap()` or `expect()` in production paths unless the invariant is documented and impossible to recover from.
- Use `Result` propagation.
- Keep provider connectors independent.
- Keep transport and parser layers separate.
- Use transactional database migrations.
- Use UTC timestamps.
- Bound child-process and HTTP responses.
- Prefer explicit enums over stringly typed states.
- Redact errors before sending them to the frontend.
- Keep secrets wrapped in approved secret types where practical.
- Do not add PTY dependencies until the spike approves them.

---

## 13. Database Rules

- Database access belongs in Rust.
- Use migrations.
- Migrations must be transactional where supported.
- Enable foreign keys.
- Validate percentages with code and schema constraints.
- Do not store raw provider output in production tables.
- Do not store secrets.
- Preserve historical snapshots when disconnecting unless the user clears them.
- Add indexes for actual query patterns.
- Write repository tests using temporary databases.

Do not modify an applied migration.

Create a new migration instead.

---

## 14. UI Rules

The UI must follow:

```text
docs/ui-specification/
```

Core rules:

- one primary visual focus per screen;
- maximum one filled primary action per state;
- fixed header, provider tabs, and footer;
- scroll provider content only;
- preserve cached data while refreshing;
- clearly label stale/offline states;
- distinguish subscription and API views;
- reliability/source remains visible;
- warning and critical states do not rely on color alone;
- minimum desktop width is 360 logical pixels;
- compact mode changes density, not accessibility.

Do not redesign established states without updating the specification.

---

## 15. Accessibility Rules

Required:

- keyboard access to all actions;
- visible focus;
- correct tab semantics;
- accessible labels for icon-only buttons;
- screen-reader equivalents for usage rings and charts;
- no color-only status;
- reduced-motion support;
- usable at 125% text scaling;
- focus restoration after dialogs;
- no keyboard traps.

Accessibility failures are functional bugs, not optional polish.

---

## 16. Mock and Test Fixtures

Mocks must use production-normalized contracts.

Do not create separate simplified mock-only component props that cannot represent production data.

Provider parser fixtures:

- must be sanitized;
- must include CLI version family;
- must include expected normalized output;
- must include malformed/error states;
- must never contain real secrets or personal identifiers.

Raw captures belong in:

```text
.private/
```

and must not be committed.

---

## 17. Dependency Rules

Before adding a dependency:

1. explain why existing code or dependencies are insufficient;
2. confirm maintenance and platform compatibility;
3. identify whether it affects bundle size or security;
4. update the lockfile;
5. document architecture-impacting decisions.

Prefer:

- official Tauri plugins;
- focused Rust crates;
- one icon library;
- one chart library;
- one validation library.

Avoid:

- generic shell plugins exposed to the WebView;
- multiple overlapping state libraries;
- multiple component libraries;
- PTY dependencies before approval;
- heavy frontend packages for Rust-owned responsibilities.

---

## 18. File and Module Rules

Suggested boundaries:

```text
src/
├── app/
├── components/
├── features/
├── ipc/
├── stores/
├── lib/
└── styles/

src-tauri/src/
├── commands/
├── domain/
├── providers/
├── process/
├── database/
├── secrets/
├── scheduler/
├── desktop/
└── telemetry/
```

Avoid dumping unrelated code into:

```text
utils.ts
helpers.rs
common.ts
misc.rs
```

Create modules based on responsibility.

---

## 19. Task Workflow

Before coding:

1. Read the active task.
2. Read relevant documents.
3. Inspect current implementation.
4. State assumptions internally.
5. Identify affected modules.
6. Confirm no security boundary is crossed.

During coding:

1. Implement the smallest complete change.
2. Avoid unrelated edits.
3. Add or update tests.
4. Run focused checks.
5. Run broader checks before completion.
6. Update documentation when behavior changes.

After coding, report:

```text
- task completed;
- files changed;
- implementation summary;
- commands run;
- tests passed;
- tests failed;
- acceptance criteria status;
- security considerations;
- architecture deviations;
- known limitations;
- recommended next task.
```

Do not claim success without running the relevant checks.

---

## 20. Required Verification

Use the repository's actual scripts once scaffolded.

Expected frontend checks:

```bash
pnpm lint
pnpm typecheck
pnpm test
pnpm build
```

Expected Rust checks:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo check
```

Expected Tauri check:

```bash
pnpm tauri build
```

A full Tauri build may be deferred in small tasks when platform tooling is unavailable, but this must be reported clearly.

Do not invent successful test results.

---

## 21. Documentation Updates

Update documentation when changing:

- architecture;
- provider strategy;
- normalized data contract;
- IPC command;
- database schema;
- secret storage;
- UI flow;
- acceptance criteria;
- supported provider version;
- release behavior.

Architecture changes require an ADR under:

```text
docs/adr/
```

Do not rewrite large documents for minor implementation details.

---

## 22. Git Rules

- Keep commits focused.
- Do not commit secrets.
- Do not commit `.private/`.
- Do not commit raw terminal captures.
- Do not commit generated build outputs unless repository policy requires them.
- Do not remove user changes unrelated to the task.
- Do not use destructive Git commands without explicit user approval.
- Do not rewrite history unless explicitly requested.

Suggested commit format:

```text
type(scope): concise description
```

Examples:

```text
feat(tray): add popup toggle behavior
feat(ui): add disconnected provider states
test(parser): add codex usage fixtures
docs(adr): record native keyring decision
fix(refresh): prevent duplicate provider fetches
```

---

## 23. Prohibited Actions

Never:

- build the entire application from one broad prompt;
- implement future milestones without approval;
- access browser cookies;
- request provider passwords;
- inspect raw provider credential files;
- expose arbitrary shell execution to React;
- return stored API keys to React;
- store secrets in SQLite or logs;
- fabricate quota data;
- merge API and subscription usage;
- silently swallow parser errors;
- delete user data to fix a migration;
- disable TLS verification;
- enable unsigned automatic updates;
- add cloud sync during the local-first MVP;
- claim macOS or Windows compatibility without testing or clearly stating it is unverified.

---

## 24. Stop Conditions

Stop implementation and report when:

- a requested action would expose credentials;
- documents conflict on a security boundary;
- the provider requires browser-cookie scraping;
- the provider output cannot be safely parsed;
- an unsupported CLI version would produce unreliable data;
- a migration risks destructive data loss;
- signing or update security cannot be verified;
- required files are missing and guessing would alter architecture;
- tests reveal an unrelated critical regression.

A safe fallback is preferred over an unreliable implementation.

---

## 25. Definition of Done for a Task

A task is complete only when:

- the requested scope is implemented;
- acceptance criteria pass;
- relevant tests exist;
- relevant checks were run;
- no unrelated changes remain;
- security rules are satisfied;
- documentation is updated when necessary;
- limitations are reported honestly.

“Code compiles on my machine” is not enough when the task requires behavior, tests, or cross-platform validation.

---

## 26. Current Project Instruction

Until `spike/results/decision.md` is complete:

Allowed:

- repository scaffolding;
- desktop shell;
- mock UI;
- normalized domain contracts;
- fake providers;
- database infrastructure;
- settings;
- scheduler interfaces;
- test infrastructure.

Not allowed:

- production Codex parser;
- production Claude parser;
- production Antigravity parser;
- PTY dependency;
- final provider login claims;
- claims that all provider quota can be read automatically.

After the spike is reviewed, follow the selected strategy for each provider.
