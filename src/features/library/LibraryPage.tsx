import { FileArchive, FolderInput, FolderOpen } from "lucide-react";
import { ThemeCard } from "../../components/ThemeCard";
import type { Theme } from "../../domain/theme";
import { useAppStore } from "../../store/app-store";
import { platformBridge } from "../../services/platform";

export function LibraryPage() {
  const allThemes = useAppStore((state) => state.themes);
  const themes = allThemes.filter((theme) => theme.installed);
  const runtime = useAppStore((state) => state.runtime);
  const selectTheme = useAppStore((state) => state.selectTheme);
  const setNotice = useAppStore((state) => state.setNotice);
  const setThemes = useAppStore((state) => state.setThemes);
  const handleImport = async () => {
    try {
      const result = await platformBridge.importThemeFolder();
      setThemes(result.themes);
      setNotice(result.message);
    } catch (error) {
      setNotice(error instanceof Error ? error.message : String(error));
    }
  };
  const handlePackageImport = async () => {
    try {
      const result = await platformBridge.importCodexThemePackage();
      setThemes(result.themes);
      setNotice(result.message);
    } catch (error) {
      setNotice(error instanceof Error ? error.message : String(error));
    }
  };
  const handleOpenFolder = async () => {
    try {
      const result = await platformBridge.openThemesFolder();
      setNotice(result.message);
    } catch (error) {
      setNotice(error instanceof Error ? error.message : String(error));
    }
  };

  return (
    <section className="page" aria-labelledby="library-title">
      <header className="page-header action-header">
        <div><p className="eyebrow">Local library</p><h1 id="library-title">My Themes</h1><p>Your built-in and locally imported themes, stored safely on this Mac.</p></div>
        <div className="header-actions">
          <button className="secondary-button" type="button" onClick={() => void handleOpenFolder()}><FolderOpen size={15} /> Open theme folder</button>
          <button className="secondary-button" type="button" onClick={() => void handleImport()}><FolderInput size={16} /> Import legacy folder</button>
          <button className="primary-button" type="button" onClick={() => void handlePackageImport()}><FileArchive size={16} /> Import .codextheme</button>
        </div>
      </header>
      <div className="theme-grid">
        {themes.map((theme) => <ThemeCard key={theme.id} theme={theme} active={runtime.activeThemeId === theme.id} onOpen={(item: Theme) => selectTheme(item.id)} />)}
      </div>
    </section>
  );
}
