# puggers

`puggers` is the command-line interface for turning bulky HTML into a more
concise Pug view.

It is best thought of as a configurable signal-extraction tool, not just a
literal HTML importer. Depending on what you care about, "signal" might mean
readable document structure, preserved comments, retained `class` attributes,
or a minimal outline with most noise removed. The CLI sits on top of
`puggers-core` and lets you tune that tradeoff with a small set of conversion
options.

## What It Does

`puggers` parses HTML, removes or preserves selected details, and emits Pug
that is easier to scan and edit.

Current controls let you:

- keep selected attributes with repeated `--allow-attr`
- trim the outer `html` / `head` / `body` shell with `--trim-outer-document`
- collapse anonymous nested wrappers with `--collapse-single-nested`
- preserve meaningful spaces around inline content with `--preserve-text-whitespace`
- drop comments with `--drop-comments`
- choose tabs or spaces with `--use-tabs` and `--indent-width`
- reflow prose and wrap long inline tag text with `--line-width`

By default, the converter is opinionated: it strips most attributes, keeps
comments, and aggressively normalizes ordinary text. That makes it useful for
finding structure quickly, even when the source is noisy.

## Use Cases

Readable document view:

```sh
printf '%s' '<article><h1>Hello</h1><p>Intro</p></article>' \
  | cargo run -p puggers -- --trim-outer-document
```

Inspect structure and Tailwind-heavy markup:

```sh
printf '%s' '<div class="grid gap-6 md:grid-cols-2"><a href="/docs">Docs</a></div>' \
  | cargo run -p puggers -- \
      --trim-outer-document \
      --allow-attr class \
      --allow-attr href
```

## Example Output

```sh
printf '<div class="card"><a href="/docs">Docs</a></div>' \
  | cargo run -p puggers -- \
      --trim-outer-document \
      --allow-attr class \
      --allow-attr href
```

```pug
div.card
  a(href="/docs") Docs
```

## Help

```sh
cargo run -p puggers -- --help
```
