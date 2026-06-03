use std::collections::BTreeSet;

use puggers_core::{ConvertOptions, convert_html_to_pug};

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
fn keeps_short_inline_text_inline_when_line_width_allows_it() {
    let output = convert_html_to_pug(
        "<p>Hello there</p>",
        &ConvertOptions {
            trim_outer_document: true,
            line_width: Some(40),
            ..Default::default()
        },
    );

    assert_eq!(output, "p Hello there\n");
}

#[test]
fn reflows_long_inline_prose_into_a_dotted_block_when_line_width_is_exceeded() {
    let output = convert_html_to_pug(
        "<p>Hello there this sentence should not stay inline once line width is small</p>",
        &ConvertOptions {
            trim_outer_document: true,
            line_width: Some(30),
            ..Default::default()
        },
    );

    assert_eq!(
        output,
        "p.\n  Hello there this sentence\n  should not stay inline once\n  line width is small\n"
    );
}

#[test]
fn counts_tag_shorthand_and_attributes_when_deciding_to_wrap_inline_text() {
    let output = convert_html_to_pug(
        "<a class=\"link primary\" href=\"/docs\">Documentation</a>",
        &ConvertOptions {
            trim_outer_document: true,
            line_width: Some(24),
            ..options_with_attributes(&["class", "href"])
        },
    );

    assert_eq!(
        output,
        "a.link.primary(href=\"/docs\")\n  | Documentation\n"
    );
}

#[test]
fn treats_source_line_breaks_as_collapsible_whitespace_when_reflowing_prose() {
    let output = convert_html_to_pug(
        "<p>First paragraph should wrap here\nSecond paragraph also wraps</p>",
        &ConvertOptions {
            trim_outer_document: true,
            line_width: Some(24),
            ..Default::default()
        },
    );

    assert_eq!(
        output,
        "p.\n  First paragraph should\n  wrap here Second\n  paragraph also wraps\n"
    );
}
