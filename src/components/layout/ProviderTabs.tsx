import { useUiStore } from "../../stores/uiStore";
import { ProviderId } from "../../ipc/types";
import "./ProviderTabs.css";

const PROVIDERS: { id: ProviderId; label: string }[] = [
  { id: "codex", label: "Codex" },
  { id: "antigravity", label: "Antigravity" },
];

export function ProviderTabs() {
  const { selectedProvider, setSelectedProvider } = useUiStore();

  return (
    <div className="provider-tabs" role="tablist" aria-label="Providers">
      {PROVIDERS.map((provider) => {
        const isSelected = selectedProvider === provider.id;
        return (
          <button
            key={provider.id}
            role="tab"
            aria-selected={isSelected}
            className={`provider-tab ${isSelected ? "selected" : ""}`}
            onClick={() => setSelectedProvider(provider.id)}
          >
            <span className="text-label-md">{provider.label}</span>
          </button>
        );
      })}
    </div>
  );
}
