mod support;

pub use support::{ast, config, formatter, lexer, parser};

use ast::{Node, StatementHead};

#[test]
fn mixin_declarations_should_be_explicitly_modeled() {
    let source = "\
mixin article(title)
  article
    h1= title
    block
";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert_keyword_statements_are_explicitly_modeled(&document.children, &["mixin"]);
}

#[test]
fn mixin_calls_should_not_remain_raw_statement_heads() {
    let source = "\
+article('Hello world')
+link('/foo', 'Foo')(class=\"btn\")
";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    for node in &document.children {
        let Node::Statement(statement) = node else {
            panic!("expected a statement node");
        };

        let rendered = statement.head.to_source(&config::Configuration::default());
        if rendered.starts_with('+') {
            assert!(
                !matches!(&statement.head, StatementHead::Raw(_)),
                "expected `{rendered}` to use an explicit mixin-call head, found {:?}",
                statement.head
            );
        }
    }
}

fn assert_keyword_statements_are_explicitly_modeled(nodes: &[Node], keywords: &[&str]) {
    let render_config = config::Configuration::default();

    for node in nodes {
        let Node::Statement(statement) = node else {
            continue;
        };

        let rendered = statement.head.to_source(&render_config);
        if keywords
            .iter()
            .copied()
            .any(|keyword| matches_keyword_head(&rendered, keyword))
        {
            assert!(
                !matches!(
                    &statement.head,
                    StatementHead::Tag(_) | StatementHead::Raw(_)
                ),
                "expected `{rendered}` to use an explicit mixin head, found {:?}",
                statement.head
            );
        }

        assert_keyword_statements_are_explicitly_modeled(&statement.children, keywords);
    }
}

fn matches_keyword_head(rendered: &str, keyword: &str) -> bool {
    rendered == keyword
        || rendered
            .strip_prefix(keyword)
            .is_some_and(|suffix| suffix.chars().next().is_some_and(|ch| ch.is_whitespace()))
}
