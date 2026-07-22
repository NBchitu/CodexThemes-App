import { ArrowUpRight, FolderInput } from "lucide-react";
import galleryHero from "../../assets/theme-gallery-preview.png";
import { platformBridge } from "../../services/platform";
import { useAppStore } from "../../store/app-store";

export function DiscoverPage() {
  const setNotice = useAppStore((state) => state.setNotice);
  const setThemes = useAppStore((state) => state.setThemes);

  const openGallery = async () => {
    try {
      const result = await platformBridge.openThemeGallery();
      setNotice(result.message);
    } catch (error) {
      setNotice(error instanceof Error ? error.message : String(error));
    }
  };

  const importTheme = async () => {
    try {
      const result = await platformBridge.importThemeFolder();
      setThemes(result.themes);
      setNotice(result.message);
    } catch (error) {
      setNotice(error instanceof Error ? error.message : String(error));
    }
  };

  return (
    <section className="page" aria-labelledby="discover-title">
      <header className="gallery-hero">
        <img src={galleryHero} alt="" />
        <div className="gallery-copy">
          <p className="eyebrow">Online theme gallery</p>
          <h1 id="discover-title">Find a new atmosphere for Codex</h1>
          <p>Browse the curated gallery in your browser. Until direct App packages are published, download a theme, extract the ZIP, then import its folder here.</p>
          <div className="gallery-actions">
            <button className="primary-button" type="button" onClick={() => void openGallery()}>
              Browse theme gallery <ArrowUpRight size={15} />
            </button>
            <button className="secondary-button" type="button" onClick={() => void importTheme()}>
              <FolderInput size={15} /> Import extracted theme
            </button>
          </div>
        </div>
      </header>
      <ol className="import-steps" aria-label="Theme installation steps">
        <li><span>1</span><div><strong>Choose</strong><p>Pick a theme from the online gallery.</p></div></li>
        <li><span>2</span><div><strong>Extract</strong><p>Download and extract the currently available theme ZIP.</p></div></li>
        <li><span>3</span><div><strong>Import</strong><p>Select the extracted folder, then apply it from My Themes.</p></div></li>
      </ol>
    </section>
  );
}
