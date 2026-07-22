import { z } from "zod";

export const themeManifestSchema = z.object({
  schemaVersion: z.literal(1),
  id: z.string().regex(/^[A-Za-z0-9_-]{2,64}$/),
  name: z.string().min(1).max(80),
  author: z.string().min(1).max(80),
  version: z.string().regex(/^\d+\.\d+\.\d+$/),
  description: z.string().max(320),
  appearance: z.enum(["auto", "light", "dark"]),
  art: z.object({
    focusX: z.number().min(0).max(1),
    focusY: z.number().min(0).max(1),
    safeArea: z.enum(["auto", "left", "right", "center", "none"]),
    taskMode: z.enum(["auto", "ambient", "banner", "off"]),
  }).strict(),
});

const runtimeColorsSchema = z.object({
  background: z.string().min(1).max(64),
  panel: z.string().min(1).max(64),
  panelAlt: z.string().min(1).max(64),
  accent: z.string().min(1).max(64),
  accentAlt: z.string().min(1).max(64),
  secondary: z.string().min(1).max(64),
  highlight: z.string().min(1).max(64),
  text: z.string().min(1).max(64),
  muted: z.string().min(1).max(64),
  line: z.string().min(1).max(64),
}).strict();

/** Public .codextheme v1 contract shared with codexthemes.app. */
export const codexThemePackageManifestSchema = themeManifestSchema.extend({
  id: z.string().regex(/^preset-[a-z0-9]+(?:-[a-z0-9]+)*$/).max(64),
  image: z.literal("background.jpg"),
  brandSubtitle: z.string().max(120),
  tagline: z.string().max(240),
  projectPrefix: z.string().max(80),
  projectLabel: z.string().max(80),
  statusText: z.string().max(120),
  quote: z.string().max(240),
  colors: runtimeColorsSchema,
  promoTitle: z.string().max(160),
  promoSub: z.string().max(160),
  promoUrl: z.string().url().startsWith("https://"),
}).strict();

export type ThemeManifest = z.infer<typeof themeManifestSchema>;
export type CodexThemePackageManifest = z.infer<typeof codexThemePackageManifestSchema>;
export type ThemeOrigin = "built-in" | "marketplace" | "imported";
export type ThemeCategory = "Featured" | "Minimal" | "Illustration" | "Cinematic";

export interface Theme extends ThemeManifest {
  origin: ThemeOrigin;
  category: ThemeCategory;
  previewUrl: string;
  previewPath?: string;
  installed: boolean;
  updateAvailable?: boolean;
}

export interface ThemeSettings {
  appearance: ThemeManifest["appearance"];
  taskMode: ThemeManifest["art"]["taskMode"];
  safeArea: ThemeManifest["art"]["safeArea"];
}

export interface ThemeLibraryResult {
  themes: Theme[];
  message: string;
  importedThemeId?: string | null;
}

export interface CodexThemePackageSummary {
  path: string;
  id: string;
  name: string;
  author: string;
  version: string;
  description: string;
  alreadyInstalled: boolean;
}

export type RuntimeStatus =
  | "preview"
  | "connected"
  | "restart-required"
  | "applying"
  | "active"
  | "restoring"
  | "error";

export interface RuntimeSnapshot {
  status: RuntimeStatus;
  activeThemeId: string | null;
  message: string;
  isNativeHost: boolean;
}

export interface OperationResult {
  ok: boolean;
  verified: boolean;
  status: RuntimeStatus;
  message: string;
}

export interface ThemeValidationItem {
  level: "success" | "warning" | "error";
  message: string;
}

export function validateThemeManifest(value: unknown): ThemeValidationItem[] {
  const result = themeManifestSchema.safeParse(value);
  if (result.success) {
    return [
      { level: "success", message: "theme.json is valid" },
      { level: "success", message: "Appearance and artwork settings are supported" },
    ];
  }
  return result.error.issues.map((issue) => ({
    level: "error" as const,
    message: `${issue.path.join(".") || "theme.json"}: ${issue.message}`,
  }));
}

export function validateCodexThemePackageManifest(value: unknown): ThemeValidationItem[] {
  const result = codexThemePackageManifestSchema.safeParse(value);
  if (result.success) {
    return [
      { level: "success", message: "codextheme-v1 theme.json is valid" },
      { level: "success", message: "Runtime colors and artwork settings are supported" },
    ];
  }
  return result.error.issues.map((issue) => ({
    level: "error" as const,
    message: `${issue.path.join(".") || "theme.json"}: ${issue.message}`,
  }));
}
