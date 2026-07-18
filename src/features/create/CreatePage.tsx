import { Dialog } from "@base-ui/react/dialog";
import { Check, Copy, Download, Expand, FileText, FolderInput, Image, X } from "lucide-react";
import { useState } from "react";
import sampleImage from "../../assets/theme-creation-example.jpg";
import codexScreenshot from "../../assets/codex-theme-message-example.png";
import { platformBridge } from "../../services/platform";
import { useAppStore } from "../../store/app-store";

const messageForCodex = "I’ve attached my source image and codex-theme-creation-guide.md. Follow the guide, create the background and theme.json, then return the finished theme as a downloadable ZIP archive.";

export function CreatePage() {
  const [copied, setCopied] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const setNotice = useAppStore((state) => state.setNotice);
  const setThemes = useAppStore((state) => state.setThemes);
  const setRuntime = useAppStore((state) => state.setRuntime);
  const setRoute = useAppStore((state) => state.setRoute);
  const setPreferredTheme = useAppStore((state) => state.setPreferredTheme);
  const runtime = useAppStore((state) => state.runtime);

  const saveGuide = async () => {
    setActionError(null);
    try {
      const result = await platformBridge.exportThemeCreationGuide();
      setNotice(result.message);
    } catch (error) {
      setActionError(error instanceof Error ? error.message : String(error));
    }
  };

  const copyMessage = async () => {
    setActionError(null);
    try {
      await navigator.clipboard.writeText(messageForCodex);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1600);
    } catch {
      setActionError("Could not copy the message. Select the text and copy it manually.");
    }
  };

  const importAndApply = async () => {
    setActionError(null);
    try {
      const imported = await platformBridge.importThemeFolder();
      setThemes(imported.themes);
      if (!imported.importedThemeId) {
        setNotice(imported.message);
        return;
      }
      setRuntime({ ...runtime, status: "applying", message: "Applying your new theme…" });
      const applied = await platformBridge.applyTheme(imported.importedThemeId);
      setRuntime({
        status: applied.status,
        activeThemeId: applied.verified ? imported.importedThemeId : runtime.activeThemeId,
        message: applied.message,
        isNativeHost: runtime.isNativeHost,
      });
      if (!applied.ok) {
        setActionError(applied.message);
        return;
      }
      setPreferredTheme(imported.importedThemeId);
      setNotice("Theme imported and applied.");
      setRoute("library");
    } catch (error) {
      setActionError(error instanceof Error ? error.message : String(error));
    }
  };

  return (
    <section className="page creator-page" aria-labelledby="create-title">
      <header className="page-header creator-header">
        <div>
          <p className="eyebrow">Guided creation</p>
          <h1 id="create-title">Create a Codex theme</h1>
          <p>Prepare two items, follow the example, then import the result.</p>
        </div>
      </header>

      <div className="creator-workflow" aria-label="Three-step theme creation workflow">
        <article className="creator-column">
          <header className="creator-column-heading"><span>1</span><div><h2>Prepare two items</h2><p>You need both before opening Codex.</p></div></header>
          <div className="creator-materials">
            <div><Image size={16} /><span><strong>Your favorite image</strong><small>Choose any JPG, PNG, or WebP you like.</small></span></div>
            <div><FileText size={16} /><span><strong>Our creation guide</strong><small>Save the Markdown file from this app.</small></span></div>
          </div>
          <figure className="creator-reference-frame">
            <img src={sampleImage} alt="Example image suitable for creating a Codex theme" />
            <figcaption>Example source image</figcaption>
          </figure>
          <button type="button" className="secondary-button creator-column-action" onClick={() => void saveGuide()}>
            <Download size={15} /> Save creation guide
          </button>
        </article>

        <article className="creator-column">
          <header className="creator-column-heading"><span>2</span><div><h2>Send to Codex</h2><p>Use this screenshot as your reference.</p></div></header>
          <Dialog.Root>
            <Dialog.Trigger className="creator-reference-frame screenshot-trigger">
              <img src={codexScreenshot} alt="Screenshot showing where to attach files in Codex" />
              <span className="screenshot-caption">Codex conversation example</span>
              <span className="expand-hint"><Expand size={13} /> Enlarge</span>
            </Dialog.Trigger>
            <Dialog.Portal>
              <Dialog.Backdrop className="dialog-backdrop" />
              <Dialog.Viewport className="dialog-viewport image-dialog-viewport">
                <Dialog.Popup className="image-dialog-popup">
                  <Dialog.Title className="sr-only">Codex conversation example</Dialog.Title>
                  <Dialog.Description className="sr-only">Attach your favorite image and the saved Markdown guide, then paste the provided message.</Dialog.Description>
                  <img src={codexScreenshot} alt="Large Codex conversation reference" />
                  <Dialog.Close className="image-dialog-close" aria-label="Close enlarged screenshot"><X size={17} /></Dialog.Close>
                </Dialog.Popup>
              </Dialog.Viewport>
            </Dialog.Portal>
          </Dialog.Root>
          <div className="copy-message-block compact-message-block">
            <p>{messageForCodex}</p>
            <button type="button" className="copy-message-action" onClick={() => void copyMessage()}>
              {copied ? <Check size={14} /> : <Copy size={14} />} {copied ? "Copied" : "Copy to Clipboard"}
            </button>
          </div>
          <p className="creator-instruction">Attach both items from step 1, then paste this message.</p>
        </article>

        <article className="creator-column">
          <header className="creator-column-heading"><span>3</span><div><h2>Import & apply</h2><p>Download and extract the ZIP from Codex.</p></div></header>
          <div className="import-cta-area">
            <FolderInput size={24} aria-hidden="true" />
            <strong>Your theme is ready</strong>
            <p>Select the extracted theme folder.</p>
            <button type="button" className="primary-button import-theme-button" onClick={() => void importAndApply()} disabled={runtime.status === "applying"}>
              <FolderInput size={16} /> {runtime.status === "applying" ? "Applying…" : "Import extracted theme"}
            </button>
          </div>
          <p className="creator-instruction"><Check size={14} /> The new theme is copied to My Themes and applied automatically.</p>
        </article>
      </div>
      {actionError && <p className="creator-error" role="alert">{actionError}</p>}
    </section>
  );
}
