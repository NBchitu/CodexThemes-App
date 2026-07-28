import { ExternalLink } from "lucide-react";
import { useState } from "react";
import { RestoreOriginalDialog } from "../../components/RestoreOriginalDialog";
import { windowBorderStyles } from "../../domain/window-effects";
import { platformBridge } from "../../services/platform";
import { useAppStore, type WindowBorderStyle } from "../../store/app-store";

interface ToggleRowProps { title: string; description: string; checked: boolean; disabled?: boolean; onChange: (checked: boolean) => void }
function ToggleRow({ title, description, checked, disabled = false, onChange }: ToggleRowProps) {
  return <label className="setting-row"><span><strong>{title}</strong><small>{description}</small></span><input className="switch" type="checkbox" checked={checked} disabled={disabled} onChange={(event) => onChange(event.target.checked)} /></label>;
}

export function SettingsPage() {
  const [windowEffectsBusy, setWindowEffectsBusy] = useState(false);
  const [windowEffectsError, setWindowEffectsError] = useState<string | null>(null);
  const [windowBorderBusy, setWindowBorderBusy] = useState(false);
  const [windowBorderError, setWindowBorderError] = useState<string | null>(null);
  const [pixelCatBusy, setPixelCatBusy] = useState(false);
  const [pixelCatError, setPixelCatError] = useState<string | null>(null);
  const preferences = useAppStore((state) => state.preferences);
  const updatePreference = useAppStore((state) => state.updatePreference);
  const setNotice = useAppStore((state) => state.setNotice);
  const runtime = useAppStore((state) => state.runtime);

  const setWindowEffects = async (enabled: boolean) => {
    setWindowEffectsBusy(true);
    setWindowEffectsError(null);
    try {
      const result = await platformBridge.setWindowEffectsEnabled(enabled);
      if (!result.ok) throw new Error(result.message);
      updatePreference("effectsEnabled", enabled);
      updatePreference("effectsDiscoverySeen", true);
      setNotice(result.message);
    } catch (error) {
      setWindowEffectsError(error instanceof Error ? error.message : String(error));
    } finally {
      setWindowEffectsBusy(false);
    }
  };

  const setWindowBorder = async (
    enabled: boolean,
    style: WindowBorderStyle = preferences.windowBorderStyle,
  ) => {
    setWindowBorderBusy(true);
    setWindowBorderError(null);
    try {
      const result = await platformBridge.setWindowBorder(enabled, style);
      if (!result.ok) throw new Error(result.message);
      updatePreference("windowBorderEnabled", enabled);
      updatePreference("windowBorderStyle", style);
      setNotice(result.message);
    } catch (error) {
      setWindowBorderError(error instanceof Error ? error.message : String(error));
    } finally {
      setWindowBorderBusy(false);
    }
  };

  const setPixelCat = async (enabled: boolean) => {
    setPixelCatBusy(true);
    setPixelCatError(null);
    try {
      const result = await platformBridge.setPixelCatEnabled(enabled);
      if (!result.ok) throw new Error(result.message);
      updatePreference("pixelCatEnabled", enabled);
      setNotice(result.message);
    } catch (error) {
      setPixelCatError(error instanceof Error ? error.message : String(error));
    } finally {
      setPixelCatBusy(false);
    }
  };

  return (
    <section className="page settings-page" aria-labelledby="settings-title">
      <header className="page-header"><div><p className="eyebrow">Preferences</p><h1 id="settings-title">Settings</h1><p>Control startup, updates, language, and diagnostics.</p></div></header>
      <div className="settings-stack">
        <section className="settings-group"><h2>General</h2><div className="settings-panel">
          <ToggleRow title="Launch at login" description="Open Codex Themes when you sign in to your Mac." checked={preferences.launchAtLogin} onChange={(value) => updatePreference("launchAtLogin", value)} />
          <ToggleRow title="Start minimized" description="Keep the app in the menu bar until you open it." checked={preferences.startMinimized} onChange={(value) => updatePreference("startMinimized", value)} />
          <ToggleRow title="Restore last theme" description="Reapply the verified active theme when appropriate." checked={preferences.restoreLastTheme} onChange={(value) => updatePreference("restoreLastTheme", value)} />
        </div></section>
        <section className="settings-group">
          <div className="settings-group-heading">
            <h2>Codex window effects</h2>
            <span className={preferences.effectsEnabled ? "settings-status-on" : "settings-status-off"}>
              {preferences.effectsEnabled ? "On" : "Off"}
            </span>
          </div>
          <div className="settings-panel">
          <ToggleRow
            title="Codex window effects"
            description="Pause or resume every visual effect without losing your combination."
            checked={preferences.effectsEnabled}
            disabled={windowEffectsBusy}
            onChange={(value) => void setWindowEffects(value)}
          />
          {windowEffectsError && <p className="setting-error" role="alert">{windowEffectsError}</p>}
          <div
            className={preferences.effectsEnabled ? "settings-effects-children" : "settings-effects-children settings-effects-children-disabled"}
            aria-disabled={!preferences.effectsEnabled}
          >
          <ToggleRow
            title="Animated window border"
            description="Show the built-in moving border around Codex with every theme."
            checked={preferences.windowBorderEnabled}
            disabled={windowEffectsBusy || windowBorderBusy || !preferences.effectsEnabled}
            onChange={(value) => void setWindowBorder(value)}
          />
          <label className="setting-row">
            <span>
              <strong>Border style</strong>
              <small>Choose a distinct marquee pattern. The 6 px border is twice as wide as the original.</small>
            </span>
            <select
              value={preferences.windowBorderStyle}
              disabled={windowEffectsBusy || windowBorderBusy || !preferences.effectsEnabled || !preferences.windowBorderEnabled}
              onChange={(event) => void setWindowBorder(
                preferences.windowBorderEnabled,
                event.target.value as WindowBorderStyle,
              )}
            >
              {windowBorderStyles.map((style) => (
                <option key={style.value} value={style.value}>{style.label}</option>
              ))}
            </select>
          </label>
          {windowBorderError && <p className="setting-error" role="alert">{windowBorderError}</p>}
          <ToggleRow
            title="Pixel cat companion"
            description="Let a light- or dark-adapted pixel cat wander through Codex."
            checked={preferences.pixelCatEnabled}
            disabled={windowEffectsBusy || pixelCatBusy || !preferences.effectsEnabled}
            onChange={(value) => void setPixelCat(value)}
          />
          {pixelCatError && <p className="setting-error" role="alert">{pixelCatError}</p>}
          </div>
        </div></section>
        <section className="settings-group"><h2>Marketplace</h2><div className="settings-panel"><ToggleRow title="Check for theme updates" description="Refresh installed marketplace theme metadata automatically." checked={preferences.autoUpdateThemes} onChange={(value) => updatePreference("autoUpdateThemes", value)} /></div></section>
        <section className="settings-group"><h2>Language & app appearance</h2><div className="settings-panel">
          <label className="setting-row"><span><strong>Language</strong><small>Chinese and Japanese are planned for a future release.</small></span><select defaultValue="en"><option value="en">English</option><option disabled>简体中文 — Coming soon</option><option disabled>日本語 — Coming soon</option></select></label>
          <label className="setting-row"><span><strong>Appearance</strong><small>Follow macOS or choose an application appearance.</small></span><select value={preferences.appearance} onChange={(event) => updatePreference("appearance", event.target.value as typeof preferences.appearance)}><option value="system">System</option><option value="light">Light</option><option value="dark">Dark</option></select></label>
        </div></section>
        <section className="settings-group"><h2>Diagnostics</h2><div className="settings-panel diagnostic-panel"><div><strong>Runtime status</strong><small>{runtime.message}</small></div><button className="secondary-button" type="button" onClick={() => setNotice("Diagnostics are available when the native macOS bridge is connected.")}><ExternalLink size={15} /> Open diagnostics</button></div></section>
        <section className="settings-group"><h2>Original appearance</h2><div className="settings-panel diagnostic-panel"><div><strong>Restore Codex</strong><small>Stop the managed injector and return Codex to its original appearance. Downloaded themes are kept.</small></div><RestoreOriginalDialog /></div></section>
      </div>
    </section>
  );
}
