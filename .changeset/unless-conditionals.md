---
default: patch
---

### Avoid false warning diagnostics for `unless` conditionals

The formatter now models Pug `unless` statements as control flow, so valid `unless`/`else` branches no longer report orphaned `else` recovery warnings.
