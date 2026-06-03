---
default: patch
---

### Reflow imported HTML prose to line width and preserve source paragraph breaks

When `lineWidth` is configured, the HTML importer now emits dotted prose blocks
for plain text that should be reflowed, wraps those blocks to the configured
width, and treats source line breaks as paragraph boundaries in the imported
Pug output.
