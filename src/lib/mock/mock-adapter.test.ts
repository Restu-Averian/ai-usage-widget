import { describe, expect, it } from "vitest";
import {
  fetchProviderUsage,
  getCachedUsageForScenario,
  MOCK_SCENARIO_OPTIONS,
} from "./mock-adapter";

describe("mock provider scenarios", () => {
  it("exposes every M3 verification scenario", () => {
    expect(MOCK_SCENARIO_OPTIONS.map((option) => option.id)).toEqual([
      "loading-without-cache",
      "refreshing-with-cache",
      "disconnected",
      "offline-with-cache",
      "stale",
      "retryable-error",
      "authentication-expired",
      "warning",
      "critical",
      "unknown-percentage",
    ]);
  });

  it("keeps cached usage available for refresh and offline states", () => {
    expect(
      getCachedUsageForScenario("codex", "refreshing-with-cache"),
    ).not.toBe(null);
    expect(
      getCachedUsageForScenario("codex", "offline-with-cache")?.stale,
    ).toBe(true);
  });

  it("returns unavailable percentage usage for the unknown scenario", async () => {
    const usage = await fetchProviderUsage("codex", "unknown-percentage");

    expect(usage?.windows[0]?.usedPercent).toBeUndefined();
  });

  it("throws a retryable error for the retryable error scenario", async () => {
    await expect(
      fetchProviderUsage("codex", "retryable-error"),
    ).rejects.toThrow("Retryable usage check failed");
  });
});
