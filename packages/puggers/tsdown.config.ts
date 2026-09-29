import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts", "src/bin.ts", "src/install.ts"],
  dts: { entry: ["src/index.ts"] },
  format: "esm",
  platform: "node",
  target: "node26",
  outDir: "dist",
  clean: true,
  fixedExtension: false
});
