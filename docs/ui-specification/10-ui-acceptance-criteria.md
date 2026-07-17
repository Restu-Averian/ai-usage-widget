# UI Acceptance Criteria

## 1. Product Shell

### UI-AC-001

Given the desktop app is running and the popup is hidden, when the user clicks the tray item, then the popup appears on the active monitor.

### UI-AC-002

Given the popup is visible, when the user clicks the tray item again, then the popup hides.

### UI-AC-003

Given no blocking dialog is active, when focus leaves the popup, then the popup hides.

### UI-AC-004

Closing or hiding the popup does not terminate the application.

### UI-AC-005

The popup never renders outside the active monitor work area.

---

## 2. Navigation

### UI-AC-010

The popup contains Codex, Claude, and Antigravity tabs in the canonical order.

### UI-AC-011

The selected provider persists between popup openings.

### UI-AC-012

Provider tabs are operable using Left, Right, Home, and End keys.

### UI-AC-013

Settings opens inside the popup and Back returns to the previously selected provider.

---

## 3. Disconnected States

### UI-AC-020

A disconnected provider shows only connection methods supported by backend capabilities.

### UI-AC-021

The screen never requests a provider password.

### UI-AC-022

Subscription and API connection actions are visually and textually distinct.

### UI-AC-023

When automatic usage is unavailable, an official dashboard fallback is displayed without proposing cookie extraction.

---

## 4. Connected Usage

### UI-AC-030

The most critical known subscription window is used as the hero.

### UI-AC-031

Every known secondary quota window remains visible.

### UI-AC-032

Unknown percentages are not rendered as zero.

### UI-AC-033

Usage direction is consistent: primary percentages represent used usage.

### UI-AC-034

Reset countdown and exact reset time are accessible.

### UI-AC-035

Reliability/source is visible.

### UI-AC-036

Subscription and API data are never merged into one metric.

---

## 5. Refresh and Freshness

### UI-AC-040

Refreshing with cached data keeps current content visible.

### UI-AC-041

Manual refresh cannot launch overlapping refreshes for the same provider.

### UI-AC-042

Stale data is clearly labeled.

### UI-AC-043

Offline mode displays the last successful snapshot when available.

### UI-AC-044

When reset time passes without a successful refresh, the UI says refresh is required to confirm.

---

## 6. Errors

### UI-AC-050

Provider errors use user-safe copy and do not display raw CLI output.

### UI-AC-051

Retry is shown only when the error is retryable.

### UI-AC-052

Authentication expiry presents a reconnect action.

### UI-AC-053

Unsupported CLI output presents an official-dashboard fallback.

### UI-AC-054

One provider error does not block navigation to other providers.

---

## 7. API Key Dialog

### UI-AC-060

The API key field is masked.

### UI-AC-061

The saved API key is never displayed after submission.

### UI-AC-062

Successful validation clears the input.

### UI-AC-063

The dialog explains secure operating-system storage.

### UI-AC-064

The frontend does not retain the secret in global state.

---

## 8. Settings

### UI-AC-070

Platform-specific settings are hidden when not applicable.

### UI-AC-071

Changing theme updates the UI without restart.

### UI-AC-072

Compact mode preserves accessible interactive target sizes.

### UI-AC-073

Destructive data actions require explicit confirmation.

### UI-AC-074

Connection details display subscription and API sections separately.

---

## 9. Accessibility

### UI-AC-080

Every icon-only button has an accessible name.

### UI-AC-081

Focus is visible for keyboard users.

### UI-AC-082

Usage rings and progress bars expose text equivalents.

### UI-AC-083

Warning and critical states do not rely only on color.

### UI-AC-084

The interface remains usable at 125% text scaling.

### UI-AC-085

Reduced motion disables nonessential animations.

### UI-AC-086

Charts provide a textual summary or accessible data equivalent.

---

## 10. Platform

### UI-AC-090 — macOS

- Menu Bar icon adapts to appearance.
- Popup can run without a Dock icon.
- Percentage title is optional.
- Popup behavior works with multiple monitors and a notched display.

### UI-AC-091 — Windows

- Tray icon is available.
- Popup does not appear in the taskbar.
- Tooltip provides summary data.
- Layout remains usable at 100–200% display scaling.

---

## 11. Visual Quality Checklist

- [ ] One clear visual focal point per provider screen.
- [ ] No more than one filled primary action per state.
- [ ] Header, tabs, and footer remain stable across states.
- [ ] Loading does not cause major layout shifts.
- [ ] Provider logos do not dominate the UI.
- [ ] Usage status colors are semantic and consistent.
- [ ] Unknown data never appears as 0%.
- [ ] Stale data remains readable.
- [ ] Connected and disconnected states are visually distinct.
- [ ] Settings rows use one consistent pattern.
- [ ] Compact mode is intentionally redesigned, not scaled down.
- [ ] Light and dark themes have equivalent hierarchy.
- [ ] Long account labels and paths do not break layout.
- [ ] Error states retain recovery actions.
- [ ] Security copy appears in credential-related flows.

---

## 12. Required Visual Stories Before Real Integration

The UI implementation is not ready for provider integration until these stories exist and are manually reviewed:

```text
Welcome
Codex disconnected
Claude CLI missing
Antigravity connecting
Codex connected with 5-hour + weekly windows
Claude connected near warning
Antigravity model-specific quota
Provider exhausted
API-only connection
Subscription + API segment switch
Refreshing with cache
Refreshing without cache
Offline with cache
Stale data
Authentication expired
Retryable provider error
Unsupported CLI version
Settings default
Settings compact mode
API-key dialog
Clear-history confirmation
Dark theme
Light theme
Minimum width
125% text scaling
Reduced motion
```
