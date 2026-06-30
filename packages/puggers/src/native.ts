import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

interface NativeBinding {
  convertHtmlToPugNative(input: string, optionsJson?: string): string;
}

const require = createRequire(import.meta.url);
let nativeBinding: NativeBinding | undefined;

export function loadNativeBinding(): NativeBinding {
  if (nativeBinding != null) {
    return nativeBinding;
  }

  const bindingPath = resolveNativeBindingPath();
  const loaded = require(bindingPath) as Partial<NativeBinding>;

  if (typeof loaded.convertHtmlToPugNative !== "function") {
    throw new Error(`Native puggers binding at ${bindingPath} does not export convertHtmlToPugNative`);
  }

  nativeBinding = loaded as NativeBinding;
  return nativeBinding;
}

export function resolveNativeExecutable(): string {
  const fileName = process.platform === "win32" ? "puggers.exe" : "puggers";
  const executablePath = join(resolveNativeDirectory(), fileName);

  if (!existsSync(executablePath)) {
    throw new Error(`Native puggers executable was not found at ${executablePath}`);
  }

  return executablePath;
}

function resolveNativeBindingPath(): string {
  const bindingPath = join(resolveNativeDirectory(), "puggers.node");

  if (!existsSync(bindingPath)) {
    throw new Error(`Native puggers binding was not found at ${bindingPath}`);
  }

  return bindingPath;
}

function resolveNativeDirectory(): string {
  const overrideDirectory = process.env.PUGGERS_NATIVE_DIR;
  if (overrideDirectory != null && overrideDirectory !== "") {
    return overrideDirectory;
  }

  const localDirectory = fileURLToPath(new URL("../.native/", import.meta.url));
  if (existsSync(join(localDirectory, "puggers.node"))) {
    return localDirectory;
  }

  const packageName = resolveNativePackageName();

  try {
    return dirname(require.resolve(`${packageName}/package.json`));
  } catch (error) {
    throw new Error(
      `Could not load native puggers package ${packageName}. ` +
        "Install optional dependencies, or set PUGGERS_NATIVE_DIR to a directory containing puggers.node and the puggers executable.",
      { cause: error }
    );
  }
}

function resolveNativePackageName(): string {
  if (process.platform === "darwin") {
    return `@puggers/darwin-${resolveSupportedArch()}`;
  }

  if (process.platform === "linux") {
    return `@puggers/linux-${resolveSupportedArch()}-${detectLinuxLibc()}`;
  }

  if (process.platform === "win32") {
    return `@puggers/win32-${resolveSupportedArch()}`;
  }

  throw new Error(`Unsupported platform for puggers native package: ${process.platform}`);
}

function resolveSupportedArch(): "arm64" | "x64" {
  if (process.arch === "arm64" || process.arch === "x64") {
    return process.arch;
  }

  throw new Error(`Unsupported architecture for puggers native package: ${process.arch}`);
}

function detectLinuxLibc(): "glibc" | "musl" {
  const report = process.report?.getReport();
  const header = typeof report === "string" ? JSON.parse(report).header : report?.header;

  if (header != null && "glibcVersionRuntime" in header) {
    return "glibc";
  }

  if (Array.isArray(report?.sharedObjects) && report.sharedObjects.some((path) => path.includes("libc.musl-") || path.includes("ld-musl-"))) {
    return "musl";
  }

  try {
    return readFileSync("/usr/bin/ldd", "utf8").includes("musl") ? "musl" : "glibc";
  } catch {
    return "glibc";
  }
}
