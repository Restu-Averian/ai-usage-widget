import { useUiStore } from "../../stores/uiStore";
import {
  MockProviderScenario,
  MOCK_SCENARIO_OPTIONS,
} from "../../lib/mock/mock-adapter";
import { ProviderId } from "../../ipc/types";
import { useThemePreference } from "../../lib/theme";
import "./SettingsLayout.css";

export function SettingsLayout() {
  const [themePreference, setThemePreference] = useThemePreference();
  const { compactMode, mockScenarios, setCompactMode, setMockScenario } =
    useUiStore();

  const handleScenarioChange = (provider: ProviderId, scenario: string) => {
    setMockScenario(provider, scenario as MockProviderScenario);
  };

  return (
    <div className="settings-layout">
      <div className="settings-section">
        <h2 className="text-heading-md section-title">Appearance</h2>

        <div className="settings-row">
          <span className="text-body-md">Compact Mode</span>
          <label className="toggle-switch">
            <input
              type="checkbox"
              checked={compactMode}
              onChange={(e) => setCompactMode(e.target.checked)}
            />
            <span className="toggle-slider"></span>
          </label>
        </div>

        <div className="settings-row">
          <label className="text-body-md" htmlFor="theme-preference">
            Theme
          </label>
          <select
            id="theme-preference"
            className="settings-select"
            value={themePreference}
            onChange={(event) =>
              setThemePreference(event.target.value as typeof themePreference)
            }
          >
            <option value="system">System</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
          </select>
        </div>
      </div>

      {import.meta.env.DEV && (
        <div className="settings-section">
          <h2 className="text-heading-md section-title">
            Developer Mock State
          </h2>

          <div className="mock-controls">
            {(["codex", "claude", "antigravity"] as ProviderId[]).map(
              (provider) => (
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
              ),
            )}
          </div>
        </div>
      )}
    </div>
  );
}
