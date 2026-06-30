# puggers-core

`puggers-core` is a set of utilities for working with Pug files.

## Example

```rust
use std::collections::BTreeSet;

use puggers_core::{ConvertOptions, PugFormatOptions, QuoteStyle, convert_html_to_pug};

let allowed_attributes = BTreeSet::from([
    String::from("class"),
]);

let output = convert_html_to_pug(
    r#"<div class="card"><p>Hello</p></div>"#,
    &ConvertOptions {
        allowed_attributes,
        trim_outer_document: true,
        formatting: PugFormatOptions {
            quote_style: QuoteStyle::Single,
            ..Default::default()
        },
        ..Default::default()
    },
);

assert_eq!(output, "div.card\n  p Hello\n");
```
