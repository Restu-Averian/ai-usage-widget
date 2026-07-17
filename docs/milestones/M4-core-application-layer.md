# M4 — Core Application Layer

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

Build the reusable Rust and IPC infrastructure shared by every provider integration.

## 2. Intended Outcome

A fake provider can complete the full production-shaped path:

```text
refresh request
→ provider registry
→ normalized usage
→ SQLite snapshot
→ backend event
→ frontend query invalidation
→ updated UI
```

## 3. Dependencies

- M1 repository foundation.
- M2 desktop lifecycle.
- M3 normalized mock contracts.
- Technical Design.
- M0 results for PTY decisions, if PTY is considered.

## 4. In Scope

- Rust domain types.
- Typed application errors.
- Provider capability model.
- Provider registry.
- Fake provider connector.
- Process-runner abstraction.
- Optional HTTP transport abstraction.
- Typed Tauri commands.
- Frontend IPC wrapper.
- Zod validation.
- SQLx SQLite.
- Migrations.
- Connection repository.
- Snapshot repository.
- Notification-state repository boundary.
- Settings service.
- Native secret-store abstraction.
- Refresh scheduler.
- Per-provider single-flight.
- Retry backoff.
- wake/network event boundaries.
- Redacted logging.
- Backend events.

## 5. Out of Scope

- Production provider parser.
- Production PTY implementation unless explicitly approved.
- Real API usage connector.
- Release signing.
- Cloud sync.

## 6. Suggested Task Groups

### M4-A — Domain contracts

- Provider ID.
- Connection type.
- Reliability.
- Usage periods.
- Usage windows.
- Provider usage.
- Application errors.

### M4-B — IPC

- Bootstrap command.
- List providers.
- Get provider state.
- Refresh provider.
- Refresh all.
- Settings commands.
- History commands.
- Runtime frontend schemas.

### M4-C — Persistence

- SQLite initialization.
- Migrations.
- connection repository.
- snapshot/window repository.
- history query.
- retention boundary.

### M4-D — Security infrastructure

- Native secret-store interface.
- save/exists/delete behavior.
- redaction.
- no secret-read IPC.
- Tauri capability review.

### M4-E — Refresh orchestration

- scheduler.
- single-flight.
- backoff.
- stale calculation.
- events.
- fake provider refresh.

## 7. Acceptance Criteria

- Fake provider uses the same registry interface planned for real providers.
- Normalized usage is persisted and restored after restart.
- Frontend cannot execute SQL.
- Frontend cannot retrieve stored secrets.
- IPC responses are runtime validated.
- One provider refresh does not block another.
- Duplicate refresh requests are coalesced.
- Child-process abstraction includes timeout and output limits.
- Database migrations have tests.
- Repositories use temporary-database tests.
- Logs redact secret-like content.
- Provider errors are typed and user-safe.
- One provider failure does not crash application startup.

## 8. Verification

```text
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo check
fake-provider integration test
database migration test
secret-store interface test
redaction tests
```

## 9. Risks

- Frontend/backend contracts may diverge.
- Secret storage may behave differently by platform.
- SQL migrations may become destructive.
- Scheduler can accidentally overlap work.
- Generic abstractions may become too broad before real providers exist.

## 10. Review Questions

- Is every backend-owned concern still in Rust?
- Are commands narrow and typed?
- Can a provider be added without changing unrelated modules?
- Are errors safe for UI?
- Does fake provider exercise the real path?

## 11. Exit and Handoff

M5 receives:

- provider interface;
- transport interfaces;
- parser boundary;
- persistence;
- scheduler;
- secure storage;
- stable UI data path.
