import { create } from "zustand";
import type { RuntimeSnapshot, Theme } from "../domain/theme";
import { themes as initialThemes } from "../domain/themes";

export type AppRoute = "discover" | "library" | "create" | "settings";
export type WindowBorderStyle = "classic-rainbow" | "candy-stripe" | "ocean" | "monochrome";

export interface Preferences {
  launchAtLogin: boolean;
  startMinimized: boolean;
  restoreLastTheme: boolean;
  autoUpdateThemes: boolean;
  effectsEnabled: boolean;
  effectsDiscoverySeen: boolean;
  windowBorderEnabled: boolean;
  windowBorderStyle: WindowBorderStyle;
  pixelCatEnabled: boolean;
  appearance: "system" | "light" | "dark";
}

interface AppState {
  route: AppRoute;
  themes: Theme[];
  runtime: RuntimeSnapshot;
  preferredThemeId: string | null;
  selectedThemeId: string | null;
  notice: string | null;
  preferences: Preferences;
  setRoute: (route: AppRoute) => void;
  selectTheme: (id: string | null) => void;
  setRuntime: (runtime: RuntimeSnapshot) => void;
  setPreferredTheme: (id: string | null) => void;
  setThemes: (themes: Theme[]) => void;
  setNotice: (notice: string | null) => void;
  updatePreference: <K extends keyof Preferences>(key: K, value: Preferences[K]) => void;
}

const storedPreferences = (): Partial<Preferences> => {
  try {
    const value = JSON.parse(localStorage.getItem("codex-themes.preferences") ?? "{}");
    if (!value || typeof value !== "object" || Array.isArray(value)) return {};
    const preferences = { ...value } as Partial<Preferences>;
    if (
      typeof preferences.effectsEnabled !== "boolean"
      && (
        typeof preferences.windowBorderEnabled === "boolean"
        || typeof preferences.pixelCatEnabled === "boolean"
      )
    ) {
      preferences.effectsEnabled =
        preferences.windowBorderEnabled === true || preferences.pixelCatEnabled === true;
    }
    return preferences;
  } catch {
    return {};
  }
};

const defaultPreferences: Preferences = {
  launchAtLogin: false,
  startMinimized: true,
  restoreLastTheme: true,
  autoUpdateThemes: true,
  effectsEnabled: true,
  effectsDiscoverySeen: false,
  windowBorderEnabled: true,
  windowBorderStyle: "classic-rainbow",
  pixelCatEnabled: true,
  appearance: "dark",
};

const storedPreferredTheme = (): string | null => {
  try {
    const value = localStorage.getItem("codex-themes.preferred-theme");
    return value && /^[A-Za-z0-9_-]{2,64}$/.test(value) ? value : null;
  } catch {
    return null;
  }
};

export const useAppStore = create<AppState>((set) => ({
  route: "library",
  themes: initialThemes,
  runtime: {
    status: "preview",
    activeThemeId: "preset-gothic-void-crusade",
    message: "UI preview — native macOS bridge is not connected",
    isNativeHost: false,
  },
  preferredThemeId: storedPreferredTheme(),
  selectedThemeId: null,
  notice: null,
  preferences: { ...defaultPreferences, ...storedPreferences() },
  setRoute: (route) => set({ route, selectedThemeId: null, notice: null }),
  selectTheme: (selectedThemeId) => set({ selectedThemeId }),
  setRuntime: (runtime) => set({ runtime }),
  setPreferredTheme: (preferredThemeId) => set(() => {
    try {
      if (preferredThemeId) localStorage.setItem("codex-themes.preferred-theme", preferredThemeId);
      else localStorage.removeItem("codex-themes.preferred-theme");
    } catch {
      // Keep the preference in memory when storage is unavailable.
    }
    return { preferredThemeId };
  }),
  setThemes: (themes) => set({ themes }),
  setNotice: (notice) => set({ notice }),
  updatePreference: (key, value) => set((state) => {
    const preferences = { ...state.preferences, [key]: value };
    try {
      localStorage.setItem("codex-themes.preferences", JSON.stringify(preferences));
    } catch {
      // Some embedded or privacy-restricted preview contexts disable storage.
      // Preferences remain valid for the current application session.
    }
    return { preferences };
  }),
}));
