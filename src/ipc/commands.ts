import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
import {
  appBootstrapSchema,
  appSettingsSchema,
  commandResultSchema,
  historyPointSchema,
  loginLaunchResultSchema,
  providerIdSchema,
  providerMetadataSchema,
  providerStateSchema,
} from "./schemas";
import {
  AppBootstrap,
  AppSettings,
  HistoryPoint,
  LoginLaunchResult,
  ProviderId,
  ProviderMetadata,
  ProviderState,
} from "./types";

type FakeScenario =
  | "connected-normal"
  | "warning"
  | "critical"
  | "unknown-percentage"
  | "stale"
  | "offline"
  | "auth-expired"
  | "retryable-error";

async function invokeResult<T>(
  command: string,
  schema: z.ZodType<T>,
  args?: Record<string, unknown>,
): Promise<T> {
  const raw = await invoke<unknown>(command, args);
  const result = commandResultSchema(schema).parse(raw);

  if (!result.ok) {
    throw new Error(result.error.message);
  }

  return result.data;
}

export function getAppBootstrap(): Promise<AppBootstrap> {
  return invokeResult("get_app_bootstrap", appBootstrapSchema);
}

export function listProviders(): Promise<ProviderMetadata[]> {
  return invokeResult("list_providers", z.array(providerMetadataSchema));
}

export function getProviderState(provider: ProviderId): Promise<ProviderState> {
  return invokeResult("get_provider_state", providerStateSchema, {
    request: { provider: providerIdSchema.parse(provider) },
  });
}

export function refreshProvider(
  provider: ProviderId,
  scenario?: FakeScenario,
): Promise<ProviderState> {
  return invokeResult("refresh_provider", providerStateSchema, {
    request: { provider, scenario },
  });
}

export function startProviderLogin(
  provider: ProviderId,
): Promise<LoginLaunchResult> {
  return invokeResult("start_provider_login", loginLaunchResultSchema, {
    request: { provider },
  });
}

export function setFakeProviderScenario(
  provider: ProviderId,
  scenario: FakeScenario,
): Promise<ProviderState> {
  return invokeResult("set_fake_provider_scenario", providerStateSchema, {
    request: { provider, scenario },
  });
}

export function getSettings(): Promise<AppSettings> {
  return invokeResult("get_settings", appSettingsSchema);
}

export function updateSettings(settings: AppSettings): Promise<AppSettings> {
  return invokeResult("update_settings", appSettingsSchema, { settings });
}

export function getUsageHistory(provider: ProviderId): Promise<HistoryPoint[]> {
  return invokeResult("get_usage_history", z.array(historyPointSchema), {
    request: { provider, limit: 30 },
  });
}
