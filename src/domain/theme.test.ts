import { describe, expect, it } from "vitest";
import {
  codexThemePackageManifestSchema,
  themeManifestSchema,
  validateCodexThemePackageManifest,
  validateThemeManifest,
} from "./theme";

const validManifest = {
  schemaVersion: 1,
  id: "quiet-studio",
  name: "Quiet Studio",
  author: "Codex Themes",
  version: "1.0.0",
  description: "A test theme.",
  appearance: "auto",
  art: { focusX: 0.72, focusY: 0.45, safeArea: "left", taskMode: "ambient" },
} as const;

const validPackageManifest = {
  ...validManifest,
  id: "preset-quiet-studio",
  image: "background.jpg",
  brandSubtitle: "CODEX THEMES",
  tagline: "A quiet workspace.",
  projectPrefix: "Project · ",
  projectLabel: "Choose project",
  statusText: "READY",
  quote: "Focus on the work",
  colors: {
    background: "#101010", panel: "#181818", panelAlt: "#202020",
    accent: "#75a68b", accentAlt: "#9ac4aa", secondary: "#896f5e",
    highlight: "#d8eadf", text: "#f4f4f4", muted: "#a0a0a0",
    line: "rgba(117, 166, 139, .24)",
  },
  promoTitle: "Quiet Studio",
  promoSub: "CodexThemes.app",
  promoUrl: "https://codexthemes.app/themes/quiet-studio",
} as const;

describe("theme manifest", () => {
  it("accepts the supported theme contract", () => {
    expect(themeManifestSchema.safeParse(validManifest).success).toBe(true);
  });

  it("rejects focus positions outside the normalized range", () => {
    const result = themeManifestSchema.safeParse({
      ...validManifest,
      art: { ...validManifest.art, focusX: 1.2 },
    });
    expect(result.success).toBe(false);
  });

  it("rejects unsupported executable-style configuration fields", () => {
    const result = themeManifestSchema.strict().safeParse({
      ...validManifest,
      script: "arbitrary.js",
    });
    expect(result.success).toBe(false);
  });

  it("returns actionable validation paths", () => {
    const items = validateThemeManifest({ ...validManifest, appearance: "neon" });
    expect(items[0]?.level).toBe("error");
    expect(items[0]?.message).toContain("appearance");
  });

  it("accepts the complete codextheme-v1 runtime contract", () => {
    expect(codexThemePackageManifestSchema.safeParse(validPackageManifest).success).toBe(true);
    expect(validateCodexThemePackageManifest(validPackageManifest)[0]?.level).toBe("success");
  });

  it("rejects executable fields and non-canonical background paths", () => {
    expect(codexThemePackageManifestSchema.safeParse({...validPackageManifest, image: "../background.jpg"}).success).toBe(false);
    expect(codexThemePackageManifestSchema.safeParse({...validPackageManifest, script: "install.sh"}).success).toBe(false);
  });
});
