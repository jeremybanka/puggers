import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import {
  copyFileSync,
  existsSync,
  globSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const repositoryRoot = fileURLToPath(new URL("../../", import.meta.url));

test("Renovate-generated pnpm lockfiles support frozen installs", () => {
  const fixture = mkdtempSync(join(tmpdir(), "puggers-renovate-lockfile-"));

  try {
    const files = [
      "pnpm-workspace.yaml",
      "pnpm-lock.yaml",
      ...globSync([
        "package.json",
        "packages/*/package.json",
        "packages/native/*/package.json"
      ], { cwd: repositoryRoot })
    ];

    // Include a hook if one is reintroduced so the test catches the mismatch.
    for (const hook of [".pnpmfile.mjs", ".pnpmfile.cjs"]) {
      if (existsSync(join(repositoryRoot, hook))) files.push(hook);
    }

    for (const file of files) {
      const destination = join(fixture, file);
      mkdirSync(dirname(destination), { recursive: true });
      copyFileSync(join(repositoryRoot, file), destination);
    }

    const install = (flags: string[]) => {
      const result = spawnSync("pnpm", [
        "install",
        "--lockfile-only",
        "--offline",
        "--ignore-scripts",
        // This probe checks configuration compatibility without downloading
        // packages or consulting registry metadata for supply-chain policies.
        "--trust-lockfile",
        ...flags
      ], {
        cwd: fixture,
        encoding: "utf8",
        env: { ...process.env, CI: "true" },
        shell: process.platform === "win32",
        timeout: 30_000
      });

      assert.equal(result.error, undefined, result.error?.message);
      assert.equal(result.status, 0, result.stdout + result.stderr);
    };

    install(["--frozen-lockfile"]);
    // Change a workspace range to force lockfile regeneration without needing
    // registry metadata for a new external package version.
    const manifestPath = join(fixture, "package.json");
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
    manifest.devDependencies["dprint-plugin-pug"] = "workspace:^";
    writeFileSync(manifestPath, JSON.stringify(manifest));
    // Renovate disables pnpm hooks when allowScripts is false (the default).
    install(["--no-frozen-lockfile", "--ignore-pnpmfile"]);
    install(["--frozen-lockfile"]);
  } finally {
    rmSync(fixture, { recursive: true, force: true });
  }
});
