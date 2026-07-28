import { Popover } from "@base-ui/react/popover";
import {
  Cat,
  ChevronRight,
  Frame,
  Settings2,
  Sparkles,
  X,
} from "lucide-react";
import { useState } from "react";
import { windowBorderStyleLabel, windowBorderStyles } from "../domain/window-effects";
import { cn } from "../lib/cn";
import { platformBridge } from "../services/platform";
import { useAppStore, type WindowBorderStyle } from "../store/app-store";

export function WindowEffectsControl() {
  const [open, setOpen] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const preferences = useAppStore((state) => state.preferences);
  const runtimeStatus = useAppStore((state) => state.runtime.status);
  const setNotice = useAppStore((state) => state.setNotice);
  const setRoute = useAppStore((state) => state.setRoute);
  const updatePreference = useAppStore((state) => state.updatePreference);
  const {
    effectsEnabled,
    effectsDiscoverySeen,
    pixelCatEnabled,
    windowBorderEnabled,
    windowBorderStyle,
  } = preferences;

  const markDiscovered = () => {
    if (!effectsDiscoverySeen) updatePreference("effectsDiscoverySeen", true);
  };

  const run = async (
    operation: () => ReturnType<typeof platformBridge.setWindowEffectsEnabled>,
    onSuccess: () => void,
  ) => {
    setBusy(true);
    setError(null);
    try {
      const result = await operation();
      if (!result.ok) throw new Error(result.message);
      onSuccess();
      setNotice(result.message);
      return true;
    } catch (operationError) {
      setError(operationError instanceof Error ? operationError.message : String(operationError));
      return false;
    } finally {
      setBusy(false);
    }
  };

  const setMasterEnabled = (enabled: boolean) => run(
    () => platformBridge.setWindowEffectsEnabled(enabled),
    () => updatePreference("effectsEnabled", enabled),
  );

  const setBorderEnabled = (enabled: boolean) => run(
    () => platformBridge.setWindowBorder(enabled, windowBorderStyle),
    () => updatePreference("windowBorderEnabled", enabled),
  );

  const setBorderStyle = (style: WindowBorderStyle) => run(
    () => platformBridge.setWindowBorder(windowBorderEnabled, style),
    () => updatePreference("windowBorderStyle", style),
  );

  const setCatEnabled = (enabled: boolean) => run(
    () => platformBridge.setPixelCatEnabled(enabled),
    () => updatePreference("pixelCatEnabled", enabled),
  );

  const activeParts = [
    windowBorderEnabled ? windowBorderStyleLabel(windowBorderStyle) : null,
    pixelCatEnabled ? "Pixel Cat" : null,
  ].filter(Boolean);
  const summary = effectsEnabled
    ? activeParts.length > 0 ? activeParts.join(" · ") : "Ready to customize"
    : "Effects off";
  const showCoachmark = runtimeStatus === "active" && !effectsDiscoverySeen;

  const handleOpenChange = (nextOpen: boolean) => {
    setOpen(nextOpen);
    if (nextOpen) markDiscovered();
  };

  return (
    <div className="effects-quick-area">
      <Popover.Root open={open} onOpenChange={handleOpenChange}>
        <div className={cn("effects-quick-card", effectsEnabled && "effects-quick-card-on")}>
          <Popover.Trigger className="effects-quick-trigger">
            <span className="effects-quick-icon" aria-hidden="true"><Sparkles size={15} /></span>
            <span className="effects-quick-copy">
              <strong>Window Effects</strong>
              <small>{summary}</small>
            </span>
            <ChevronRight className="effects-quick-chevron" size={14} aria-hidden="true" />
          </Popover.Trigger>
          <label className="effects-quick-toggle">
            <span className="sr-only">Enable Codex window effects</span>
            <input
              className="switch"
              type="checkbox"
              checked={effectsEnabled}
              disabled={busy}
              onChange={(event) => void setMasterEnabled(event.target.checked)}
            />
          </label>
        </div>
        <Popover.Portal>
          <Popover.Positioner className="effects-popover-positioner" side="right" align="end" sideOffset={12}>
            <Popover.Popup className="effects-popover">
              <div className="effects-popover-header">
                <span className="effects-popover-mark" aria-hidden="true"><Sparkles size={16} /></span>
                <div>
                  <Popover.Title>Codex Window Effects</Popover.Title>
                  <Popover.Description>Add motion and personality to every theme.</Popover.Description>
                </div>
                <Popover.Close className="effects-popover-close" aria-label="Close window effects">
                  <X size={15} />
                </Popover.Close>
              </div>

              <label className="effects-control-row effects-master-row">
                <span><strong>Effects</strong><small>Keep your combination ready when paused.</small></span>
                <input
                  className="switch"
                  type="checkbox"
                  checked={effectsEnabled}
                  disabled={busy}
                  onChange={(event) => void setMasterEnabled(event.target.checked)}
                />
              </label>

              <div className={cn("effects-control-section", !effectsEnabled && "effects-control-disabled")}>
                <label className="effects-control-row">
                  <span className="effects-control-title"><Frame size={14} aria-hidden="true" /><span><strong>Animated border</strong><small>Six-pixel classic marquee.</small></span></span>
                  <input
                    className="switch"
                    type="checkbox"
                    checked={windowBorderEnabled}
                    disabled={busy || !effectsEnabled}
                    onChange={(event) => void setBorderEnabled(event.target.checked)}
                  />
                </label>
                <div className="effects-style-grid" aria-label="Animated border style">
                  {windowBorderStyles.map((style) => (
                    <button
                      className={cn(
                        "effects-style-option",
                        windowBorderStyle === style.value && "effects-style-option-selected",
                      )}
                      type="button"
                      key={style.value}
                      aria-pressed={windowBorderStyle === style.value}
                      disabled={busy || !effectsEnabled || !windowBorderEnabled}
                      onClick={() => void setBorderStyle(style.value)}
                    >
                      <span className={cn("effects-style-swatch", `effects-style-${style.value}`)} aria-hidden="true" />
                      <span>{style.shortLabel}</span>
                    </button>
                  ))}
                </div>
              </div>

              <label className={cn("effects-control-row", !effectsEnabled && "effects-control-disabled")}>
                <span className="effects-control-title"><Cat size={14} aria-hidden="true" /><span><strong>Pixel Cat</strong><small>Continuous slow walk.</small></span></span>
                <input
                  className="switch"
                  type="checkbox"
                  checked={pixelCatEnabled}
                  disabled={busy || !effectsEnabled}
                  onChange={(event) => void setCatEnabled(event.target.checked)}
                />
              </label>

              {error && <p className="effects-popover-error" role="alert">{error}</p>}
              <button
                className="effects-advanced-link"
                type="button"
                onClick={() => {
                  markDiscovered();
                  setOpen(false);
                  setRoute("settings");
                }}
              >
                <Settings2 size={14} aria-hidden="true" />
                <span>Open advanced settings</span>
                <ChevronRight size={13} aria-hidden="true" />
              </button>
            </Popover.Popup>
          </Popover.Positioner>
        </Popover.Portal>
      </Popover.Root>

      {showCoachmark && (
        <aside className="effects-coachmark" aria-label="Window effects introduction">
          <button
            type="button"
            className="effects-coachmark-close"
            aria-label="Dismiss window effects introduction"
            onClick={markDiscovered}
          >
            <X size={14} />
          </button>
          <span className="effects-coachmark-icon" aria-hidden="true"><Sparkles size={17} /></span>
          <strong>Window Effects are on</strong>
          <p>Animated borders and Pixel Cat bring your theme to life.</p>
          <div>
            <button
              type="button"
              className="effects-coachmark-primary"
              onClick={() => {
                markDiscovered();
                setOpen(true);
              }}
            >
              Customize
            </button>
            <button
              type="button"
              className="effects-coachmark-secondary"
              disabled={busy}
              onClick={() => {
                markDiscovered();
                void setMasterEnabled(false);
              }}
            >
              Turn off
            </button>
          </div>
        </aside>
      )}
    </div>
  );
}
