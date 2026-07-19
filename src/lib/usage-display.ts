import { UsageWindow } from "../ipc/types";

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
      accessibleLabel: "Remaining quota unavailable";
    };

export function isKnownUsagePercent(
  value: UsageWindow["usedPercent"],
): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

export function toRemainingPercent(
  usedPercent: UsageWindow["usedPercent"],
): number | null {
  if (!isKnownUsagePercent(usedPercent)) {
    return null;
  }

  return Math.min(100, Math.max(0, 100 - usedPercent));
}

export function getUsageTone(value: UsageWindow["usedPercent"]): UsageTone {
  const remaining = toRemainingPercent(value);
  if (remaining == null) {
    return "unknown";
  }

  if (remaining <= 5) {
    return "critical";
  }

  if (remaining <= 15) {
    return "warning";
  }

  return "normal";
}

export function getUsagePercentDisplay(
  value: UsageWindow["usedPercent"],
): UsagePercentDisplay {
  const remaining = toRemainingPercent(value);
  if (remaining == null) {
    return {
      kind: "unknown",
      label: "—",
      progressValue: null,
      accessibleLabel: "Remaining quota unavailable",
    };
  }

  const rounded = Math.round(remaining);

  return {
    kind: "known",
    label: `${rounded}%`,
    progressValue: remaining,
    accessibleLabel: `${rounded} percent remaining`,
  };
}

export function formatReset(resetAt: string | null | undefined): string {
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
