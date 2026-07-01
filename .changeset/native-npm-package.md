---
default: minor
---

### Add a native npm package surface

Add an ESM-only `puggers` npm package with TypeScript declarations, a native
Node-API `convertHtmlToPug` runtime function, and a CLI bin that forwards to the
Rust `puggers` executable instead of reimplementing CLI parsing in JavaScript.
