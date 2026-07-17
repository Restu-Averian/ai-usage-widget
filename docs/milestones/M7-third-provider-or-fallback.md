# M7 — Third Provider or Safe Fallback

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

Complete the initial Codex, Claude, and Antigravity product promise without using unsafe or misleading integration methods.

## 2. Intended Outcome

Every initial provider has either:

- a working automatic integration; or
- a complete, honest official-dashboard fallback.

## 3. Dependencies

- M0 decision for the third provider.
- M5 and M6 provider architecture.
- Unsupported/fallback UI state.
- Provider capability model.

## 4. In Scope

### Automatic path

- detection;
- authentication;
- usage fetch;
- parser;
- persistence;
- UI;
- cache;
- errors;
- tests.

### Fallback path

- unsupported explanation;
- official usage/dashboard action;
- capability-driven connection UI;
- recheck after CLI update;
- supported-version messaging.

## 5. Out of Scope

- Browser-cookie scraping.
- Private network-call reverse engineering.
- Unlabeled estimation.
- Treating API usage as subscription usage.
- Blocking the entire release because one provider is dashboard-only.

## 6. Suggested Task Groups

- Revalidate spike result against current CLI version.
- Implement automatic connector or explicit fallback.
- Add capability and version states.
- Add provider-specific copy.
- Add dashboard/open action.
- Add recheck behavior.
- Complete three-provider regression suite.

## 7. Acceptance Criteria

- Codex has automatic integration or fallback.
- Claude has automatic integration or fallback.
- Antigravity has automatic integration or fallback.
- UI only offers supported connection methods.
- Unsupported state explains the limitation safely.
- No unsafe credential method is introduced.
- No unavailable percentage is estimated silently.
- Product copy does not promise complete automation where it is absent.
- Recheck behavior works after provider CLI change.
- Full provider regression tests pass.

## 8. Verification

```text
review provider capability matrix
test each provider disconnected state
test each provider connected/fallback state
test official dashboard action
test unsupported version
test provider update re-detection
run full suite
```

## 9. Risks

- Product marketing may overstate provider coverage.
- Dashboard URLs may change.
- A provider may appear integrated but lack reset accuracy.
- Users may confuse fallback with a failed setup.

## 10. Review Questions

- Is each provider status honest?
- Is partial automation still useful?
- Are capability differences understandable?
- Should public beta list supported CLI versions prominently?

## 11. Exit and Handoff

After M7, the subscription-focused MVP has complete initial-provider coverage and can proceed to API usage and monitoring enhancements.
