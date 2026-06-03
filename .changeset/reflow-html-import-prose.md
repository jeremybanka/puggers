---
default: patch
---

### Reflow imported HTML prose to line width

When `lineWidth` is configured, the HTML importer now emits dotted prose blocks
for plain text that should be reflowed and wraps those blocks to the configured
width while still treating ordinary HTML source line breaks as collapsible
whitespace.
