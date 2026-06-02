---
puggers: patch
---

### Model more Pug syntax explicitly

The `dprint-plugin-pug` parser now distinguishes operator-prefixed code,
control-flow heads, `include`, `extends`, `block`, mixin declarations, and
mixin calls instead of treating those forms as generic tags or opaque raw
statements. This preserves existing permissive formatting behavior while making
real-world Pug files safer to analyze, test, and extend with future validation
and formatting rules.
