# puggers

`puggers` is the command-line interface for converting HTML files or stdin into Pug using `puggers-core`.

## Example

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
