# Changelog

All notable user-facing changes to `puggers` will be documented here.

## 0.1.7 (2026-06-30)

### Fixes

#### Avoid false warning diagnostics for `unless` conditionals

The formatter now models Pug `unless` statements as control flow, so valid `unless`/`else` branches no longer report orphaned `else` recovery warnings.

## 0.1.6 (2026-06-24)

### Fixes

- Keep the dprint plugin compatible with dprint-core 0.68

## 0.1.5 (2026-06-05)

### Fixes

#### Normalize structural Pug whitespace more safely

The formatter now normalizes structural Pug whitespace more safely and more consistently. It compacts repeated spaces and tabs in statement heads, handles blank-line compaction more reliably across structural contexts, recovers some missing separators, preserves significant trailing inline whitespace, and now formats inline attributes in canonical space-separated form instead of inserting commas.

## 0.1.4 (2026-06-03)

### Fixes

#### Preserve HTML comments more faithfully during import

The HTML importer now keeps empty comments, preserves spacing-sensitive comment
payloads, and emits multiline or whitespace-sensitive comments as pipeless Pug
comment blocks instead of trimming or flattening them by default.

#### Add an opt-in HTML import mode that preserves meaningful inline text spacing

The `puggers` HTML importer now supports a preserve-more text whitespace mode
for keeping significant spaces around inline content such as adjacent tags and
mixed text/tag prose, while leaving the default aggressive normalization
behavior unchanged.

#### Reflow imported HTML prose to line width

When `lineWidth` is configured, the HTML importer now emits dotted prose blocks
for plain text that should be reflowed and wraps those blocks to the configured
width while still treating ordinary HTML source line breaks as collapsible
whitespace.

#### Respect configured line width for inline HTML import output

The HTML importer now accepts a `lineWidth`-style option and moves long inline
`tag text` output into multiline `tag` plus piped-text form when the rendered
line would exceed the configured width.

## 0.1.3 (2026-06-03)

### Fixes

#### Improve formatter recovery for malformed Pug input

The `dprint-plugin-pug` formatter now recovers more gracefully from malformed
Pug instead of dropping later structure when it encounters inconsistent
indentation or invalid control-flow heads. It also records warning-level
diagnostics for recoverable issues such as orphaned `else`/`default` branches,
missing `include`/`extends` paths, and bare `when`/`while` expressions, while
avoiding warnings for valid loop `else` branches, `default:` case shorthand,
blank-line-separated child blocks, and multiline mixin-call argument layout.

## 0.1.2 (2026-06-02)

### Fixes

#### Model more Pug syntax explicitly

The `dprint-plugin-pug` parser now distinguishes operator-prefixed code,
control-flow heads, `include`, `extends`, `block`, mixin declarations, and
mixin calls instead of treating those forms as generic tags or opaque raw
statements. This preserves existing permissive formatting behavior while making
real-world Pug files safer to analyze, test, and extend with future validation
and formatting rules.

#### Preserve distinct comments and explicit filter blocks in the Pug formatter

The `dprint-plugin-pug` formatter now keeps `//` and `//-` comments distinct,
preserves pipeless comment payloads, and treats filter blocks such as
`:markdown-it`, `:css`, and `:javascript` as explicit raw-payload constructs
instead of generic dotted blocks. This makes comment and filter formatting more
predictable while preserving blank-line-sensitive payload content losslessly.

## 0.1.1 (2026-06-02)

### Fixes

#### Normalize Pug attribute formatting

The `dprint-plugin-pug` formatter now parses tag attributes into structured
entries instead of preserving them as opaque text. This normalizes attribute
spacing, keeps attribute ordering stable, preserves boolean attributes, and
defaults quoted attribute output to double quotes with configurable single-quote
support.

#### Add conservative `lineWidth` support for Pug formatting

The `dprint-plugin-pug` formatter now reads `lineWidth` from configuration and
uses it for conservative wrapping in two places: long tag attribute lists can be
split across multiple lines when they exceed the configured width, and plain
low-risk dotted prose blocks can be reflowed to fit the configured width.
Piped text, interpolation-heavy inline text, literal HTML text, and raw dotted
blocks such as `script.`, `style.`, `pre.`, and `textarea.` remain excluded
from width-driven reflow.

#### Classify Pug text forms more conservatively

The `dprint-plugin-pug` parser now distinguishes prose-like dotted blocks,
raw-text dotted blocks, piped text lines, and inline text forms instead of
treating them as a single broad text category. This keeps whitespace-sensitive
content safer to format, preserves significant trailing spaces, and lays the
groundwork for future width-aware formatting decisions.

## 0.1.0 (2026-06-01)

### Features

- Initial release of the `puggers` workspace, including the `puggers-core`
  shared library, the `puggers` CLI, and the `dprint-plugin-pug` formatter
  plugin.
