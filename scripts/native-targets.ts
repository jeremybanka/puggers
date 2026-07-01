import { readFileSync } from "node:fs";
import { arch, platform, report } from "node:process";

import { z } from "zod/v4";

export type SupportedArch = "arm64" | "x64";
export type SupportedOs = "darwin" | "linux" | "win32";
export type LinuxLibc = "glibc" | "musl";
export type RustTarget =
  | "aarch64-apple-darwin"
  | "aarch64-pc-windows-msvc"
  | "aarch64-unknown-linux-gnu"
  | "aarch64-unknown-linux-musl"
  | "x86_64-apple-darwin"
  | "x86_64-pc-windows-msvc"
  | "x86_64-unknown-linux-gnu"
  | "x86_64-unknown-linux-musl";

export interface NativeTargetMetadata {
  os: SupportedOs;
  cpu: SupportedArch;
  libc?: LinuxLibc;
  rustTarget: RustTarget;
  executableName: "puggers" | "puggers.exe";
  addonName: "libpuggers_node.dylib" | "libpuggers_node.so" | "puggers_node.dll";
}

export const supportedTargets = [
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64-glibc",
  "linux-arm64-musl",
  "linux-x64-glibc",
  "linux-x64-musl",
  "win32-arm64",
  "win32-x64"
] as const;
export type SupportedTarget = (typeof supportedTargets)[number];

export const targetSchema = z.enum(supportedTargets);

export const nativeTargetMetadataByTarget = {
  "darwin-arm64": {
    os: "darwin",
    cpu: "arm64",
    rustTarget: "aarch64-apple-darwin",
    executableName: "puggers",
    addonName: "libpuggers_node.dylib"
  },
  "darwin-x64": {
    os: "darwin",
    cpu: "x64",
    rustTarget: "x86_64-apple-darwin",
    executableName: "puggers",
    addonName: "libpuggers_node.dylib"
  },
  "linux-arm64-glibc": {
    os: "linux",
    cpu: "arm64",
    libc: "glibc",
    rustTarget: "aarch64-unknown-linux-gnu",
    executableName: "puggers",
    addonName: "libpuggers_node.so"
  },
  "linux-arm64-musl": {
    os: "linux",
    cpu: "arm64",
    libc: "musl",
    rustTarget: "aarch64-unknown-linux-musl",
    executableName: "puggers",
    addonName: "libpuggers_node.so"
  },
  "linux-x64-glibc": {
    os: "linux",
    cpu: "x64",
    libc: "glibc",
    rustTarget: "x86_64-unknown-linux-gnu",
    executableName: "puggers",
    addonName: "libpuggers_node.so"
  },
  "linux-x64-musl": {
    os: "linux",
    cpu: "x64",
    libc: "musl",
    rustTarget: "x86_64-unknown-linux-musl",
    executableName: "puggers",
    addonName: "libpuggers_node.so"
  },
  "win32-arm64": {
    os: "win32",
    cpu: "arm64",
    rustTarget: "aarch64-pc-windows-msvc",
    executableName: "puggers.exe",
    addonName: "puggers_node.dll"
  },
  "win32-x64": {
    os: "win32",
    cpu: "x64",
    rustTarget: "x86_64-pc-windows-msvc",
    executableName: "puggers.exe",
    addonName: "puggers_node.dll"
  }
} satisfies Record<SupportedTarget, NativeTargetMetadata>;

export function parseTarget(target: string): SupportedTarget {
  return targetSchema.parse(target);
}

export function detectTarget(): SupportedTarget {
  if (platform === "darwin") {
    return parseTarget(`darwin-${supportedArch()}`);
  }

  if (platform === "linux") {
    return parseTarget(`linux-${supportedArch()}-${detectLinuxLibc()}`);
  }

  if (platform === "win32") {
    return parseTarget(`win32-${supportedArch()}`);
  }

  throw new Error(`unsupported platform: ${platform}`);
}

export function readTargetFromEnv(env = process.env): SupportedTarget | undefined {
  const target = env.PUGGERS_NPM_TARGET;
  return target == null || target === "" ? undefined : parseTarget(target);
}

function supportedArch(): SupportedArch {
  if (arch === "arm64" || arch === "x64") {
    return arch;
  }

  throw new Error(`unsupported architecture: ${arch}`);
}

function detectLinuxLibc(): LinuxLibc {
  const runtimeReport: unknown = report?.getReport();
  const parsedReport =
    typeof runtimeReport === "string"
      ? (JSON.parse(runtimeReport) as unknown)
      : runtimeReport;

  if (parsedReport == null || typeof parsedReport !== "object") {
    return detectLinuxLibcFromLdd();
  }

  if (!("header" in parsedReport)) {
    return detectLinuxLibcFromLdd();
  }

  const { header } = parsedReport;
  if (header != null && typeof header === "object" && "glibcVersionRuntime" in header) {
    return "glibc";
  }

  if (!("sharedObjects" in parsedReport)) {
    return detectLinuxLibcFromLdd();
  }

  const { sharedObjects } = parsedReport;
  if (
    Array.isArray(sharedObjects) &&
    sharedObjects.some(
      (path) =>
        typeof path === "string" &&
        (path.includes("libc.musl-") || path.includes("ld-musl-"))
    )
  ) {
    return "musl";
  }

  return detectLinuxLibcFromLdd();
}

function detectLinuxLibcFromLdd(): LinuxLibc {
  try {
    return readFileSync("/usr/bin/ldd", "utf8").includes("musl") ? "musl" : "glibc";
  } catch {
    return "glibc";
  }
}
