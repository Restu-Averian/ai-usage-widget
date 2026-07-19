import { create } from "zustand";
import { ProviderId } from "../ipc/types";
import {
  DEFAULT_MOCK_SCENARIOS,
  MockProviderScenario,
} from "../lib/mock/mock-adapter";

export type ViewState = "dashboard" | "settings";

interface UiState {
  selectedProvider: ProviderId;
  viewState: ViewState;
  isApiKeyDialogOpen: boolean;
  mockScenarios: Record<ProviderId, MockProviderScenario>;

  setSelectedProvider: (provider: ProviderId) => void;
  setViewState: (view: ViewState) => void;
  setApiKeyDialogOpen: (isOpen: boolean) => void;
  setMockScenario: (
    provider: ProviderId,
    scenario: MockProviderScenario,
  ) => void;
}

export const useUiStore = create<UiState>((set) => ({
  selectedProvider: "codex",
  viewState: "dashboard",
  isApiKeyDialogOpen: false,
  mockScenarios: DEFAULT_MOCK_SCENARIOS,

  setSelectedProvider: (provider) => set({ selectedProvider: provider }),
  setViewState: (view) => set({ viewState: view }),
  setApiKeyDialogOpen: (isOpen) => set({ isApiKeyDialogOpen: isOpen }),
  setMockScenario: (provider, scenario) =>
    set((state) => ({
      mockScenarios: { ...state.mockScenarios, [provider]: scenario },
    })),
}));
