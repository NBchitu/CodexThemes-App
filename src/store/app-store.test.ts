import { afterEach, describe, expect, it, vi } from "vitest";

function installStorage(initial: Record<string, string> = {}) {
  const values = new Map(Object.entries(initial));
  vi.stubGlobal("localStorage", {
    getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => values.set(key, value),
    removeItem: (key: string) => values.delete(key),
  });
}

afterEach(() => {
  vi.unstubAllGlobals();
  vi.resetModules();
});

describe("application preferences", () => {
  it("defaults new installations to dark appearance with window effects enabled", async () => {
    installStorage();
    const { useAppStore } = await import("./app-store");
    expect(useAppStore.getState().preferences).toMatchObject({
      appearance: "dark",
      effectsEnabled: true,
      effectsDiscoverySeen: false,
      windowBorderEnabled: true,
      windowBorderStyle: "classic-rainbow",
      pixelCatEnabled: true,
    });
  });

  it("preserves existing appearance while adding the compatible border-style default", async () => {
    installStorage({
      "codex-themes.preferences": JSON.stringify({
        appearance: "light",
        windowBorderEnabled: true,
      }),
    });
    const { useAppStore } = await import("./app-store");
    expect(useAppStore.getState().preferences.appearance).toBe("light");
    expect(useAppStore.getState().preferences.effectsEnabled).toBe(true);
    expect(useAppStore.getState().preferences.windowBorderEnabled).toBe(true);
    expect(useAppStore.getState().preferences.windowBorderStyle).toBe("classic-rainbow");
  });

  it("preserves an existing user's fully disabled effects state", async () => {
    installStorage({
      "codex-themes.preferences": JSON.stringify({
        windowBorderEnabled: false,
        pixelCatEnabled: false,
      }),
    });
    const { useAppStore } = await import("./app-store");
    expect(useAppStore.getState().preferences.effectsEnabled).toBe(false);
  });

  it("derives the compatible master state when any existing effect is enabled", async () => {
    installStorage({
      "codex-themes.preferences": JSON.stringify({
        windowBorderEnabled: false,
        pixelCatEnabled: true,
      }),
    });
    const { useAppStore } = await import("./app-store");
    expect(useAppStore.getState().preferences.effectsEnabled).toBe(true);
  });

  it("persists a selected animated border style", async () => {
    const values = new Map<string, string>();
    vi.stubGlobal("localStorage", {
      getItem: (key: string) => values.get(key) ?? null,
      setItem: (key: string, value: string) => values.set(key, value),
      removeItem: (key: string) => values.delete(key),
    });
    const { useAppStore } = await import("./app-store");
    useAppStore.getState().updatePreference("windowBorderStyle", "ocean");
    expect(JSON.parse(values.get("codex-themes.preferences") ?? "{}").windowBorderStyle)
      .toBe("ocean");
  });

  it("persists dismissal of the one-time effects introduction", async () => {
    const values = new Map<string, string>();
    vi.stubGlobal("localStorage", {
      getItem: (key: string) => values.get(key) ?? null,
      setItem: (key: string, value: string) => values.set(key, value),
      removeItem: (key: string) => values.delete(key),
    });
    const { useAppStore } = await import("./app-store");
    useAppStore.getState().updatePreference("effectsDiscoverySeen", true);
    expect(JSON.parse(values.get("codex-themes.preferences") ?? "{}").effectsDiscoverySeen)
      .toBe(true);
  });
});
