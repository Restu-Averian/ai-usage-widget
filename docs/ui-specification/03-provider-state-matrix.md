# Provider State Matrix

## 1. State Source

The backend owns provider connection and refresh state.

The frontend derives visual state from:

```text
installation
authentication
connection type
cached snapshot
freshness
active refresh
last error
provider capability
```

The frontend must not infer authentication solely from the presence of cached usage.

---

## 2. Canonical Provider States

| State                     | Meaning                                          | Primary UI                        |
| ------------------------- | ------------------------------------------------ | --------------------------------- |
| `unknown`                 | No detection has run                             | Neutral skeleton                  |
| `detecting`               | CLI/account capability check active              | Detection progress                |
| `not-installed`           | Required CLI missing                             | Installation guidance             |
| `authentication-required` | CLI exists, user not signed in                   | Connect action                    |
| `connecting`              | Official login flow active                       | Waiting state                     |
| `connected`               | Usable connection and current data               | Usage dashboard                   |
| `refreshing`              | Fetch active                                     | Cached data + subtle progress     |
| `stale`                   | Cached data exists but refresh failed/expired    | Banner + cached dashboard         |
| `offline`                 | Network unavailable                              | Offline banner + cached dashboard |
| `error`                   | Fetch failed and no usable data                  | Error state                       |
| `unsupported`             | Installed provider version cannot be read safely | Dashboard fallback                |
| `path-changed`            | Approved CLI path changed                        | Approval warning                  |
| `permission-denied`       | Required local permission denied                 | Permission guidance               |

---

## 3. Derivation Matrix

| Installation | Auth       | Snapshot | Refresh | Error              | UI State                  |
| ------------ | ---------- | -------- | ------- | ------------------ | ------------------------- |
| Unknown      | Unknown    | None     | No      | None               | `unknown`                 |
| Detecting    | Unknown    | Any      | Yes     | None               | `detecting`               |
| Missing      | N/A        | None     | No      | None               | `not-installed`           |
| Installed    | Required   | None     | No      | None               | `authentication-required` |
| Installed    | Connecting | Any      | Yes     | None               | `connecting`              |
| Installed    | Valid      | Fresh    | No      | None               | `connected`               |
| Installed    | Valid      | Fresh    | Yes     | None               | `refreshing`              |
| Installed    | Valid      | Stale    | No      | Network            | `offline` or `stale`      |
| Installed    | Expired    | Any      | No      | Auth               | `authentication-required` |
| Installed    | Valid      | None     | No      | Retryable          | `error`                   |
| Installed    | Valid      | Any      | No      | Unsupported parser | `unsupported`             |
| Changed path | Unknown    | Any      | No      | Trust              | `path-changed`            |

---

## 4. Provider Capability States

Connection methods must come from backend capabilities.

Example:

```ts
type ProviderCapabilities = {
  canDetectInstallation: boolean;
  canDetectAuthentication: boolean;
  canStartLogin: boolean;
  canFetchSubscriptionUsage: boolean;
  canFetchApiUsage: boolean;
  canDisconnectLocalConnection: boolean;
  requiresTty: boolean;
  supportsMultipleWindows: boolean;
  supportsModelWindows: boolean;
};
```

### UI rule

Do not render:

```text
Connect Local CLI
```

unless `canFetchSubscriptionUsage` and the required detection capability are true.

Do not render:

```text
Add API Key
```

unless `canFetchApiUsage` is true.

---

## 5. Snapshot Freshness

| Snapshot condition                | UI                                    |
| --------------------------------- | ------------------------------------- |
| No snapshot                       | Empty or error state                  |
| Fresh                             | Normal connected state                |
| Stale but usable                  | Connected dashboard with stale banner |
| Older than 24 hours               | Stronger stale warning                |
| Unknown percentages               | Textual state, no empty ring          |
| Reset time passed without refresh | Keep prior state and mark unconfirmed |

Example reset-passed copy:

```text
Reset time passed · Refresh to confirm current usage
```

---

## 6. Multi-Connection Provider States

A provider may have:

- subscription only;
- API only;
- both;
- neither.

| Subscription | API | Presentation                    |
| ------------ | --- | ------------------------------- |
| No           | No  | Disconnected state              |
| Yes          | No  | Subscription dashboard          |
| No           | Yes | API dashboard                   |
| Yes          | Yes | Subscription/API segment switch |

Errors remain connection-specific.

Example:

```text
Subscription: connected
API: invalid key
```

The provider tab should not show a total error when one usable connection remains.

---

## 7. Refresh Priority

When a provider has both connection types:

1. Refresh the currently visible segment first.
2. Refresh the other segment afterward.
3. Maintain separate loading indicators.
4. A subscription refresh failure must not erase successful API data.

---

## 8. Attention Indicators

Provider tab indicators:

| Condition                 | Indicator          |
| ------------------------- | ------------------ |
| Usage warning             | small warning dot  |
| Usage critical            | small critical dot |
| Authentication required   | attention dot      |
| Error with no cached data | error dot          |
| Stale with cached data    | no dot by default  |
| Refreshing                | no tab spinner     |
| Connected normal          | none               |

Only one dot is shown. Priority:

```text
error
authentication
critical
warning
none
```

---

## 9. Reliability Labels

Possible labels:

```text
Official API
Official SDK
Official CLI
CLI parser
Experimental
Manual
Dashboard only
```

Display rules:

- Always visible in connection details.
- Visible below usage history in provider dashboard.
- `Experimental` uses a tooltip:

```text
This integration depends on a provider CLI format
that may change in future versions.
```

- Reliability does not use alarming red styling unless data is unsupported.

---

## 10. Error-State Actions

| Error code               | Primary action          | Secondary action |
| ------------------------ | ----------------------- | ---------------- |
| CLI not installed        | Open installation guide | Retry            |
| Not authenticated        | Connect provider        | Open dashboard   |
| Authentication expired   | Reconnect               | View details     |
| Network unavailable      | Retry                   | None             |
| Provider unavailable     | Retry                   | Open dashboard   |
| Rate limited             | Retry later             | Open dashboard   |
| Command timeout          | Retry                   | View details     |
| CLI path changed         | Review path             | Disconnect       |
| Parser unsupported       | Check again             | Open dashboard   |
| Invalid API key          | Update key              | Disconnect API   |
| Secret store unavailable | Open settings/help      | Retry            |
| Database error           | Restart app             | Open logs        |
