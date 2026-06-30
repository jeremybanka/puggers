#!/usr/bin/env node
import {
  chmodSync,
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync
} from "node:fs";
import { arch, platform, report } from "node:process";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { cli, options, required } from "comline";
import { z } from "zod/v4";

type SupportedArch = "arm64" | "x64";
type SupportedOs = "darwin" | "linux" | "win32";
type LinuxLibc = "glibc" | "musl";

interface NativeArtifacts {
  executablePath: string;
  addonPath: string;
  outputExecutableName: string;
}

interface NativeTargetMetadata {
  os: SupportedOs;
  cpu: SupportedArch;
  libc?: LinuxLibc;
}

const supportedTargets = [
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64-glibc",
  "linux-arm64-musl",
  "linux-x64-glibc",
  "linux-x64-musl",
  "win32-arm64",
  "win32-x64"
] as const;
type SupportedTarget = (typeof supportedTargets)[number];

const targetSchema = z.enum(supportedTargets);
const cliTargetSchema = targetSchema.optional();
const nativeTargetMetadataByTarget = {
  "darwin-arm64": { os: "darwin", cpu: "arm64" },
  "darwin-x64": { os: "darwin", cpu: "x64" },
  "linux-arm64-glibc": { os: "linux", cpu: "arm64", libc: "glibc" },
  "linux-arm64-musl": { os: "linux", cpu: "arm64", libc: "musl" },
  "linux-x64-glibc": { os: "linux", cpu: "x64", libc: "glibc" },
  "linux-x64-musl": { os: "linux", cpu: "x64", libc: "musl" },
  "win32-arm64": { os: "win32", cpu: "arm64" },
  "win32-x64": { os: "win32", cpu: "x64" }
} satisfies Record<SupportedTarget, NativeTargetMetadata>;

const cliRoutes = required({
  "stage-local": null,
  "stage-dist": null,
  "print-dist-path": null
});

const targetOptions = options(
  "stage native npm artifacts",
  z.object({
    target: cliTargetSchema
  }),
  {
    target: {
      description: "native npm target triple",
      example: "--target=linux-x64-glibc",
      flag: "t",
      required: false
    }
  }
);

const routeOptions = {
  "stage-local": targetOptions,
  "stage-dist": targetOptions,
  "print-dist-path": targetOptions
};

const npmNativeCli = cli({
  cliName: "npm-native",
  cliDescription: "Stage native puggers npm artifacts.",
  discoverConfigPath: () => undefined,
  routes: cliRoutes,
  routeOptions
});

const root = fileURLToPath(new URL("..", import.meta.url));
const { inputs } = npmNativeCli(process.argv);
const target = inputs.opts.target ?? readTargetFromEnv() ?? detectTarget();

switch (inputs.case) {
  case "stage-local":
    stageLocalArtifacts(target);
    break;
  case "stage-dist":
    stageNativePackage(target);
    break;
  case "print-dist-path":
    console.log(nativePackageDirectory(target));
    break;
}

function stageLocalArtifacts(target: SupportedTarget): void {
  copyNativeArtifacts(
    join(root, "packages", "puggers", ".native"),
    resolveNativeArtifacts(target)
  );
}

function stageNativePackage(packageTarget: SupportedTarget): string {
  const version = readWorkspaceVersion();
  const metadata = nativeTargetMetadataByTarget[packageTarget];
  const artifacts = resolveNativeArtifacts(packageTarget);
  const packageDirectory = nativePackageDirectory(packageTarget);

  rmSync(packageDirectory, { recursive: true, force: true });
  copyNativeArtifacts(packageDirectory, artifacts);

  writeFileSync(
    join(packageDirectory, "package.json"),
    `${JSON.stringify(
      {
        name: `@puggers/${packageTarget}`,
        version,
        description: `${packageTarget} native distribution for puggers`,
        repository: {
          type: "git",
          url: "git+https://github.com/jeremybanka/puggers.git"
        },
        license: "MIT",
        preferUnplugged: true,
        os: [metadata.os],
        cpu: [metadata.cpu],
        ...("libc" in metadata ? { libc: [metadata.libc] } : {}),
        files: [artifacts.outputExecutableName, "puggers.node", "README.md"],
        publishConfig: {
          access: "public"
        }
      },
      null,
      2
    )}\n`
  );

  writeFileSync(
    join(packageDirectory, "README.md"),
    `# @puggers/${packageTarget}\n\nNative ${packageTarget} distribution for puggers.\n`
  );

  return packageDirectory;
}

function nativePackageDirectory(packageTarget: SupportedTarget): string {
  return join(root, "target", "npm", "@puggers", packageTarget);
}

function copyNativeArtifacts(outputDirectory: string, artifacts: NativeArtifacts): void {
  mkdirSync(outputDirectory, { recursive: true });
  copyFileSync(artifacts.executablePath, join(outputDirectory, artifacts.outputExecutableName));
  copyFileSync(artifacts.addonPath, join(outputDirectory, "puggers.node"));
  chmodIfPossible(join(outputDirectory, artifacts.outputExecutableName));
}

function resolveNativeArtifacts(packageTarget: SupportedTarget): NativeArtifacts {
  const releaseDirectory = process.env.PUGGERS_RELEASE_DIR ?? join(root, "target", "release");
  const outputExecutableName = packageTarget.startsWith("win32-") ? "puggers.exe" : "puggers";
  const sourceExecutableName = platform === "win32" ? "puggers.exe" : "puggers";
  const sourceAddonName =
    platform === "win32"
      ? "puggers_node.dll"
      : platform === "darwin"
        ? "libpuggers_node.dylib"
        : "libpuggers_node.so";

  const executablePath =
    process.env.PUGGERS_EXE ?? join(releaseDirectory, sourceExecutableName);
  const addonPath =
    process.env.PUGGERS_NODE_ADDON ?? join(releaseDirectory, sourceAddonName);

  assertExists(executablePath, "native puggers executable");
  assertExists(addonPath, "native puggers Node-API addon");

  return {
    executablePath,
    addonPath,
    outputExecutableName
  };
}

function detectTarget(): SupportedTarget {
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

function readTargetFromEnv(): SupportedTarget | undefined {
  const target = process.env.PUGGERS_NPM_TARGET;
  return target == null || target === "" ? undefined : parseTarget(target);
}

function parseTarget(target: string): SupportedTarget {
  return targetSchema.parse(target);
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

function readWorkspaceVersion(): string {
  const cargoToml = readFileSync(join(root, "Cargo.toml"), "utf8");
  const match = cargoToml.match(/\[workspace\.package\][\s\S]*?version = "([^"]+)"/);
  if (match == null) {
    throw new Error("could not read workspace package version from Cargo.toml");
  }

  return match[1];
}

function assertExists(path: string, label: string): void {
  if (!existsSync(path)) {
    throw new Error(
      `missing ${label} at ${path}. Run cargo build -p puggers --release and cargo build -p puggers-node --release first.`
    );
  }
}

function chmodIfPossible(path: string): void {
  try {
    chmodSync(path, 0o755);
  } catch {
    // Windows and readonly filesystems can ignore this; package managers preserve executable metadata where supported.
  }
}
