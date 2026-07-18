import { useQuery } from "@tanstack/react-query";
import { getUsageHistory, setFakeProviderScenario } from "../ipc/commands";
import { HistoryPoint, ProviderId, ProviderUsage } from "../ipc/types";
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
  return useQuery<ProviderUsage | null, Error>({
    queryKey: ["providerUsage", providerId, scenario],
    queryFn: async () => {
      if (scenario === "disconnected" || scenario === "loading-without-cache") {
        return null;
      }

      const state = await setFakeProviderScenario(
        providerId,
        toBackendScenario(scenario),
      );
      return state.usage ?? null;
    },
    refetchInterval: 30000,
    retry: false,
    staleTime: 10000,
  });
}
