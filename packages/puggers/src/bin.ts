#!/usr/bin/env node
import { spawnSync } from "node:child_process";

import { resolveNativeExecutable } from "./native.js";

try {
  const executable = resolveNativeExecutable();
  const result = spawnSync(executable, process.argv.slice(2), {
    stdio: "inherit"
  });

  if (result.error != null) {
    throw result.error;
  }

  process.exit(result.status ?? 1);
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
