# Native macOS Bridge

## Purpose

The Tauri host connects the React product interface to the existing, tested macOS CDP runtime. It deliberately exposes a narrow command surface rather than general shell execution.

## Exposed commands

- `get_runtime_status`
- `apply_theme { themeId }`
- `restore_original`
- `open_codex`

The renderer cannot submit a command path, shell fragment, CDP expression, or arbitrary argument list.

## Runtime resolution

The bridge looks for required scripts in this order:

1. User-managed runtime at `~/.codex/codex-dream-skin-studio`
2. Runtime resources bundled inside `Codex Themes.app`
3. Repository `macos/` directory during development

Before the first theme switch, the bridge installs the bundled runtime into the managed user directory with desktop launchers and automatic launch disabled. The existing installer retains responsibility for Codex identity checks, signed Node runtime validation, theme seeding, and safe configuration backup.

## Apply flow

1. Reject theme identifiers containing characters outside ASCII letters, digits, hyphen, and underscore.
2. Ensure the managed runtime exists.
3. Execute the fixed `switch-theme-macos.sh --id <validated-id>` command.
4. Request deep runtime status, including the loopback CDP probe.
5. Read the active theme manifest from managed application state.
6. Return success only when the session is active and the exact theme identifier matches.

## Restore flow

1. Require explicit confirmation in an accessible Alert Dialog.
2. Execute the fixed restore script with `--restore-base-theme --restart-codex`.
3. Verify that there is no active theme identifier and no active theme session.
4. Keep downloaded and imported themes in the user's managed library.

## Bundled resources

The application packages these existing runtime directories without duplicating their source:

- `macos/assets/`
- `macos/scripts/`
- `macos/presets/`
- `macos/VERSION`

## Distribution boundary

Local builds use an ad-hoc signature. A public release must use Developer ID signing and Apple notarization. The official Codex application is never modified or re-signed.
