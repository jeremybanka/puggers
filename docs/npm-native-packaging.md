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
- `packages/native/*`: private workspace placeholders so pnpm can resolve the
  optional native package names locally before they are published
- `@puggers/<target>`: generated native packages containing:
  - `puggers` or `puggers.exe`
  - `puggers.node`

The first npm surface is not a browser or bundler package. A wasm package can
be added later when browser support has its own tests and initialization model.

## Versioning

The npm packages use the same version as the Rust workspace and crates. Knope
updates the top-level `packages/puggers/package.json` and the private
`packages/native/*/package.json` placeholders during release preparation.

Keep those placeholder versions aligned: pnpm rewrites the top-level package's
`workspace:*` optional dependencies to the linked package versions when packing
or publishing.

## Local Build

Install the declared toolchain:

```sh
mise install
pnpm install
```

The npm package targets Node 26 or newer.

Build the Rust native artifacts, copy them into `packages/puggers/.native`, and
compile the TypeScript package:

```sh
just build-npm
```

Run the Node tests:

```sh
just test-npm
```

## Native Package Artifacts

Generate a native package directory for the current host target:

```sh
just dist-npm-native-directory
```

Generate and pack a tarball:

```sh
just dist-npm-native
```

Pass an explicit target when packaging artifacts built elsewhere:

```sh
PUGGERS_RELEASE_DIR=target/release just dist-npm-native linux-x64-glibc
```

Supported target names are:

- `darwin-arm64`
- `darwin-x64`
- `linux-arm64-glibc`
- `linux-arm64-musl`
- `linux-x64-glibc`
- `linux-x64-musl`
- `win32-arm64`
- `win32-x64`

Cross-compilation is intentionally not hidden in the package script. When
packaging for a non-host target, set `PUGGERS_EXE` and `PUGGERS_NODE_ADDON` to
the already-built artifacts.

## Publishing Order

Publish all native packages first:

```sh
just publish-npm-native linux-x64-glibc
```

Then publish the top-level package:

```sh
just publish-npm
```

For CI-backed publishes with npm provenance, use the provenance variants:

```sh
just publish-npm-native-provenance linux-x64-glibc
just publish-npm-provenance
```

Use npm trusted publishing/provenance for both package layers once the npm
package names have been claimed and the trusted publisher relationships are
configured.
