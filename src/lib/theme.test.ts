import { describe, expect, it } from "vitest";
import {
  applyThemePreference,
  getStoredThemePreference,
  resolveThemePreference,
  THEME_STORAGE_KEY,
} from "./theme";

class MemoryStorage implements Storage {
  private values = new Map<string, string>();
  length = 0;

  clear(): void {
    this.values.clear();
    this.length = 0;
  }

  getItem(key: string): string | null {
    return this.values.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.values.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.values.delete(key);
    this.length = this.values.size;
  }

  setItem(key: string, value: string): void {
    this.values.set(key, value);
    this.length = this.values.size;
  }
}

describe("theme helpers", () => {
  it("defaults invalid or missing stored preferences to system", () => {
    const storage = new MemoryStorage();

    expect(getStoredThemePreference(storage)).toBe("system");

    storage.setItem(THEME_STORAGE_KEY, "sepia");

    expect(getStoredThemePreference(storage)).toBe("system");
  });

  it.each([
    ["light", true, "light"],
    ["dark", false, "dark"],
    ["system", true, "dark"],
    ["system", false, "light"],
  ] as const)(
    "resolves %s with dark=%s to %s",
    (preference, dark, expected) => {
      expect(resolveThemePreference(preference, dark)).toBe(expected);
    },
  );

  it("persists only the theme preference and applies resolved attributes", () => {
    const storage = new MemoryStorage();
    const element = {
      dataset: {},
      style: {},
    } as HTMLElement;

    applyThemePreference("dark", {
      element,
      storage,
      systemPrefersDark: false,
    });

    expect(storage.getItem(THEME_STORAGE_KEY)).toBe("dark");
    expect(element.dataset.themePreference).toBe("dark");
    expect(element.dataset.theme).toBe("dark");
    expect(element.style.colorScheme).toBe("dark");
  });
});
