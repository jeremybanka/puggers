---
default: minor
---

### Remove trim outer document in favor of root selection

Removed `trim_outer_document` and `--trim-outer-document`; use explicit
`root` / `--root` selection instead. The main core conversion API is now
fallible so missing roots return an error instead of being hidden.
