import { useUiStore } from "../../stores/uiStore";
import { useProviderUsage } from "../../stores/queries";
import "./StatusFooter.css";

export function StatusFooter() {
  const { selectedProvider, mockScenarios } = useUiStore();
  const scenario = mockScenarios[selectedProvider];
  const { data, isLoading, isError, isFetching } = useProviderUsage(
    selectedProvider,
    scenario,
  );

  let statusText = "Loading...";
  let statusColorClass = "status-indicator-loading";

  if (scenario === "loading-without-cache") {
    statusText = "Loading...";
    statusColorClass = "status-indicator-loading";
  } else if (isError || scenario === "retryable-error") {
    statusText = "Error";
    statusColorClass = "status-indicator-error";
  } else if (
    scenario === "authentication-expired" ||
    scenario === "auth-expired"
  ) {
    statusText = "Reconnect";
    statusColorClass = "status-indicator-error";
  } else if (scenario === "disconnected") {
    statusText = "Disconnected";
    statusColorClass = "status-indicator-offline";
  } else if (scenario === "offline-with-cache" || scenario === "offline") {
    statusText = "Offline";
    statusColorClass = "status-indicator-offline";
  } else if (data && data.stale) {
    statusText = "Stale";
    statusColorClass = "status-indicator-offline";
  } else if (data) {
    statusText = "Online";
    statusColorClass = "status-indicator-online";
  }

  return (
    <footer className="status-footer">
      <span className="text-caption footer-text">
        {data?.fetchedAt
          ? `Updated ${new Date(data.fetchedAt).toLocaleTimeString()}`
          : isLoading || isFetching
            ? "Refreshing..."
            : "Data unavailable"}
      </span>
      <div className="status-indicator-container">
        <div className={`status-dot ${statusColorClass}`} />
        <span className="text-caption footer-text">{statusText}</span>
      </div>
    </footer>
  );
}
