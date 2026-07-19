import { useQuery } from "@tanstack/react-query";
import {
  getUsageHistory,
  refreshProvider,
  setFakeProviderScenario,
} from "../ipc/commands";
import { HistoryPoint, ProviderId, ProviderState } from "../ipc/types";
import { MockProviderScenario } from "../lib/mock/mock-adapter";

function toBackendScenario(scenario: MockProviderScenario) {
  switch (scenario) {
    case "warning":
    case "connected-warning":
      return "warning";
    case "critical":
    case "connected-critical":
      return "critical";
    case "unknown-percentage":
      return "unknown-percentage";
    case "stale":
      return "stale";
    case "offline":
    case "offline-with-cache":
      return "offline";
    case "authentication-expired":
    case "auth-expired":
      return "auth-expired";
    case "retryable-error":
    case "error":
      return "retryable-error";
    default:
      return "connected-normal";
  }
}

export function useUsageHistory(providerId: ProviderId) {
  return useQuery<HistoryPoint[], Error>({
    queryKey: ["usageHistory", providerId],
    queryFn: () => getUsageHistory(providerId),
    retry: false,
    staleTime: 10000,
  });
}

export function useProviderUsage(
  providerId: ProviderId,
  scenario: MockProviderScenario,
) {
  return useQuery<ProviderState, Error>({
    queryKey: ["providerUsage", providerId, scenario],
    queryFn: async () => {
      if (providerId === "codex") {
        return refreshProvider(providerId);
      }

      if (scenario === "disconnected" || scenario === "loading-without-cache") {
        return {
          provider: providerId,
          status: "authentication-required",
          usage: null,
          lastError: null,
          isRefreshing: false,
        };
      }

      return setFakeProviderScenario(providerId, toBackendScenario(scenario));
    },
    refetchInterval: 30000,
    retry: false,
    staleTime: 10000,
  });
}
