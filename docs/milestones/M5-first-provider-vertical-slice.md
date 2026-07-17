# M5 — First Provider Vertical Slice

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

Implement one real provider from local detection through visible, persisted usage.

## 2. Provider Selection

The provider must be selected from:

```text
spike/results/decision.md
```

Do not assume Codex, Claude, or Antigravity in advance.

Selection criteria:

- safest supported interface;
- highest parsing stability;
- accurate quota and reset data;
- cross-platform viability;
- lowest maintenance risk.

## 3. Intended Outcome

```text
Detect
→ authenticate or connect
→ fetch
→ parse
→ normalize
→ persist
→ display
→ refresh
→ cache offline
→ notify state changes
```

## 4. Dependencies

- M0 provider decision.
- M4 core application layer.
- M3 production-ready UI states.
- Sanitized provider fixtures.
- Supported CLI version policy.

## 5. In Scope

- CLI installation detection.
- CLI path approval.
- CLI version detection.
- Authentication-state detection.
- Official login launching where supported.
- Real usage retrieval.
- Parser implementation.
- Normalization.
- Persistence.
- Manual refresh.
- Scheduled refresh.
- Offline cached state.
- Authentication expiry.
- Unsupported-version state.
- Provider-specific error mapping.
- Dashboard fallback.
- Parser and integration tests.

## 6. Out of Scope

- Second provider.
- API usage unless this provider's selected vertical slice explicitly requires it.
- Cloud sync.
- Broad parser abstraction refactor.
- Public installer.

## 7. Required States

```text
not-installed
authentication-required
connecting
connected-normal
connected-warning
connected-critical
refreshing
offline-with-cache
stale
authentication-expired
command-timeout
output-unrecognized
version-unsupported
dashboard-fallback
```

## 8. Suggested Task Groups

### M5-A — Detection and trust

- Resolve executable.
- Record version.
- Approve path.
- Handle path change.

### M5-B — Authentication

- Detect auth.
- Launch official login.
- Poll completion.
- Handle cancellation/expiry.

### M5-C — Fetch and parse

- Run safe command.
- Enforce timeout/output size.
- Parse versioned fixture families.
- Normalize usage.
- Reject unknown output.

### M5-D — Application integration

- Save snapshots.
- Emit refresh events.
- Update UI.
- Implement cache/offline.
- Add dashboard fallback.

### M5-E — Testing and review

- Parser fixtures.
- transport tests.
- database integration.
- manual real-account test.
- security review.

## 9. Acceptance Criteria

- Real authenticated usage appears in the popup.
- Reliability/source is visible.
- Every supported version has fixtures.
- Parser failure never becomes zero usage.
- No browser cookie or raw stored credential is accessed.
- Unknown values remain unknown.
- Reset timestamps are validated.
- Last successful snapshot survives restart.
- Offline mode displays cached data.
- Authentication expiry shows reconnect.
- Command timeout is enforced.
- Unsupported versions produce a safe state.
- Provider failure does not break other providers or app startup.
- Manual and scheduled refresh both work.
- Relevant tests pass.

## 10. Verification

Record:

```text
provider CLI version
OS and architecture
authentication mode
real usage test timestamp
fixtures used
parser test command
integration test command
offline test
timeout test
unsupported-version test
```

## 11. Risks

- Provider format may change after release.
- PTY behavior may differ by OS.
- Authentication detection may be unreliable.
- User-visible reset labels may be relative rather than absolute.
- CLI path may be replaced by an untrusted binary.

## 12. Review Questions

- Is the integration truly supported or only incidentally parseable?
- Is reliability labeled honestly?
- Can output changes fail closed?
- Is the implementation isolated?
- Should this pattern be reused for the next provider?

## 13. Exit and Handoff

Before M6:

- document lessons learned;
- update provider interface only when justified;
- update supported version matrix;
- record known limitations;
- preserve independent provider boundaries.
