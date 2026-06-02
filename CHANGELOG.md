# Changelog

All notable user-facing changes to `puggers` will be documented here.

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
