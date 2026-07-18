import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import type { OperationResult, RuntimeSnapshot, Theme, ThemeLibraryResult } from "../domain/theme";
import { themes as previewThemes } from "../domain/themes";

export interface PlatformBridge {
  getRuntimeStatus(): Promise<RuntimeSnapshot>;
  applyTheme(themeId: string): Promise<OperationResult>;
  initializeThemeLibrary(): Promise<ThemeLibraryResult>;
  importThemeFolder(): Promise<ThemeLibraryResult>;
  deleteTheme(themeId: string): Promise<ThemeLibraryResult>;
  openThemesFolder(): Promise<OperationResult>;
  exportThemeCreationGuide(): Promise<OperationResult>;
  openThemeGallery(): Promise<OperationResult>;
  restoreOriginal(): Promise<OperationResult>;
  openCodex(): Promise<OperationResult>;
}

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

const delay = (duration: number) => new Promise((resolve) => window.setTimeout(resolve, duration));

class NativePlatformBridge implements PlatformBridge {
  getRuntimeStatus() {
    return invoke<RuntimeSnapshot>("get_runtime_status");
  }

  applyTheme(themeId: string) {
    return invoke<OperationResult>("apply_theme", { themeId });
  }

  async initializeThemeLibrary() {
    const result = await invoke<ThemeLibraryResult>("initialize_theme_library");
    return { ...result, themes: result.themes.map(withNativePreview) };
  }

  async importThemeFolder() {
    const result = await invoke<ThemeLibraryResult>("import_theme_folder");
    return { ...result, themes: result.themes.map(withNativePreview) };
  }

  async deleteTheme(themeId: string) {
    const result = await invoke<ThemeLibraryResult>("delete_theme", { themeId });
    return { ...result, themes: result.themes.map(withNativePreview) };
  }

  openThemesFolder() {
    return invoke<OperationResult>("open_themes_folder");
  }

  exportThemeCreationGuide() {
    return invoke<OperationResult>("export_theme_creation_guide");
  }


  openThemeGallery() {
    return invoke<OperationResult>("open_theme_gallery");
  }

  restoreOriginal() {
    return invoke<OperationResult>("restore_original");
  }

  openCodex() {
    return invoke<OperationResult>("open_codex");
  }
}

function withNativePreview(theme: Theme): Theme {
  return { ...theme, previewUrl: theme.previewPath ? convertFileSrc(theme.previewPath) : theme.previewUrl };
}

class PreviewPlatformBridge implements PlatformBridge {
  private activeThemeId: string | null = "preset-gothic-void-crusade";

  async getRuntimeStatus(): Promise<RuntimeSnapshot> {
    return {
      status: "preview",
      activeThemeId: this.activeThemeId,
      message: "UI preview — native macOS bridge is not connected",
      isNativeHost: false,
    };
  }

  async applyTheme(): Promise<OperationResult> {
    await delay(450);
    return {
      ok: false,
      verified: false,
      status: "preview",
      message: "Open this interface in the native macOS build to apply themes to Codex.",
    };
  }

  async initializeThemeLibrary(): Promise<ThemeLibraryResult> {
    return { themes: previewThemes.filter((theme) => theme.installed), message: "Preview themes are ready." };
  }

  async importThemeFolder(): Promise<ThemeLibraryResult> {
    throw new Error("Open the native macOS app to import a theme folder.");
  }

  async deleteTheme(): Promise<ThemeLibraryResult> {
    throw new Error("Open the native macOS app to delete an imported theme.");
  }

  async openThemesFolder(): Promise<OperationResult> {
    return { ok: false, verified: false, status: "preview", message: "Open the native macOS app to view the managed theme folder." };
  }

  async exportThemeCreationGuide(): Promise<OperationResult> {
    return { ok: false, verified: false, status: "preview", message: "Open the native macOS app to save the creation guide." };
  }


  async openThemeGallery(): Promise<OperationResult> {
    window.open("https://codexthemes.app/?utm_source=codex_themes_desktop&utm_medium=desktop_app&utm_campaign=theme_gallery", "_blank", "noopener,noreferrer");
    return { ok: true, verified: true, status: "connected", message: "Theme gallery opened in your browser." };
  }

  async restoreOriginal(): Promise<OperationResult> {
    await delay(350);
    return {
      ok: false,
      verified: false,
      status: "preview",
      message: "Original appearance can only be restored by the native macOS bridge.",
    };
  }

  async openCodex(): Promise<OperationResult> {
    return {
      ok: false,
      verified: false,
      status: "preview",
      message: "Codex launch is unavailable in browser preview mode.",
    };
  }
}

export const isNativeHost = Boolean(window.__TAURI_INTERNALS__);
export const platformBridge: PlatformBridge = isNativeHost
  ? new NativePlatformBridge()
  : new PreviewPlatformBridge();
