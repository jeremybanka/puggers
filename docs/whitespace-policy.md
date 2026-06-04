# Pug Whitespace Policy

This note captures the current formatter policy behind tasks `0010` and `0011`.

## Safe to normalize

- Attribute separators inside `(...)` may be spaces, tabs, commas, or newlines in the input.
- The formatter rewrites those separators to the canonical inline form `, `, or to one attribute per line when wrapping.
- A missing separator after a quoted attribute value may be recovered when the next token is unambiguously another attribute.
- A missing separator between a closed attribute list and inline tag text may be recovered as one space.
- Tabs used only as structural separators are treated the same as spaces and normalize to a single space in formatted output.

## Conditionally safe

- Whitespace immediately after a tag head is split into one structural separator plus any remaining text whitespace.
- This allows the formatter to normalize the separator itself without erasing authored leading text whitespace.
- Example: `p   hello` stays `p   hello`, but the model is `p` + one separator + text `  hello`.

## Must preserve exactly

- Piped text, dotted prose, raw text blocks, and inline text trailing whitespace remain content, not decoration.
- A line like `ul ` is treated as a tag with a trailing text space, not as a line with disposable trailing formatting.
- Tabs and spaces that survive after the structural separator are preserved as text content.

## Unsafe to compact blindly

- Removing trailing whitespace from inline tag text can change rendered output.
- Collapsing all whitespace after a tag head to one space can erase meaningful leading text whitespace.
- Treating every missing boundary as recoverable is unsafe outside grammar-closed regions such as attribute lists.
