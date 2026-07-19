import { ProviderState } from "../ipc/types";
import { MockProviderScenario } from "./mock/mock-adapter";

export type FooterStatusDisplay = {
  text: string;
  colorClass: string;
};

export function getFooterStatusDisplay(
  state: ProviderState | undefined,
  scenario: MockProviderScenario,
  isError: boolean,
): FooterStatusDisplay {
  if (isError || state?.status === "error" || scenario === "retryable-error") {
    return { text: "Error", colorClass: "status-indicator-error" };
  }
  if (state?.status === "not-installed") {
    return { text: "CLI missing", colorClass: "status-indicator-error" };
  }
  if (state?.status === "unsupported") {
    return { text: "Unsupported", colorClass: "status-indicator-error" };
  }
  if (
    state?.status === "authentication-required" ||
    scenario === "authentication-expired" ||
    scenario === "auth-expired"
  ) {
    return { text: "Sign in", colorClass: "status-indicator-error" };
  }
  if (scenario === "disconnected") {
    return { text: "Disconnected", colorClass: "status-indicator-offline" };
  }
  if (
    state?.status === "offline" ||
    scenario === "offline-with-cache" ||
    scenario === "offline"
  ) {
    return { text: "Offline", colorClass: "status-indicator-offline" };
  }
  if (state?.usage?.stale) {
    return { text: "Stale", colorClass: "status-indicator-offline" };
  }
  if (state?.usage) {
    return { text: "Online", colorClass: "status-indicator-online" };
  }
  return { text: "Loading...", colorClass: "status-indicator-loading" };
}
