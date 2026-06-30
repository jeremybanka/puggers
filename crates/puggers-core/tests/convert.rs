use std::collections::BTreeSet;

use puggers_core::{
    CollapseSingleNestedMode, ConvertOptions, PugFormatOptions, QuoteStyle, RootSelection,
    convert_html_to_pug,
};

fn convert(input: &str, options: &ConvertOptions) -> String {
    convert_html_to_pug(input, options).expect("conversion should succeed")
}

fn root(value: &str) -> RootSelection {
    RootSelection::parse(value).expect("root path should parse")
}

fn options_with_attributes(attributes: &[&str]) -> ConvertOptions {
    ConvertOptions {
        allowed_attributes: attributes
            .iter()
            .map(|value| value.to_string())
            .collect::<BTreeSet<_>>(),
        ..Default::default()
    }
}

#[test]
fn removes_all_attributes_by_default() {
    let output = convert(
        "<div id=\"shell\" class=\"layout stack\" data-x=\"1\"><a href=\"/docs\">Docs</a></div>",
        &ConvertOptions::default(),
    );

    assert_eq!(output, "html\n  head\n  body\n    div\n      a Docs\n");
}

#[test]
fn keeps_allowlisted_attributes_and_prefers_pug_shorthand() {
    let output = convert(
        "<div id=\"shell\" class=\"layout stack\" data-x=\"1\"><a href=\"/docs\">Docs</a></div>",
        &options_with_attributes(&["id", "class", "href"]),
    );

    assert_eq!(
        output,
        "html\n  head\n  body\n    div#shell.layout.stack\n      a(href=\"/docs\") Docs\n"
    );
}

#[test]
fn selects_the_requested_root() {
    let output = convert(
        "<!doctype html><html><head><title>Ignored</title></head><body><main><h1>Hello</h1></main></body></html>",
        &ConvertOptions {
            root: Some(root("html>body>main")),
            ..Default::default()
        },
    );

    assert_eq!(output, "main\n  h1 Hello\n");
}

#[test]
fn collapses_single_nested_anonymous_divs() {
    let output = convert(
        "<div><div><section><p>Hello</p></section></div></div>",
        &ConvertOptions {
            root: Some(root("div")),
            collapse_single_nested: CollapseSingleNestedMode::BestTagWins,
            ..Default::default()
        },
    );

    assert_eq!(output, "section\n  p Hello\n");
}

#[test]
fn preserves_raw_text_blocks_for_script_and_textarea() {
    let output = convert(
        "<textarea>\nline one\n  line two\n</textarea><script>console.log('hi');</script>",
        &ConvertOptions::default(),
    );

    assert_eq!(
        output,
        "html\n  head\n  body\n    textarea.\n      line one\n        line two\n    script.\n      console.log('hi');\n"
    );
}

#[test]
fn supports_tab_indentation() {
    let output = convert(
        "<div><p>Hello</p></div>",
        &ConvertOptions {
            root: Some(root("div")),
            formatting: PugFormatOptions {
                use_tabs: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    assert_eq!(output, "div\n\tp Hello\n");
}

#[test]
fn supports_configured_space_indentation_width() {
    let output = convert(
        "<div><p>Hello</p></div>",
        &ConvertOptions {
            root: Some(root("div")),
            formatting: PugFormatOptions {
                indent_width: 4,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    assert_eq!(output, "div\n    p Hello\n");
}

#[test]
fn renders_imported_attributes_with_configured_quote_style() {
    let output = convert(
        "<a href=\"/docs\" title=\"Jeremy's docs\">Docs</a>",
        &ConvertOptions {
            root: Some(root("a")),
            formatting: PugFormatOptions {
                quote_style: QuoteStyle::Single,
                ..Default::default()
            },
            ..options_with_attributes(&["href", "title"])
        },
    );

    assert_eq!(output, "a(href='/docs', title='Jeremy\\'s docs') Docs\n");
}
