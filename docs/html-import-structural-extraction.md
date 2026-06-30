# HTML Import Structural Extraction Policy

The HTML importer treats structure as signal unless a user asks for a narrower
view. Structural extraction is separate from formatter compatibility and text
layout: formatter settings decide how generated Pug is rendered, while importer
structural policy decides which HTML scaffolding is represented at all.

## Document Shells

By default, import preserves the parsed document shell. A full document can
therefore emit `doctype html`, `html`, `head`, and `body` nodes.

`trim_outer_document` is the only document-shell selector today. When enabled,
it emits all parsed `body` children. It does not select `main`, discard
headers, discard footers, trim front matter, trim back matter, or accept a CSS
selector.

Broader region selection is intentionally deferred. Selector-based extraction
needs a clear API for misses, multiple matches, diagnostics, and CLI/npm
behavior. Until that shape exists, `trim_outer_document` stays a body-shell
control rather than becoming a proxy for content-region extraction.

## Single-Child Collapse

`collapse_single_nested` controls source-anonymous single-child element chains.
The importer only treats an element as a collapsible link when it has exactly
one element child, no source attributes, no raw-text payload, and no kept text or
comment siblings.

The supported modes are:

| Mode | Behavior |
| --- | --- |
| `Off` | Preserve the chain exactly as imported. |
| `TopWins` | Keep the outermost tag and attach the innermost children to it. |
| `BottomWins` | Keep the innermost collapsible tag and attach its children to it. |
| `BestTagWins` | Keep the highest-ranked tag in the chain and attach the innermost children to it. |

`BestTagWins` uses a puggers-owned internal hierarchy. The current order is
`main`, `article`, `section`, `nav`, `aside`, `header`, `footer`, `form`,
`table`, `ul`, `ol`, `dl`, `figure`, and `blockquote`. Unknown non-`div` tags
rank ahead of `div` so `div > section > div` collapses to `section`.

## Classification

Safe to collapse:

- Source-anonymous single-child element chains when the selected mode is not
  `Off`.
- Kept representative tags whose source attributes were empty.
- Chains that crossed comments only because comments were explicitly dropped.

Unsafe to collapse:

- Elements with source attributes, even when those attributes are filtered from
  the rendered Pug output.
- Elements with multiple children.
- Elements with kept comments or text siblings.
- Raw-text elements such as `pre`, `script`, `style`, and `textarea`.

Option-dependent:

- Whether a collapsible chain is preserved, reduced to the top tag, reduced to
  the bottom tag, or reduced by best-tag ranking.
- Whether comments count as structure, because `drop_comments` removes them
  before collapse policy runs.
- Whether a future user-facing profile chooses collapse by default. That belongs
  to the importer intent work, not this structural policy layer.
