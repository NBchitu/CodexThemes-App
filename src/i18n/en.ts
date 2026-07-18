export const en = {
  appName: "Codex Themes",
  discover: "Discover",
  myThemes: "My Themes",
  create: "Create",
  settings: "Settings",
  search: "Search themes",
  featured: "Featured themes",
  featuredCopy: "Thoughtful themes for a more personal Codex workspace.",
  browseAll: "Browse all",
  currentTheme: "Current theme",
  imported: "Imported",
  builtIn: "Built-in",
  installed: "Installed",
  apply: "Apply theme",
  download: "Download",
  update: "Update",
  openDetails: "Open theme details",
} as const;

export type TranslationKey = keyof typeof en;
export const t = (key: TranslationKey) => en[key];
