import { describe, expect, it } from "vitest";
import { themeManifestSchema, validateThemeManifest } from "./theme";

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
});
