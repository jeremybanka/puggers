import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import {
  chmodSync,
  existsSync,
  mkdtempSync,
  rmSync,
  statSync,
  writeFileSync
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

import { convertHtmlToPug } from "../dist/index.js";

test("convertHtmlToPug calls the native converter", () => {
  const output = convertHtmlToPug("<main><a class=\"button\" href=\"/docs\">Docs</a></main>", {
    root: "main",
    allowedAttributes: ["class", "href"],
    quoteStyle: "single"
  });

  assert.equal(output, "main\n  a.button(href='/docs') Docs\n");
});

test("puggers bin forwards to the native CLI", () => {
  const binPath = fileURLToPath(new URL("../dist/bin.js", import.meta.url));
  const inputDirectory = mkdtempSync(join(tmpdir(), "puggers-"));
  const inputPath = join(inputDirectory, "input.html");
  writeFileSync(inputPath, "<main><h1>Hello</h1></main>");

  const result = spawnSync(process.execPath, [binPath, "--root", "main", inputPath], {
    encoding: "utf8"
  });

  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout, "main\n  h1 Hello\n");
});

test(
  "puggers bin prepares a chmodded top-level native executable",
  { skip: process.platform === "win32" },
  () => {
    const binPath = fileURLToPath(new URL("../dist/bin.js", import.meta.url));
    const packageRoot = fileURLToPath(new URL("..", import.meta.url));
    const installedExecutablePath = join(packageRoot, "puggers");
    const nativeExecutablePath = resolveTestNativeExecutablePath(packageRoot);
    const originalMode = statSync(nativeExecutablePath).mode;
    const inputDirectory = mkdtempSync(join(tmpdir(), "puggers-"));
    const inputPath = join(inputDirectory, "input.html");

    writeFileSync(inputPath, "<main><h1>Hello</h1></main>");
    rmSync(installedExecutablePath, { force: true });
    chmodSync(nativeExecutablePath, originalMode & ~0o111);

    try {
      const result = spawnSync(process.execPath, [binPath, "--root", "main", inputPath], {
        encoding: "utf8"
      });

      assert.equal(result.status, 0, result.stderr);
      assert.equal(result.stdout, "main\n  h1 Hello\n");
      assert.equal(existsSync(installedExecutablePath), true);
      assert.notEqual(statSync(installedExecutablePath).mode & 0o111, 0);
    } finally {
      chmodSync(nativeExecutablePath, originalMode);
      rmSync(installedExecutablePath, { force: true });
    }
  }
);

test(
  "Linux musl reports an unsupported native package",
  { skip: process.platform !== "linux" },
  () => {
    const packageUrl = new URL("../dist/index.js", import.meta.url).href;
    const script = `
      process.report.getReport = () => ({
        header: {},
        sharedObjects: ["/lib/ld-musl-x86_64.so.1"]
      });

      const { convertHtmlToPug } = await import(${JSON.stringify(packageUrl)});
      convertHtmlToPug("<main></main>");
    `;

    const result = spawnSync(process.execPath, ["--input-type=module", "--eval", script], {
      encoding: "utf8"
    });

    assert.notEqual(result.status, 0);
    assert.match(
      result.stderr,
      /Unsupported Linux libc for puggers native package: musl/
    );
  }
);

function resolveTestNativeExecutablePath(packageRoot: string): string {
  const nativePackageRoot = join(packageRoot, "..", "native", resolveTestNativeTarget());
  return join(nativePackageRoot, process.platform === "win32" ? "puggers.exe" : "puggers");
}

function resolveTestNativeTarget(): string {
  if (process.platform === "darwin" && (process.arch === "arm64" || process.arch === "x64")) {
    return `darwin-${process.arch}`;
  }

  if (process.platform === "linux" && (process.arch === "arm64" || process.arch === "x64")) {
    return `linux-${process.arch}-glibc`;
  }

  if (process.platform === "win32" && (process.arch === "arm64" || process.arch === "x64")) {
    return `win32-${process.arch}`;
  }

  throw new Error(`Unsupported test target: ${process.platform} ${process.arch}`);
}
