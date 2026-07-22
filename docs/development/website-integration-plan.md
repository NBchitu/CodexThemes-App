# CodexThemes.app integration baseline

The canonical cross-product implementation plan is maintained in the website repository at:

```text
docs/CodexThemes_App_Integration_Development_Plan.md
```

## Confirmed boundary

Codex Themes Desktop owns theme import, local management, application, live verification, and restoration. The historical `Codex-Dream-Skin` repository is not a user-facing dependency or download target. The macOS runtime bundled inside this repository is an application implementation detail retained for compatibility while its behavior is migrated behind native adapters.

## Package contract

The public `codextheme-v1` file is a ZIP archive with exactly two regular root files:

```text
theme.json
background.jpg
```

It contains no scripts, executables, HTML, links, nested folders, website previews, or marketing content. The App creates the managed directory from the validated `theme.json.id`; it never trusts an archive directory name.

The website repository owns the machine-readable JSON Schema. This App enforces the equivalent runtime contract with `codexThemePackageManifestSchema` in `src/domain/theme.ts`. Changes to either contract require synchronized tests in both repositories.

## Delivery order

1. Safe Rust import for `.codextheme` archives. (Implemented; public packages are not published yet.)
2. Publisher Studio package generation and validation. (Implemented.)
3. Website download to local `.codextheme`, macOS file association, confirmation, import, and Apply. (Implemented.)
4. Versioned App release manifest and website macOS download page. (Implemented.)
5. Publish and manually accept the first reviewed R2 theme package on Apple Silicon and Intel.
6. In-App remote catalog/download/update remains deferred and is not required for the website-to-App loop.

Until step 5 passes end-to-end acceptance, the homepage must not claim one-click App installation. Theme detail pages expose the App download path only for records that actually exist in the generated App package index.

### File-open acceptance

- Finder opening a `.codextheme` must reach the same confirmation flow whether the App is already running or cold-started. Native open requests are queued until the webview explicitly consumes them; an early event must never be the only copy of the path.
- A new theme uses a short Import/Cancel confirmation. If its validated theme ID already exists locally, the dialog must say so and offer Replace/Cancel.
- Replacement uses staging plus backup/rollback. A failed replacement must leave the previously installed theme intact.

### Codex version compatibility

- Theme application does not use an App-version allowlist. Compatibility is evaluated in layers: official code identity, signed bundled Node availability, loopback CDP startup, renderer target identity, payload injection, and post-injection verification.
- Whole-bundle code-signature validation remains the preferred path. For official releases whose nested Dock plugin cannot be read across macOS user accounts, fallback validation may ignore the resource seal only for the exact main Mach-O executable; it must still enforce the OpenAI Developer ID requirement and Team ID, while the bundled Node runtime remains independently signature-validated.
- A successful signature preflight does not prove UI compatibility. Each newly observed Codex version must still pass Home, Chat, Diff, theme switching, restoration, and restart acceptance because renderer selectors and DOM structure can change independently of CDP availability.
