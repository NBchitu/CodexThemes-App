# Codex Themes v0.1.4 for macOS

## What changed

- Enables Codex window effects by default and adds a prominent quick control in the sidebar.
- Adds multiple animated border styles, including the new Electric edge effect, while preserving existing saved preferences.
- Uses separate light and dark pixel-cat sprite sheets with a slower continuous walk cycle.
- Restores the last selected theme after Codex is reopened and keeps theme switching responsive across App upgrades.
- Reduces theme verification timeouts and prevents stale `Launching...` state from surviving an App restart.
- Improves task-background visibility with lighter masks while retaining readable text surfaces.
- Strengthens `.codextheme` validation, import limits, and URL/image safety checks without changing the existing package format.
- Vendors the exact macOS injection engine used by the release so the GitHub source can reproduce the shipped App.

## Downloads

- Apple Silicon: `Codex-Themes-v0.1.4-macOS-Apple-Silicon-arm64.dmg`
- Intel Mac: `Codex-Themes-v0.1.4-macOS-Intel-x86_64.dmg`

This community release is ad-hoc signed and is not notarized by Apple.

## SHA-256

```text
1e1211cbf8235c5aab559b74f10e59bd381077e270c5e2d51c1582967e5d67b2  Codex-Themes-v0.1.4-macOS-Apple-Silicon-arm64.dmg
b1fc34e4a53fa2511bf5ff2cc7227cd8fcfcc1c2c31a697c9519708eb86d7347  Codex-Themes-v0.1.4-macOS-Intel-x86_64.dmg
```
