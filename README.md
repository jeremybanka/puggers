# puggers

Workspace for a small family of Pug tools.

## Tooling

This repo uses `mise` as the source of truth for local tool versions, including
Rust, Node, and pnpm.

```sh
mise install
pnpm install
```

## Formatting

Build the local `dprint` plugin, then run `dprint` against the repo config:

```sh
just build-wasm
dprint fmt
```

The repo's `dprint.json` points at
`target/wasm32-unknown-unknown/release/dprint_plugin_pug.wasm` and excludes the
checked-in upstream fixture corpus from bulk formatting.

## Crates

- `crates/dprint-plugin-pug`: the existing tiny `dprint` formatter plugin
- `crates/puggers-core`: shared Rust library for conversion and other reusable logic
- `crates/puggers-cli`: the `puggers` CLI package built on top of `puggers-core`
- `crates/puggers-node`: native Node-API bindings for the npm package

## CLI

```sh
cargo run -p puggers -- --help
```

## npm

The npm package lives in `packages/puggers`. It is an ESM-only TypeScript
package built with `tsdown`, backed by native platform packages that carry the
Rust CLI executable and Node-API addon.

```sh
just build-npm
just test-npm
```

The native npm packaging policy and publish order are documented in
[`docs/npm-native-packaging.md`](docs/npm-native-packaging.md).

Example:

```sh
cargo run -p puggers -- \
  --root 'html>body main' \
  --allow-attr id \
  --allow-attr class \
  --allow-attr href \
  path/to/input.html
```

## Tests

```sh
cargo test
```

## Releases

Puggers uses a declarative release-note workflow with Knope.

```sh
just notes
just version
just publish
```

Release notes live in `.changeset/`, the coordinated changelog lives in
`CHANGELOG.md`, and the release architecture and workflow are documented in
`docs/release-packaging-architecture.md` and `docs/release-notes-workflow.md`.

After the initial manual publish, GitHub Actions automatically keeps a
`release` PR updated from pushes to `main` using Knope's pull-request flow and
publishes merged releases via crates.io trusted publishing (OIDC).
