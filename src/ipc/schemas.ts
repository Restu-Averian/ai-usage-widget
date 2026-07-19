import { z } from "zod";

export const providerIdSchema = z.enum(["codex", "antigravity"]);

export const connectionTypeSchema = z.enum([
  "subscription-cli",
  "oauth-cli",
  "api-key",
  "admin-api-key",
  "cloud-project",
  "dashboard-only",
]);

export const reliabilitySchema = z.enum([
  "official-api",
  "official-sdk",
  "official-cli-json",
  "official-cli-text",
  "experimental-pty",
  "manual",
  "dashboard-only",
]);

export const usagePeriodSchema = z.enum([
  "session",
  "five-hour",
  "daily",
  "weekly",
  "monthly",
  "model-specific",
  "unknown",
]);

const percentSchema = z.number().min(0).max(100).nullable().optional();

export const usageWindowSchema = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
  period: usagePeriodSchema,
  model: z.string().optional().nullable(),
  usedPercent: percentSchema,
  remainingPercent: percentSchema,
  resetAt: z.string().datetime({ offset: true }).optional().nullable(),
  derivedUsedPercent: z.boolean().default(false),
  derivedRemainingPercent: z.boolean().default(false),
});

export const usageWarningSchema = z.enum([
  "offline",
  "stale",
  "percentage-unavailable",
  "experimental-parser",
  "reset-time-passed",
]);

export const tokenUsageSchema = z.object({
  input: z.number().int().nonnegative(),
  output: z.number().int().nonnegative(),
  cached: z.number().int().nonnegative().optional().nullable(),
  requests: z.number().int().nonnegative().optional().nullable(),
});

export const costUsageSchema = z.object({
  used: z.number().nonnegative(),
  budget: z.number().nonnegative().optional().nullable(),
  currency: z.literal("USD"),
});

export const providerUsageSchema = z.object({
  id: z.string().min(1),
  provider: providerIdSchema,
  connectionType: connectionTypeSchema,
  accountLabel: z.string().optional().nullable(),
  planName: z.string().optional().nullable(),
  windows: z.array(usageWindowSchema),
  tokenUsage: tokenUsageSchema.optional().nullable(),
  costUsage: costUsageSchema.optional().nullable(),
  reliability: reliabilitySchema,
  fetchedAt: z.string().datetime({ offset: true }),
  stale: z.boolean(),
  warnings: z.array(usageWarningSchema),
});

export const providerStateKindSchema = z.enum([
  "unknown",
  "detecting",
  "not-installed",
  "authentication-required",
  "connecting",
  "connected",
  "refreshing",
  "stale",
  "offline",
  "error",
  "unsupported",
]);

export const appErrorCodeSchema = z.enum([
  "internal",
  "invalid-input",
  "provider-unavailable",
  "authentication-expired",
  "network-unavailable",
  "retryable-provider-error",
  "unsupported",
  "database",
  "secret-store",
  "contract-violation",
]);

export const appErrorPayloadSchema = z.object({
  code: appErrorCodeSchema,
  message: z.string(),
  retryable: z.boolean(),
});

export const providerStateSchema = z.object({
  provider: providerIdSchema,
  status: providerStateKindSchema,
  usage: providerUsageSchema.optional().nullable(),
  lastError: appErrorPayloadSchema.optional().nullable(),
  isRefreshing: z.boolean(),
});

export const providerCapabilitiesSchema = z.object({
  canDetectInstallation: z.boolean(),
  canDetectAuthentication: z.boolean(),
  canStartLogin: z.boolean(),
  canFetchSubscriptionUsage: z.boolean(),
  canFetchApiUsage: z.boolean(),
  canDisconnectLocalConnection: z.boolean(),
  requiresTty: z.boolean(),
  supportsMultipleWindows: z.boolean(),
  supportsModelWindows: z.boolean(),
});

export const providerMetadataSchema = z.object({
  id: providerIdSchema,
  label: z.string(),
  capabilities: providerCapabilitiesSchema,
});

export const loginLaunchResultSchema = z.object({
  launched: z.boolean(),
  message: z.string().optional().nullable(),
});

export const appSettingsSchema = z.object({
  launchAtLogin: z.boolean(),
  startMinimized: z.boolean(),
  showDockIcon: z.boolean(),
  closePanelWhenUnfocused: z.boolean(),
  menuBar: z.object({
    showPercentage: z.boolean(),
    displayedUsage: z.enum(["highest", "codex", "antigravity"]),
    warningThreshold: z.number().int().min(0).max(100),
    criticalThreshold: z.number().int().min(0).max(100),
  }),
  refresh: z.object({
    enabled: z.boolean(),
    intervalMinutes: z.union([
      z.literal(1),
      z.literal(5),
      z.literal(10),
      z.literal(15),
      z.literal(30),
    ]),
    refreshAfterWake: z.boolean(),
    refreshWhenPopupOpens: z.boolean(),
  }),
});

export const appBootstrapSchema = z.object({
  providers: z.array(providerMetadataSchema),
  providerStates: z.array(providerStateSchema),
  settings: appSettingsSchema,
});

export const commandResultSchema = <T extends z.ZodTypeAny>(dataSchema: T) =>
  z.discriminatedUnion("ok", [
    z.object({
      ok: z.literal(true),
      data: dataSchema,
      requestId: z.string().min(1),
    }),
    z.object({
      ok: z.literal(false),
      error: appErrorPayloadSchema,
      requestId: z.string().min(1),
    }),
  ]);
