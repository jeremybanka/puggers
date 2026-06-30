#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));

interface PackageJson {
  version?: string;
  optionalDependencies?: Record<string, string>;
}

const workspaceVersion = readWorkspaceVersion();
const packageJsonPaths: string[] = [
  "packages/puggers/package.json",
  "packages/native/darwin-arm64/package.json",
  "packages/native/darwin-x64/package.json",
  "packages/native/linux-arm64-glibc/package.json",
  "packages/native/linux-arm64-musl/package.json",
  "packages/native/linux-x64-glibc/package.json",
  "packages/native/linux-x64-musl/package.json",
  "packages/native/win32-arm64/package.json",
  "packages/native/win32-x64/package.json"
];

const errors: string[] = [];

for (const packageJsonPath of packageJsonPaths) {
  const packageJson = readJson(packageJsonPath);
  if (packageJson.version !== workspaceVersion) {
    errors.push(
      `${packageJsonPath} has version ${packageJson.version}; expected ${workspaceVersion}`
    );
  }
}

const topLevelPackage = readJson("packages/puggers/package.json");
for (const [name, version] of Object.entries(topLevelPackage.optionalDependencies ?? {})) {
  if (name.startsWith("@puggers/") && version !== "workspace:*") {
    errors.push(
      `packages/puggers/package.json optional dependency ${name} uses ${version}; expected workspace:*`
    );
  }
}

if (errors.length > 0) {
  for (const error of errors) {
    console.error(error);
  }
  process.exit(1);
}

console.log(`All npm package manifests match workspace version ${workspaceVersion}.`);

function readWorkspaceVersion(): string {
  const cargoToml = readFileSync(join(root, "Cargo.toml"), "utf8");
  const match = cargoToml.match(/\[workspace\.package\][\s\S]*?version = "([^"]+)"/);
  if (match == null) {
    throw new Error("Could not find workspace.package version in Cargo.toml");
  }

  return match[1];
}

function readJson(path: string): PackageJson {
  return JSON.parse(readFileSync(join(root, path), "utf8")) as PackageJson;
}
