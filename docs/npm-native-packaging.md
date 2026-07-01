# Native npm Packaging

The `puggers` npm package is Node-first and native-backed.

The top-level package is ESM-only TypeScript built with `tsdown`. It does not
contain converter logic. Runtime calls load a Node-API addon from the installed
platform package, and the npm `puggers` bin forwards directly to the native Rust
CLI executable.

This keeps the CLI implementation in Rust while giving Node users a typed
`convertHtmlToPug` API.

## Package Layout

- `packages/puggers`: top-level ESM package users install as `puggers`
- `crates/puggers-node`: Node-API wrapper around `puggers-core`
- `packages/native/*`: workspace platform packages that receive generated
  native artifacts for local development
- `@puggers/<target>`: generated native packages containing:
  - `puggers` or `puggers.exe`
  - `puggers.node`

The first npm surface is not a browser or bundler package. A wasm package can
be added later when browser support has its own tests and initialization model.

## Versioning

The npm packages use the same version as the Rust workspace and crates. Knope
updates the top-level `packages/puggers/package.json` and the
`packages/native/*/package.json` platform packages during release preparation.

Keep those package versions aligned: pnpm rewrites the top-level package's
`workspace:*` optional dependencies to the linked package versions when packing
or publishing.

## Local Build

Install the declared toolchain:

```sh
mise install
pnpm install
```

The npm package targets Node 26 or newer.

Build the TypeScript package, then build the current host's Rust native target
and copy the artifacts into the matching `packages/native/*` workspace package:

```sh
just build-npm
```

Run the Node tests:

```sh
just test-npm
```

## Native Package Artifacts

The `scripts/npm-stage.node.ts` helper stages native files and package metadata.
`pnpm publish` stays in the Justfile so release actions remain visible at the
command layer. Its routes are `copy-binaries`, `write-manifest`, and
`print-staging-path`.

Build and copy binaries for the current host target:

```sh
just build-npm-native
```

Pass an explicit target in CI matrix jobs:

```sh
just build-npm-native --target=linux-x64-glibc
```

Use `--target=<target>` or `PUGGERS_NPM_TARGET=<target>` to override host target
detection.

When publishing artifacts built outside the default Cargo target directory, pass
the same target and point the staging helper at those files:

```sh
PUGGERS_RELEASE_DIR=target/release just publish-npm-native --target=linux-x64-glibc
```

Supported target names are:

- `darwin-arm64`
- `darwin-x64`
- `linux-arm64-glibc`
- `linux-x64-glibc`
- `win32-arm64`
- `win32-x64`

Each CI job builds one supported npm target. Host targets use Cargo's default
target directory, while alternate or cross targets use `cargo build --target`
and expect the corresponding Rust target and linker to be installed. When
packaging externally built artifacts, set `PUGGERS_EXE` and
`PUGGERS_NODE_ADDON` to the already-built files.

## Publishing Order

Publish all native packages first:

```sh
just publish-npm-native --target=linux-x64-glibc
```

Then publish the top-level package:

```sh
just publish-npm
```

Use npm trusted publishing through CI OIDC for provenance once the npm package
names have been claimed and the trusted publisher relationships are configured.
