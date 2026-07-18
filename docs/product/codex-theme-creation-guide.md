# Codex Theme Creation Guide

You are creating an importable theme for the Codex Themes macOS application. Use the image attached to the same conversation as the visual source. Complete the work autonomously and return one ZIP archive that the user can download.

## Deliverable

Create a folder containing exactly:

```text
my-theme/
├── theme.json
├── background.jpg
└── preview.jpg
```

Compress that folder as `my-theme.zip` and provide it as the final downloadable artifact. Do not include scripts, executables, HTML, nested folders, symbolic links, or unrelated source files.

## Background image

- Treat the attached image as the subject and art-direction reference.
- Produce a clean background, not a screenshot or a mock interface.
- Remove readable text, controls, logos, watermarks, window chrome, and fake UI.
- Export an RGB JPEG named `background.jpg` in a 16:9 composition, preferably 2560 × 1440.
- Keep the main subject away from the area where Codex navigation and content need to remain readable.
- Preserve the source image's identity and visual intent; do not introduce unrelated subjects.
- Avoid extreme contrast directly behind expected text and controls.
- Keep the file below 16 MB.

Also create `preview.jpg`, a 16:9 preview of the same background, preferably 1280 × 720 and below 4 MB.

## Theme identifier

Choose a stable identifier between 2 and 64 characters using letters, numbers, hyphens, or underscores. The identifier must be the same as the uncompressed folder name. Do not reuse the identifier of another theme supplied in the conversation.

## theme.json

Create valid UTF-8 JSON with `schemaVersion` set to `1`. Use the exact generated image filename and choose colors by sampling and balancing the attached artwork.

```json
{
  "schemaVersion": 1,
  "id": "my-theme",
  "name": "My Theme",
  "author": "Theme Creator",
  "version": "1.0.0",
  "description": "A concise description of the atmosphere.",
  "image": "background.jpg",
  "preview": "preview.jpg",
  "appearance": "auto",
  "art": {
    "focusX": 0.72,
    "focusY": 0.45,
    "safeArea": "left",
    "taskMode": "ambient"
  },
  "colors": {
    "background": "#101312",
    "panel": "#191d1b",
    "panelAlt": "#222824",
    "accent": "#5f9f82",
    "accentAlt": "#82b99e",
    "secondary": "#526a5e",
    "highlight": "#79ad92",
    "text": "#f1f5f2",
    "muted": "#a3afa8",
    "line": "rgba(95, 159, 130, .28)"
  }
}
```

Allowed values:

- `appearance`: `auto`, `light`, or `dark`
- `safeArea`: `auto`, `left`, `right`, `center`, or `none`
- `taskMode`: `auto`, `ambient`, `banner`, or `off`
- `focusX` and `focusY`: numbers from `0` to `1`

## Final checks

Before returning the ZIP:

1. Parse `theme.json` and confirm it is valid JSON.
2. Confirm the folder name exactly matches `theme.json.id`.
3. Confirm both referenced images exist and are regular files.
4. Confirm the archive contains no absolute paths, `..` paths, links, scripts, or executable files.
5. Confirm the background is a clean image without baked-in Codex UI.
6. Return the ZIP and a short note telling the user to extract it and import the extracted folder into Codex Themes.
# Archived documentation copy

The guide exported by the application is maintained at
`desktop-app/resources/guides/codex-theme-creation-guide.md`.

Do not edit this archived documentation copy as a runtime resource.
