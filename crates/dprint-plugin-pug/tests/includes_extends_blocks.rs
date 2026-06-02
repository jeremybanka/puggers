mod support;

pub use support::{ast, config, formatter, lexer, parser};

use ast::{Node, StatementHead};

#[test]
fn include_extends_and_block_heads_should_be_explicitly_modeled() {
    let source = "\
extends layout.pug
include shared/head.pug
block content
  p Body
block append scripts
  script(src=\"/app.js\")
block prepend head
  meta(charset=\"utf-8\")
";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert_keyword_statements_are_explicitly_modeled(
        &document.children,
        &["extends", "include", "block"],
    );
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
                "expected `{rendered}` to use an explicit include/extends/block head, found {:?}",
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
