# puggers

Workspace for a small family of Pug tools.

## Crates

- `crates/dprint-plugin-pug`: the existing tiny `dprint` formatter plugin
- `crates/puggers-html`: Rust library for converting HTML into Pug
- `crates/puggers-cli`: standalone CLI built on top of `puggers-html`

## CLI

```sh
cargo run -p puggers-cli -- --help
```

Example:

```sh
cargo run -p puggers-cli -- \
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
