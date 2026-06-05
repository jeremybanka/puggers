mod support;

pub use support::{ast, config, formatter, lexer, parser};

use std::fs;

use ast::{InlineTextKind, Node, StatementHead};
use config::Configuration;
use support::format_source;

#[test]
fn normalizes_space_tab_and_newline_separated_attributes() {
    let source = "input(type = 'text'  disabled\tdata-count = items.length)\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "input(type=\"text\" disabled data-count=items.length)\n"
    );

    let multiline = "input(\n  type = 'text'\n\tdisabled\n  data-count = items.length\n)\n";
    let formatted_multiline = format_source(multiline, &Configuration::default());

    assert_eq!(
        formatted_multiline,
        "input(type=\"text\" disabled data-count=items.length)\n"
    );
}

#[test]
fn restores_a_missing_separator_between_attributes_and_inline_text() {
    let source = "a(href='/docs')Docs\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(formatted, "a(href=\"/docs\") Docs\n");
}

#[test]
fn restores_a_missing_separator_between_quoted_attributes() {
    let source = "input(type='text'disabled data-count=items.length)\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "input(type=\"text\" disabled data-count=items.length)\n"
    );
}

#[test]
fn trims_structural_trailing_spaces_but_preserves_inline_text_whitespace() {
    let source = "ul \np   hello world\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);

    assert!(matches!(
        &document.children[0],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Tag(head)
                    if head.inline_space.is_none()
                        && head.inline_text.is_none()
            )
    ));

    assert!(matches!(
        &document.children[1],
        Node::Statement(statement)
            if matches!(
                &statement.head,
                StatementHead::Tag(head)
                    if head.inline_space.as_deref() == Some(" ")
                        && head.inline_text.as_ref().is_some_and(|text|
                            text.kind == InlineTextKind::Plain
                                && text.content == "  hello world"
                        )
            )
    ));

    assert_eq!(
        format_source(source, &Configuration::default()),
        "ul\np   hello world\n"
    );
}

#[test]
fn canonicalizes_tab_separators_without_dropping_significant_whitespace() {
    let source = "p\tHello\np\t\tHello\n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(formatted, "p Hello\np \tHello\n");
}

#[test]
fn trims_structural_trailing_spaces_in_supported_statement_heads() {
    let source = "\
doctype html   
include partials/nav   
if condition   
div(class='hero')   
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "\
doctype html
include partials/nav
if condition
div(class=\"hero\")
"
    );
}

#[test]
fn compacts_repeated_spaces_in_non_tag_statement_heads() {
    let source = "\
doctype   html
include    partials/nav
extends    layouts/base
block   append   scripts
if   user.admin
else if   user.editor
case   kind
when   'admin'
default   
each   item,   index   in   items
while   remaining > 0
mixin   button(label)
-   const enabled = true
=   user.name
!=   html
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "\
doctype html
include partials/nav
extends layouts/base
block append scripts
if user.admin
else if user.editor
case kind
when 'admin'
default
each item, index in items
while remaining > 0
mixin button(label)
- const enabled = true
= user.name
!= html
"
    );
}

#[test]
fn compacts_tab_separators_in_non_tag_statement_heads() {
    let source = "\
doctype\thtml
include\t\tpartials/nav
block\tappend\tscripts
if\t\tuser.admin
else if\t\tuser.editor
each\titem,\tindex\tin\titems
mixin\t\tbutton(label)
-\t\tconst enabled = true
=\t\tuser.name
!=\t\thtml
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "\
doctype html
include partials/nav
block append scripts
if user.admin
else if user.editor
each item, index in items
mixin button(label)
- const enabled = true
= user.name
!= html
"
    );
}

#[test]
fn collapses_trailing_blank_lines_at_end_of_file_all_at_once() {
    let source = "p.\n  line one\n  \n  \n";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(formatted, "p.\n  line one\n");
    assert_eq!(
        format_source(&formatted, &Configuration::default()),
        formatted
    );
}

#[test]
fn keeps_plain_text_doc_canonical_at_end_of_file() {
    let path = support::docs_dir().join("plain-text.pug");
    let source = fs::read_to_string(&path).expect("plain-text.pug should read");
    let formatted = format_source(&source, &Configuration::default());

    let source_trailing = source.chars().rev().take_while(|ch| *ch == '\n').count();
    let formatted_trailing = formatted.chars().rev().take_while(|ch| *ch == '\n').count();

    assert_eq!(
        source_trailing,
        1,
        "unexpected source tail in {}",
        path.display()
    );
    assert_eq!(
        formatted_trailing,
        1,
        "formatter should keep the canonical EOF newline shape for {}",
        path.display()
    );
    assert_eq!(formatted, source);
}

#[test]
fn collapses_augmented_plain_text_doc_eof_blank_lines_in_one_pass() {
    let path = support::docs_dir().join("plain-text.pug");
    let source = fs::read_to_string(&path).expect("plain-text.pug should read");
    let augmented = format!("{source}\n\n\n\n\n\n");
    let formatted = format_source(&augmented, &Configuration::default());
    let reformatted = format_source(&formatted, &Configuration::default());

    let formatted_trailing = formatted.chars().rev().take_while(|ch| *ch == '\n').count();

    assert_eq!(
        formatted_trailing,
        1,
        "formatter should collapse augmented EOF blank lines in one pass for {}",
        path.display()
    );
    assert_eq!(reformatted, formatted);
}
