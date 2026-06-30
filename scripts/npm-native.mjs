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
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));
const command = process.argv[2] ?? "local";
const target = process.argv[3] ?? process.env.PUGGERS_NPM_TARGET ?? detectTarget();

switch (command) {
  case "local":
    copyLocalArtifacts();
    break;
  case "package":
    createNativePackage(target);
    break;
  case "pack":
    packNativePackage(createNativePackage(target));
    break;
  case "publish":
    publishNativePackage(createNativePackage(target));
    break;
  default:
    throw new Error(`unknown npm native command: ${command}`);
}

function copyLocalArtifacts() {
  const outputDirectory = join(root, "packages", "puggers", ".native");
  mkdirSync(outputDirectory, { recursive: true });

  const artifacts = resolveNativeArtifacts(target);
  copyFileSync(artifacts.executablePath, join(outputDirectory, artifacts.outputExecutableName));
  copyFileSync(artifacts.addonPath, join(outputDirectory, "puggers.node"));
  chmodIfPossible(join(outputDirectory, artifacts.outputExecutableName));
}

function createNativePackage(packageTarget) {
  const version = readWorkspaceVersion();
  const metadata = nativeTargetMetadata(packageTarget);
  const artifacts = resolveNativeArtifacts(packageTarget);
  const packageDirectory = join(root, "target", "npm", "@puggers", packageTarget);

  rmSync(packageDirectory, { recursive: true, force: true });
  mkdirSync(packageDirectory, { recursive: true });

  copyFileSync(artifacts.executablePath, join(packageDirectory, artifacts.outputExecutableName));
  copyFileSync(artifacts.addonPath, join(packageDirectory, "puggers.node"));
  chmodIfPossible(join(packageDirectory, artifacts.outputExecutableName));

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
        ...(metadata.libc == null ? {} : { libc: [metadata.libc] }),
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

function packNativePackage(packageDirectory) {
  runPnpm(["pack", "--pack-destination", join(root, "target", "npm")], packageDirectory);
}

function publishNativePackage(packageDirectory) {
  runPnpm(["publish", "--access", "public", "--provenance"], packageDirectory);
}

function runPnpm(args, cwd) {
  const result = spawnSync("pnpm", args, { cwd, stdio: "inherit" });
  if (result.error != null) {
    throw result.error;
  }

  process.exit(result.status ?? 1);
}

function resolveNativeArtifacts(packageTarget) {
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

function detectTarget() {
  if (platform === "darwin") {
    return `darwin-${supportedArch()}`;
  }

  if (platform === "linux") {
    return `linux-${supportedArch()}-${detectLinuxLibc()}`;
  }

  if (platform === "win32") {
    return `win32-${supportedArch()}`;
  }

  throw new Error(`unsupported platform: ${platform}`);
}

function nativeTargetMetadata(packageTarget) {
  const [targetOs, targetCpu, targetLibc] = packageTarget.split("-");
  if (
    !["darwin", "linux", "win32"].includes(targetOs) ||
    !["arm64", "x64"].includes(targetCpu)
  ) {
    throw new Error(`unsupported npm native target: ${packageTarget}`);
  }

  if (targetOs === "linux" && !["glibc", "musl"].includes(targetLibc ?? "")) {
    throw new Error(`linux npm native targets must specify glibc or musl: ${packageTarget}`);
  }

  if (targetOs !== "linux" && targetLibc != null) {
    throw new Error(`only linux npm native targets may specify libc: ${packageTarget}`);
  }

  return {
    os: targetOs,
    cpu: targetCpu,
    libc: targetLibc
  };
}

function supportedArch() {
  if (arch === "arm64" || arch === "x64") {
    return arch;
  }

  throw new Error(`unsupported architecture: ${arch}`);
}

function detectLinuxLibc() {
  const runtimeReport = report?.getReport();
  const header =
    typeof runtimeReport === "string" ? JSON.parse(runtimeReport).header : runtimeReport?.header;

  if (header != null && "glibcVersionRuntime" in header) {
    return "glibc";
  }

  if (
    Array.isArray(runtimeReport?.sharedObjects) &&
    runtimeReport.sharedObjects.some((path) => path.includes("libc.musl-") || path.includes("ld-musl-"))
  ) {
    return "musl";
  }

  try {
    return readFileSync("/usr/bin/ldd", "utf8").includes("musl") ? "musl" : "glibc";
  } catch {
    return "glibc";
  }
}

function readWorkspaceVersion() {
  const cargoToml = readFileSync(join(root, "Cargo.toml"), "utf8");
  const match = cargoToml.match(/\[workspace\.package\][\s\S]*?version = "([^"]+)"/);
  if (match == null) {
    throw new Error("could not read workspace package version from Cargo.toml");
  }

  return match[1];
}

function assertExists(path, label) {
  if (!existsSync(path)) {
    throw new Error(
      `missing ${label} at ${path}. Run cargo build -p puggers --release and cargo build -p puggers-node --release first.`
    );
  }
}

function chmodIfPossible(path) {
  try {
    chmodSync(path, 0o755);
  } catch {
    // Windows and readonly filesystems can ignore this; package managers preserve executable metadata where supported.
  }
}
