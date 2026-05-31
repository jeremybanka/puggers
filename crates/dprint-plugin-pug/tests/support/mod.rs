#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

#[path = "../../src/ast.rs"]
pub mod ast;
#[path = "../../src/config.rs"]
pub mod config;
#[path = "../../src/formatter.rs"]
pub mod formatter;
#[path = "../../src/lexer.rs"]
pub mod lexer;
#[path = "../../src/parser.rs"]
pub mod parser;

pub fn format_source(source: &str, config: &config::Configuration) -> String {
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);
    formatter::format(&document, config)
}

pub fn docs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/pug/2026-05-31")
}

pub fn pug_doc_sources() -> Vec<(PathBuf, String)> {
    let mut docs = Vec::new();

    for entry in fs::read_dir(docs_dir()).expect("docs directory should exist") {
        let entry = entry.expect("directory entry should load");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("pug") {
            continue;
        }

        let source = fs::read_to_string(&path).expect("pug file should read");
        docs.push((path, source));
    }

    docs.sort_by(|left, right| left.0.cmp(&right.0));
    docs
}

pub fn assert_same_text(actual: &str, expected: &str, context: &str) {
    if actual == expected {
        return;
    }

    let mismatch = actual
        .chars()
        .zip(expected.chars())
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| actual.len().min(expected.len()));

    let actual_snippet: String = actual.chars().skip(mismatch).take(120).collect();
    let expected_snippet: String = expected.chars().skip(mismatch).take(120).collect();

    panic!(
        "{context}\nfirst mismatch at char {mismatch}\nactual:   {:?}\nexpected: {:?}",
        actual_snippet, expected_snippet
    );
}
