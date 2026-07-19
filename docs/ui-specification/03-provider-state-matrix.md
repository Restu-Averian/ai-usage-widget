# 03 — Provider State Matrix

| State | Ring | Primary copy | Actions |
| --- | --- | --- | --- |
| Loading | `—` | Loading provider data | None |
| Not connected | `—` | Not connected | Connect |
| Authentication required | `—` | Sign in required | Connect |
| Connected fresh | Remaining % | Remaining period | Refresh |
| Refreshing | Previous value or `—` | Refreshing | Disabled refresh |
| Cached | Remaining % | Cached data | Refresh |
| Stale | Remaining % or `—` | Data may be outdated | Refresh |
| Authentication expired | `—` or cached value | Reconnect required | Reconnect |
| Provider unavailable | `—` or cached value | Provider unavailable | Retry |
| Output unrecognized | `—` or cached value | Usage format unsupported | Retry / dashboard |
| Internal error | `—` or cached value | Could not load usage | Retry |

Unknown values never render as `0%`.
