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
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { cli, options, required } from "comline";
import takua from "takua";
import { z } from "zod/v4";
import {
  detectTarget,
  nativeTargetMetadataByTarget,
  readTargetFromEnv,
  targetSchema,
  type SupportedTarget
} from "./native-targets.ts";

interface NativeArtifacts {
  executablePath: string;
  addonPath: string;
  outputExecutableName: string;
}

const binaryDestinationSchema = z.enum(["workspace", "staging"]);
const cliTargetSchema = targetSchema.optional();

const cliRoutes = required({
  "copy-binaries": null,
  "write-manifest": null,
  "print-staging-path": null
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

const copyBinariesOptions = options(
  "copy native binaries into a package directory",
  z.object({
    destination: binaryDestinationSchema.optional(),
    target: cliTargetSchema
  }),
  {
    destination: {
      description: "binary destination package directory",
      example: "--destination=staging",
      flag: "d",
      required: false
    },
    target: {
      description: "native npm target triple",
      example: "--target=linux-x64-glibc",
      flag: "t",
      required: false
    }
  }
);

const routeOptions = {
  "copy-binaries": copyBinariesOptions,
  "write-manifest": targetOptions,
  "print-staging-path": targetOptions
};

const npmStageCli = cli({
  cliName: "npm-stage",
  cliDescription: "Stage native puggers npm artifacts.",
  discoverConfigPath: () => undefined,
  routes: cliRoutes,
  routeOptions
});

const root = fileURLToPath(new URL("..", import.meta.url));
const { inputs } = npmStageCli(process.argv);
const targetOverride = inputs.opts.target ?? readTargetFromEnv();

switch (inputs.case) {
  case "copy-binaries":
    copyBinaries(targetOverride ?? detectTarget(), inputs.opts.destination ?? "workspace");
    break;
  case "write-manifest":
    writeManifest(targetOverride ?? detectTarget());
    break;
  case "print-staging-path":
    console.log(nativeStagingPackageDirectory(targetOverride ?? detectTarget()));
    break;
}

function copyBinaries(packageTarget: SupportedTarget, destination: "workspace" | "staging"): void {
  const outputDirectory =
    destination === "workspace"
      ? nativeWorkspacePackageDirectory(packageTarget)
      : nativeStagingPackageDirectory(packageTarget);

  if (destination === "staging") {
    rmSync(outputDirectory, { recursive: true, force: true });
  }

  copyNativeArtifacts(outputDirectory, resolveNativeArtifacts(packageTarget));
  takua.info("npm-stage", "copy-binaries", `${packageTarget} -> ${outputDirectory}`);
}

function writeManifest(packageTarget: SupportedTarget): string {
  const version = readWorkspaceVersion();
  const metadata = nativeTargetMetadataByTarget[packageTarget];
  const packageDirectory = nativeStagingPackageDirectory(packageTarget);

  mkdirSync(packageDirectory, { recursive: true });

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
        files: [metadata.executableName, "puggers.node", "README.md"],
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

  takua.info("npm-stage", "write-manifest", `${packageTarget} -> ${packageDirectory}`);

  return packageDirectory;
}

function nativeStagingPackageDirectory(packageTarget: SupportedTarget): string {
  return join(root, "target", "npm", "@puggers", packageTarget);
}

function nativeWorkspacePackageDirectory(packageTarget: SupportedTarget): string {
  return join(root, "packages", "native", packageTarget);
}

function copyNativeArtifacts(outputDirectory: string, artifacts: NativeArtifacts): void {
  mkdirSync(outputDirectory, { recursive: true });
  copyFileSync(artifacts.executablePath, join(outputDirectory, artifacts.outputExecutableName));
  copyFileSync(artifacts.addonPath, join(outputDirectory, "puggers.node"));
  chmodIfPossible(join(outputDirectory, artifacts.outputExecutableName));
}

function resolveNativeArtifacts(
  packageTarget: SupportedTarget,
  releaseDirectory = targetReleaseDirectory(packageTarget),
  buildHint = targetBuildHint(packageTarget)
): NativeArtifacts {
  const metadata = nativeTargetMetadataByTarget[packageTarget];

  const executablePath =
    process.env.PUGGERS_EXE ?? join(releaseDirectory, metadata.executableName);
  const addonPath =
    process.env.PUGGERS_NODE_ADDON ?? join(releaseDirectory, metadata.addonName);

  assertExists(executablePath, "native puggers executable", buildHint);
  assertExists(addonPath, "native puggers Node-API addon", buildHint);

  return {
    executablePath,
    addonPath,
    outputExecutableName: metadata.executableName
  };
}

function targetReleaseDirectory(packageTarget: SupportedTarget): string {
  const metadata = nativeTargetMetadataByTarget[packageTarget];
  if (process.env.PUGGERS_RELEASE_DIR != null) {
    return process.env.PUGGERS_RELEASE_DIR;
  }

  if (packageTarget === detectTarget()) {
    return join(root, "target", "release");
  }

  return join(root, "target", metadata.rustTarget, "release");
}

function targetBuildHint(packageTarget: SupportedTarget): string {
  const { rustTarget } = nativeTargetMetadataByTarget[packageTarget];
  const targetArg = `--target=${packageTarget}`;
  return [
    `Run just build-npm-native-binaries ${targetArg}, or`,
    `cargo build -p puggers --release --locked --target ${rustTarget}`,
    "and",
    `cargo build -p puggers-node --release --locked --target ${rustTarget}`,
    "first."
  ].join(" ");
}

function readWorkspaceVersion(): string {
  const cargoToml = readFileSync(join(root, "Cargo.toml"), "utf8");
  const match = cargoToml.match(/\[workspace\.package\][\s\S]*?version = "([^"]+)"/);
  if (match == null) {
    throw new Error("could not read workspace package version from Cargo.toml");
  }

  return match[1];
}

function assertExists(path: string, label: string, buildHint: string): void {
  if (!existsSync(path)) {
    throw new Error(`missing ${label} at ${path}. ${buildHint}`);
  }
}

function chmodIfPossible(path: string): void {
  try {
    chmodSync(path, 0o755);
  } catch {
    // Windows and readonly filesystems can ignore this; package managers preserve executable metadata where supported.
  }
}
