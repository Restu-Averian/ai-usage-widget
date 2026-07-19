import { z } from "zod";
import {
  appBootstrapSchema,
  appErrorPayloadSchema,
  appSettingsSchema,
  providerIdSchema,
  providerMetadataSchema,
  providerStateSchema,
  loginLaunchResultSchema,
  providerUsageSchema,
  usageWindowSchema,
} from "./schemas";

export type ProviderId = z.infer<typeof providerIdSchema>;
export type UsageWindow = z.infer<typeof usageWindowSchema>;
export type ProviderUsage = z.infer<typeof providerUsageSchema>;
export type ProviderState = z.infer<typeof providerStateSchema>;
export type ProviderMetadata = z.infer<typeof providerMetadataSchema>;
export type LoginLaunchResult = z.infer<typeof loginLaunchResultSchema>;
export type AppSettings = z.infer<typeof appSettingsSchema>;
export type AppBootstrap = z.infer<typeof appBootstrapSchema>;
export type AppErrorPayload = z.infer<typeof appErrorPayloadSchema>;

export type RefreshReason =
  "manual" | "startup" | "scheduled" | "popup-open" | "wake";
