# M8 — API Usage

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

Add API token, request, cost, and optional budget reporting as a connection type separate from subscription quota.

## 2. Intended Outcome

Users can securely add supported API/Admin keys and view API usage without exposing stored secrets to the WebView.

## 3. Dependencies

- M4 secret-store abstraction.
- M4 HTTP transport.
- API-key dialog UI.
- Provider API feasibility and permission requirements.
- Stable subscription/API separation.

## 4. In Scope

- API/Admin key input.
- Rust-side validation.
- Native credential storage.
- Key deletion.
- Provider API usage request.
- Token metrics.
- Request count when available.
- Cost metrics.
- Optional monthly budget.
- API usage history.
- Subscription/API segmented UI.
- API-specific errors.
- API reliability label.

## 5. Out of Scope

- Combining API and subscription percentages.
- Returning saved keys to frontend.
- Storing keys in SQLite.
- Cloud-hosted key storage.
- Organization administration beyond required usage permissions.

## 6. Suggested Task Groups

### M8-A — Secret flow

- Input dialog.
- validation command.
- secure save.
- exists/delete.
- clear frontend input.

### M8-B — Provider API connector

- HTTP client.
- usage request.
- cost request.
- permission detection.
- error normalization.

### M8-C — Data and UI

- API snapshot type.
- token/cost persistence.
- budget settings.
- history.
- segment switching.

### M8-D — Security testing

- no read-key command.
- log redaction.
- invalid-key handling.
- insufficient permission.
- secret deletion.

## 7. Acceptance Criteria

- At least one real API connector works.
- Secret is validated in Rust.
- Secret is stored in native credential store.
- Saved secret is never returned to frontend.
- API key never enters SQLite.
- API key never appears in logs.
- Disconnect deletes secret.
- Invalid key and insufficient-permission states work.
- API and subscription data remain separate.
- Budget percentage only appears when configured.
- API budget does not become default Menu Bar quota.
- Relevant security tests pass.

## 8. Verification

```text
validate a test key
restart app and refresh API usage
inspect SQLite for absence of key
inspect logs for absence of key
test invalid key
test insufficient permissions
delete connection
verify key no longer exists
```

## 9. Risks

- Organization-level APIs may require admin privileges.
- Cost endpoints may lag.
- Users may expect subscription quota from an API key.
- Keyring behavior may differ across OS.
- Provider permission errors may be unclear.

## 10. Review Questions

- Is permission copy accurate?
- Is secret exposure minimized?
- Are API metrics useful without a configured budget?
- Does UI avoid comparing unrelated percentages?

## 11. Exit and Handoff

M9 may use both subscription and API histories while keeping their alert policies distinct.
