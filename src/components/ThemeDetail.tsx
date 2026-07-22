import { AlertDialog } from "@base-ui/react/alert-dialog";
import { Select } from "@base-ui/react/select";
import { ArrowLeft, Check, ChevronDown, CircleCheck, RefreshCw, Trash2 } from "lucide-react";
import { useState } from "react";
import type { Theme, ThemeSettings } from "../domain/theme";
import { platformBridge } from "../services/platform";
import { useAppStore } from "../store/app-store";

interface ThemeDetailProps {
  theme: Theme;
  onClose: () => void;
}

interface ThemeFactSelectProps<T extends string> {
  label: string;
  value: T;
  options: readonly T[];
  onChange?: (value: T) => void;
}

const formatFactValue = (value: string) => value.replace("-", " ").replace(/^./, (character) => character.toUpperCase());

function ThemeFactSelect<T extends string>({ label, value, options, onChange }: ThemeFactSelectProps<T>) {
  const items = options.map((option) => ({ value: option, label: formatFactValue(option) }));
  return (
    <Select.Root items={items} value={value} onValueChange={(nextValue) => nextValue && onChange?.(nextValue)}>
      <Select.Trigger className="theme-fact-trigger" aria-label={label}>
        <Select.Value />
        <Select.Icon className="theme-fact-chevron"><ChevronDown size={11} /></Select.Icon>
      </Select.Trigger>
      <Select.Portal>
        <Select.Positioner sideOffset={5} align="end" className="theme-fact-positioner">
          <Select.Popup className="theme-fact-popup">
            <Select.List>
              {items.map((item) => (
                <Select.Item key={item.value} value={item.value} className="theme-fact-option">
                  <Select.ItemText>{item.label}</Select.ItemText>
                  <Select.ItemIndicator className="theme-fact-option-check"><Check size={12} /></Select.ItemIndicator>
                </Select.Item>
              ))}
            </Select.List>
          </Select.Popup>
        </Select.Positioner>
      </Select.Portal>
    </Select.Root>
  );
}

export function ThemeDetail({ theme, onClose }: ThemeDetailProps) {
  const [deleteError, setDeleteError] = useState<string | null>(null);
  const [settings, setSettings] = useState<ThemeSettings>({
    appearance: theme.appearance,
    taskMode: theme.art.taskMode,
    safeArea: theme.art.safeArea,
  });
  const setNotice = useAppStore((state) => state.setNotice);
  const setThemes = useAppStore((state) => state.setThemes);
  const setRuntime = useAppStore((state) => state.setRuntime);
  const setPreferredTheme = useAppStore((state) => state.setPreferredTheme);
  const runtime = useAppStore((state) => state.runtime);
  const isActive = runtime.activeThemeId === theme.id;

  const handlePrimaryAction = async () => {
    setRuntime({ ...runtime, status: "applying", message: `Applying ${theme.name}…` });
    try {
      const settingsChanged = settings.appearance !== theme.appearance
        || settings.taskMode !== theme.art.taskMode
        || settings.safeArea !== theme.art.safeArea;
      if (settingsChanged) {
        const updated = await platformBridge.updateThemeSettings(theme.id, settings);
        setThemes(updated.themes);
      }
      const result = await platformBridge.applyTheme(theme.id);
      setRuntime({
        status: result.status,
        activeThemeId: result.verified ? theme.id : runtime.activeThemeId,
        message: result.message,
        isNativeHost: runtime.isNativeHost,
      });
      if (result.verified) setPreferredTheme(theme.id);
      setNotice(result.verified && isActive ? "Theme reapplied." : result.message);
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
      <div className="detail-layout">
        <div className="detail-main">
          <img className="detail-hero" src={theme.previewUrl} alt={`${theme.name} preview`} />
          <div className="detail-copy">
            <div className="eyebrow">{theme.category} · Version {theme.version}</div>
            <h1 id="theme-detail-title">{theme.name}</h1>
            <p className="detail-author">by {theme.author}</p>
            <p className="detail-description">{theme.description}</p>
          </div>
        </div>
        <div className="detail-action-panel">
          {isActive && <div className="active-theme-status"><Check size={14} /> Currently active</div>}
          <button className="primary-button" type="button" disabled={runtime.status === "applying"} onClick={handlePrimaryAction}>
            {isActive ? <RefreshCw size={15} /> : <Check size={16} />}
            {runtime.status === "applying" ? "Applying…" : isActive ? "Apply again" : "Apply theme"}
          </button>
          <dl className="theme-facts">
            <div><dt>Appearance</dt><dd><ThemeFactSelect label="Appearance" value={settings.appearance} options={["auto", "light", "dark"]} onChange={(appearance) => setSettings((current) => ({ ...current, appearance }))} /></dd></div>
            <div><dt>Task mode</dt><dd><ThemeFactSelect label="Task mode" value={settings.taskMode} options={["auto", "ambient", "banner", "off"]} onChange={(taskMode) => setSettings((current) => ({ ...current, taskMode }))} /></dd></div>
            <div><dt>Safe area</dt><dd><ThemeFactSelect label="Safe area" value={settings.safeArea} options={["auto", "left", "right", "center", "none"]} onChange={(safeArea) => setSettings((current) => ({ ...current, safeArea }))} /></dd></div>
            <div><dt>Source</dt><dd><ThemeFactSelect label="Source" value={theme.origin} options={[theme.origin]} /></dd></div>
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
          <div className="panel-trust-note">
            <CircleCheck size={13} aria-hidden="true" />
            <span>Validated theme package</span>
          </div>
        </div>
      </div>
    </section>
  );
}
