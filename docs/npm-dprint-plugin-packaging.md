# dprint Plugin npm Packaging

The `dprint-plugin-pug` npm package publishes the Rust dprint plugin as a
single Wasm module plus the small JavaScript entrypoint shape dprint expects.

## Package Layout

- `packages/dprint-plugin-pug`: tracked npm package metadata, README, and
  `getPath()` entrypoint
- `target/wasm32-unknown-unknown/release/dprint_plugin_pug.wasm`: Rust build
  output
- `packages/dprint-plugin-pug/plugin.wasm`: ignored local copy for
  package-manager-managed dprint resolver checks
- `target/npm/dprint-plugin-pug`: staged package used for pack and publish

The published package contains only:

- `package.json`
- `README.md`
- `index.js`
- `index.d.ts`
- `plugin.wasm`

## Usage

Install the package when the project lockfile should manage the plugin:

```sh
pnpm add -D dprint-plugin-pug
```

Then use the unversioned local npm resolver in `dprint.json`:

```json
{
  "plugins": ["npm:dprint-plugin-pug"]
}
```

Use a pinned npm specifier when the dprint config should resolve the exact
registry version without relying on `node_modules`:

```json
{
  "plugins": ["npm:dprint-plugin-pug@0.1.9"]
}
```

## Local Build

Build the Wasm plugin and copy it into the workspace package:

```sh
just build-npm-dprint-plugin
```

Stage and pack the publishable npm package:

```sh
just pack-npm-dprint-plugin
```

When packaging an externally built artifact, set
`PUGGERS_DPRINT_PLUGIN_WASM=/path/to/dprint_plugin_pug.wasm` before running the
staging or publish command.

## Publishing

Manual publishing remains available for initial package setup and recovery:

```sh
just publish-npm-dprint-plugin
```

CI publishes the dprint plugin package from the shared npm package-group
workflow with npm trusted publishing and provenance. Before the first release,
the `dprint-plugin-pug` npm package name must be claimed and configured with a
trusted publisher entry for `.github/workflows/release.yml`.
