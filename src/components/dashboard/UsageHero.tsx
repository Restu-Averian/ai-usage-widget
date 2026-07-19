import { ProviderUsage } from "../../ipc/types";
import {
  formatReset,
  getUsagePercentDisplay,
  getUsageTone,
} from "../../lib/usage-display";
import { UsageRing } from "./UsageRing";
import "./UsageHero.css";

interface UsageHeroProps {
  usage: ProviderUsage;
}

export function UsageHero({ usage }: UsageHeroProps) {
  // Find the primary window to display in the hero
  // In a real app this logic would determine "most critical" window
  const primaryWindow = usage.windows[0];

  if (!primaryWindow) {
    return <div className="usage-hero">No usage data</div>;
  }

  const display = getUsagePercentDisplay(primaryWindow.usedPercent);
  const tone = getUsageTone(primaryWindow.usedPercent);

  return (
    <div className={`usage-hero usage-tone-${tone}`}>
      <div className="hero-header">
        <span className="text-heading-md">
          {usage.planName || usage.provider}
        </span>
        <span className={`text-label-sm connection-status status-${tone}`}>
          {tone === "critical"
            ? "Critical"
            : tone === "warning"
              ? "Warning"
              : "Connected"}
        </span>
      </div>

      <div className="hero-ring-container">
        <UsageRing
          percentage={primaryWindow.usedPercent}
          label={`Remaining ${primaryWindow.period}`}
        />
        <span className="text-body-sm period-label">
          {display.kind === "unknown"
            ? `Remaining ${primaryWindow.period}`
            : `Remaining ${primaryWindow.period}`}
        </span>
      </div>

      <div className="hero-footer text-body-sm">
        <span>{formatReset(primaryWindow.resetAt)}</span>
      </div>
    </div>
  );
}
