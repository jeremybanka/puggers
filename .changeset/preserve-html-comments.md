---
default: patch
---

### Preserve HTML comments more faithfully during import

The HTML importer now keeps empty comments, preserves spacing-sensitive comment
payloads, and emits multiline or whitespace-sensitive comments as pipeless Pug
comment blocks instead of trimming or flattening them by default.
