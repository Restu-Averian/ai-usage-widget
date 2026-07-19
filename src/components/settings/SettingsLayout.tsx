import { useUiStore } from "../../stores/uiStore";
import {
  MockProviderScenario,
  MOCK_SCENARIO_OPTIONS,
} from "../../lib/mock/mock-adapter";
import { ProviderId } from "../../ipc/types";
import { getSettings, updateSettings } from "../../ipc/commands";
import { useEffect, useState } from "react";
import "./SettingsLayout.css";

const REFRESH_INTERVALS = [1, 5, 10, 15, 30] as const;

export function SettingsLayout() {
  const { mockScenarios, setMockScenario } = useUiStore();
  const [launchAtLogin, setLaunchAtLogin] = useState(false);
  const [refreshInterval, setRefreshInterval] =
    useState<(typeof REFRESH_INTERVALS)[number]>(5);

  useEffect(() => {
    void getSettings().then((settings) => {
      setLaunchAtLogin(settings.launchAtLogin);
      setRefreshInterval(settings.refresh.intervalMinutes);
    });
  }, []);

  const handleScenarioChange = (provider: ProviderId, scenario: string) => {
    setMockScenario(provider, scenario as MockProviderScenario);
  };

  const saveLaunchAtLogin = (next: boolean) => {
    setLaunchAtLogin(next);
    void getSettings().then((settings) =>
      updateSettings({ ...settings, launchAtLogin: next }),
    );
  };

  const saveRefreshInterval = (next: (typeof REFRESH_INTERVALS)[number]) => {
    setRefreshInterval(next);
    void getSettings().then((settings) =>
      updateSettings({
        ...settings,
        refresh: { ...settings.refresh, intervalMinutes: next },
      }),
    );
  };

  return (
    <div className="settings-layout">
      <div className="settings-section">
        <h2 className="text-heading-md section-title">Settings</h2>

        <div className="settings-row">
          <span className="text-body-md">Launch at login</span>
          <label className="toggle-switch">
            <input
              type="checkbox"
              checked={launchAtLogin}
              onChange={(e) => saveLaunchAtLogin(e.target.checked)}
            />
            <span className="toggle-slider"></span>
          </label>
        </div>

        <div className="settings-row">
          <label className="text-body-md" htmlFor="refresh-interval">
            Refresh interval
          </label>
          <select
            id="refresh-interval"
            className="settings-select"
            value={refreshInterval}
            onChange={(event) =>
              saveRefreshInterval(
                Number(
                  event.target.value,
                ) as (typeof REFRESH_INTERVALS)[number],
              )
            }
          >
            {REFRESH_INTERVALS.map((minutes) => (
              <option key={minutes} value={minutes}>
                {minutes} min
              </option>
            ))}
          </select>
        </div>
      </div>

      {import.meta.env.DEV && (
        <div className="settings-section">
          <h2 className="text-heading-md section-title">
            Developer Mock State
          </h2>

          <div className="mock-controls">
            {(["codex", "antigravity"] as ProviderId[]).map((provider) => (
              <div key={provider} className="settings-row mock-row">
                <span className="text-label-md capitalize">{provider}</span>
                <select
                  className="settings-select"
                  value={mockScenarios[provider]}
                  onChange={(e) =>
                    handleScenarioChange(provider, e.target.value)
                  }
                >
                  {MOCK_SCENARIO_OPTIONS.map((scenario) => (
                    <option key={scenario.id} value={scenario.id}>
                      {scenario.label}
                    </option>
                  ))}
                </select>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
