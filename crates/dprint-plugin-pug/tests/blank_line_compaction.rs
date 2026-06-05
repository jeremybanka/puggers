mod support;

pub use support::{ast, config, formatter, lexer, parser};

use config::Configuration;
use support::format_source;

#[test]
fn compacts_blank_lines_between_plain_statements_and_nested_children() {
    let source = "\
main


  h1 Title


  p Intro


footer


p After
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(formatted, "main\n  h1 Title\n  p Intro\nfooter\np After\n");
}

#[test]
fn compacts_blank_lines_around_comments_but_preserves_comment_payload_gaps() {
    let source = "\
p Before


// note

  first line

  second line


p After
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "\
p Before
// note
  first line

  second line
p After
"
    );
}

#[test]
fn compacts_blank_lines_around_filters_but_preserves_filter_payload_gaps() {
    let source = "\
p Before


:markdown-it
  # Heading

  Paragraph text


p After
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "\
p Before
:markdown-it
  # Heading

  Paragraph text
p After
"
    );
}

#[test]
fn compacts_blank_lines_around_dotted_and_raw_text_blocks_but_preserves_internal_gaps() {
    let source = "\
article


  p.
    First paragraph line

    Second paragraph line


  script.
    if (ready) {

      start();
    }


p After
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "\
article
  p.
    First paragraph line

    Second paragraph line
  script.
    if (ready) {

      start();
    }
p After
"
    );
}

#[test]
fn compacts_blank_lines_in_mixed_content_files_without_disturbing_significant_text_gaps() {
    let source = "\
header


  h1 Title


  // note

    first line

    second line


main


  :markdown-it
    # Heading

    Paragraph text


  pre.
    keep

    gap


footer


  p Done
";
    let formatted = format_source(source, &Configuration::default());

    assert_eq!(
        formatted,
        "\
header
  h1 Title
  // note
    first line

    second line
main
  :markdown-it
    # Heading

    Paragraph text
  pre.
    keep

    gap
footer
  p Done
"
    );
}
