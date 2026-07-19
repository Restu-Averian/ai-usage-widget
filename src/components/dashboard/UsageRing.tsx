import "./UsageRing.css";
import { getUsagePercentDisplay, getUsageTone } from "../../lib/usage-display";

interface UsageRingProps {
  percentage: number | null | undefined;
  size?: number;
  strokeWidth?: number;
  label?: string;
}

export function UsageRing({
  percentage,
  size = 140,
  strokeWidth = 12,
  label = "Usage",
}: UsageRingProps) {
  const display = getUsagePercentDisplay(percentage);

  if (display.kind === "unknown") {
    return (
      <div
        className="usage-ring-container usage-ring-unknown"
        style={{ width: size, height: size }}
        role="img"
        aria-label={`${label}: ${display.accessibleLabel}`}
      >
        <span className="text-display-sm">{display.label}</span>
        <span className="text-body-sm">Remaining unavailable</span>
      </div>
    );
  }

  const radius = (size - strokeWidth) / 2;
  const circumference = radius * 2 * Math.PI;
  const offset = circumference - (display.progressValue / 100) * circumference;

  let colorVar = "var(--color-accent)";
  const tone = getUsageTone(percentage);
  if (tone === "critical") {
    colorVar = "var(--color-critical)";
  } else if (tone === "warning") {
    colorVar = "var(--color-warning)";
  }

  return (
    <div className="usage-ring-container" style={{ width: size, height: size }}>
      <svg
        width={size}
        height={size}
        className="usage-ring-svg"
        role="img"
        aria-label={`${label}: ${display.accessibleLabel}`}
      >
        <circle
          className="usage-ring-track"
          strokeWidth={strokeWidth}
          r={radius}
          cx={size / 2}
          cy={size / 2}
        />
        <circle
          className="usage-ring-fill"
          strokeWidth={strokeWidth}
          stroke={colorVar}
          strokeDasharray={circumference}
          strokeDashoffset={offset}
          strokeLinecap="round"
          r={radius}
          cx={size / 2}
          cy={size / 2}
        />
      </svg>
      <div className="usage-ring-text">
        <span className="text-display-sm">{display.label}</span>
      </div>
    </div>
  );
}
