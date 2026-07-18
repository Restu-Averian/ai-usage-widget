import { useUiStore } from "../../stores/uiStore";
import { useProviderUsage } from "../../stores/queries";
import { UsageHero } from "./UsageHero";
import { QuotaCard } from "./QuotaCard";
import { HistoryChart } from "./HistoryChart";
import { AlertCircle } from "lucide-react";
import "./DashboardContent.css";

export function DashboardContent() {
  const { selectedProvider, mockScenarios } = useUiStore();
  const scenario = mockScenarios[selectedProvider];
  const { data, isLoading, isError, isFetching } = useProviderUsage(
    selectedProvider,
    scenario,
  );

  if (scenario === "authentication-expired" || scenario === "auth-expired") {
    return (
      <div className="dashboard-content centered state-panel auth-state">
        <AlertCircle size={32} className="warning-icon" />
        <span className="text-heading-md">Authentication expired</span>
        <span className="text-body-sm text-secondary">
          {selectedProvider} needs to reconnect.
        </span>
        <button className="primary-button">Reconnect</button>
      </div>
    );
  }

  if (isError || scenario === "retryable-error") {
    return (
      <div className="dashboard-content centered state-panel error-state">
        <AlertCircle size={32} className="error-icon" />
        <span className="text-heading-md">Retryable error</span>
        <span className="text-body-sm text-secondary">
          The usage check failed. Cached data is not available for this state.
        </span>
        <button className="primary-button">Retry</button>
      </div>
    );
  }

  if ((isLoading && !data) || scenario === "loading-without-cache") {
    return (
      <div className="dashboard-content centered">
        <span className="text-body-md">Loading usage data…</span>
      </div>
    );
  }

  if (!data) {
    return (
      <div className="dashboard-content centered state-panel">
        <span className="text-heading-md">
          {selectedProvider} is not connected
        </span>
        <span className="text-body-sm text-secondary">
          Connect a local provider session to view usage.
        </span>
        <button className="primary-button">Connect {selectedProvider}</button>
      </div>
    );
  }

  // Render secondary windows (skipping the first which is in Hero)
  const secondaryWindows = data.windows.slice(1);

  return (
    <div className="dashboard-content">
      {(isFetching || scenario === "refreshing-with-cache") && (
        <div className="refresh-banner">
          <AlertCircle size={16} />
          <span className="text-label-sm">Refreshing cached data…</span>
        </div>
      )}

      {(scenario === "offline-with-cache" || scenario === "offline") && (
        <div className="offline-banner">
          <AlertCircle size={16} />
          <span className="text-label-sm">Offline · showing cached usage</span>
        </div>
      )}

      {data.stale && (
        <div className="stale-banner">
          <AlertCircle size={16} />
          <span className="text-label-sm">Data may be outdated</span>
        </div>
      )}

      <UsageHero usage={data} />

      {secondaryWindows.length > 0 && (
        <div className="secondary-windows">
          {secondaryWindows.map((window) => (
            <QuotaCard key={window.id} window={window} />
          ))}
        </div>
      )}

      <HistoryChart />

      <div className="source-info text-caption">Source: {data.reliability}</div>
    </div>
  );
}
