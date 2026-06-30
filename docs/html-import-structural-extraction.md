# HTML Import Structural Extraction Policy

The HTML importer treats structure as signal unless a user asks for a narrower
view. Structural extraction is separate from formatter compatibility and text
layout: formatter settings decide how generated Pug is rendered, while importer
structural policy decides which HTML scaffolding is represented at all.

## Root Selection

By default, import preserves the parsed document shell. A full document can
therefore emit `doctype html`, `html`, `head`, and `body` nodes.

`root` / `--root` selects the first element matching a small root path grammar
and emits that element as the generated Pug root. For example,
`--root 'html>body article'` selects the first `article` descendant found under
the direct `html > body` path.

The root path grammar intentionally starts small:

- A tag name selects an element by tag.
- `>` means direct child.
- Whitespace means descendant.
- Matching uses the first complete path match in document order.

If a root path does not match, `convert_html_to_pug` returns
`ConvertError::RootNotFound`. The CLI reports that error, writes no converted
Pug to stdout, and exits unsuccessfully.

The older `trim_outer_document` / `--trim-outer-document` control has been
removed. New region-selection behavior should use `root` / `--root` so callers
can name the structural region they actually want.

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
