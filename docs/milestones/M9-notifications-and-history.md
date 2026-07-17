# M9 — Notifications and History

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

Make AI Usage Dock useful as an ambient monitor without requiring constant manual opening.

## 2. Intended Outcome

Users receive deduplicated, meaningful notifications and can inspect recent local usage trends.

## 3. Dependencies

- At least one working real provider.
- Snapshot persistence.
- Notification-state repository.
- OS notification integration.
- History chart UI.

## 4. In Scope

- Warning notification.
- Critical notification.
- Confirmed quota-reset notification.
- Authentication-expired notification.
- Notification deduplication.
- Notification click routing.
- Seven-day history.
- Subscription history.
- API history.
- Retention settings.
- Retention cleanup.
- Empty and reset-history states.
- Accessible chart summary.

## 5. Out of Scope

- Push notifications to mobile.
- Cloud notification service.
- Notifications based on unconfirmed resets.
- High-frequency usage polling beyond provider-safe policy.

## 6. Notification Identity

Deduplication key:

```text
provider
connection type
quota window
threshold
quota period identifier
```

## 7. Suggested Task Groups

- OS notification setup.
- threshold evaluation.
- deduplication.
- reset detection.
- click routing.
- history query.
- retention cleanup.
- accessible chart summaries.
- notification settings.

## 8. Acceptance Criteria

- Warning sends once per window/period.
- Critical sends once per window/period.
- Notifications do not repeat every refresh.
- Reset notification requires successful refreshed evidence.
- Authentication-expired notification is deduplicated.
- Notification click opens correct provider/segment.
- History survives restart.
- Retention cleanup is transactional.
- Empty history is handled.
- Quota reset is visible in history.
- Charts include accessible text equivalent.
- Subscription and API alert policies remain distinct.

## 9. Verification

```text
simulate crossing warning threshold
simulate crossing critical threshold
repeat refresh and confirm no duplicate
simulate confirmed reset
click notification
change retention
run cleanup
restart and inspect history
keyboard/screen-reader chart review
```

## 10. Risks

- Reset detection may be false if provider data is stale.
- Excess notifications can reduce trust.
- Historical percentages may be incomparable across changed provider plans.
- Cleanup can accidentally remove too much data.

## 11. Review Questions

- Does each notification help the user act?
- Is deduplication robust?
- Is reset evidence strong enough?
- Is history useful without becoming a full analytics dashboard?

## 12. Exit and Handoff

M10 receives a functionally useful product ready for packaging and reliability hardening.
