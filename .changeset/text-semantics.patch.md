---
default: patch
---

### Classify Pug text forms more conservatively

The `dprint-plugin-pug` parser now distinguishes prose-like dotted blocks,
raw-text dotted blocks, piped text lines, and inline text forms instead of
treating them as a single broad text category. This keeps whitespace-sensitive
content safer to format, preserves significant trailing spaces, and lays the
groundwork for future width-aware formatting decisions.
