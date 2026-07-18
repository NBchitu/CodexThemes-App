# Codex Themes Desktop — Design Constraints

## 1. Design direction

The interface follows an **Editorial Utility** direction:

> A quiet, trustworthy macOS utility that presents expressive themes without competing with them.

The application shell must remain neutral and restrained. Theme previews provide visual personality and color. The manager itself should feel clean, reliable, and well crafted rather than like a gaming launcher or promotional wallpaper website.

## 2. Core principles

1. **Themes are the visual content.** Application chrome must not compete with preview art.
2. **One primary action per view.** Download, Apply, Update, or Import should be visually dominant according to context.
3. **Status must be trustworthy.** Active, connected, failed, and restart-required states reflect verified backend state.
4. **Use macOS conventions.** Prefer familiar sidebar, toolbar, settings, alerts, file picker, and menu bar patterns.
5. **Keep advanced controls progressive.** Common theme actions remain obvious; diagnostics and low-frequency actions stay secondary.
6. **Accessibility is a release requirement.** Keyboard, focus, contrast, motion preferences, and screen-reader semantics are designed from the beginning.

## 3. Application structure

### Window

- Use a standard resizable macOS window with native traffic-light controls.
- Recommended initial size: approximately `1180 × 760` logical pixels.
- Recommended minimum size: approximately `960 × 640` logical pixels.
- Respect macOS title-bar and safe-area insets.
- Do not use a frameless custom window for the first release.

### Sidebar

- Fixed navigation containing Discover, My Themes, Create, and Settings.
- Place Codex connection status near the bottom.
- Use a compact status dot, icon, and label instead of a permanent large status card.
- Selection uses a soft accent background and accent text/icon.

### Content

- Use clear page titles and restrained supporting copy.
- Use responsive two- or three-column theme grids depending on available width.
- Prefer dedicated detail pages over complex modal dialogs.
- Settings use grouped rows rather than a dashboard of cards.

## 4. Color system

Use a graphite neutral foundation with a cool green accent. Do not dynamically recolor the whole application from the active theme.

### Brand accent

| Token | Light | Dark | Usage |
|---|---:|---:|---|
| `accent` | `#087F5B` | `#34D399` | Primary action, selected state, link |
| `accent-hover` | `#066B4D` | `#6EE7B7` | Hover state |
| `accent-soft` | `#E6F4EF` | `#12372D` | Selected navigation and subtle status |
| `accent-contrast` | `#FFFFFF` | `#06281F` | Content placed on the accent |

The accent is an independent product color and must not imply that the application is an official OpenAI product.

### Light appearance

| Token | Value | Usage |
|---|---:|---|
| `app-background` | `#F5F5F4` | Window background |
| `sidebar-background` | `#ECECEA` | Sidebar |
| `surface` | `#FFFFFF` | Cards and panels |
| `surface-secondary` | `#FAFAF9` | Secondary regions |
| `surface-hover` | `#F0F0EE` | Hover background |
| `surface-selected` | `#E6F4EF` | Selected background |
| `border` | `#DDDCD8` | Standard border |
| `border-strong` | `#C8C7C2` | Input and emphasized border |
| `text-primary` | `#1C1C1B` | Headings and primary content |
| `text-secondary` | `#62625F` | Descriptions and authors |
| `text-tertiary` | `#8A8985` | Metadata and placeholders |

### Dark appearance

| Token | Value | Usage |
|---|---:|---|
| `app-background` | `#151515` | Window background |
| `sidebar-background` | `#1B1B1A` | Sidebar |
| `surface` | `#20201F` | Cards and panels |
| `surface-secondary` | `#262625` | Secondary regions |
| `surface-hover` | `#2C2C2A` | Hover background |
| `surface-selected` | `#12372D` | Selected background |
| `border` | `#343432` | Standard border |
| `border-strong` | `#494946` | Input and emphasized border |
| `text-primary` | `#F4F4F2` | Headings and primary content |
| `text-secondary` | `#ABABA6` | Descriptions and authors |
| `text-tertiary` | `#777773` | Metadata and placeholders |

Avoid pure white as the full light-mode background and pure black as the full dark-mode background.

### Semantic colors

| State | Light | Dark | Example |
|---|---:|---:|---|
| Success | `#17803D` | `#4ADE80` | Theme applied |
| Warning | `#B45309` | `#FBBF24` | Restart required |
| Error | `#C2413D` | `#FB7185` | Injection failed |
| Information | `#2563EB` | `#60A5FA` | Download or update information |
| Neutral | `#71717A` | `#A1A1AA` | Disconnected or paused |

Never rely on color alone. Pair semantic colors with an icon and visible text.

### Color usage ratio

- Approximately 80% neutral application surfaces.
- Approximately 15% theme preview imagery.
- No more than approximately 5% accent and semantic color.
- Limit each view to one accent color plus necessary semantic feedback.

## 5. Typography

Use the system font stack:

```css
font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", sans-serif;
```

- Use system sizing and weight conventions rather than decorative display fonts.
- Use balanced wrapping for headings and readable wrapping for body copy.
- Use tabular numerals for versions, file sizes, progress, and diagnostic data.
- Truncate or line-clamp dense card metadata.
- Do not customize letter spacing unless a future brand specification requires it.
- Body text must remain comfortably readable at the default macOS scaling.

## 6. Spacing, shape, and elevation

- Base spacing unit: `4px`; common gaps use `8`, `12`, `16`, `24`, and `32px`.
- Buttons and inputs: `6–8px` radius.
- Cards and panels: `10–12px` radius.
- Theme preview images: `8–10px` radius.
- Avoid pill-shaped controls except for genuine tags or compact segmented choices.
- Prefer thin borders and restrained default-scale shadows.
- Do not use glow effects as affordances.
- Do not use gradients in the application chrome.
- Avoid large persistent backdrop-blur surfaces.

## 7. Theme cards

- Use consistent 16:9 preview images.
- Place name and author below the preview.
- Keep status labels compact: Installed, Update Available, or Active.
- Do not overload cards with descriptions, ratings, tags, and multiple buttons.
- Hover changes border or neutral surface only; do not scale the preview image.
- The active theme uses an accent border or small accent status treatment, not a fully green card.
- Card click opens a dedicated detail page.

## 8. Buttons and actions

### Primary

- Use the accent fill for the single main action on a view.
- Examples: Download, Apply Theme, Import Theme, Download Starter Template.
- Ensure sufficient contrast for both appearances.

### Secondary

- Use a neutral surface or transparent background with a standard border.
- Use for View Files, Cancel, Refresh, and similar supporting actions.

### Destructive

- Use red sparingly.
- Normal delete entry points may use red text or an icon.
- Use a red filled button only for the final destructive confirmation.
- Always use an accessible alert dialog for destructive or irreversible actions.

**Restore Original Appearance** is a recovery action, not theme deletion. Present it as a secondary action and explain its effect in the confirmation flow.

## 9. Interaction and feedback

- Display errors beside the action or field that caused them.
- Use structural skeletons that resemble the final marketplace layout.
- Every empty state has one clear next action.
- Show download and apply progress without blocking unrelated navigation when safe.
- Do not mark a theme active until backend verification succeeds.
- A concise toast may confirm success, for example: `Theme applied to Codex`.
- Do not block paste in text inputs or text areas.
- Icon-only buttons require accessible labels and tooltips where helpful.
- Use accessible component primitives for focus, keyboard, dialogs, menus, and popovers; do not reimplement these behaviors manually.

## 10. Motion

The first release does not require decorative motion.

- Interaction feedback must complete within `200ms`.
- Animate only `transform` and `opacity` when animation is necessary.
- Prefer `ease-out` for entrances.
- Respect `prefers-reduced-motion`.
- Do not animate large preview images, backgrounds, blur, layout dimensions, or full-window surfaces.
- Avoid bouncing cards, floating backgrounds, continuous glow, and celebration animations.

## 11. Import and validation UI

The import surface supports a folder picker and drag-and-drop.

Validation feedback is presented as a readable checklist, for example:

```text
Theme validation

✓ theme.json found
✓ Preview image valid
✓ Background image valid
! Author field is missing
× Unsupported taskMode value
```

- Use semantic icons, text, and color together.
- State the exact file and field when possible.
- Provide a concrete correction for every blocking issue.
- Keep the user's selected folder visible while corrections are needed.

## 12. Create tutorial UI

- Present a linear numbered guide, not a fake visual editor.
- Use one primary action at the top: **Download Starter Template**.
- Include a real folder-structure example and field documentation.
- Allow the user to validate a completed theme from the tutorial.
- Keep long reference content readable and searchable.

## 13. Settings UI

- Use grouped macOS-style setting rows.
- Do not turn each setting into an oversized dashboard card.
- Separate **Launch at Login** from actions that launch or restart Codex.
- Mark unavailable languages as **Coming soon** without making them selectable.
- Keep diagnostics and restore-default actions in clearly labeled sections.

## 14. Accessibility

- Meet WCAG AA contrast for text and essential controls.
- Support full keyboard navigation and visible focus indicators.
- Preserve native macOS focus and menu behavior where possible.
- Provide meaningful accessibility names for images, icons, controls, and statuses.
- Do not encode status only in color or position.
- Support text scaling without clipping core actions.
- Respect reduced-motion and system appearance preferences.
- Keep touch/click targets appropriately sized for desktop use.

## 15. Localization constraints

- English is the first shipped language.
- All user-facing strings use localization keys from the start.
- Layout must accommodate longer Simplified Chinese, Japanese, and English strings without fixed-width assumptions.
- Do not place important user-facing text inside raster images.
- Theme titles and author names must support Unicode.

## 16. Explicit visual prohibitions

- No purple or multicolor gradients.
- No decorative gradients in application chrome.
- No neon glow as a primary affordance.
- No excessive glassmorphism.
- No image scaling on card hover.
- No dynamically recolored global UI based on the active theme.
- No multiple competing primary buttons in one view.
- No oversized status cards for routine connection state.
- No arbitrary CSS or JavaScript supplied by marketplace themes.

