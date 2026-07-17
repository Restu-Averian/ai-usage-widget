# M6 — Second Provider Integration

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

Add the second real provider while proving that provider integrations remain independent.

## 2. Intended Outcome

Two providers can:

- connect independently;
- refresh concurrently;
- maintain separate cache;
- surface separate errors;
- coexist without shared-parser coupling.

## 3. Dependencies

- M5 reviewed vertical slice.
- M0 strategy for the second provider.
- Stable provider registry and persistence.
- UI state coverage.

## 4. In Scope

- Detection.
- Authentication.
- Usage fetch.
- Parsing.
- Normalization.
- Persistence.
- UI integration.
- Offline cache.
- Dashboard fallback.
- Provider-specific errors.
- Refresh All behavior.
- Tab attention indicator.
- Tests.

## 5. Out of Scope

- Third provider.
- Cross-provider quota blending.
- API usage unless separately scheduled.
- Premature shared parser framework.

## 6. Additional Design Constraints

- Extract shared code only if both providers genuinely use the same concern.
- Keep command names, output formats, and version support provider-local.
- A login flow for one provider must not block navigation to the other.
- A failure in one provider must not mark the entire app unavailable.
- Refresh All may run providers concurrently within safe limits.

## 7. Suggested Task Groups

- Provider detection and path trust.
- Authentication flow.
- Fetch/parser.
- persistence/UI.
- concurrency and Refresh All.
- parser fixtures.
- regression tests for first provider.

## 8. Acceptance Criteria

- Two providers show real usage or selected safe fallback.
- Each provider preserves independent state.
- Each has separate cached snapshots.
- Refresh All works.
- One provider timeout does not delay the other beyond intended orchestration.
- Authentication changes remain provider-specific.
- Errors remain provider-specific.
- First-provider tests still pass.
- New parser fixtures are sanitized and version-aware.
- No shared abstraction weakens security or correctness.

## 9. Verification

```text
refresh both providers
force one provider error
verify other provider updates
restart and restore both snapshots
test independent login state
run full parser suite
run full Rust/frontend suite
```

## 10. Risks

- Shared scheduling can create contention.
- Error state may be accidentally global.
- Provider-specific fields may be lost through over-normalization.
- Refactor pressure may destabilize M5.

## 11. Review Questions

- Did the second provider require a different transport?
- Are differences represented by capabilities rather than conditionals in UI?
- Did any M5 assumption prove too provider-specific?
- Is Refresh All robust?

## 12. Exit and Handoff

M7 receives a proven multi-provider architecture and a list of remaining limitations for the third provider.
