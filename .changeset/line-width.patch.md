---
puggers: patch
---

### Add initial `lineWidth` support for long attribute lists

The `dprint-plugin-pug` formatter now reads `lineWidth` from configuration and
uses it for a conservative first wrapping rule: long tag attribute lists can be
split across multiple lines when they exceed the configured width. Text blocks,
piped text, interpolation-heavy inline text, and other whitespace-sensitive
forms remain excluded from width-driven reflow for now.
