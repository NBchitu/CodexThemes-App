import type { WindowBorderStyle } from "../store/app-store";

export const windowBorderStyles: Array<{
  value: WindowBorderStyle;
  label: string;
  shortLabel: string;
}> = [
  { value: "classic-rainbow", label: "Classic rainbow", shortLabel: "Rainbow" },
  { value: "candy-stripe", label: "Candy stripe", shortLabel: "Candy" },
  { value: "ocean", label: "Ocean blue", shortLabel: "Ocean" },
  { value: "monochrome", label: "Electric edge", shortLabel: "Electric" },
];

export const windowBorderStyleLabel = (style: WindowBorderStyle) =>
  windowBorderStyles.find((item) => item.value === style)?.label ?? "Classic rainbow";
