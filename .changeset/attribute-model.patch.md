---
puggers: patch
---

### Normalize Pug attribute formatting

The `dprint-plugin-pug` formatter now parses tag attributes into structured
entries instead of preserving them as opaque text. This normalizes attribute
spacing, keeps attribute ordering stable, preserves boolean attributes, and
defaults quoted attribute output to double quotes with configurable single-quote
support.
