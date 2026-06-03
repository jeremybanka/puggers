---
default: patch
---

### Improve formatter recovery for malformed Pug input

The `dprint-plugin-pug` formatter now recovers more gracefully from malformed
Pug instead of dropping later structure when it encounters inconsistent
indentation or invalid control-flow heads. It also records warning-level
diagnostics for recoverable issues such as orphaned `else`/`default` branches,
missing `include`/`extends` paths, and bare `when`/`while` expressions, while
avoiding warnings for valid loop `else` branches, `default:` case shorthand,
blank-line-separated child blocks, and multiline mixin-call argument layout.
