---
default: patch
---

### Respect configured line width for inline HTML import output

The HTML importer now accepts a `lineWidth`-style option and moves long inline
`tag text` output into multiline `tag` plus piped-text form when the rendered
line would exceed the configured width.
