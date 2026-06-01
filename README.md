# puggers

Workspace for a small family of Pug tools.

## Crates

- `crates/dprint-plugin-pug`: the existing tiny `dprint` formatter plugin
- `crates/puggers-core`: shared Rust library for conversion and other reusable logic
- `crates/puggers-cli`: the `puggers` CLI package built on top of `puggers-core`

## CLI

```sh
cargo run -p puggers -- --help
```

Example:

```sh
cargo run -p puggers -- \
  --trim-outer-document \
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

After the initial manual publish, GitHub Actions automatically prepares release
PRs from pushes to `main` and publishes via crates.io trusted publishing
(OIDC).
