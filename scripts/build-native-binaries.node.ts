#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

import { cli, options } from "comline";
import takua from "takua";
import { z } from "zod/v4";

import {
  detectTarget,
  nativeTargetMetadataByTarget,
  readTargetFromEnv,
  targetSchema
} from "./native-targets.ts";

const root = fileURLToPath(new URL("..", import.meta.url));
const buildOptions = options(
  "build native puggers artifacts",
  z.object({
    target: targetSchema.optional()
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

const buildCli = cli({
  cliName: "build-native-binaries",
  cliDescription: "Build native puggers artifacts for one npm target.",
  discoverConfigPath: () => undefined,
  routeOptions: {
    "": buildOptions
  }
});

const { inputs } = buildCli(process.argv);
const npmTarget = inputs.opts.target ?? readTargetFromEnv() ?? detectTarget();
const { rustTarget } = nativeTargetMetadataByTarget[npmTarget];
const cargoTargetArgs = npmTarget === detectTarget() ? [] : ["--target", rustTarget];

takua.info("build", npmTarget, rustTarget);
run("cargo", ["build", "-p", "puggers", "--release", "--locked", ...cargoTargetArgs]);
run("cargo", [
  "build",
  "-p",
  "puggers-node",
  "--release",
  "--locked",
  ...cargoTargetArgs
]);

function run(command: string, args: string[], env = process.env): void {
  const result = spawnSync(command, args, {
    cwd: root,
    env,
    stdio: "inherit"
  });

  if (result.error != null) {
    throw result.error;
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}
