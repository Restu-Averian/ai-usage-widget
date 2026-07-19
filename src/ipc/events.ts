import { listen } from "@tauri-apps/api/event";
import { QueryClient } from "@tanstack/react-query";
import { providerIdSchema } from "./schemas";

type ProviderEventPayload = {
  provider?: unknown;
};

export function registerBackendEventInvalidation(queryClient: QueryClient) {
  const providerEvents = [
    "provider://refresh-completed",
    "provider://refresh-failed",
    "provider://authentication-changed",
    "provider://installation-changed",
  ];

  const unlisten = providerEvents.map((eventName) =>
    listen<ProviderEventPayload>(eventName, (event) => {
      const provider = providerIdSchema.safeParse(event.payload.provider);
      if (!provider.success) {
        return;
      }

      void queryClient.invalidateQueries({
        queryKey: ["providerUsage", provider.data],
      });
    }),
  );

  unlisten.push(
    listen("settings://changed", () => {
      void queryClient.invalidateQueries({ queryKey: ["settings"] });
    }),
  );

  return async () => {
    const listeners = await Promise.all(unlisten);
    listeners.forEach((dispose) => dispose());
  };
}
