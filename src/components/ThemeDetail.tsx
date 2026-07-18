import { AlertDialog } from "@base-ui/react/alert-dialog";
import { ArrowLeft, Check, ShieldCheck, Trash2 } from "lucide-react";
import { useState } from "react";
import type { Theme } from "../domain/theme";
import { platformBridge } from "../services/platform";
import { useAppStore } from "../store/app-store";

interface ThemeDetailProps {
  theme: Theme;
  onClose: () => void;
}

export function ThemeDetail({ theme, onClose }: ThemeDetailProps) {
  const [deleteError, setDeleteError] = useState<string | null>(null);
  const setNotice = useAppStore((state) => state.setNotice);
  const setThemes = useAppStore((state) => state.setThemes);
  const setRuntime = useAppStore((state) => state.setRuntime);
  const setPreferredTheme = useAppStore((state) => state.setPreferredTheme);
  const runtime = useAppStore((state) => state.runtime);
  const isActive = runtime.activeThemeId === theme.id;

  const handlePrimaryAction = async () => {
    setRuntime({ ...runtime, status: "applying", message: `Applying ${theme.name}…` });
    try {
      const result = await platformBridge.applyTheme(theme.id);
      setRuntime({
        status: result.status,
        activeThemeId: result.verified ? theme.id : runtime.activeThemeId,
        message: result.message,
        isNativeHost: runtime.isNativeHost,
      });
      if (result.verified) setPreferredTheme(theme.id);
      setNotice(result.message);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setRuntime({ ...runtime, status: "error", message });
      setNotice(message);
    }
  };

  const handleDelete = async () => {
    setDeleteError(null);
    try {
      const result = await platformBridge.deleteTheme(theme.id);
      setThemes(result.themes);
      setNotice(result.message);
      onClose();
    } catch (error) {
      setDeleteError(error instanceof Error ? error.message : String(error));
    }
  };

  return (
    <section className="detail-page" aria-labelledby="theme-detail-title">
      <button className="back-button" type="button" onClick={onClose}>
        <ArrowLeft size={16} /> Back
      </button>
      <img className="detail-hero" src={theme.previewUrl} alt={`${theme.name} preview`} />
      <div className="detail-layout">
        <div>
          <div className="eyebrow">{theme.category} · Version {theme.version}</div>
          <h1 id="theme-detail-title">{theme.name}</h1>
          <p className="detail-author">by {theme.author}</p>
          <p className="detail-description">{theme.description}</p>
          <div className="trust-note">
            <ShieldCheck size={18} aria-hidden="true" />
            <span>Validated theme package. No executable JavaScript is included.</span>
          </div>
        </div>
        <div className="detail-action-panel">
          <button className="primary-button" type="button" disabled={isActive || runtime.status === "applying"} onClick={handlePrimaryAction}>
            <Check size={16} />
            {runtime.status === "applying" ? "Applying…" : isActive ? "Currently active" : "Apply theme"}
          </button>
          <dl className="theme-facts">
            <div><dt>Appearance</dt><dd>{theme.appearance}</dd></div>
            <div><dt>Task mode</dt><dd>{theme.art.taskMode}</dd></div>
            <div><dt>Safe area</dt><dd>{theme.art.safeArea}</dd></div>
            <div><dt>Source</dt><dd>{theme.origin}</dd></div>
          </dl>
          {theme.origin === "imported" && (
            <div className="delete-theme-section">
              <AlertDialog.Root>
                <AlertDialog.Trigger className="danger-button" disabled={isActive}>
                  <Trash2 size={14} /> Delete theme
                </AlertDialog.Trigger>
                <AlertDialog.Portal>
                  <AlertDialog.Backdrop className="dialog-backdrop" />
                  <AlertDialog.Viewport className="dialog-viewport">
                    <AlertDialog.Popup className="dialog-popup">
                      <div className="delete-dialog-icon"><Trash2 size={18} /></div>
                      <AlertDialog.Title>Delete “{theme.name}”?</AlertDialog.Title>
                      <AlertDialog.Description>
                        This imported theme will be removed from My Themes and deleted from this Mac. This cannot be undone.
                      </AlertDialog.Description>
                      <div className="dialog-actions">
                        <AlertDialog.Close className="secondary-button">Cancel</AlertDialog.Close>
                        <AlertDialog.Close className="danger-button" onClick={() => void handleDelete()}>Delete theme</AlertDialog.Close>
                      </div>
                    </AlertDialog.Popup>
                  </AlertDialog.Viewport>
                </AlertDialog.Portal>
              </AlertDialog.Root>
              {isActive && <small>Switch themes or restore the original appearance before deleting.</small>}
              {deleteError && <p role="alert">{deleteError}</p>}
            </div>
          )}
        </div>
      </div>
    </section>
  );
}
