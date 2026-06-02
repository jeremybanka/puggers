mod support;

pub use support::{ast, config, formatter, lexer, parser};

use config::Configuration;
use support::format_source;

#[test]
fn wraps_long_attribute_lists_when_line_width_is_exceeded() {
    let source =
        "div\n  a.link(data-z=\"last\", aria-label=\"Documentation\", href=\"/docs\") Docs\n";
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
fn reflows_safe_prose_blocks_when_line_width_is_set() {
    let source = "p.\n  This paragraph is made of plain prose lines that should wrap into a narrower block when line width is configured.\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(40),
            ..Configuration::default()
        },
    );

    assert_eq!(
        formatted,
        "p.\n  This paragraph is made of plain\n  prose lines that should wrap into a\n  narrower block when line width is\n  configured.\n"
    );
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

#[test]
fn reflows_safe_prose_blocks_for_custom_tags() {
    let source = "marketing-copy.\n  This paragraph belongs to a custom element but should still wrap based on content shape rather than tag name alone.\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(38),
            ..Configuration::default()
        },
    );

    assert_eq!(
        formatted,
        "marketing-copy.\n  This paragraph belongs to a custom\n  element but should still wrap based\n  on content shape rather than tag\n  name alone.\n"
    );
}

#[test]
fn does_not_reflow_raw_tag_dotted_blocks() {
    let source = "script.\n  const message = \"This should stay on one line even when it is long enough to exceed the configured width.\";\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(32),
            ..Configuration::default()
        },
    );

    assert_eq!(formatted, source);
}

#[test]
fn does_not_reflow_interpolated_dotted_prose_blocks() {
    let source = "custom-copy.\n  This paragraph has #[strong interpolation] and should stay exactly as written even if the line width is small.\n";
    let formatted = format_source(
        source,
        &Configuration {
            line_width: Some(32),
            ..Configuration::default()
        },
    );

    assert_eq!(formatted, source);
}
