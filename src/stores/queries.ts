import { useQuery } from "@tanstack/react-query";
import {
  fetchProviderUsage,
  getCachedUsageForScenario,
  MockProviderScenario,
} from "../lib/mock/mock-adapter";
import { ProviderId, ProviderUsage } from "../lib/mock/mock-data";

export function useProviderUsage(
  providerId: ProviderId,
  scenario: MockProviderScenario,
) {
  return useQuery<ProviderUsage | null, Error>({
    queryKey: ["providerUsage", providerId, scenario],
    queryFn: () => fetchProviderUsage(providerId, scenario),
    placeholderData:
      getCachedUsageForScenario(providerId, scenario) ?? undefined,
    refetchInterval: 30000,
    retry: false,
    staleTime: 10000,
  });
}
