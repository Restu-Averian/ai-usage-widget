import { useUiStore } from "../../stores/uiStore";
import { useProviderUsage } from "../../stores/queries";
import { startProviderLogin } from "../../ipc/commands";
import { UsageHero } from "./UsageHero";
import { QuotaCard } from "./QuotaCard";
import { HistoryChart } from "./HistoryChart";
import { AlertCircle } from "lucide-react";
import { useState } from "react";
import "./DashboardContent.css";

export function DashboardContent() {
  const { selectedProvider, mockScenarios } = useUiStore();
  const [loginMessage, setLoginMessage] = useState<string | null>(null);
  const scenario = mockScenarios[selectedProvider];
  const {
    data: state,
    isLoading,
    isError,
    isFetching,
    refetch,
  } = useProviderUsage(selectedProvider, scenario);
  const data = state?.usage ?? null;
  const status = state?.status;

  if (
    status === "authentication-required" ||
    scenario === "authentication-expired" ||
    scenario === "auth-expired"
  ) {
    return (
      <div className="dashboard-content centered state-panel auth-state">
        <AlertCircle size={32} className="warning-icon" />
        <span className="text-heading-md">Sign in required</span>
        <span className="text-body-sm text-secondary">
          Run the official Codex login flow, then refresh this provider.
        </span>
        {loginMessage && (
          <span className="text-caption text-secondary">{loginMessage}</span>
        )}
        <button
          className="primary-button"
          onClick={() => {
            if (selectedProvider !== "codex") {
              void refetch();
              return;
            }
            void startProviderLogin(selectedProvider)
              .then((result) => setLoginMessage(result.message ?? null))
              .catch((error: unknown) =>
                setLoginMessage(
                  error instanceof Error ? error.message : "Login failed.",
                ),
              );
          }}
        >
          Sign in
        </button>
      </div>
    );
  }

  if (status === "not-installed") {
    return (
      <div className="dashboard-content centered state-panel error-state">
        <AlertCircle size={32} className="error-icon" />
        <span className="text-heading-md">Codex CLI unavailable</span>
        <span className="text-body-sm text-secondary">
          Install the official Codex CLI, then refresh.
        </span>
        <button className="primary-button" onClick={() => void refetch()}>
          Retry
        </button>
      </div>
    );
  }

  if (status === "unsupported") {
    return (
      <div className="dashboard-content centered state-panel error-state">
        <AlertCircle size={32} className="error-icon" />
        <span className="text-heading-md">Codex CLI unsupported</span>
        <span className="text-body-sm text-secondary">
          This Codex CLI version is outside the supported M5 range.
        </span>
        <button className="primary-button" onClick={() => void refetch()}>
          Retry
        </button>
      </div>
    );
  }

  if (isError || status === "error" || scenario === "retryable-error") {
    return (
      <div className="dashboard-content centered state-panel error-state">
        <AlertCircle size={32} className="error-icon" />
        <span className="text-heading-md">Retryable error</span>
        <span className="text-body-sm text-secondary">
          {state?.lastError?.message ??
            "The usage check failed. Cached data is not available for this state."}
        </span>
        <button className="primary-button" onClick={() => void refetch()}>
          Retry
        </button>
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
        <button className="primary-button" onClick={() => void refetch()}>
          Connect {selectedProvider}
        </button>
      </div>
    );
  }

  // Render secondary windows (skipping the first which is in Hero)
  const secondaryWindows = data.windows.slice(1);

  return (
    <div className="dashboard-content">
      {(isFetching ||
        state?.isRefreshing ||
        scenario === "refreshing-with-cache") && (
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
