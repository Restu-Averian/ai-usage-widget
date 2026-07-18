import { UsageWindow } from "../../ipc/types";
import {
  formatReset,
  getUsagePercentDisplay,
  getUsageTone,
} from "../../lib/usage-display";
import "./QuotaCard.css";

interface QuotaCardProps {
  window: UsageWindow;
}

export function QuotaCard({ window }: QuotaCardProps) {
  const display = getUsagePercentDisplay(window.usedPercent);
  const tone = getUsageTone(window.usedPercent);

  let colorVar = "var(--color-accent)";
  if (tone === "critical") {
    colorVar = "var(--color-critical)";
  } else if (tone === "warning") {
    colorVar = "var(--color-warning)";
  }

  return (
    <div className={`quota-card quota-card-${tone}`}>
      <div className="quota-card-header">
        <span className="text-label-md">{window.label}</span>
        <span className="text-label-sm">{display.label}</span>
      </div>
      <div className="quota-progress-track">
        {display.kind === "known" && (
          <div
            className="quota-progress-fill"
            style={{
              width: `${display.progressValue}%`,
              backgroundColor: colorVar,
            }}
          />
        )}
      </div>
      <div className="quota-card-footer text-caption">
        {window.model && <span>{window.model} &middot; </span>}
        <span>{formatReset(window.resetAt)}</span>
      </div>
    </div>
  );
}
