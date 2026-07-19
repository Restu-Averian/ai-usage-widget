import {
  ProviderId,
  ProviderUsage,
  codexConnected,
  claudeCritical,
  claudeWarning,
  antigravityModels,
  usageWithoutPercentage,
  staleClaude,
  offlineCodex,
} from "./mock-data";

export type MockProviderScenario =
  | "loading-without-cache"
  | "refreshing-with-cache"
  | "disconnected"
  | "offline-with-cache"
  | "stale"
  | "retryable-error"
  | "authentication-expired"
  | "warning"
  | "critical"
  | "unknown-percentage"
  | "connected-normal"
  | "connected-warning"
  | "connected-critical"
  | "offline"
  | "error"
  | "auth-expired";

export const MOCK_SCENARIO_OPTIONS: {
  id: MockProviderScenario;
  label: string;
}[] = [
  { id: "loading-without-cache", label: "Loading without cache" },
  { id: "refreshing-with-cache", label: "Refreshing with cache" },
  { id: "disconnected", label: "Disconnected" },
  { id: "offline-with-cache", label: "Offline with cache" },
  { id: "stale", label: "Stale" },
  { id: "retryable-error", label: "Retryable error" },
  { id: "authentication-expired", label: "Authentication expired" },
  { id: "warning", label: "Warning" },
  { id: "critical", label: "Critical" },
  { id: "unknown-percentage", label: "Unknown usage" },
];

export const DEFAULT_MOCK_SCENARIOS: Record<ProviderId, MockProviderScenario> =
  {
    codex: "connected-normal",
    claude: "warning",
    antigravity: "connected-normal",
  };

const DELAY_MS = 350;

function baseUsage(providerId: ProviderId): ProviderUsage {
  if (providerId === "antigravity") {
    return antigravityModels;
  }

  if (providerId === "claude") {
    return claudeWarning;
  }

  return codexConnected;
}

function withPrimaryPercent(
  usage: ProviderUsage,
  usedPercent: number,
  remainingPercent: number,
): ProviderUsage {
  return {
    ...usage,
    windows: usage.windows.map((window, index) =>
      index === 0 ? { ...window, usedPercent, remainingPercent } : window,
    ),
  };
}

export function getCachedUsageForScenario(
  providerId: ProviderId,
  scenario: MockProviderScenario,
): ProviderUsage | null {
  switch (scenario) {
    case "refreshing-with-cache":
      return baseUsage(providerId);
    case "offline":
    case "offline-with-cache":
      return providerId === "claude" ? staleClaude : offlineCodex;
    case "stale":
      return providerId === "claude"
        ? staleClaude
        : { ...baseUsage(providerId), stale: true };
    default:
      return null;
  }
}

export async function fetchProviderUsage(
  providerId: ProviderId,
  scenario: MockProviderScenario,
): Promise<ProviderUsage | null> {
  await new Promise((resolve) => setTimeout(resolve, DELAY_MS));

  switch (scenario) {
    case "loading-without-cache":
    case "disconnected":
      return null;
    case "authentication-expired":
    case "auth-expired":
      return null;
    case "connected-normal":
      return baseUsage(providerId);
    case "warning":
    case "connected-warning":
      return providerId === "claude"
        ? claudeWarning
        : withPrimaryPercent(baseUsage(providerId), 84, 16);
    case "critical":
    case "connected-critical":
      return providerId === "claude"
        ? claudeCritical
        : withPrimaryPercent(baseUsage(providerId), 96, 4);
    case "unknown-percentage":
      return usageWithoutPercentage;
    case "refreshing-with-cache":
      return baseUsage(providerId);
    case "offline":
    case "offline-with-cache":
      return providerId === "claude" ? staleClaude : offlineCodex;
    case "stale":
      return providerId === "claude"
        ? staleClaude
        : { ...baseUsage(providerId), stale: true };
    case "retryable-error":
    case "error":
      throw new Error(`Retryable usage check failed for ${providerId}`);
    default:
      return baseUsage(providerId);
  }
}
