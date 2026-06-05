---
default: patch
---

### Normalize structural Pug whitespace more safely

The formatter now normalizes structural Pug whitespace more safely and more consistently. It compacts repeated spaces and tabs in statement heads, handles blank-line compaction more reliably across structural contexts, recovers some missing separators, preserves significant trailing inline whitespace, and now formats inline attributes in canonical space-separated form instead of inserting commas.
