# 11 — Development Mock Data and States

Mock scenarios exist only for development and automated UI tests.

Required scenarios for each active provider:

- loading;
- not connected;
- authentication required;
- connected with known remaining quota;
- connected with unknown quota;
- refreshing;
- cached;
- stale;
- authentication expired;
- provider unavailable;
- unrecognized output;
- internal error.

Example source fixture:

```ts
{ usedPercent: 45, period: 'weekly' }
```

Expected presentation:

```ts
{ remainingPercent: 55, label: 'Remaining weekly' }
```

Production builds must not expose a mock selector, fake provider command, or fallback fake percentage.
