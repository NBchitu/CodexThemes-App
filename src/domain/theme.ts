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
  }),
});

export type ThemeManifest = z.infer<typeof themeManifestSchema>;
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

export interface ThemeLibraryResult {
  themes: Theme[];
  message: string;
  importedThemeId?: string | null;
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
