import { describe, expect, it } from "vitest";
import { ProviderState } from "../ipc/types";
import { getFooterStatusDisplay } from "./provider-state-display";

function state(status: ProviderState["status"]): ProviderState {
  return {
    provider: "codex",
    status,
    usage: null,
    lastError: null,
    isRefreshing: false,
  };
}

describe("provider state display", () => {
  it("maps backend provider states to footer labels", () => {
    expect(
      getFooterStatusDisplay(state("not-installed"), "connected-normal", false)
        .text,
    ).toBe("CLI missing");
    expect(
      getFooterStatusDisplay(
        state("authentication-required"),
        "connected-normal",
        false,
      ).text,
    ).toBe("Sign in");
    expect(
      getFooterStatusDisplay(state("unsupported"), "connected-normal", false)
        .text,
    ).toBe("Unsupported");
  });

  it("does not treat unknown usage as an error", () => {
    const connected = state("connected");
    connected.usage = {
      id: "usage-1",
      provider: "codex",
      connectionType: "subscription-cli",
      accountLabel: null,
      planName: null,
      windows: [
        {
          id: "primary",
          label: "Primary",
          period: "weekly",
          model: null,
          usedPercent: null,
          remainingPercent: null,
          resetAt: null,
          derivedUsedPercent: false,
          derivedRemainingPercent: false,
        },
      ],
      tokenUsage: null,
      costUsage: null,
      reliability: "official-cli-json",
      fetchedAt: "2026-07-18T00:00:00Z",
      stale: false,
      warnings: ["percentage-unavailable"],
    };

    expect(
      getFooterStatusDisplay(connected, "unknown-percentage", false).text,
    ).toBe("Online");
  });
});
