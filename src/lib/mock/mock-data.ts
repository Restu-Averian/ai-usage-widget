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
  usedPercent?: number | null;
  remainingPercent?: number | null;
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

export const claudeWarning: ProviderUsage = {
  ...claudeCritical,
  windows: claudeCritical.windows.map((window, index) =>
    index === 0
      ? {
          ...window,
          usedPercent: 84,
          remainingPercent: 16,
        }
      : window,
  ),
  stale: false,
  warnings: ["experimental-parser"],
};

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

export const staleClaude: ProviderUsage = {
  ...claudeCritical,
  stale: true,
  fetchedAt: "2026-07-16T06:10:00+07:00",
  warnings: ["offline", "experimental-parser"],
};

export const offlineCodex: ProviderUsage = {
  ...codexConnected,
  stale: true,
  fetchedAt: "2026-07-16T08:48:00+07:00",
  warnings: ["offline"],
};

export const openAiApiUsage = {
  provider: "codex" as ProviderId,
  connectionType: "admin-api-key" as const,
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
  reliability: "official-api" as Reliability,
  stale: false,
};

export const sevenDayHistory = [
  { timestamp: "2026-07-10T09:00:00+07:00", value: 18 },
  { timestamp: "2026-07-11T09:00:00+07:00", value: 24 },
  { timestamp: "2026-07-12T09:00:00+07:00", value: 31 },
  { timestamp: "2026-07-13T09:00:00+07:00", value: 45 },
  { timestamp: "2026-07-14T09:00:00+07:00", value: 57 },
  { timestamp: "2026-07-15T09:00:00+07:00", value: 63 },
  { timestamp: "2026-07-16T09:00:00+07:00", value: 68 },
];
