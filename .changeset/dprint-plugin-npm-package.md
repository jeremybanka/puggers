---
default: minor
---

### Publish the dprint plugin as an npm package

Add a `dprint-plugin-pug` npm package that stages the release Wasm plugin as
`plugin.wasm`, exposes a dprint-compatible `getPath()` entrypoint, and wires the
package into the npm release workflow with trusted publishing.
