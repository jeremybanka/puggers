mod support;

pub use support::{ast, config, format_source, formatter, lexer, parser};

use config::Configuration;
use puggers_core::{ConvertOptions, PugFormatOptions, QuoteStyle, convert_html_to_pug};
use std::collections::BTreeSet;

#[test]
fn formatter_configuration_maps_to_shared_format_options() {
    let formatting = Configuration {
        indent_width: Some(4),
        line_width: Some(72),
        quote_style: Some(QuoteStyle::Single),
        use_tabs: Some(true),
    }
    .format_options();

    assert_eq!(
        formatting,
        PugFormatOptions {
            indent_width: 4,
            line_width: Some(72),
            use_tabs: true,
            quote_style: QuoteStyle::Single,
        }
    );
}

#[test]
fn formatter_and_converter_share_quote_style_for_attribute_values() {
    let formatter_output = format_source(
        "a(title=\"Jeremy's docs\") Docs\n",
        &Configuration {
            quote_style: Some(QuoteStyle::Single),
            ..Default::default()
        },
    );

    let converter_output = convert_html_to_pug(
        "<a title=\"Jeremy's docs\">Docs</a>",
        &ConvertOptions {
            allowed_attributes: BTreeSet::from([String::from("title")]),
            trim_outer_document: true,
            formatting: PugFormatOptions {
                quote_style: QuoteStyle::Single,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    assert_eq!(formatter_output, "a(title='Jeremy\\'s docs') Docs\n");
    assert_eq!(converter_output, formatter_output);
}

#[test]
fn formatter_and_converter_share_line_width_prose_wrapping() {
    let prose = "Hello there this sentence should not stay inline once line width is small";
    let formatter_output = format_source(
        &format!("p.\n  {prose}\n"),
        &Configuration {
            line_width: Some(30),
            ..Default::default()
        },
    );

    let converter_output = convert_html_to_pug(
        &format!("<p>{prose}</p>"),
        &ConvertOptions {
            trim_outer_document: true,
            formatting: PugFormatOptions {
                line_width: Some(30),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    assert_eq!(converter_output, formatter_output);
}
