---
default: patch
---

### Normalize structural Pug whitespace more safely

The formatter now normalizes space-, tab-, and newline-separated attribute lists more consistently, recovers some missing separators, and preserves significant trailing inline whitespace instead of dropping it.
