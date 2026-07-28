#!/bin/bash

set -euo pipefail
. "$(cd "$(dirname "$0")" && pwd -P)/common-macos.sh"

ENABLED=""
STYLE="classic-rainbow"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --enabled) ENABLED="${2:-}"; shift 2 ;;
    --style) STYLE="${2:-}"; shift 2 ;;
    *) fail "Unknown argument: $1" ;;
  esac
done

case "$ENABLED" in
  true|false) ;;
  *) fail "Usage: set-window-border-macos.sh --enabled true|false" ;;
esac
case "$STYLE" in
  classic-rainbow|candy-stripe|ocean|monochrome) ;;
  *) fail "Unsupported animated window border style: $STYLE" ;;
esac

ensure_state_root
ensure_node_runtime
"$NODE" "$PROJECT_ROOT/scripts/update-window-effect.mjs" \
  "$WINDOW_EFFECTS_PATH" windowBorderEnabled "$ENABLED" "$STYLE"

PORT=9341
if [ -f "$STATE_PATH" ]; then
  saved="$(state_field port 2>/dev/null || true)"
  [ -n "${saved:-}" ] && PORT="$saved"
fi

if verified_cdp_endpoint "$PORT" 2>/dev/null; then
  hot_reapply_theme "$PORT" 5000 \
    || fail "Window border was saved, but the active Codex renderer could not be updated."
fi
