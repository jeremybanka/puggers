---
default: patch
---

### Add explicit HTML import root selection

HTML import now supports a `--root` path such as `html>body article`, plus a
fallible core conversion API that reports missing roots with a typed error.
