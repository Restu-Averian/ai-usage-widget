# M0 — Provider Feasibility

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

Determine whether Codex, Claude, and Antigravity expose subscription usage in a safe, accurate, and maintainable format.

This milestone addresses the largest product risk before production connectors are implemented.

## 2. Intended Outcome

A reviewed decision exists for every provider:

```text
official-api
official-sdk
official-cli-json
official-cli-text
experimental-pty
dashboard-only
unsupported
```

The result also identifies which provider should be implemented first.

## 3. Dependencies

- Technical Spike document.
- Local access to the provider CLIs.
- Authenticated test accounts where available.
- `.private/` excluded from Git.
- Security rules from `AGENTS.md`.

## 4. In Scope

- Detect CLI executable paths.
- Record CLI versions.
- Inspect documented commands and help output.
- Capture authenticated usage.
- Capture logged-out behavior.
- Capture offline and error behavior.
- Determine whether TTY or PTY is required.
- Sanitize captured output.
- Build throwaway parser prototypes.
- Add fixture-based parser tests.
- Score integration strategies.
- Produce a final provider decision.

## 5. Out of Scope

- Production Tauri UI.
- Production provider connector.
- API token/cost reporting.
- Browser-cookie extraction.
- Reading raw provider credential stores.
- Reverse-engineering private IDE network calls.
- Public distribution.

## 6. Deliverables

```text
spike/
├── scripts/
│   ├── detect-tools.*
│   ├── capture-command.*
│   ├── redact-output.*
│   └── inspect-ansi.*
├── parsers/
├── fixtures/
├── tests/
└── results/
    ├── codex.md
    ├── claude.md
    ├── antigravity.md
    └── decision.md
```

## 7. Suggested Task Groups

### M0-A — Spike foundation

- Create directory structure.
- Add `.private/` protection.
- Add cross-platform command runner.
- Add redaction utility.
- Add output-size and timeout controls.

### M0-B — CLI discovery

- Detect Codex.
- Detect Claude.
- Detect Antigravity.
- Record paths, versions, and supported help flags.

### M0-C — Real provider experiments

- Test Codex authenticated and logged-out states.
- Test Claude subscription and API-key modes.
- Test Antigravity authentication modes and model usage.
- Test offline behavior.
- Test terminal width differences.

### M0-D — Parsing and fixtures

- Sanitize captures.
- Create version-family fixtures.
- Build parser prototypes.
- Reject malformed and unsupported output.
- Add expected normalized results.

### M0-E — Decision

- Score approaches.
- Document security risk.
- Select first provider.
- Decide whether the MVP can support all providers, partial providers, or only fallback.

## 8. Acceptance Criteria

- All installed providers have recorded executable paths and versions.
- Real authenticated usage is inspected for every available provider.
- No provider password is requested.
- No browser cookie or raw stored credential is read.
- Raw captures remain under `.private/`.
- Committed fixtures are sanitized.
- Every prototype parser has fixture tests.
- Unsupported output produces an explicit error.
- Percentages and reset times are never fabricated.
- `spike/results/decision.md` contains a recommendation for every provider.
- The first production provider is selected based on evidence.

## 9. Verification

Record:

```text
- operating system;
- architecture;
- provider CLI versions;
- commands tested;
- exit codes;
- timeout behavior;
- fixture test command;
- fixture test result;
- files checked for accidental secrets.
```

## 10. Risks

- Interactive TUI output may change frequently.
- A CLI may expose visible usage but no automation surface.
- Terminal width or color may alter output.
- Subscription and API authentication may be confused.
- Provider updates may invalidate parser fixtures.
- Windows behavior may differ from macOS.

## 11. Review Questions

- Is every selected strategy safe?
- Is any PTY integration actually justified?
- Can unsupported providers use an honest dashboard fallback?
- Does the product still have enough value with partial provider support?
- Which provider offers the best first vertical slice?

## 12. Exit and Handoff

This milestone ends with one project decision:

```text
Proceed with all providers
Proceed with partial MVP
Reframe around API usage
Stop
```

Handoff to M5 must identify:

- first provider;
- supported CLI version family;
- parser strategy;
- required fixtures;
- known unsupported states;
- dashboard fallback URL strategy.
