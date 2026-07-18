# Codex Themes Desktop — Delivery Status

## Saved increment

Version: `0.1.1`

This increment establishes the desktop product frontend and platform-neutral application layer. It is intentionally honest about its runtime boundary: browser preview mode does not execute or simulate a successful Codex CDP injection.

## Completed

- Detailed functional specification
- Detailed design and color constraints
- Milestone-based development plan and acceptance criteria
- React 19, TypeScript, Vite, and Tailwind project scaffold
- macOS-style responsive application shell
- Discover marketplace with search and category filtering
- Theme cards, active/installed states, and theme detail view
- Local My Themes library and native folder-import handoff
- Theme creation tutorial and safe package structure
- Settings for login launch, minimized startup, theme restore, updates, language, and appearance
- Light, dark, and system appearances using the approved graphite/green tokens
- English localization dictionary and localization-ready structure
- Versioned Zod theme manifest schema
- Actionable manifest validation results
- Typed platform bridge for apply, restore, status, and Codex launch
- Browser preview adapter that refuses to claim native operations succeeded
- Local preference persistence
- Unit tests for supported and rejected theme manifests
- Successful production frontend build
- Tauri 2 native macOS application shell
- Restricted Rust commands for runtime status, theme switching, original-appearance restore, and opening Codex
- Existing macOS CDP scripts, assets, and presets bundled as application resources
- Automatic managed-runtime installation before the first native theme switch
- Exact active-theme and live CDP verification after a theme switch
- Accessible confirmation dialog before restoring and restarting Codex
- Successful arm64 macOS application bundle build
- Valid local ad-hoc signature with sealed application resources
- Upgrade-safe seeding for bundled presets missing from older managed runtimes
- React 19-safe My Themes state selection without unstable external-store snapshots
- Vite-bundled theme previews that render in both development and native builds
- Refined compact typography hierarchy for navigation, headings, metadata, and actions
- First-launch creation of private managed themes, runtime, cache, logs, state, backup directories
- Upgrade-safe startup seeding of every bundled preset without overwriting existing copies
- Disk-backed native theme library scanning and local preview asset loading
- Native macOS folder picker with safe extracted-folder validation and atomic import
- Website-assisted gallery landing page with browser handoff and import instructions
- Privacy-safe UTM attribution for theme-gallery visits opened from the desktop application
- Built-in and imported themes use direct Apply actions with no simulated downloads
- Visual Codex-assisted theme creation workflow with an exportable Markdown instruction file, copy-ready chat message, source-image sample, Codex UI reference, and automatic application after import
- Finder access to the managed theme library and confirmed deletion for inactive imported themes, with built-in, active-theme, and path-containment protection
- Lower-overhead renderer lifecycle: structural mutation filtering, 300 ms coalescing, 15-second fallback checks, slower active CDP discovery, disconnected exponential backoff, and no backdrop blur on streaming surfaces
- Prominent global Launch Codex action that stays visible across tabs and reuses the last selected theme's verified CDP launch path

## Active prioritized backlog

- Add the final two curated built-in theme packs (the initializer already supports them automatically)
- Native menu bar and login item
- Real diagnostics (original-appearance restoration from the GUI is complete)
- Application updater
- macOS signing, notarization, DMG packaging, and clean-machine acceptance
- Chinese and Japanese translations
- Windows platform adapter

The macOS MVP no longer includes an in-app remote marketplace index or downloader. Four curated built-in themes are planned; additional themes are downloaded as ZIP archives from the public website, extracted by the user, and imported as local folders.

## Development environment

The macOS development environment now includes Xcode 26.3, Apple Clang 17, the macOS SDK, Node.js 24, npm 11, Rust 1.97.1, Cargo 1.97.1, rustfmt, and Tauri CLI 2.11.4. Rust is installed through rustup for the current user and loaded from `.zprofile`.

## Verification commands

```bash
cd desktop-app
npm test
npm run build
```

## Next engineering task

Add diagnostics, local logs, a copyable privacy-reviewed report, Open Logs Folder, and actionable recovery guidance. The report must exclude theme images, API keys, conversations, and unrelated Codex content. Rewriting the CDP injector in Rust should happen only after behavioral parity tests exist.

## Native build artifact

The current local arm64 development bundle is generated at:

```text
desktop-app/src-tauri/target/release/bundle/macos/Codex Themes.app
```

The saved local artifact is ad-hoc signed for development. Public distribution still requires an Apple Developer ID Application certificate, hardened runtime configuration, notarization, and stapling.
