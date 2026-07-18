import type { Theme } from "./theme";
import gothicPreview from "../../../docs/images/presets/gothic-void-crusade-preview.jpg";
import arinaPreview from "../../../docs/images/presets/arina-hashimoto-light.jpg";

export const themes: Theme[] = [
  {
    schemaVersion: 1,
    id: "preset-gothic-void-crusade",
    name: "Gothic Void Crusade",
    author: "Sean Song",
    version: "1.2.0",
    description: "A restrained gothic science-fiction atmosphere with a cinematic focal composition.",
    appearance: "dark",
    art: { focusX: 0.74, focusY: 0.46, safeArea: "left", taskMode: "ambient" },
    origin: "built-in",
    category: "Featured",
    previewUrl: gothicPreview,
    installed: true,
  },
  {
    schemaVersion: 1,
    id: "preset-arina-hashimoto",
    name: "Arina — Soft Studio",
    author: "Codex Themes",
    version: "1.0.0",
    description: "A quiet portrait-led workspace with balanced content safety areas for light mode.",
    appearance: "auto",
    art: { focusX: 0.72, focusY: 0.45, safeArea: "left", taskMode: "auto" },
    origin: "built-in",
    category: "Featured",
    previewUrl: arinaPreview,
    installed: true,
  },
];
