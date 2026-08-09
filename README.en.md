# Codex Themes Desktop: Codex App Theme Manager

<p align="center">
  <strong>Discover, download, import, switch, create, and restore custom themes for Codex App / Codex Desktop.</strong><br>
  Codex themes · Codex App themes · Codex theme manager · Codex Dream Skin
</p>

<p align="center">
  <a href="./README.md">中文</a> · <strong>English</strong>
</p>

<p align="center">
  <a href="https://codexthemes.app/">CodexThemes.app</a> ·
  <a href="https://codexthemes.app/themes">Browse Codex themes</a> ·
  <a href="https://codexthemes.app/desktop-app">Download Codex Themes Desktop</a> ·
  <a href="https://codexthemes.app/guides/how-to-install-codex-theme">Installation guide</a> ·
  <a href="https://codexthemes.app/codex-cli-themes">Codex CLI themes</a> ·
  <a href="#download-codex-themes-desktop">Download</a> ·
  <a href="#quick-start">Quick start</a>
</p>

> Codex Themes Desktop is an independent open-source community project. It is not affiliated with, endorsed by, sponsored by, or approved by OpenAI. Codex and related trademarks belong to their respective owners.

**Codex Themes Desktop** is an open-source theme manager for Codex App / Codex Desktop. It helps developers discover Codex themes, download theme packages, import local themes, switch saved themes, create custom Codex Dream Skin-style themes, and restore the official appearance when needed. It works with the [CodexThemes.app](https://codexthemes.app/) theme gallery for users who want a more personal Codex desktop workspace.

The app loads themes through Chrome DevTools Protocol (CDP) on the local loopback address only. It does not modify the official Codex app bundle, `app.asar`, WindowsApps, or code signature.

## Codex Theme Links

| Goal | Recommended page |
| --- | --- |
| Find ready-made Codex App themes | [Browse Codex themes](https://codexthemes.app/themes) |
| Download the Codex desktop theme manager | [Codex Themes Desktop download page](https://codexthemes.app/desktop-app) |
| Learn how to install a Codex theme | [Codex theme installation guide](https://codexthemes.app/guides/how-to-install-codex-theme) |
| Customize Codex CLI colors | [Codex CLI themes](https://codexthemes.app/codex-cli-themes) |
| Read common answers | [Codex Themes FAQ](https://codexthemes.app/faq) |

## Keyword Focus

This repository targets: `Codex themes`, `Codex App themes`, `Codex Desktop themes`, `Codex theme manager`, `custom Codex themes`, `Codex Dream Skin`, `Codex CLI themes`, `Codex skin`, and `OpenAI Codex themes`.

## Download Codex Themes Desktop

> **Latest listed version: v0.1.5.** Choose the DMG that matches your Mac chip.

| System and chip | Download | Status |
| --- | --- | --- |
| macOS · Apple Silicon (M1 / M2 / M3 / M4 and later Apple chips) | **[Download Apple Silicon DMG](https://github.com/NBchitu/CodexThemes-App/releases/latest/download/Codex-Themes-v0.1.5-macOS-Apple-Silicon-arm64.dmg)** | Released |
| macOS · Intel | **[Download Intel DMG](https://github.com/NBchitu/CodexThemes-App/releases/latest/download/Codex-Themes-v0.1.5-macOS-Intel-x86_64.dmg)** | Released |
| Windows · x64 | Not available yet | In active development |

Not sure which Mac you have? Open **Apple menu -> About This Mac**. If you see “Chip Apple M…”, use the Apple Silicon build. If you see “Processor Intel…”, use the Intel build.

You can also visit [GitHub Releases](https://github.com/NBchitu/CodexThemes-App/releases) for all versions, release notes, and SHA-256 checksums. The current community build uses ad-hoc integrity signing and is not Apple-notarized yet. If macOS blocks the first launch, open **System Settings -> Privacy & Security** and choose **Open Anyway**.

v0.1.5 supports the newer Codex 26.727 interface structure and remains compatible with older Codex builds that use `main.main-surface`. It also fixes cases where theme colors applied but the background image did not appear, runtime status was misreported, or the theme did not restore automatically after reopening Codex.

## Screenshots

### App overview and online theme gallery

Codex Themes Desktop uses a unified sidebar for theme discovery, theme management, theme creation, and settings. It can open the online Codex themes gallery in one click.

![Codex Themes Desktop app overview and online theme gallery](https://raw.githubusercontent.com/NBchitu/CodexThemes-App/main/docs/screenshots/codex-themes-desktop-overview.png)

### Local theme management

“My Themes” shows bundled themes and imported local themes. You can open the managed theme directory, import an extracted theme folder, and apply a theme to Codex.

![Codex Themes Desktop local theme library](https://raw.githubusercontent.com/NBchitu/CodexThemes-App/main/docs/screenshots/codex-themes-theme-library.png)

### Theme creation guide

The “Create” page turns theme creation into three steps: prepare an image and guide, send them to Codex, then import and apply the generated theme package.

![Codex Themes Desktop theme creation guide](https://raw.githubusercontent.com/NBchitu/CodexThemes-App/main/docs/screenshots/codex-themes-create-theme-guide.png)

## What the Codex Theme Manager Does

- Browse themes from the [CodexThemes.app theme gallery](https://codexthemes.app/).
- Import extracted local theme folders and validate their basic structure.
- View bundled and imported themes in “My Themes”.
- Apply a theme and verify that it actually takes effect in the native environment.
- Restore the official Codex appearance while keeping downloaded and imported themes.
- Export a theme creation guide and use Codex to create a new theme from an image.
- Switch the app itself between light, dark, and system appearance modes.

The current release focuses on macOS, with Apple Silicon and Intel builds available. The Windows build is in active development. Chinese localization, Japanese localization, auto-update, signing, and notarized distribution are planned or in preparation. See the [delivery status](docs/development/delivery-status.md).

## Why Use Codex Themes Desktop

| Problem | How Codex Themes Desktop handles it |
| --- | --- |
| You want Codex App themes without modifying the official app package | It loads an external theme layer through local-loopback CDP without changing `.app`, `app.asar`, or code signature |
| You want to manage several Codex Desktop themes | “My Themes” lets you view, import, switch, and restore themes |
| You want ready-made themes from a website | The Discover page opens the CodexThemes.app theme gallery |
| You want to create a Codex theme from your own image | The Create page exports a guide you can send to Codex with your image |
| You worry about broken themes after a Codex update | The Settings page includes official appearance restore and runtime status guidance |

## Quick Start

### 1. Install and open

Download the DMG for your Mac chip from this README or from [GitHub Releases](https://github.com/NBchitu/CodexThemes-App/releases). Do not run repackaged builds from unknown sources. Open the DMG and drag Codex Themes into Applications.

After the first launch, the sidebar shows four main areas:

- **Discover**: Open the official CodexThemes.app gallery and import downloaded themes.
- **My Themes**: Manage local themes.
- **Create**: Follow a guided flow for creating a theme.
- **Settings**: Change appearance, startup preferences, and restore the official Codex look.

### 2. Install a theme from the gallery

1. Open **Discover** and click **Browse theme gallery**.
2. Your browser opens [https://codexthemes.app/](https://codexthemes.app/).
3. Pick a theme and download its ZIP package.
4. Double-click the ZIP in Finder to extract it into a folder.
5. Return to the app and click **Import extracted theme**.
6. Select the extracted theme folder.
7. After import, open **My Themes**, open the theme detail, and click **Apply**.

A theme folder usually looks like this:

```text
my-theme/
├── theme.json       # required theme manifest
├── background.jpg   # required background image
├── preview.jpg      # recommended gallery or library preview
└── README.md        # optional author notes
```

If import fails, make sure you selected the extracted folder, not the ZIP file and not only `theme.json`.

### 3. Switch themes

1. Open **My Themes**.
2. Select a theme.
3. Click **Apply**.
4. If Codex needs to restart, save any unsent input before continuing.
5. Wait for verification. The app only marks the theme as current after verification succeeds.

If switching fails, the app tries to keep the previous working theme. Follow the error message and do not treat browser preview mode as proof that the theme has been applied to Codex.

### 4. Restore the official Codex appearance

1. Open **Settings**.
2. Find **Original appearance**.
3. Click restore and confirm.
4. The app stops the managed theme injection and may restart Codex if needed.

Restoring the official look does not delete your theme files. You can still use them again from **My Themes**.

## Create Your Own Codex Theme

The in-app **Create** page breaks the flow into three steps:

1. Prepare a JPG, PNG, or WebP image and click **Save creation guide**.
2. Send the image and guide to Codex, then paste the instruction text provided by the app.
3. Download the generated theme ZIP, extract it, and click **Import extracted theme**.

After a successful import, the app copies the theme into the managed theme directory and tries to apply it. Before sharing a theme publicly, confirm that you have rights to use and redistribute the background image, likeness, fonts, logos, and other assets.

See the [theme creation guide](docs/product/codex-theme-creation-guide.md) for field details and safety requirements.

For SEO and backlink publishing, see the [Codex Themes Desktop backlink and README SEO playbook](docs/seo-backlink-playbook.en.md).

## FAQ

### What is Codex Themes Desktop?

Codex Themes Desktop is an open-source theme manager for Codex App / Codex Desktop. It helps users discover and download themes from CodexThemes.app, import local theme folders, switch Codex Desktop themes, create custom themes, and restore the official appearance.

### Is Codex Themes Desktop an official OpenAI product?

No. Codex Themes Desktop is an independent community project and is not affiliated with, endorsed by, sponsored by, or approved by OpenAI. It does not modify the official app bundle, `app.asar`, WindowsApps, or code signature.

### How is Codex Themes Desktop different from Codex CLI themes?

Codex Themes Desktop is for visual backgrounds, theme packages, theme galleries, and local skin workflows in the Codex desktop app. Codex CLI themes are color themes for the terminal Codex CLI experience. See [Codex CLI themes](https://codexthemes.app/codex-cli-themes) for CLI-specific guidance.

### What image should I use for a custom Codex theme?

Use a UI-free, text-free 16:9 JPG, PNG, or WebP image. Screenshots, interface mockups, images with logos, and images with baked-in text are not suitable as importable theme backgrounds.

### Which page should backlinks point to?

For the overall project, link to [CodexThemes.app](https://codexthemes.app/). For ready-made themes, link to [Browse Codex themes](https://codexthemes.app/themes). For the app download, link to [Codex Themes Desktop](https://codexthemes.app/desktop-app). For tutorials, link to the [Codex theme installation guide](https://codexthemes.app/guides/how-to-install-codex-theme).

## Privacy and Safety

- Theme runtime uses the local loopback address and should not listen on a public network address.
- The app does not modify the official Codex installation package or code signature.
- Theme import checks the manifest and file structure and rejects unsupported executable content.
- Download themes only from trusted sources, and remove secrets, private conversations, and local paths before sharing diagnostics.
- Do not run untrusted local programs while CDP is enabled.

## Local Development

### Requirements

- macOS (the native theme bridge currently supports macOS only)
- Node.js 20 or newer
- npm
- Rust stable and Cargo
- Xcode Command Line Tools
- macOS dependencies required by Tauri 2

Install dependencies and start the frontend preview:

```bash
npm install
npm run dev
```

Open `http://127.0.0.1:1420`. Browser mode is for UI development only. It cannot import, apply, or restore real Codex themes.

Start the native Tauri development app:

```bash
npm run tauri -- dev
```

Run automated checks:

```bash
npm test
npm run build
```

Build the macOS app:

```bash
npm run tauri -- build --bundles app
```

> The current native packaging setup still reads managed macOS theme engine resources from the parent workspace. When this repository is cloned standalone, frontend development and tests work, but native app packaging requires resource independence first. This is a known limitation of the current open-source version and should not be treated as a publishable build without the missing resources.

## Tech Stack and Project Structure

- Tauri 2 + Rust: native window and restricted platform commands
- React 19 + TypeScript: app UI
- Vite 6: development and frontend build
- Tailwind CSS 4: styling system
- Zustand: local app state
- Zod: theme manifest validation
- Vitest: unit tests

```text
src/                  React app, theme domain model, and platform bridge
src-tauri/            Tauri/Rust native host
resources/            Guide resources used by the app
docs/product/         Product and theme creation specs
docs/development/     Development plan and delivery status
docs/design/          Visual constraints
docs/screenshots/     README screenshots and placeholders
```

## Contributing

Issues and pull requests are welcome. Before changing the app, read the [functional spec](docs/product/functional-spec.md) and [development plan](docs/development/development-plan.md), and keep these principles:

- Do not fake successful theme application; the native bridge must verify the result.
- Do not modify the official Codex app package, `app.asar`, or signature.
- Do not add images, secrets, conversation content, or private local paths to logs or diagnostics.
- New features should include tests, and `npm test` plus `npm run build` should pass.

## Credits

Thanks to these open-source projects and maintainers for their exploration of the Codex theme and skin tooling ecosystem:

- [Fei-Away/Codex-Dream-Skin](https://github.com/Fei-Away/Codex-Dream-Skin)
- [Finderchangchang/codex-autoskin](https://github.com/Finderchangchang/codex-autoskin)

Please respect each project's license, copyright notices, and usage boundaries. This credit is an acknowledgment of open-source contributions and does not imply an official partnership, endorsement, or approval from those projects or OpenAI.

## License and Disclaimer

Source code is released under the [MIT License](LICENSE). Third-party dependencies and theme assets may have their own licenses. The MIT License does not automatically grant rights to images, likenesses, trademarks, or branded assets.

Theme gallery: [https://codexthemes.app/](https://codexthemes.app/)
