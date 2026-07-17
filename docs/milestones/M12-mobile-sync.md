# M12 — Mobile Sync

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

Allow users to view normalized AI usage snapshots from Android and iOS without running desktop provider CLIs on mobile.

## 2. Intended Outcome

The desktop remains the trusted connector. Mobile clients receive encrypted normalized snapshots and present current quota and reset information.

## 3. Preconditions

- Stable desktop product.
- Confirmed user demand.
- Stable normalized snapshot schema.
- Privacy and threat-model review.
- Own-app authentication design.
- Device revocation design.
- Backend operating budget.

## 4. Architecture

```text
Desktop provider connector
        ↓
Normalized snapshot
        ↓
Encrypted sync API
        ↓
Android / iOS app
        ↓
Native home-screen widget
```

## 5. In Scope

- Own-app account/authentication.
- Encrypted snapshot upload.
- Encrypted snapshot retrieval.
- Desktop sync client.
- Device registration.
- Device revocation.
- Android/iOS application.
- Mobile provider overview.
- Mobile history.
- Native iOS WidgetKit widget.
- Native Android Glance/AppWidget.
- Sync status.
- Conflict/latest-snapshot policy.

## 6. Out of Scope

- Running provider CLIs on mobile.
- Syncing raw provider API keys by default.
- Syncing browser cookies.
- Remote account control.
- Remote CLI execution.
- Full desktop settings parity.

## 7. Data Allowed to Sync

Example:

```json
{
  "provider": "claude",
  "connectionType": "subscription-cli",
  "windows": [
    {
      "id": "weekly",
      "usedPercent": 68,
      "remainingPercent": 32,
      "resetAt": "2026-07-20T01:00:00Z"
    }
  ],
  "fetchedAt": "2026-07-16T02:30:00Z",
  "stale": false
}
```

Not allowed by default:

```text
provider API key
provider OAuth token
browser cookie
raw CLI output
full local logs
raw credential file
```

## 8. Suggested Task Groups

- Threat model.
- Authentication.
- encrypted storage.
- sync API.
- desktop client.
- mobile application shell.
- snapshot UI.
- device management.
- iOS widget.
- Android widget.
- privacy and deletion.

## 9. Acceptance Criteria

- Only normalized snapshots sync.
- Provider credentials remain on desktop.
- Data is encrypted in transit.
- Sensitive stored data is encrypted at rest according to the approved design.
- Device revocation works.
- Account deletion removes cloud snapshots.
- Mobile shows provider, usage, freshness, and reset time.
- Stale status remains visible.
- Native widgets use synchronized snapshots.
- Sync failure does not affect desktop provider refresh.
- Privacy documentation is updated.

## 10. Verification

```text
register desktop and phone
upload snapshot
read snapshot
revoke phone
verify access denied
delete account data
offline mobile cache test
stale snapshot test
iOS widget refresh test
Android widget refresh test
desktop without sync test
```

## 11. Risks

- Cloud accounts change the local-first trust model.
- Encryption/key management increases complexity.
- Mobile background-refresh restrictions affect freshness.
- Native widget APIs require separate platform code.
- Provider data may become sensitive when centralized.
- Backend cost may exceed product value.

## 12. Review Questions

- Is mobile usage frequent enough to justify cloud complexity?
- Can sync remain opt-in?
- Is end-to-end encryption necessary and feasible?
- Which data is genuinely required by widgets?
- Can users fully delete synced data?

## 13. Exit

M12 ends when mobile viewing works without weakening desktop credential security or misrepresenting snapshot freshness.
