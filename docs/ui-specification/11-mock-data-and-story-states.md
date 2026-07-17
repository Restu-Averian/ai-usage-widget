# Mock Data and Story States

## 1. Purpose

Mock data allows the complete UI to be built before real provider connectors are ready.

Mocks must use the same normalized contracts as production IPC responses.

---

## 2. Shared Types

```ts
export type ProviderId = "codex" | "claude" | "antigravity";

export type Reliability =
  | "official-api"
  | "official-sdk"
  | "official-cli-json"
  | "official-cli-text"
  | "experimental-pty"
  | "manual"
  | "dashboard-only";

export type UsageWindow = {
  id: string;
  label: string;
  period:
    | "session"
    | "five-hour"
    | "daily"
    | "weekly"
    | "monthly"
    | "model-specific"
    | "unknown";
  model?: string;
  usedPercent?: number;
  remainingPercent?: number;
  resetAt?: string;
};

export type ProviderUsage = {
  provider: ProviderId;
  connectionType:
    | "subscription-cli"
    | "oauth-cli"
    | "api-key"
    | "admin-api-key"
    | "cloud-project"
    | "dashboard-only";
  accountLabel?: string;
  planName?: string;
  windows: UsageWindow[];
  reliability: Reliability;
  fetchedAt: string;
  stale: boolean;
  warnings: string[];
};
```

---

## 3. Codex Connected

```ts
export const codexConnected: ProviderUsage = {
  provider: "codex",
  connectionType: "subscription-cli",
  accountLabel: "r••••@example.com",
  planName: "ChatGPT Plus",
  windows: [
    {
      id: "five-hour",
      label: "5-hour limit",
      period: "five-hour",
      usedPercent: 68,
      remainingPercent: 32,
      resetAt: "2026-07-16T14:00:00+07:00",
    },
    {
      id: "weekly",
      label: "Weekly limit",
      period: "weekly",
      usedPercent: 43,
      remainingPercent: 57,
      resetAt: "2026-07-20T08:00:00+07:00",
    },
  ],
  reliability: "official-cli-text",
  fetchedAt: "2026-07-16T09:30:00+07:00",
  stale: false,
  warnings: [],
};
```

---

## 4. Claude Critical

```ts
export const claudeCritical: ProviderUsage = {
  provider: "claude",
  connectionType: "subscription-cli",
  accountLabel: "r••••@example.com",
  planName: "Claude Max",
  windows: [
    {
      id: "weekly",
      label: "Weekly limit",
      period: "weekly",
      usedPercent: 96,
      remainingPercent: 4,
      resetAt: "2026-07-20T08:00:00+07:00",
    },
    {
      id: "five-hour",
      label: "5-hour session",
      period: "five-hour",
      usedPercent: 81,
      remainingPercent: 19,
      resetAt: "2026-07-16T10:20:00+07:00",
    },
  ],
  reliability: "experimental-pty",
  fetchedAt: "2026-07-16T09:34:00+07:00",
  stale: false,
  warnings: ["experimental-parser"],
};
```

---

## 5. Antigravity Model-Specific

```ts
export const antigravityModels: ProviderUsage = {
  provider: "antigravity",
  connectionType: "oauth-cli",
  accountLabel: "r••••@gmail.com",
  planName: "Google AI Plan",
  windows: [
    {
      id: "gemini-pro",
      label: "Model quota",
      period: "model-specific",
      model: "Gemini Pro",
      usedPercent: 84,
      remainingPercent: 16,
      resetAt: "2026-07-16T13:00:00+07:00",
    },
    {
      id: "gemini-flash",
      label: "Model quota",
      period: "model-specific",
      model: "Gemini Flash",
      usedPercent: 35,
      remainingPercent: 65,
      resetAt: "2026-07-16T13:00:00+07:00",
    },
  ],
  reliability: "official-cli-text",
  fetchedAt: "2026-07-16T09:35:00+07:00",
  stale: false,
  warnings: [],
};
```

---

## 6. Unknown Percentage

```ts
export const usageWithoutPercentage: ProviderUsage = {
  provider: "codex",
  connectionType: "subscription-cli",
  planName: "ChatGPT Plus",
  windows: [
    {
      id: "weekly",
      label: "Weekly limit",
      period: "weekly",
      resetAt: "2026-07-20T08:00:00+07:00",
    },
  ],
  reliability: "official-cli-text",
  fetchedAt: "2026-07-16T09:30:00+07:00",
  stale: false,
  warnings: ["percentage-unavailable"],
};
```

Expected UI:

```text
Weekly usage
Percentage not reported

Resets in 3d 22h
```

No empty `0%` ring.

---

## 7. Stale Snapshot

```ts
export const staleClaude: ProviderUsage = {
  ...claudeCritical,
  stale: true,
  fetchedAt: "2026-07-16T06:10:00+07:00",
  warnings: ["offline", "experimental-parser"],
};
```

Expected UI:

```text
Offline banner
Cached usage remains visible
Footer shows exact freshness
```

---

## 8. API Usage Mock

```ts
export const openAiApiUsage = {
  provider: "codex",
  connectionType: "admin-api-key",
  accountLabel: "Sample Organization",
  planName: "OpenAI API",
  period: "monthly",
  tokenUsage: {
    input: 1_240_000,
    output: 240_000,
    cached: 610_000,
    requests: 8_214,
  },
  costUsage: {
    used: 12,
    budget: 25,
    currency: "USD",
  },
  fetchedAt: "2026-07-16T09:30:00+07:00",
  reliability: "official-api",
  stale: false,
};
```

---

## 9. Story State IDs

Use stable IDs for Storybook, visual tests, or mock routes:

```text
welcome/default
provider/disconnected/codex
provider/disconnected/claude
provider/disconnected/antigravity
provider/not-installed/claude
provider/connecting/antigravity
provider/connected/codex-normal
provider/connected/claude-warning
provider/connected/claude-critical
provider/connected/antigravity-models
provider/connected/unknown-percentage
provider/connected/subscription-and-api
provider/refreshing/with-cache
provider/refreshing/no-cache
provider/stale/with-cache
provider/offline/with-cache
provider/auth-expired
provider/error/retryable
provider/error/no-cache
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

---

## 10. History Data

```ts
export const sevenDayHistory = [
  { timestamp: "2026-07-10T09:00:00+07:00", value: 18 },
  { timestamp: "2026-07-11T09:00:00+07:00", value: 24 },
  { timestamp: "2026-07-12T09:00:00+07:00", value: 31 },
  { timestamp: "2026-07-13T09:00:00+07:00", value: 45 },
  { timestamp: "2026-07-14T09:00:00+07:00", value: 57 },
  { timestamp: "2026-07-15T09:00:00+07:00", value: 63 },
  { timestamp: "2026-07-16T09:00:00+07:00", value: 68 },
];
```

Required edge cases:

```text
empty history
one point
two points
flat values
decreasing after reset
100% values
missing days
stale last point
```

---

## 11. Mock Provider Adapter

Development should include a fake connector capable of switching states.

Conceptual API:

```ts
type MockProviderScenario =
  | "disconnected"
  | "not-installed"
  | "connecting"
  | "connected-normal"
  | "connected-warning"
  | "connected-critical"
  | "unknown-percentage"
  | "refreshing"
  | "stale"
  | "offline"
  | "auth-expired"
  | "error"
  | "unsupported";
```

The fake adapter should support:

- configurable response delay;
- deterministic errors;
- changing values after refresh;
- simulated quota reset;
- simulated authentication completion.

Do not embed mock-switching controls in production builds.
