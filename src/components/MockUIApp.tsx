import { useUiStore } from "../stores/uiStore";
import { AppHeader } from "./layout/AppHeader";
import { ProviderTabs } from "./layout/ProviderTabs";
import { StatusFooter } from "./layout/StatusFooter";
import { DashboardContent } from "./dashboard/DashboardContent";
import { SettingsLayout } from "./settings/SettingsLayout";
import "./MockUIApp.css";

export function MockUIApp() {
  const { viewState } = useUiStore();

  return (
    <div className="app-container">
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
