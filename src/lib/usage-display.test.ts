import { describe, expect, it } from "vitest";
import {
  getUsagePercentDisplay,
  getUsageTone,
  isKnownUsagePercent,
  toRemainingPercent,
} from "./usage-display";

describe("usage display helpers", () => {
  it.each([
    [0, 100],
    [45, 55],
    [100, 0],
    [null, null],
    [-5, 100],
    [120, 0],
  ] as const)("maps %s used to %s remaining", (used, remaining) => {
    expect(toRemainingPercent(used)).toBe(remaining);
  });

  it.each([undefined, null])(
    "treats %s usage as unavailable instead of zero",
    (value) => {
      const display = getUsagePercentDisplay(value);

      expect(isKnownUsagePercent(value)).toBe(false);
      expect(display).toEqual({
        kind: "unknown",
        label: "—",
        progressValue: null,
        accessibleLabel: "Remaining quota unavailable",
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
    [80, "normal"],
    [85, "warning"],
    [96, "critical"],
  ] as const)("maps %s used percent to %s remaining tone", (percent, tone) => {
    expect(getUsageTone(percent)).toBe(tone);
  });
});
