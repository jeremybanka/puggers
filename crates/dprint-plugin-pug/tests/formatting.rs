mod support;

pub use support::{ast, config, formatter, lexer, parser};

use config::Configuration;
use support::format_source;

#[test]
fn formats_nested_tags_consistently() {
    let source = "div#app.main\n    p   hello world\n      span.label  neat\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "div#app.main\n  p   hello world\n    span.label  neat\n"
    );
}

#[test]
fn formats_with_tabs_when_requested() {
    let source = "div\n  span hello\n";
    let formatted = format_source(
        source,
        &Configuration {
            use_tabs: Some(true),
            indent_width: Some(2),
        },
    );

    assert_eq!(formatted, "div\n\tspan hello\n");
}

#[test]
fn preserves_comments_and_text_lines() {
    let source = "//note\n|  hello\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(formatted, "// note\n|  hello\n");
}

#[test]
fn preserves_attributes_and_text_blocks() {
    let source = "doctype html\nhtml(lang=\"en\")\n  body\n    textarea(data-x=\"1\").\n      line one\n        line two\n    a.link(href=\"/docs\") Docs\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "doctype html\nhtml(lang=\"en\")\n  body\n    textarea(data-x=\"1\").\n      line one\n        line two\n    a.link(href=\"/docs\") Docs\n"
    );
}
