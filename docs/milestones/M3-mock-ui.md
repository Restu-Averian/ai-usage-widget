# M3 — Mock UI

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

Implement the full product interface using normalized fake data before real provider integration.

## 2. Intended Outcome

Every major visual state can be reviewed, tested, and refined independently of Codex, Claude, or Antigravity connectivity.

## 3. Dependencies

- M1 repository foundation.
- UI Specification.
- Technical Design normalized contracts.
- M2 shell preferred, but browser development may begin earlier.

## 4. In Scope

- Welcome screen.
- Provider tabs.
- Disconnected states.
- CLI missing state.
- Connecting state.
- Connected subscription usage.
- Connected API usage.
- Subscription/API segment control.
- Refreshing with and without cache.
- Stale and offline.
- Authentication expired.
- Retryable error.
- Unsupported provider.
- Settings.
- Connection details.
- API-key dialog UI.
- Confirmation dialogs.
- Light/dark/system themes.
- Compact mode.
- History chart.
- Keyboard interaction.
- Accessibility.
- Mock story states.

## 5. Out of Scope

- Real CLI execution.
- Real API validation.
- Persistent secrets.
- Production SQL storage.
- Real notifications.
- Production provider parsing.

## 6. Required Stories

Use stable story IDs from:

```text
../ui-specification/11-mock-data-and-story-states.md
```

At minimum:

```text
welcome/default
provider/disconnected/*
provider/not-installed/claude
provider/connecting/antigravity
provider/connected/codex-normal
provider/connected/claude-critical
provider/connected/antigravity-models
provider/connected/unknown-percentage
provider/connected/subscription-and-api
provider/refreshing/with-cache
provider/refreshing/no-cache
provider/offline/with-cache
provider/auth-expired
provider/error/retryable
provider/unsupported/version
settings/default
settings/compact
dialog/api-key
dialog/clear-history
theme/light
theme/dark
layout/min-width
layout/text-scale-125
accessibility/reduced-motion
```

## 7. Suggested Task Groups

### M3-A — Design tokens

- Semantic colors.
- Typography.
- Spacing.
- Radius.
- motion tokens.
- light/dark themes.

### M3-B — Shared shell UI

- App header.
- Provider tabs.
- Status footer.
- Scroll regions.
- Settings route.

### M3-C — Usage components

- Usage hero.
- Usage ring.
- Quota cards.
- Reset countdown.
- History chart.
- Reliability badge.

### M3-D — Connection states

- Disconnected.
- CLI missing.
- Connecting.
- Authentication expired.
- Unsupported.
- Error.

### M3-E — Settings and dialogs

- Settings sections.
- Connection details.
- API key dialog.
- destructive confirmation.

### M3-F — Accessibility and layout

- Keyboard navigation.
- focus management.
- reduced motion.
- minimum width.
- text scaling.
- screen-reader labels.

## 8. Acceptance Criteria

- All required stories render using normalized mock contracts.
- One clear focal point exists per provider screen.
- Unknown percentage never appears as `0%`.
- Subscription and API usage are distinct.
- Cached content remains visible while refreshing.
- Stale and offline states are explicit.
- Header, tabs, and footer remain stable.
- Provider tabs support keyboard navigation.
- Icon-only buttons have accessible names.
- Usage graphs have text equivalents.
- Light and dark themes preserve hierarchy.
- 360 px width remains usable.
- 125% text scaling remains usable.
- Reduced motion removes nonessential animation.
- No development scenario selector appears in production builds.

## 9. Verification

```text
pnpm lint
pnpm typecheck
pnpm test
pnpm build
visual review for all required stories
keyboard-only walkthrough
screen-reader semantics inspection
minimum-width inspection
light/dark comparison
```

## 10. Risks

- Mock contracts may drift from Rust domain types.
- Too much visual polish may delay architecture.
- Charts may be inaccessible.
- Compact mode may reduce touch/focus targets.
- Provider branding may overpower semantic usage states.

## 11. Review Questions

- Can the UI be understood in under a few seconds?
- Is the most critical quota obvious?
- Are source and freshness honest?
- Are error and fallback states complete?
- Can production data replace mocks without component rewrites?

## 12. Exit and Handoff

M4 receives:

- stable normalized frontend schemas;
- stable component boundaries;
- expected IPC response shapes;
- complete fake provider scenarios.
