mod support;

pub use support::{ast, config, formatter, lexer, parser};

use ast::{Node, StatementHead};

#[test]
fn major_operator_prefixed_code_forms_should_be_structured() {
    let source = "- var title = 'hello'\n= title\n!= '<strong>hello</strong>'\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert_all_statement_heads_are_structured(
        &document.children,
        "major operator-prefixed code forms should not remain raw statement heads",
    );
}

#[test]
fn core_control_flow_constructs_should_be_structured() {
    let source = "\
if user
  p Hello
else if admin
  p Admin
else
  p Guest
case friends
  when 0
    p You have no friends
  default
    p You have #{friends} friends
ul
  each item, index in items
    li= item
  while remaining > 0
    li= remaining--
";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert_keyword_statements_are_explicitly_modeled(
        &document.children,
        &["if", "else if", "else", "case", "when", "default", "each", "while"],
    );
}

fn assert_all_statement_heads_are_structured(nodes: &[Node], context: &str) {
    for node in nodes {
        assert_structured_statement_head(node, context);
    }
}

fn assert_structured_statement_head(node: &Node, context: &str) {
    let Node::Statement(statement) = node else {
        panic!("{context}: expected a statement node");
    };

    assert!(
        !matches!(&statement.head, StatementHead::Raw(_)),
        "{context}: expected a structured statement head, found {:?}",
        statement.head
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
                !matches!(&statement.head, StatementHead::Tag(_) | StatementHead::Raw(_)),
                "expected `{rendered}` to use an explicit code/control-flow head, found {:?}",
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
