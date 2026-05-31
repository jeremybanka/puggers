# dprint-plugin-pug

Tiny `dprint` Wasm plugin for a small subset of Pug.

## Scope

This intentionally minimal formatter currently understands:

- indentation-based nesting
- tags
- `.class` and `#id` shorthand
- inline text
- `//` comments

Everything else is preserved as text where possible instead of trying to be clever.

## Use with dprint

```json
{
  "plugins": [
    "target/wasm32-unknown-unknown/release/dprint_plugin_pug.wasm"
  ]
}
```

## Reference docs

A dated local Pug reference set lives at `docs/pug/2026-05-31/`.

## Build

```sh
mise exec rust -- cargo build --target wasm32-unknown-unknown --release
```

The plugin artifact will be at:

```txt
target/wasm32-unknown-unknown/release/dprint_plugin_pug.wasm
```
