# dprint-plugin-pug

`dprint-plugin-pug` is a small `dprint` formatter plugin for `.pug` files.

## Example

Build the plugin:

```sh
cargo build -p dprint-plugin-pug --target wasm32-unknown-unknown --release
```

Point `dprint` at the generated Wasm file:

```json
{
  "plugins": [
    "target/wasm32-unknown-unknown/release/dprint_plugin_pug.wasm"
  ],
  "pug": {
    "quoteStyle": "single",
    "lineWidth": 80
  }
}
```
