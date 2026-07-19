import { useUiStore } from "../../stores/uiStore";
import { useProviderUsage } from "../../stores/queries";
import { getFooterStatusDisplay } from "../../lib/provider-state-display";
import "./StatusFooter.css";

export function StatusFooter() {
  const { selectedProvider, mockScenarios } = useUiStore();
  const scenario = mockScenarios[selectedProvider];
  const {
    data: state,
    isLoading,
    isError,
    isFetching,
  } = useProviderUsage(selectedProvider, scenario);
  const data = state?.usage ?? null;

  const statusDisplay = getFooterStatusDisplay(state, scenario, isError);

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
        <div className={`status-dot ${statusDisplay.colorClass}`} />
        <span className="text-caption footer-text">{statusDisplay.text}</span>
      </div>
    </footer>
  );
}
