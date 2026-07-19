import { useEffect, useState } from "react";

export const THEME_STORAGE_KEY = "ai-usage-dock-theme";

export type ThemePreference = "system" | "light" | "dark";
export type ResolvedTheme = "light" | "dark";

const THEME_PREFERENCES: ThemePreference[] = ["system", "light", "dark"];

type ThemeTarget = {
  dataset: DOMStringMap;
  style: Pick<CSSStyleDeclaration, "colorScheme">;
};

type ApplyThemeOptions = {
  element: ThemeTarget;
  storage?: Pick<Storage, "setItem">;
  systemPrefersDark: boolean;
  persist?: boolean;
};

export function isThemePreference(
  value: string | null,
): value is ThemePreference {
  return THEME_PREFERENCES.includes(value as ThemePreference);
}

export function getStoredThemePreference(
  storage: Pick<Storage, "getItem"> | undefined,
): ThemePreference {
  if (!storage) {
    return "system";
  }

  const value = storage.getItem(THEME_STORAGE_KEY);
  return isThemePreference(value) ? value : "system";
}

export function resolveThemePreference(
  preference: ThemePreference,
  systemPrefersDark: boolean,
): ResolvedTheme {
  return preference === "system"
    ? systemPrefersDark
      ? "dark"
      : "light"
    : preference;
}

export function applyThemePreference(
  preference: ThemePreference,
  { element, storage, systemPrefersDark, persist = true }: ApplyThemeOptions,
): ResolvedTheme {
  const resolvedTheme = resolveThemePreference(preference, systemPrefersDark);

  element.dataset.themePreference = preference;
  element.dataset.theme = resolvedTheme;
  element.style.colorScheme = resolvedTheme;

  if (persist) {
    storage?.setItem(THEME_STORAGE_KEY, preference);
  }

  return resolvedTheme;
}

export function useThemePreference(): [
  ThemePreference,
  (theme: ThemePreference) => void,
] {
  const [preference, setPreference] = useState<ThemePreference>(() =>
    typeof window === "undefined"
      ? "system"
      : getStoredThemePreference(window.localStorage),
  );

  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }

    const media = window.matchMedia?.("(prefers-color-scheme: dark)");
    const apply = () =>
      applyThemePreference(preference, {
        element: document.documentElement,
        storage: window.localStorage,
        systemPrefersDark: media?.matches ?? false,
      });

    apply();

    if (preference !== "system" || !media) {
      return;
    }

    media.addEventListener("change", apply);
    return () => media.removeEventListener("change", apply);
  }, [preference]);

  return [preference, setPreference];
}
