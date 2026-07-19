import { describe, expect, it } from "vitest";
import { providerUsageSchema } from "./schemas";

describe("IPC schemas", () => {
  it("preserves unknown usage percentages", () => {
    const usage = providerUsageSchema.parse({
      id: "snapshot-1",
      provider: "codex",
      connectionType: "subscription-cli",
      windows: [
        {
          id: "weekly",
          label: "Weekly limit",
          period: "weekly",
          usedPercent: null,
          remainingPercent: null,
          derivedUsedPercent: false,
          derivedRemainingPercent: false,
        },
      ],
      reliability: "official-cli-text",
      fetchedAt: "2026-07-18T03:00:00Z",
      stale: false,
      warnings: ["percentage-unavailable"],
    });

    expect(usage.windows[0]?.usedPercent).toBeNull();
  });

  it("rejects percentages outside 0 to 100", () => {
    expect(() =>
      providerUsageSchema.parse({
        id: "snapshot-1",
        provider: "codex",
        connectionType: "subscription-cli",
        windows: [
          {
            id: "weekly",
            label: "Weekly limit",
            period: "weekly",
            usedPercent: 101,
            remainingPercent: null,
            derivedUsedPercent: false,
            derivedRemainingPercent: false,
          },
        ],
        reliability: "official-cli-text",
        fetchedAt: "2026-07-18T03:00:00Z",
        stale: false,
        warnings: [],
      }),
    ).toThrow();
  });
});
