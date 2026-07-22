import { useQueryClient } from "@tanstack/react-query";
import { useUiStore } from "../../stores/uiStore";
import { RefreshCw, Settings, ChevronLeft } from "lucide-react";
import "./AppHeader.css";

export function AppHeader() {
  const queryClient = useQueryClient();
  const { viewState, selectedProvider, mockScenarios, setViewState } =
    useUiStore();

  if (viewState === "settings") {
    return (
      <header className="app-header">
        <button
          className="icon-button"
          onClick={() => setViewState("dashboard")}
          aria-label="Back to dashboard"
        >
          <ChevronLeft size={20} />
        </button>
        <span className="text-heading-md header-title">Settings</span>
      </header>
    );
  }

  return (
    <header className="app-header">
      <div className="header-logo-container">
        {/* Placeholder for app icon */}
        <div className="app-icon-placeholder" />
        <span className="text-heading-md header-title">AI Usage Widget</span>
      </div>
      <div className="header-actions">
        <button
          className="icon-button"
          aria-label="Refresh"
          onClick={() =>
            queryClient.invalidateQueries({
              queryKey: [
                "providerUsage",
                selectedProvider,
                mockScenarios[selectedProvider],
              ],
            })
          }
        >
          <RefreshCw size={18} />
        </button>
        <button
          className="icon-button"
          onClick={() => setViewState("settings")}
          aria-label="Settings"
        >
          <Settings size={18} />
        </button>
      </div>
    </header>
  );
}
