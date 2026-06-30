# puggers-core

`puggers-core` is a set of utilities for working with Pug files.

## Example

```rust
use std::collections::BTreeSet;

use puggers_core::{
    ConvertOptions, PugFormatOptions, QuoteStyle, RootSelection, convert_html_to_pug,
};

let allowed_attributes = BTreeSet::from([
    String::from("class"),
]);

let output = convert_html_to_pug(
    r#"<main><article class="card"><p>Hello</p></article></main>"#,
    &ConvertOptions {
        allowed_attributes,
        root: Some(RootSelection::parse("main>article").expect("root path should parse")),
        formatting: PugFormatOptions {
            quote_style: QuoteStyle::Single,
            ..Default::default()
        },
        ..Default::default()
    },
)
.expect("root should match");

assert_eq!(output, "article.card\n  p Hello\n");
```
