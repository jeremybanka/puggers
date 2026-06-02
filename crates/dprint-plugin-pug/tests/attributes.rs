mod support;

pub use support::{ast, config, formatter, lexer, parser};

use ast::{Attribute, AttributeValue, Node, QuoteStyle, StatementHead};
use config::Configuration;
use support::format_source;

#[test]
fn parses_attributes_into_structured_entries() {
    let source =
        "input(type='text', disabled, data-empty='', data-count=items.length, checked=true)\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert!(matches!(
        &document.children[0],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Tag(head)
                    if head.attributes == Some(vec![
                        Attribute {
                            name: String::from("type"),
                            value: Some(AttributeValue::Quoted {
                                value: String::from("text"),
                                quote_style: QuoteStyle::Single,
                            }),
                        },
                        Attribute {
                            name: String::from("disabled"),
                            value: None,
                        },
                        Attribute {
                            name: String::from("data-empty"),
                            value: Some(AttributeValue::Quoted {
                                value: String::new(),
                                quote_style: QuoteStyle::Single,
                            }),
                        },
                        Attribute {
                            name: String::from("data-count"),
                            value: Some(AttributeValue::Expression(String::from("items.length"))),
                        },
                        Attribute {
                            name: String::from("checked"),
                            value: Some(AttributeValue::Expression(String::from("true"))),
                        },
                    ])
            )
    ));
}

#[test]
fn normalizes_attribute_spacing_and_defaults_to_double_quotes() {
    let source =
        "input( type = 'text' , disabled , data-empty = '' , data-count = items.length )\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "input(type=\"text\", disabled, data-empty=\"\", data-count=items.length)\n"
    );
}

#[test]
fn supports_single_quotes_when_configured() {
    let source = "input(type=\"text\", title=\"Hello world\")\n";
    let formatted = format_source(
        source,
        &Configuration {
            quote_style: Some(config::QuoteStyle::Single),
            ..Configuration::default()
        },
    );

    assert_eq!(formatted, "input(type='text', title='Hello world')\n");
}

#[test]
fn preserves_attribute_order_while_normalizing_formatting() {
    let source = "a(data-z='last', aria-label = 'Docs' , href = '/docs') Docs\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "a(data-z=\"last\", aria-label=\"Docs\", href=\"/docs\") Docs\n"
    );
}
