mod support;

pub use support::{ast, config, formatter, lexer, parser};

use ast::{Attribute, AttributeValue, InlineTextKind, Node, QuoteStyle, StatementHead};

#[test]
fn parses_statement_heads_into_structured_ast() {
    let source = "doctype html\n#app.shell(data-mode=\"demo\")\np   hello world\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert!(matches!(
        &document.children[0],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Doctype(head)
                    if head.spacing.as_deref() == Some(" ")
                        && head.value.as_deref() == Some("html")
            )
    ));

    assert!(matches!(
        &document.children[1],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Tag(head)
                    if head.tag_name.is_none()
                        && head.shorthand_id.as_deref() == Some("app")
                        && head.shorthand_classes == vec![String::from("shell")]
                        && head.attributes == Some(vec![Attribute {
                            name: String::from("data-mode"),
                            value: Some(AttributeValue::Quoted {
                                value: String::from("demo"),
                                quote_style: QuoteStyle::Double,
                            }),
                        }])
            )
    ));

    assert!(matches!(
        &document.children[2],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Tag(head)
                    if head.tag_name.as_deref() == Some("p")
                        && head.inline_space.as_deref() == Some("   ")
                        && head.inline_text.as_ref().is_some_and(|text|
                            text.kind == InlineTextKind::Plain
                                && text.content == "hello world"
                        )
            )
    ));
}

#[test]
fn preserves_raw_statement_heads_for_unsupported_syntax() {
    let source = "a: img\nfoo/\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    for node in &document.children {
        assert!(matches!(
            node,
            Node::Statement(statement) if matches!(&statement.head, StatementHead::Raw(_))
        ));
    }

    let formatted = support::format_source(source, &config::Configuration::default());
    assert_eq!(formatted, source);
}
