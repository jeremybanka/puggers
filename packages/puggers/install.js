import { existsSync } from "node:fs";

const builtInstaller = new URL("./dist/install.js", import.meta.url);
const sourceInstaller = new URL("./src/install.ts", import.meta.url);

if (existsSync(builtInstaller)) {
  await import(builtInstaller.href);
} else if (!existsSync(sourceInstaller)) {
  throw new Error("puggers npm package is missing dist/install.js");
}
