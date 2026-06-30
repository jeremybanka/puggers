# puggers

Native npm package for converting HTML to Pug.

Requires Node 26 or newer.

```sh
pnpm add puggers
```

```ts
import { convertHtmlToPug } from "puggers";

const pug = convertHtmlToPug("<main><h1>Hello</h1></main>", {
  root: "main"
});
```

```sh
puggers --root main input.html
```

The npm package is ESM-only. It loads a platform-specific native package for
the Node-API runtime converter and forwards the CLI to the same Rust binary
used by the Cargo package.

Browser and bundler-oriented wasm builds are intentionally out of scope for
this package surface.
