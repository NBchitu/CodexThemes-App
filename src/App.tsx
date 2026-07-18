import { CheckCircle2, X } from "lucide-react";
import { Sidebar } from "./components/Sidebar";
import { ThemeDetail } from "./components/ThemeDetail";
import { DiscoverPage } from "./features/discover/DiscoverPage";
import { LibraryPage } from "./features/library/LibraryPage";
import { CreatePage } from "./features/create/CreatePage";
import { SettingsPage } from "./features/settings/SettingsPage";
import { platformBridge } from "./services/platform";
import { useAppStore } from "./store/app-store";
import { useEffect } from "react";

export default function App() {
  const route = useAppStore((state) => state.route);
  const themes = useAppStore((state) => state.themes);
  const selectedThemeId = useAppStore((state) => state.selectedThemeId);
  const selectTheme = useAppStore((state) => state.selectTheme);
  const notice = useAppStore((state) => state.notice);
  const setNotice = useAppStore((state) => state.setNotice);
  const setRuntime = useAppStore((state) => state.setRuntime);
  const setThemes = useAppStore((state) => state.setThemes);
  const setPreferredTheme = useAppStore((state) => state.setPreferredTheme);
  const appearance = useAppStore((state) => state.preferences.appearance);
  const selectedTheme = themes.find((theme) => theme.id === selectedThemeId) ?? null;

  useEffect(() => {
    void platformBridge.getRuntimeStatus().then((runtime) => {
      setRuntime(runtime);
      if (runtime.activeThemeId) setPreferredTheme(runtime.activeThemeId);
    }).catch((error) => {
      setRuntime({ status: "error", activeThemeId: null, message: String(error), isNativeHost: true });
    });
    void platformBridge.initializeThemeLibrary().then((result) => {
      setThemes(result.themes);
    }).catch((error) => {
      setNotice(`Theme library: ${String(error)}`);
    });
  }, [setNotice, setPreferredTheme, setRuntime, setThemes]);

  useEffect(() => {
    document.documentElement.dataset.appearance = appearance;
  }, [appearance]);

  return (
    <div className="app-shell">
      <Sidebar />
      <main className="main-content">
        {selectedTheme ? <ThemeDetail theme={selectedTheme} onClose={() => selectTheme(null)} /> : (
          <>
            {route === "discover" && <DiscoverPage />}
            {route === "library" && <LibraryPage />}
            {route === "create" && <CreatePage />}
            {route === "settings" && <SettingsPage />}
          </>
        )}
      </main>
      {notice && <div className="toast" role="status"><CheckCircle2 size={17} aria-hidden="true" /><span><strong>Codex Themes</strong><small>{notice}</small></span><button type="button" aria-label="Dismiss message" onClick={() => setNotice(null)}><X size={15} /></button></div>}
    </div>
  );
}
