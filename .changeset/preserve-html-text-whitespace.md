---
default: patch
---

### Add an opt-in HTML import mode that preserves meaningful inline text spacing

The `puggers` HTML importer now supports a preserve-more text whitespace mode
for keeping significant spaces around inline content such as adjacent tags and
mixed text/tag prose, while leaving the default aggressive normalization
behavior unchanged.
