mod support;

pub use support::{ast, config, formatter, lexer, parser};

use config::Configuration;
use support::{
    DiagnosticSeverity, assert_has_diagnostic, assert_same_text, format_source_with_diagnostics,
};

#[test]
fn recovers_inconsistent_indentation_without_dropping_following_siblings() {
    let source = "\
html
  body
      p Hey
    p Is this in <body> or in <p>?
";
    let report = format_source_with_diagnostics(source, &Configuration::default());

    assert_same_text(
        &report.formatted,
        "html\n  body\n    p Hey\n    p Is this in <body> or in <p>?\n",
        "recovered output should preserve both body children after inconsistent indentation",
    );
    assert_has_diagnostic(
        &report.diagnostics,
        DiagnosticSeverity::Warning,
        3,
        "indent",
    );
}

#[test]
fn warns_when_include_is_missing_a_path_but_preserves_surrounding_formatting() {
    let source = "\
include
p After
";
    let report = format_source_with_diagnostics(source, &Configuration::default());

    assert_same_text(
        &report.formatted,
        "include\np After\n",
        "missing include payload should stay quarantined without blocking surrounding formatting",
    );
    assert_has_diagnostic(
        &report.diagnostics,
        DiagnosticSeverity::Warning,
        1,
        "include",
    );
}

#[test]
fn warns_for_bare_when_while_still_formatting_the_rest_of_the_case_block() {
    let source = "\
case pet
  when
    p mystery
  when 'dog'
    p bark
";
    let report = format_source_with_diagnostics(source, &Configuration::default());

    assert_same_text(
        &report.formatted,
        "case pet\n  when\n    p mystery\n  when 'dog'\n    p bark\n",
        "recoverable control-flow damage should not prevent surrounding valid branches from formatting",
    );
    assert_has_diagnostic(&report.diagnostics, DiagnosticSeverity::Warning, 2, "when");
}

#[test]
fn warns_for_orphaned_else_while_preserving_its_structure() {
    let source = "\
else
  .foo
";
    let report = format_source_with_diagnostics(source, &Configuration::default());

    assert_same_text(
        &report.formatted,
        "else\n  .foo\n",
        "orphaned else should remain structurally recoverable instead of collapsing to raw text",
    );
    assert_has_diagnostic(&report.diagnostics, DiagnosticSeverity::Warning, 1, "else");
}

#[test]
fn does_not_warn_for_else_attached_to_each_or_for_loops() {
    let source = "\
ul
  each item in items
    li= item
  else
    li empty
ul
  for item in items
    li= item
  else
    li empty
";
    let report = format_source_with_diagnostics(source, &Configuration::default());

    assert_same_text(
        &report.formatted,
        "ul\n  each item in items\n    li= item\n  else\n    li empty\nul\n  for item in items\n    li= item\n  else\n    li empty\n",
        "loop else branches should remain warning-free recoverable structure",
    );
    assert!(
        !report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("orphaned `else`")),
        "did not expect loop else branches to be treated as orphaned: {:?}",
        report.diagnostics
    );
}

#[test]
fn warns_for_invalid_default_heads_without_destructuring_the_case_block() {
    let source = "\
case pet
  default foo
    p mystery
default
  p outside
";
    let report = format_source_with_diagnostics(source, &Configuration::default());

    assert_same_text(
        &report.formatted,
        "case pet\n  default foo\n    p mystery\ndefault\n  p outside\n",
        "invalid default heads should stay modeled and formatted even when warned on",
    );
    assert_has_diagnostic(
        &report.diagnostics,
        DiagnosticSeverity::Warning,
        2,
        "default",
    );
    assert_has_diagnostic(
        &report.diagnostics,
        DiagnosticSeverity::Warning,
        4,
        "default",
    );
}
