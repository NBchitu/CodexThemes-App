# Codex Themes Desktop — Functional Specification

## 1. Product definition

Codex Themes Desktop is a third-party graphical theme manager for the official Codex desktop application. It allows users to discover, install, import, apply, and manage Codex themes without modifying the official application bundle, `app.asar`, code signature, or WindowsApps installation.

Themes are applied through a controlled local Chrome DevTools Protocol (CDP) connection bound to the loopback interface.

## 2. Release strategy

### Phase 1: macOS

- Deliver a signed and notarized macOS desktop application.
- Ship the user interface in English.
- Prepare the localization structure for Simplified Chinese and Japanese.
- Reuse the project's proven macOS launch, injection, verification, and recovery behavior.

### Phase 2: Windows

- Implement Windows only after the macOS version is stable.
- Preserve platform-neutral theme, marketplace, and UI contracts.
- Add a Windows platform adapter without changing product-level workflows.

## 3. Primary navigation

The application contains four primary destinations:

1. **My Themes** — default destination for built-in, downloaded, and imported themes.
2. **Discover** — browse the theme marketplace.
3. **Create** — learn how to create and validate a theme.
4. **Settings** — configure startup, language, updates, and diagnostics.

The sidebar also displays the current Codex connection and theme status.

## 4. Theme marketplace

### 4.1 Browse

- Display featured and recently added themes.
- Show a 16:9 preview, theme name, author, version, and installation status.
- Support search and basic category filtering.
- Open a dedicated theme detail page.
- Provide loading skeletons, an actionable empty state, and local error recovery.

### 4.2 Theme details

- Display a large preview and optional additional screenshots.
- Display name, author, version, description, compatibility, and update date.
- Show one primary action according to state: **Download**, **Apply**, or **Update**.
- Provide secondary actions such as opening local files or removing a download.
- Display a clear third-party product and asset-rights notice.

### 4.3 Download and update

- Download theme packages from a remote marketplace index.
- Verify package metadata, expected files, size limits, and integrity before installation.
- Extract packages only into the application-managed theme directory.
- Prevent path traversal, unsafe symbolic links, and unexpected executable content.
- Download to a temporary location and commit the theme atomically after validation.
- Preserve an installed version if an update fails.

The first marketplace implementation may use a signed or integrity-protected remote JSON index with statically hosted theme archives. Accounts, payment, reviews, and author dashboards are not required for the first release.

## 5. My Themes

### 5.1 Unified library

Display all locally available themes in one library:

- Built-in themes
- Marketplace downloads
- User-imported themes
- The currently active theme

Filters may distinguish **All**, **Installed**, **Imported**, and **Built-in** without splitting them into separate products or storage systems.

### 5.2 Theme actions

- Apply a theme.
- View theme metadata and validation state.
- Update a marketplace theme when available.
- Open the managed theme directory.
- Delete an inactive downloaded or imported theme after confirmation.
- Prevent deletion of the active theme until another theme or the original appearance is applied.

## 6. Import a local theme folder

Users can select a local folder through the macOS file picker or drag a folder into the import surface.

Recommended package structure:

```text
my-theme/
├── theme.json
├── background.jpg
├── preview.jpg
└── README.md
```

### 6.1 Import validation

- Require a valid `theme.json`.
- Require a supported background image.
- Validate image type, file size, dimensions, and total pixel count.
- Validate appearance, focus, safe-area, and task-mode fields.
- Reject executable scripts and unsupported arbitrary payloads.
- Reject path traversal and links escaping the selected folder.
- Show each validation result next to the import workflow.

### 6.2 Managed copy

- Copy a validated theme into application-managed storage.
- Never depend on the user's original folder after import.
- Resolve identifier conflicts explicitly instead of silently overwriting a theme.
- Allow the user to apply the theme immediately after successful import.

## 7. Apply and switch themes

### 7.1 Hot path

When the controlled Codex CDP session is already available:

1. Validate the selected theme.
2. Atomically activate its managed copy.
3. Ask the injector to refresh the payload.
4. Verify the exact theme and payload revision in the Codex renderer.
5. Mark the theme active only after successful verification.

### 7.2 Cold path

When Codex is running without the required CDP endpoint:

1. Explain that Codex must restart to enable the theme.
2. Warn that unsent content may be lost.
3. Continue only after user confirmation.
4. Launch Codex with a loopback-only CDP address and an available port.
5. Start the managed injection watcher.
6. Verify the endpoint, renderer target, and injected revision.

### 7.3 Failure behavior

- Never report success before renderer verification.
- Preserve the previous valid theme when switching fails.
- Stop an unverified injector when it can be identified safely.
- Keep diagnostic evidence when a process cannot be stopped safely.
- Present an actionable error with access to diagnostics.

## 8. Restore the original Codex appearance

The application provides **Restore Original Appearance**.

This action:

- Removes injected CSS and decorative DOM when a verified live session is available.
- Stops the managed injection watcher.
- Clears the active theme runtime state.
- Restarts Codex without CDP arguments when required.
- Verifies that the theme runtime is no longer active.
- Does not modify, reinstall, or patch the official Codex application.
- Does not delete downloaded, imported, or built-in themes.

Restoring the appearance is not the same as deleting user themes.

## 9. Create a theme

The first release provides a guided tutorial rather than a full visual editor.

Tutorial steps:

1. Download the starter template.
2. Add a supported background image.
3. Configure `theme.json`.
4. Understand focus position and safe areas.
5. Configure home and task-page behavior.
6. Create a marketplace preview.
7. Validate and import the local theme.
8. Test it in Codex.
9. Prepare it for marketplace submission.

The tutorial must include image recommendations, field documentation, a complete sample folder, validation tools, and copyright/portrait/trademark guidance.

## 10. Settings

### General

- Launch Codex Themes Desktop at login.
- Start minimized in the menu bar.
- Restore the last active theme when appropriate.
- Optionally open Codex from the theme manager.

Launching the manager at login and launching or restarting Codex must be separate settings.

### Marketplace

- Automatically check for theme updates.
- Manually refresh the marketplace index.
- Display the current index refresh state.

### Language

- English — available in the first release.
- Simplified Chinese — planned.
- Japanese — planned.

All user-facing strings must use localization keys from the first implementation. English must not be scattered as hard-coded component text.

### Diagnostics

- Detect the official Codex installation.
- Show Codex version and application identity.
- Show CDP and injector status without exposing secrets.
- Verify the active theme revision.
- Open the log directory.
- Copy a privacy-reviewed diagnostic report.
- Restore default application settings.

### About and updates

- Display application version and third-party disclaimer.
- Check for application updates.
- Link to licenses and privacy information.

## 11. Menu bar

The macOS menu bar item provides:

- Open Theme Manager
- Current Theme
- Apply Last Theme
- Restore Original Appearance
- Open Codex
- Quit

The menu bar and main window must use the same single backend instance and runtime state.

## 12. Platform architecture contract

Product workflows must depend on a platform-neutral adapter, conceptually:

```ts
interface PlatformAdapter {
  detectCodex(): Promise<CodexInstallation>;
  getRuntimeStatus(): Promise<RuntimeStatus>;
  launchCodexWithCdp(): Promise<CdpSession>;
  applyTheme(themeId: string): Promise<ApplyResult>;
  restoreOriginal(): Promise<RestoreResult>;
  verifyInjection(): Promise<VerificationResult>;
}
```

The first release implements only the macOS adapter. A future Windows adapter will handle Windows package discovery and application activation while reusing the same theme and marketplace services.

## 13. Security requirements

- Bind CDP only to a loopback address.
- Select an available port instead of assuming a fixed port.
- Validate that the endpoint belongs to the expected Codex process.
- Validate browser identity and accepted `app://` targets.
- Accept only the expected CDP WebSocket endpoint shapes.
- Do not expose arbitrary JavaScript evaluation to the UI.
- Do not permit marketplace themes to contain executable JavaScript.
- Keep injection payloads application-owned and signed with the release.
- Store user themes and runtime state with appropriate user-only permissions.
- Do not modify API keys, base URLs, provider settings, or unrelated Codex configuration.

## 14. Out of scope for the first release

- Windows runtime implementation
- User accounts and cloud synchronization
- Paid themes and payments
- Ratings, comments, likes, and creator dashboards
- Full drag-and-drop visual theme editor
- Arbitrary marketplace CSS or JavaScript execution
- Embedded or proxied Codex UI
- Production Chinese and Japanese translations
