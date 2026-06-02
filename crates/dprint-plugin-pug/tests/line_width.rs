mod support;

pub use support::{ast, config, formatter, lexer, parser};

use config::Configuration;
use support::format_source;

#[test]
fn wraps_long_attribute_lists_when_line_width_is_exceeded() {
    let source = "div\n  a.link(data-z=\"last\", aria-label=\"Documentation\", href=\"/docs\") Docs\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(40),
            ..Configuration::default()
        },
    );

    assert_eq!(
        formatted,
        "div\n  a.link(\n    data-z=\"last\"\n    aria-label=\"Documentation\"\n    href=\"/docs\"\n  ) Docs\n"
    );
}

#[test]
fn keeps_short_attribute_lists_inline_even_with_line_width() {
    let source = "a.link(href=\"/docs\", title=\"Docs\") Docs\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(80),
            ..Configuration::default()
        },
    );

    assert_eq!(formatted, source);
}

#[test]
fn does_not_reflow_prose_blocks_when_line_width_is_set() {
    let source = "p.\n  Using regular tags can help keep your lines short,\n  but interpolated tags may be easier to #[em visualize]\n  whether the tags and text are whitespace-separated.\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(20),
            ..Configuration::default()
        },
    );

    assert_eq!(formatted, source);
}

#[test]
fn reparses_wrapped_attribute_layout_idempotently() {
    let source =
        "a.link(\n  data-z=\"last\"\n  aria-label=\"Documentation\"\n  href=\"/docs\"\n) Docs\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(40),
            ..Configuration::default()
        },
    );

    assert_eq!(formatted, source);
}
