import { Check } from "lucide-react";
import type { Theme } from "../domain/theme";
import { cn } from "../lib/cn";

interface ThemeCardProps {
  theme: Theme;
  active?: boolean;
  onOpen: (theme: Theme) => void;
}

export function ThemeCard({ theme, active = false, onOpen }: ThemeCardProps) {
  return (
    <article className={cn("theme-card", active && "theme-card-active")}>
      <button
        className="theme-preview-button"
        type="button"
        onClick={() => onOpen(theme)}
        aria-label={`Open ${theme.name} details`}
      >
        <img className="theme-preview" src={theme.previewUrl} alt="" loading="lazy" />
        {active && <span className="active-badge"><Check size={13} /> Active</span>}
      </button>
      <div className="theme-meta">
        <div className="theme-copy">
          <h3>{theme.name}</h3>
          <p>by {theme.author}</p>
        </div>
        <span className="theme-state">
          <><Check size={13} /> {theme.origin === "built-in" ? "Built-in" : "Imported"}</>
        </span>
      </div>
    </article>
  );
}
