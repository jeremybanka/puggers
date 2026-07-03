# dprint-plugin-pug

Wasm npm package for the puggers dprint Pug formatter plugin.

## Install

For package-manager-managed projects, install the package and use the local
`node_modules` resolver:

```sh
pnpm add -D dprint-plugin-pug
```

```json
{
  "plugins": ["npm:dprint-plugin-pug"],
  "pug": {
    "quoteStyle": "single",
    "lineWidth": 80,
    "indentWidth": 2,
    "useTabs": false
  }
}
```

Use a pinned npm specifier when the dprint configuration itself should declare
the exact registry version:

```json
{
  "plugins": ["npm:dprint-plugin-pug@0.1.9"]
}
```

The package entrypoint exports `getPath(): string` for tools that need the
absolute path to `plugin.wasm` directly.
