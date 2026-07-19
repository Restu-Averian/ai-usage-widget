import { describe, expect, it } from "vitest";
import {
  getUsagePercentDisplay,
  getUsageTone,
  isKnownUsagePercent,
} from "./usage-display";

describe("usage display helpers", () => {
  it.each([undefined, null])(
    "treats %s usage as unavailable instead of zero",
    (value) => {
      const display = getUsagePercentDisplay(value);

      expect(isKnownUsagePercent(value)).toBe(false);
      expect(display).toEqual({
        kind: "unknown",
        label: "—",
        progressValue: null,
        accessibleLabel: "Usage unavailable",
      });
    },
  );

  it("does not render unavailable usage as a 0 percent value", () => {
    const display = getUsagePercentDisplay(undefined);

    expect(display.label).not.toBe("0%");
    expect(display.progressValue).not.toBe(0);
  });

  it.each([
    [79.99, "normal"],
    [80, "warning"],
    [95, "critical"],
  ] as const)("maps %s percent to %s tone", (percent, tone) => {
    expect(getUsageTone(percent)).toBe(tone);
  });
});
