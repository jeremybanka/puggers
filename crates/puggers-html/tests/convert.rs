use std::collections::BTreeSet;

use puggers_html::{ConvertOptions, convert_html_to_pug};

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
    let output = convert_html_to_pug(
        "<div id=\"shell\" class=\"layout stack\" data-x=\"1\"><a href=\"/docs\">Docs</a></div>",
        &ConvertOptions::default(),
    );

    assert_eq!(output, "html\n  head\n  body\n    div\n      a Docs\n");
}

#[test]
fn keeps_allowlisted_attributes_and_prefers_pug_shorthand() {
    let output = convert_html_to_pug(
        "<div id=\"shell\" class=\"layout stack\" data-x=\"1\"><a href=\"/docs\">Docs</a></div>",
        &options_with_attributes(&["id", "class", "href"]),
    );

    assert_eq!(
        output,
        "html\n  head\n  body\n    div#shell.layout.stack\n      a(href=\"/docs\") Docs\n"
    );
}

#[test]
fn trims_the_outer_document_when_requested() {
    let output = convert_html_to_pug(
        "<!doctype html><html><head><title>Ignored</title></head><body><main><h1>Hello</h1></main></body></html>",
        &ConvertOptions {
            trim_outer_document: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "main\n  h1 Hello\n");
}

#[test]
fn collapses_single_nested_anonymous_divs() {
    let output = convert_html_to_pug(
        "<div><div><section><p>Hello</p></section></div></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "section\n  p Hello\n");
}

#[test]
fn preserves_raw_text_blocks_for_script_and_textarea() {
    let output = convert_html_to_pug(
        "<textarea>\nline one\n  line two\n</textarea><script>console.log('hi');</script>",
        &ConvertOptions {
            trim_outer_document: true,
            ..Default::default()
        },
    );

    assert_eq!(
        output,
        "textarea.\n  line one\n    line two\nscript.\n  console.log('hi');\n"
    );
}

#[test]
fn supports_tab_indentation() {
    let output = convert_html_to_pug(
        "<div><p>Hello</p></div>",
        &ConvertOptions {
            trim_outer_document: true,
            use_tabs: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "div\n\tp Hello\n");
}
