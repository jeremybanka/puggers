mod support;

pub use support::{ast, config, formatter, lexer, parser};

use support::ast::{CommentKind, Node};
use support::format_source;

#[test]
fn normalizes_comment_heads_but_keeps_buffered_and_unbuffered_distinct() {
    let source = "//note\n//-note\n";
    let formatted = format_source(source, &config::Configuration::default());

    assert_eq!(formatted, "// note\n//- note\n");
}

#[test]
fn preserves_pipeless_comment_blocks_and_their_payload_indentation() {
    let source = "// block\n  first line\n    second line\n//- hidden\n  third line\n";
    let formatted = format_source(source, &config::Configuration::default());

    assert_eq!(formatted, source);
}

#[test]
fn parses_unbuffered_comment_payload_without_treating_the_dash_as_text() {
    let source = "// visible\n//- hidden\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert!(matches!(
        &document.children[0],
        Node::Comment(comment)
            if comment.kind == CommentKind::Buffered
                && comment.value.as_deref() == Some("visible")
    ));
    assert!(matches!(
        &document.children[1],
        Node::Comment(comment)
            if comment.kind == CommentKind::Unbuffered
                && comment.value.as_deref() == Some("hidden")
    ));
}

#[test]
fn preserves_filter_blocks_losslessly() {
    let source = ":markdown-it\n  # Heading\n\n  code sample\n:css\n  body {\n    color: red;\n  }\n:javascript\n  console.log('hi');\n";
    let formatted = format_source(source, &config::Configuration::default());

    assert_eq!(formatted, source);
}

#[test]
fn preserves_nested_filter_blocks_under_surrounding_tags() {
    let source = "template\n  :markdown-it\n    # Heading\n\n    Paragraph text\n";
    let formatted = format_source(source, &config::Configuration::default());

    assert_eq!(formatted, source);
}
