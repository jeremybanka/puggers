---
default: patch
---

### Preserve distinct comments and explicit filter blocks in the Pug formatter

The `dprint-plugin-pug` formatter now keeps `//` and `//-` comments distinct,
preserves pipeless comment payloads, and treats filter blocks such as
`:markdown-it`, `:css`, and `:javascript` as explicit raw-payload constructs
instead of generic dotted blocks. This makes comment and filter formatting more
predictable while preserving blank-line-sensitive payload content losslessly.
