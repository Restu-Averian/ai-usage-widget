import { useUiStore } from "../stores/uiStore";
import { AppHeader } from "./layout/AppHeader";
import { ProviderTabs } from "./layout/ProviderTabs";
import { StatusFooter } from "./layout/StatusFooter";
import { DashboardContent } from "./dashboard/DashboardContent";
import { SettingsLayout } from "./settings/SettingsLayout";
import "./MockUIApp.css";

export function MockUIApp() {
  const { viewState, compactMode } = useUiStore();

  return (
    <div className={`app-container ${compactMode ? "compact-mode" : ""}`}>
      <AppHeader />

      {viewState === "dashboard" ? (
        <>
          <ProviderTabs />
          <DashboardContent />
          <StatusFooter />
        </>
      ) : (
        <SettingsLayout />
      )}
    </div>
  );
}
