import fs from "node:fs";

const [configPath, key, rawValue, rawStyle] = process.argv.slice(2);
const booleanKeys = new Set(["effectsEnabled", "windowBorderEnabled", "pixelCatEnabled"]);
const borderStyles = new Set(["classic-rainbow", "candy-stripe", "ocean", "monochrome"]);
if (!configPath || ![...booleanKeys, "windowBorderStyle"].includes(key)) {
  throw new Error("Usage: update-window-effect.mjs <config> <effect-key> <value>");
}
if (booleanKeys.has(key) && !["true", "false"].includes(rawValue)) {
  throw new Error("Effect value must be true or false");
}
if (key === "windowBorderStyle" && !borderStyles.has(rawValue)) {
  throw new Error("Unsupported animated window border style");
}
if (rawStyle !== undefined && !borderStyles.has(rawStyle)) {
  throw new Error("Unsupported animated window border style");
}

let current = {};
try {
  current = JSON.parse(fs.readFileSync(configPath, "utf8"));
} catch (error) {
  if (error?.code !== "ENOENT") throw error;
}
const next = {
  schemaVersion: 1,
  effectsEnabled: typeof current?.effectsEnabled === "boolean"
    ? current.effectsEnabled
    : current?.schemaVersion === 1
      ? current?.windowBorderEnabled === true || current?.pixelCatEnabled === true
      : true,
  windowBorderEnabled: typeof current?.windowBorderEnabled === "boolean"
    ? current.windowBorderEnabled
    : true,
  windowBorderStyle: rawStyle ?? (borderStyles.has(current?.windowBorderStyle)
    ? current.windowBorderStyle
    : "classic-rainbow"),
  pixelCatEnabled: typeof current?.pixelCatEnabled === "boolean"
    ? current.pixelCatEnabled
    : true,
  [key]: booleanKeys.has(key) ? rawValue === "true" : rawValue,
};
fs.writeFileSync(`${configPath}.${process.pid}`, `${JSON.stringify(next)}\n`, { mode: 0o600 });
fs.renameSync(`${configPath}.${process.pid}`, configPath);
