import { BookOpen, Compass, Github, Library, Play, Settings } from "lucide-react";
import logoMark from "../assets/brand/codexthemes-logo-mark.svg";
import { cn } from "../lib/cn";
import { t } from "../i18n/en";
import { platformBridge } from "../services/platform";
import { useAppStore, type AppRoute } from "../store/app-store";

const items: Array<{ route: AppRoute; label: ReturnType<typeof t>; icon: typeof Compass }> = [
  { route: "library", label: t("myThemes"), icon: Library },
  { route: "discover", label: t("discover"), icon: Compass },
  { route: "create", label: t("create"), icon: BookOpen },
  { route: "settings", label: t("settings"), icon: Settings },
];

export function Sidebar() {
  const route = useAppStore((state) => state.route);
  const setRoute = useAppStore((state) => state.setRoute);
  const runtime = useAppStore((state) => state.runtime);
  const setRuntime = useAppStore((state) => state.setRuntime);
  const setNotice = useAppStore((state) => state.setNotice);
  const preferredThemeId = useAppStore((state) => state.preferredThemeId);
  const setPreferredTheme = useAppStore((state) => state.setPreferredTheme);
  const isLaunching = runtime.status === "applying";

  const launchCodex = async () => {
    setRuntime({ ...runtime, status: "applying", message: "Launching Codex…" });
    try {
      const themeId = runtime.activeThemeId ?? preferredThemeId;
      const result = themeId && runtime.status !== "active"
        ? await platformBridge.applyTheme(themeId)
        : await platformBridge.openCodex();
      if (result.ok && themeId && runtime.status !== "active") {
        await platformBridge.openCodex();
      }
      if (result.verified && themeId) setPreferredTheme(themeId);
      setRuntime({
        status: result.status,
        activeThemeId: result.verified && themeId ? themeId : runtime.activeThemeId,
        message: result.message,
        isNativeHost: runtime.isNativeHost,
      });
      setNotice(result.message);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setRuntime({ ...runtime, status: "error", message });
      setNotice(message);
    }
  };

  const openProjectHome = async () => {
    try {
      const result = await platformBridge.openProjectHome();
      if (!result.ok) setNotice(result.message);
    } catch (error) {
      setNotice(error instanceof Error ? error.message : String(error));
    }
  };

  return (
    <aside className="sidebar">
      <div className="brand">
        <span className="brand-mark" aria-hidden="true"><img src={logoMark} alt="" /></span>
        <span>{t("appName")}</span>
      </div>
      <nav className="nav-list" aria-label="Primary navigation">
        {items.map(({ route: itemRoute, label, icon: Icon }) => (
          <button
            type="button"
            key={itemRoute}
            className={cn("nav-item", route === itemRoute && "nav-item-active")}
            aria-current={route === itemRoute ? "page" : undefined}
            onClick={() => setRoute(itemRoute)}
          >
            <Icon size={17} strokeWidth={1.8} aria-hidden="true" />
            <span>{label}</span>
          </button>
        ))}
      </nav>
      <div className="codex-launch-area">
        <button className="codex-launch-button" type="button" onClick={() => void launchCodex()} disabled={isLaunching}>
          <span className="codex-launch-icon"><Play size={13} fill="currentColor" aria-hidden="true" /></span>
          <strong>{isLaunching ? "Launching…" : "Run Codex"}</strong>
        </button>
      </div>
      <div className="runtime-card" aria-live="polite">
        <div className="runtime-heading">
          <span className={cn("status-dot", runtime.status === "active" && "status-success")} />
          <span>Codex</span>
        </div>
        <p>{runtime.status === "preview" ? "Preview mode" : runtime.message}</p>
      </div>
      <button className="project-link" type="button" onClick={() => void openProjectHome()}>
        <Github size={15} strokeWidth={1.8} aria-hidden="true" />
        <span>GitHub · Latest releases</span>
      </button>
    </aside>
  );
}
