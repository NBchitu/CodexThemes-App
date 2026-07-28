import { AlertDialog } from "@base-ui/react/alert-dialog";
import { listen } from "@tauri-apps/api/event";
import { CheckCircle2, FileArchive, X } from "lucide-react";
import { Sidebar } from "./components/Sidebar";
import { ThemeDetail } from "./components/ThemeDetail";
import { DiscoverPage } from "./features/discover/DiscoverPage";
import { LibraryPage } from "./features/library/LibraryPage";
import { CreatePage } from "./features/create/CreatePage";
import { SettingsPage } from "./features/settings/SettingsPage";
import { platformBridge } from "./services/platform";
import { useAppStore } from "./store/app-store";
import { useEffect, useRef, useState } from "react";
import { cn } from "./lib/cn";
import type { CodexThemePackageSummary } from "./domain/theme";

export default function App() {
  const [packageSummary, setPackageSummary] = useState<CodexThemePackageSummary | null>(null);
  const [importingPackage, setImportingPackage] = useState(false);
  const route = useAppStore((state) => state.route);
  const runtimeStatus = useAppStore((state) => state.runtime.status);
  const themes = useAppStore((state) => state.themes);
  const selectedThemeId = useAppStore((state) => state.selectedThemeId);
  const selectTheme = useAppStore((state) => state.selectTheme);
  const notice = useAppStore((state) => state.notice);
  const setNotice = useAppStore((state) => state.setNotice);
  const setRuntime = useAppStore((state) => state.setRuntime);
  const setThemes = useAppStore((state) => state.setThemes);
  const setPreferredTheme = useAppStore((state) => state.setPreferredTheme);
  const appearance = useAppStore((state) => state.preferences.appearance);
  const restoreLastTheme = useAppStore((state) => state.preferences.restoreLastTheme);
  const preferredThemeId = useAppStore((state) => state.preferredThemeId);
  const nativeHost = useAppStore((state) => state.runtime.isNativeHost);
  const autoRestoreInFlight = useRef(false);
  const autoRestoreAttempted = useRef(false);
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
    if (runtimeStatus !== "applying") return;
    let disposed = false;
    const refresh = () => {
      void platformBridge.getRuntimeStatus().then((runtime) => {
        if (disposed) return;
        setRuntime(runtime);
        if (runtime.activeThemeId) setPreferredTheme(runtime.activeThemeId);
        if (runtime.status === "error") setNotice(runtime.message);
      }).catch((error) => {
        if (!disposed) {
          const message = error instanceof Error ? error.message : String(error);
          setRuntime({ status: "error", activeThemeId: null, message, isNativeHost: true });
          setNotice(message);
        }
      });
    };
    const timer = window.setInterval(refresh, 2000);
    refresh();
    return () => { disposed = true; window.clearInterval(timer); };
  }, [runtimeStatus, setNotice, setPreferredTheme, setRuntime]);

  useEffect(() => {
    if (!nativeHost || runtimeStatus === "applying" || runtimeStatus === "restoring") return;
    let disposed = false;
    const refresh = () => {
      void platformBridge.getRuntimeStatus().then((runtime) => {
        if (!disposed) setRuntime(runtime);
      }).catch(() => {
        // The foreground actions surface errors. Background reconciliation is
        // deliberately quiet so a transient status read does not spam users.
      });
    };
    const timer = window.setInterval(refresh, 10000);
    return () => {
      disposed = true;
      window.clearInterval(timer);
    };
  }, [nativeHost, runtimeStatus, setRuntime]);

  useEffect(() => {
    if (runtimeStatus === "active" || runtimeStatus === "connected") {
      autoRestoreAttempted.current = false;
    }
  }, [runtimeStatus]);

  useEffect(() => {
    if (!nativeHost || !restoreLastTheme || !preferredThemeId
      || runtimeStatus !== "restart-required" || autoRestoreInFlight.current
      || autoRestoreAttempted.current) return;
    autoRestoreInFlight.current = true;
    autoRestoreAttempted.current = true;
    setRuntime({
      status: "applying",
      activeThemeId: null,
      message: "Restoring the last verified theme…",
      isNativeHost: true,
    });
    void platformBridge.applyTheme(preferredThemeId).then((result) => {
      setRuntime({
        status: result.status,
        activeThemeId: result.verified ? preferredThemeId : null,
        message: result.message,
        isNativeHost: true,
      });
      if (!result.ok) setNotice(result.message);
    }).catch((error) => {
      const message = error instanceof Error ? error.message : String(error);
      setRuntime({ status: "error", activeThemeId: null, message, isNativeHost: true });
      setNotice(message);
    }).finally(() => {
      autoRestoreInFlight.current = false;
    });
  }, [
    nativeHost,
    preferredThemeId,
    restoreLastTheme,
    runtimeStatus,
    setNotice,
    setRuntime,
  ]);

  useEffect(() => {
    let disposed = false;
    const inspect = async (path: string) => {
      try {
        const summary = await platformBridge.inspectCodexThemePackage(path);
        if (!disposed) setPackageSummary(summary);
      } catch (error) {
        if (!disposed) setNotice(error instanceof Error ? error.message : String(error));
      }
    };
    const consumePending = async () => {
      const path = await platformBridge.pendingCodexThemePath();
      if (path) await inspect(path);
    };
    const unlisten = listen("codextheme-open-requested", () => void consumePending());
    void unlisten.then(() => consumePending());
    return () => { disposed = true; void unlisten.then((stop) => stop()); };
  }, [setNotice]);

  useEffect(() => {
    document.documentElement.dataset.appearance = appearance;
  }, [appearance]);

  useEffect(() => {
    if (!notice) return;
    const timer = window.setTimeout(() => setNotice(null), runtimeStatus === "error" ? 8000 : 4000);
    return () => window.clearTimeout(timer);
  }, [notice, runtimeStatus, setNotice]);

  const confirmPackageImport = async () => {
    if (!packageSummary) return;
    setImportingPackage(true);
    try {
      const result = await platformBridge.importCodexThemePath(packageSummary.path, packageSummary.alreadyInstalled);
      setThemes(result.themes);
      if (result.importedThemeId) selectTheme(result.importedThemeId);
      setNotice(result.message);
      setPackageSummary(null);
    } catch (error) {
      setNotice(error instanceof Error ? error.message : String(error));
    } finally {
      setImportingPackage(false);
    }
  };

  return (
    <div className="app-shell">
      <Sidebar />
      <main className={cn("main-content", notice && "main-content-with-toast")}>
        {selectedTheme ? <ThemeDetail key={selectedTheme.id} theme={selectedTheme} onClose={() => selectTheme(null)} /> : (
          <>
            {route === "discover" && <DiscoverPage />}
            {route === "library" && <LibraryPage />}
            {route === "create" && <CreatePage />}
            {route === "settings" && <SettingsPage />}
          </>
        )}
      </main>
      {notice && <div className="toast" role={runtimeStatus === "error" ? "alert" : "status"} aria-live={runtimeStatus === "error" ? "assertive" : "polite"}><CheckCircle2 size={15} aria-hidden="true" /><span>{notice}</span><button type="button" aria-label="Dismiss message" onClick={() => setNotice(null)}><X size={14} /></button></div>}
      <AlertDialog.Root open={Boolean(packageSummary)} onOpenChange={(open) => { if (!open && !importingPackage) setPackageSummary(null); }}>
        <AlertDialog.Portal>
          <AlertDialog.Backdrop className="dialog-backdrop" />
          <AlertDialog.Viewport className="dialog-viewport">
            <AlertDialog.Popup className="dialog-popup">
              <div className="delete-dialog-icon"><FileArchive size={18} /></div>
              <AlertDialog.Title>{packageSummary?.alreadyInstalled ? `Replace “${packageSummary.name}”?` : `Import “${packageSummary?.name}”?`}</AlertDialog.Title>
              <AlertDialog.Description>
                {packageSummary?.alreadyInstalled ? "This theme is already in your library." : `Version ${packageSummary?.version} · ${packageSummary?.author}`}
              </AlertDialog.Description>
              <div className="dialog-actions">
                <AlertDialog.Close className="secondary-button" disabled={importingPackage}>Cancel</AlertDialog.Close>
                <button className="primary-button" type="button" disabled={importingPackage} onClick={() => void confirmPackageImport()}>{importingPackage ? (packageSummary?.alreadyInstalled ? "Replacing…" : "Importing…") : (packageSummary?.alreadyInstalled ? "Replace theme" : "Import theme")}</button>
              </div>
            </AlertDialog.Popup>
          </AlertDialog.Viewport>
        </AlertDialog.Portal>
      </AlertDialog.Root>
    </div>
  );
}
