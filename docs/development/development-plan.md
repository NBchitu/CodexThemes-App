# Codex Themes Desktop — Development Plan

## 1. Objective

Build a production-oriented macOS desktop theme manager for Codex, using the existing loopback CDP runtime without modifying the official Codex application. The first product release is English-only, while all user-facing copy is localization-ready. Windows remains an explicit platform adapter boundary and is not implemented until macOS is stable.

## 2. Delivery strategy

Development is divided into vertical milestones. Each milestone must leave the repository buildable and must not claim runtime success unless the underlying operation is verified.

### Milestone 0 — Product and engineering baseline

Deliverables:

- Functional specification
- Design constraints and color tokens
- This implementation plan
- Desktop application directory isolated from the existing script runtimes
- Clear definitions for theme packages, platform adapters, and operation results

Acceptance criteria:

- Product scope distinguishes first-release features from later work.
- macOS and Windows code do not leak into product-level domain contracts.
- Security constraints explicitly forbid arbitrary marketplace JavaScript.

### Milestone 1 — Interactive frontend foundation

Deliverables:

- React, TypeScript, Vite, and Tailwind application
- Responsive macOS-style application shell
- Discover, My Themes, Create, and Settings destinations
- Light and dark appearance support
- English localization dictionary and typed translation keys
- Accessible navigation, buttons, forms, menus, and status semantics

Acceptance criteria:

- Production frontend build succeeds.
- Primary navigation is keyboard accessible.
- Minimum layout remains usable at `960 × 640`.
- No global theme-preview color is applied to application chrome.
- Empty, loading, success, warning, and error states have defined treatments.

### Milestone 2 — Theme domain and local persistence

Deliverables:

- Versioned theme manifest schema
- Built-in, marketplace, and imported theme origin model
- Managed theme library service
- Import validation results with actionable field-level messages
- Active-theme and application-preference persistence
- Safe archive/folder ingestion design

Acceptance criteria:

- Invalid appearance, focus, safe-area, and task-mode values are rejected.
- Imported content is copied into managed storage instead of referenced in place.
- Active theme is committed only after the platform adapter reports verification.
- A failed update preserves the previous installed version.

### Milestone 3 — macOS platform integration

Deliverables:

- Tauri 2 application shell
- Rust macOS platform adapter
- Codex discovery and signature/identity verification
- Existing macOS CDP injector integration as a managed sidecar during migration
- Apply, switch, verify, and restore commands
- Single-instance behavior and menu bar controls
- Structured diagnostics and privacy-reviewed report export

Acceptance criteria:

- CDP binds only to loopback.
- The selected port is available and verified as belonging to Codex.
- A running non-CDP Codex instance requires explicit restart confirmation.
- Theme application succeeds only after exact revision verification.
- Restore stops the managed watcher and returns Codex to the original appearance.
- The application is signed and notarized for test distribution.

### Milestone 4 — Marketplace delivery

Deliverables:

- Remote versioned marketplace index
- Search and category filters
- Download, integrity verification, atomic install, and update
- Offline cache and retry behavior
- Theme detail metadata and asset-rights notices

Acceptance criteria:

- Invalid archives cannot escape managed temporary storage.
- Executable files and unsupported payloads are rejected.
- Interrupted downloads do not create installed themes.
- The last valid marketplace cache remains usable offline.

### Milestone 5 — Release readiness

Deliverables:

- Unit tests for manifests, validation, state transitions, and localization
- Integration tests for platform command result mapping
- Manual macOS acceptance inventory
- Accessibility and keyboard audit
- Update channel and rollback procedure
- DMG release artifact, release notes, licenses, and privacy statement

Acceptance criteria:

- Clean install, upgrade, apply, hot switch, cold restart, restore, and uninstall flows pass.
- Existing Codex configuration unrelated to appearance remains unchanged.
- Application failure never leaves an untracked injection watcher.
- Logs contain no theme image data, API keys, conversations, or unrelated Codex content.

## 3. First implementation increment

The first saved implementation in this repository covers Milestones 0 and 1 plus the platform-neutral portion of Milestone 2:

- Complete interactive English UI
- In-memory theme marketplace and library fixtures
- Theme application state machine through a typed platform bridge
- Browser preview adapter used when the native Tauri host is unavailable
- Preference persistence abstraction
- Theme manifest and validation types
- macOS bridge contract ready for Tauri command wiring

Native Tauri compilation is deferred until a Rust toolchain is available in the development environment. The frontend must not pretend that the browser preview adapter has performed a real CDP injection.

## 3.1 Prioritized MVP backlog — local gallery model

The macOS MVP uses a deliberately small distribution model: six user-curated themes currently ship with the application, the public theme website handles discovery and ZIP downloads, and the desktop application imports the extracted folder into managed storage. A remote marketplace API, in-app downloads, accounts, reviews, and payments are not part of this MVP.

### P0 — usable local theme manager

- [x] Create `~/Library/Application Support/CodexDreamSkinStudio/` and its managed subdirectories on first launch with user-only permissions.
- [x] Seed every missing bundled `preset-*` directory into `themes/` without overwriting an existing copy.
- [x] Scan the managed theme library at startup and expose real disk-backed themes to the UI.
- [x] Replace in-memory installed state with the native library result.
- [x] Add a native macOS folder picker.
- [x] Validate an extracted local theme folder: manifest, identifier, referenced image, file types, sizes, path containment, and symbolic links.
- [x] Copy a valid import through a private staging directory and publish it atomically.
- [x] Refresh My Themes immediately after import and allow the imported theme to be applied.
- [x] Keep built-in themes directly applicable; never show a download action for them.
- [x] Reduce renderer overhead by filtering non-structural DOM mutations, debouncing route scans, lowering fallback frequency, backing off disconnected CDP discovery, and disabling blur on frequently repainted surfaces.

### P1 — complete the website-assisted MVP

- [x] Replace the simulated marketplace grid with a focused gallery landing surface.
- [x] Open the official theme website in the user's default browser from a fixed trusted URL.
- [x] Add UTM attribution to desktop-app gallery visits without exposing user-specific identifiers.
- [x] Explain the download, extract, import, and apply flow in the Discover view.
- [x] Replace the Create view with a visual three-step Codex-assisted creation flow, a downloadable Markdown guide, one-click message copy, reference images, and import-then-apply behavior.
- [x] Add safe deletion for inactive imported themes and an Open Theme Folder action.
- [ ] Add diagnostics, local logs, copyable privacy-reviewed reports, and recovery guidance.
- [ ] Connect Launch at Login, start minimized, and restore-last-theme preferences to native macOS behavior.
- [x] Add a persistent, tab-independent Launch Codex action that restores the selected theme through the verified CDP launch path after a full quit.
- [x] Move the canonical creation guide and reference image into app-owned resource directories.
- [x] Use My Themes as the default destination and visually separate the orange Launch Codex action from green theme actions.
- [x] Bundle the six themes selected from the managed system library without overwriting existing user copies.
- [x] Present app feedback at the top of the content area with concise, user-facing copy.
- [x] Confirm deletion of inactive imported themes with an accessible destructive dialog.

### P2 — public macOS release

- [ ] Add the native menu-bar experience and application updater.
- [ ] Complete Developer ID signing, hardened runtime, notarization, stapling, DMG presentation, Universal Binary output, and clean-account acceptance testing.
- [ ] Finish English localization-key coverage, then add Simplified Chinese and Japanese.

### Deferred until after macOS stability

- Windows platform adapter and installer.
- Direct ZIP import and in-app package downloads.
- Remote marketplace index, update service, favorites, accounts, ratings, comments, payments, creator dashboard, and cloud sync.

## 4. Proposed source architecture

```text
desktop-app/
├── docs/
├── src/
│   ├── app/              # application composition and navigation
│   ├── components/       # reusable accessible UI
│   ├── features/
│   │   ├── discover/
│   │   ├── library/
│   │   ├── create/
│   │   └── settings/
│   ├── domain/           # theme and runtime contracts
│   ├── services/         # marketplace, persistence, platform bridge
│   ├── i18n/             # localization dictionaries
│   └── styles/
└── src-tauri/            # added when the Rust toolchain is enabled
```

## 5. Theme application state machine

```text
idle
  → validating
  → restart-required → awaiting-confirmation
  → launching
  → injecting
  → verifying
  → active

Any non-terminal stage → failed
active → restoring → original
```

The UI must render backend state rather than infer success from elapsed time.

## 6. Testing plan

### Unit tests

- Theme manifest validation
- Theme origin and identifier handling
- Appearance, focus, safe-area, and task-mode boundaries
- Runtime state transitions
- Localization key completeness
- Preference serialization

### Frontend tests

- Primary navigation
- Search and category filtering
- Download/apply button state
- Restart confirmation presentation
- Settings persistence
- Import validation feedback

### Native integration tests

- Codex not installed
- Codex running without CDP
- Port conflict
- CDP target identity mismatch
- Renderer navigation and reinjection
- Injection verification failure
- Restore with and without a live verified session
- Watcher crash and application restart recovery

### Manual acceptance

- Clean macOS install
- Light and dark appearances
- Keyboard-only operation
- VoiceOver labels and reading order
- Reduced motion
- Offline marketplace cache
- Login item behavior
- Signed/notarized launch on a clean test account

## 7. Implementation rules

- Use `apply_patch` for authored source changes.
- Preserve the existing `macos/` and `windows/` runtimes until replacements pass parity tests.
- Do not expose arbitrary CDP evaluation through the renderer UI.
- Do not report a theme as active before verification.
- Keep Windows-specific implementation out of the first macOS release while retaining adapter interfaces.
- Every milestone ends with build/test evidence and an updated delivery-status document.
