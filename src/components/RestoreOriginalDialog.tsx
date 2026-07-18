import { AlertDialog } from "@base-ui/react/alert-dialog";
import { RotateCcw } from "lucide-react";
import { platformBridge } from "../services/platform";
import { useAppStore } from "../store/app-store";

export function RestoreOriginalDialog() {
  const runtime = useAppStore((state) => state.runtime);
  const setRuntime = useAppStore((state) => state.setRuntime);
  const setNotice = useAppStore((state) => state.setNotice);
  const setPreferredTheme = useAppStore((state) => state.setPreferredTheme);

  const restore = async () => {
    setRuntime({ ...runtime, status: "restoring", message: "Restoring original appearance…" });
    try {
      const result = await platformBridge.restoreOriginal();
      setRuntime({
        status: result.status,
        activeThemeId: result.verified ? null : runtime.activeThemeId,
        message: result.message,
        isNativeHost: runtime.isNativeHost,
      });
      if (result.verified) setPreferredTheme(null);
      setNotice(result.message);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setRuntime({ ...runtime, status: "error", message });
      setNotice(message);
    }
  };

  return (
    <AlertDialog.Root>
      <AlertDialog.Trigger className="secondary-button">
        <RotateCcw size={15} /> Restore original appearance
      </AlertDialog.Trigger>
      <AlertDialog.Portal>
        <AlertDialog.Backdrop className="dialog-backdrop" />
        <AlertDialog.Viewport className="dialog-viewport">
          <AlertDialog.Popup className="dialog-popup">
            <AlertDialog.Title>Restore Codex original appearance?</AlertDialog.Title>
            <AlertDialog.Description>
              Codex will restart, the managed CDP injector will stop, and injected theme styles will be removed. Your downloaded themes will be kept.
            </AlertDialog.Description>
            <div className="dialog-actions">
              <AlertDialog.Close className="secondary-button">Cancel</AlertDialog.Close>
              <AlertDialog.Close className="primary-button" onClick={() => void restore()}>Restore and restart Codex</AlertDialog.Close>
            </div>
          </AlertDialog.Popup>
        </AlertDialog.Viewport>
      </AlertDialog.Portal>
    </AlertDialog.Root>
  );
}
