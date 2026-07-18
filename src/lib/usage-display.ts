import { UsageWindow } from "./mock/mock-data";

export type UsageTone = "normal" | "warning" | "critical" | "unknown";

export type UsagePercentDisplay =
  | {
      kind: "known";
      label: string;
      progressValue: number;
      accessibleLabel: string;
    }
  | {
      kind: "unknown";
      label: "—";
      progressValue: null;
      accessibleLabel: "Usage unavailable";
    };

export function isKnownUsagePercent(
  value: UsageWindow["usedPercent"],
): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

export function getUsageTone(value: UsageWindow["usedPercent"]): UsageTone {
  if (!isKnownUsagePercent(value)) {
    return "unknown";
  }

  if (value >= 95) {
    return "critical";
  }

  if (value >= 80) {
    return "warning";
  }

  return "normal";
}

export function getUsagePercentDisplay(
  value: UsageWindow["usedPercent"],
): UsagePercentDisplay {
  if (!isKnownUsagePercent(value)) {
    return {
      kind: "unknown",
      label: "—",
      progressValue: null,
      accessibleLabel: "Usage unavailable",
    };
  }

  const rounded = Math.round(value);

  return {
    kind: "known",
    label: `${rounded}%`,
    progressValue: Math.min(100, Math.max(0, value)),
    accessibleLabel: `${rounded} percent used`,
  };
}

export function formatReset(resetAt: string | undefined): string {
  if (!resetAt) {
    return "Reset time unavailable";
  }

  return `Resets ${new Date(resetAt).toLocaleString(undefined, {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  })}`;
}
