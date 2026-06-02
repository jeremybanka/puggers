---
default: patch
---

### Add conservative `lineWidth` support for Pug formatting

The `dprint-plugin-pug` formatter now reads `lineWidth` from configuration and
uses it for conservative wrapping in two places: long tag attribute lists can be
split across multiple lines when they exceed the configured width, and plain
low-risk dotted prose blocks can be reflowed to fit the configured width.
Piped text, interpolation-heavy inline text, literal HTML text, and raw dotted
blocks such as `script.`, `style.`, `pre.`, and `textarea.` remain excluded
from width-driven reflow.
