import { ExternalLink } from "lucide-react";
import { RestoreOriginalDialog } from "../../components/RestoreOriginalDialog";
import { useAppStore } from "../../store/app-store";

interface ToggleRowProps { title: string; description: string; checked: boolean; onChange: (checked: boolean) => void }
function ToggleRow({ title, description, checked, onChange }: ToggleRowProps) {
  return <label className="setting-row"><span><strong>{title}</strong><small>{description}</small></span><input className="switch" type="checkbox" checked={checked} onChange={(event) => onChange(event.target.checked)} /></label>;
}

export function SettingsPage() {
  const preferences = useAppStore((state) => state.preferences);
  const updatePreference = useAppStore((state) => state.updatePreference);
  const setNotice = useAppStore((state) => state.setNotice);
  const runtime = useAppStore((state) => state.runtime);

  return (
    <section className="page settings-page" aria-labelledby="settings-title">
      <header className="page-header"><div><p className="eyebrow">Preferences</p><h1 id="settings-title">Settings</h1><p>Control startup, updates, language, and diagnostics.</p></div></header>
      <div className="settings-stack">
        <section className="settings-group"><h2>General</h2><div className="settings-panel">
          <ToggleRow title="Launch at login" description="Open Codex Themes when you sign in to your Mac." checked={preferences.launchAtLogin} onChange={(value) => updatePreference("launchAtLogin", value)} />
          <ToggleRow title="Start minimized" description="Keep the app in the menu bar until you open it." checked={preferences.startMinimized} onChange={(value) => updatePreference("startMinimized", value)} />
          <ToggleRow title="Restore last theme" description="Reapply the verified active theme when appropriate." checked={preferences.restoreLastTheme} onChange={(value) => updatePreference("restoreLastTheme", value)} />
        </div></section>
        <section className="settings-group"><h2>Marketplace</h2><div className="settings-panel"><ToggleRow title="Check for theme updates" description="Refresh installed marketplace theme metadata automatically." checked={preferences.autoUpdateThemes} onChange={(value) => updatePreference("autoUpdateThemes", value)} /></div></section>
        <section className="settings-group"><h2>Language & appearance</h2><div className="settings-panel">
          <label className="setting-row"><span><strong>Language</strong><small>Chinese and Japanese are planned for a future release.</small></span><select defaultValue="en"><option value="en">English</option><option disabled>简体中文 — Coming soon</option><option disabled>日本語 — Coming soon</option></select></label>
          <label className="setting-row"><span><strong>Appearance</strong><small>Follow macOS or choose an application appearance.</small></span><select value={preferences.appearance} onChange={(event) => updatePreference("appearance", event.target.value as typeof preferences.appearance)}><option value="system">System</option><option value="light">Light</option><option value="dark">Dark</option></select></label>
        </div></section>
        <section className="settings-group"><h2>Diagnostics</h2><div className="settings-panel diagnostic-panel"><div><strong>Runtime status</strong><small>{runtime.message}</small></div><button className="secondary-button" type="button" onClick={() => setNotice("Diagnostics are available when the native macOS bridge is connected.")}><ExternalLink size={15} /> Open diagnostics</button></div></section>
        <section className="settings-group"><h2>Original appearance</h2><div className="settings-panel diagnostic-panel"><div><strong>Restore Codex</strong><small>Stop the managed injector and return Codex to its original appearance. Downloaded themes are kept.</small></div><RestoreOriginalDialog /></div></section>
      </div>
    </section>
  );
}
