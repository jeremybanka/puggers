mod support;

pub use support::{ast, config, formatter, lexer, parser};

use ast::{InlineTextKind, Node, StatementHead, TextBlockKind, TextLineKind};
use config::Configuration;
use support::format_source;

#[test]
fn classifies_prose_and_raw_text_blocks() {
    let source = "p.\n  Hello #[strong there]\nscript.\n  if (usingPug)\n.\n  This text belongs to the parent.\ntextarea.\n  line one\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert!(matches!(
        &document.children[0],
        Node::Statement(statement)
            if matches!(&statement.head, StatementHead::Tag(head) if head.tag_name.as_deref() == Some("p"))
                && statement.text_block_kind == Some(TextBlockKind::Prose)
    ));

    assert!(matches!(
        &document.children[1],
        Node::Statement(statement)
            if matches!(&statement.head, StatementHead::Tag(head) if head.tag_name.as_deref() == Some("script"))
                && statement.text_block_kind == Some(TextBlockKind::Raw)
    ));

    assert!(matches!(
        &document.children[2],
        Node::Statement(statement)
            if matches!(&statement.head, StatementHead::Raw(content) if content.is_empty())
                && statement.text_block_kind == Some(TextBlockKind::Prose)
    ));

    assert!(matches!(
        &document.children[3],
        Node::Statement(statement)
            if matches!(&statement.head, StatementHead::Tag(head) if head.tag_name.as_deref() == Some("textarea"))
                && statement.text_block_kind == Some(TextBlockKind::Raw)
    ));
}

#[test]
fn classifies_inline_text_without_trimming_significant_trailing_spaces() {
    let source = "p Hello #[strong there]  \np <em>literal</em>\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert!(matches!(
        &document.children[0],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Tag(head)
                    if head.inline_text.as_ref().is_some_and(|text|
                        text.kind == InlineTextKind::Interpolated
                            && text.content == "Hello #[strong there]  "
                    )
            )
    ));

    assert!(matches!(
        &document.children[1],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Tag(head)
                    if head.inline_text.as_ref().is_some_and(|text|
                        text.kind == InlineTextKind::LiteralHtml
                            && text.content == "<em>literal</em>"
                    )
            )
    ));
}

#[test]
fn preserves_piped_lines_and_blank_lines_with_significant_whitespace() {
    let source = "p\n  | The pipe line keeps trailing spaces.  \n  |\n  |  And it keeps leading spaces.\np.\n  Whether the tags and text are whitespace-separated.  \n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(formatted, source);
}

#[test]
fn parses_piped_lines_into_explicit_text_nodes() {
    let source = "p\n  | One line\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    let Node::Statement(statement) = &document.children[0] else {
        panic!("expected first child to be a statement");
    };

    assert!(matches!(
        &statement.children[0],
        Node::Text(text)
            if text.kind == TextLineKind::Piped && text.content == " One line"
    ));
}
