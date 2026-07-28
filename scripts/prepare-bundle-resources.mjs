import { chmod, cp, lstat, mkdir, readdir, rm } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const sourceRoot = path.join(appRoot, "resources", "macos-engine");
const stagingRoot = path.join(appRoot, "src-tauri", ".bundle-resources", "macos-engine");

const copyResource = async (name) => {
  const source = path.join(sourceRoot, name);
  const destination = path.join(stagingRoot, name);
  await cp(source, destination, {
    recursive: true,
    filter: (entry) => path.basename(entry) !== ".DS_Store",
  });
};

const normalizePermissions = async (entryPath, executableFiles = false) => {
  const metadata = await lstat(entryPath);
  if (metadata.isSymbolicLink()) {
    throw new Error(`Bundled runtime resources must not contain symbolic links: ${entryPath}`);
  }
  if (metadata.isDirectory()) {
    await chmod(entryPath, 0o755);
    const entries = await readdir(entryPath);
    await Promise.all(entries.map((entry) => normalizePermissions(path.join(entryPath, entry), executableFiles)));
    return;
  }
  if (!metadata.isFile()) {
    throw new Error(`Unsupported bundled runtime resource: ${entryPath}`);
  }
  await chmod(entryPath, executableFiles ? 0o755 : 0o644);
};

await rm(stagingRoot, { recursive: true, force: true });
await mkdir(stagingRoot, { recursive: true, mode: 0o755 });
await Promise.all([copyResource("assets"), copyResource("scripts"), copyResource("presets"), copyResource("VERSION")]);

await Promise.all([
  normalizePermissions(path.join(stagingRoot, "assets")),
  normalizePermissions(path.join(stagingRoot, "scripts"), true),
  normalizePermissions(path.join(stagingRoot, "presets")),
  normalizePermissions(path.join(stagingRoot, "VERSION")),
]);

console.log(`Prepared readable macOS runtime resources in ${stagingRoot}`);
