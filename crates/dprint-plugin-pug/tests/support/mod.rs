#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::{collections::BTreeMap, fmt};

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

pub fn vendored_upstream_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/upstream/pug")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FixtureRole {
    Example,
    Case,
    AntiCase,
    Support,
}

impl fmt::Display for FixtureRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixtureRole::Example => write!(f, "example"),
            FixtureRole::Case => write!(f, "case"),
            FixtureRole::AntiCase => write!(f, "anti-case"),
            FixtureRole::Support => write!(f, "support"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpstreamFixture {
    pub path: PathBuf,
    pub relative_path: String,
    pub bucket: String,
    pub role: FixtureRole,
    pub source: String,
}

pub fn upstream_pug_sources() -> Vec<UpstreamFixture> {
    let root = vendored_upstream_dir();
    let mut paths = Vec::new();
    collect_pug_paths(&root, &mut paths);
    paths.sort();

    paths.into_iter()
        .map(|path| {
            let relative_path = path
                .strip_prefix(&root)
                .expect("fixture should be under vendored upstream root")
                .to_string_lossy()
                .replace('\\', "/");
            let source = fs::read_to_string(&path).expect("pug fixture should read");
            let bucket = upstream_bucket(&relative_path);
            let role = upstream_role(&relative_path);

            UpstreamFixture {
                path,
                relative_path,
                bucket,
                role,
                source,
            }
        })
        .collect()
}

fn collect_pug_paths(dir: &Path, output: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("fixture directory should exist") {
        let entry = entry.expect("fixture directory entry should load");
        let path = entry.path();

        if path.is_dir() {
            collect_pug_paths(&path, output);
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) == Some("pug") {
            output.push(path);
        }
    }
}

fn upstream_bucket(relative_path: &str) -> String {
    const PREFIXES: &[&str] = &[
        "packages/pug/examples/",
        "packages/pug/test/anti-cases/",
        "packages/pug/test/browser/",
        "packages/pug/test/cases/",
        "packages/pug/test/cases-es2015/",
        "packages/pug/test/dependencies/",
        "packages/pug/test/duplicate-block/",
        "packages/pug/test/eachOf/error/",
        "packages/pug/test/eachOf/passing/",
        "packages/pug/test/extends-not-top-level/",
        "packages/pug/test/fixtures/",
        "packages/pug/test/markdown-it/",
        "packages/pug/test/regression-2436/",
        "packages/pug/test/shadowed-block/",
        "packages/pug/test/temp/",
        "packages/pug-lexer/test/cases/",
        "packages/pug-lexer/test/errors/",
        "packages/pug-filters/test/cases/",
        "packages/pug-linker/test/cases-src/",
        "packages/pug-linker/test/errors-src/",
        "packages/pug-linker/test/fixtures/",
        "packages/pug-linker/test/special-cases-src/",
    ];

    for prefix in PREFIXES {
        if relative_path.starts_with(prefix) {
            return prefix.trim_end_matches('/').to_string();
        }
    }

    panic!("unclassified upstream fixture path: {relative_path}");
}

fn upstream_role(relative_path: &str) -> FixtureRole {
    if relative_path.starts_with("packages/pug/examples/") {
        return FixtureRole::Example;
    }

    if relative_path.starts_with("packages/pug/test/anti-cases/")
        || relative_path.starts_with("packages/pug/test/eachOf/error/")
        || relative_path.starts_with("packages/pug-lexer/test/errors/")
        || relative_path.starts_with("packages/pug-linker/test/errors-src/")
    {
        return FixtureRole::AntiCase;
    }

    if relative_path.starts_with("packages/pug/test/cases/")
        || relative_path.starts_with("packages/pug/test/browser/")
        || relative_path.starts_with("packages/pug/test/cases-es2015/")
        || relative_path.starts_with("packages/pug/test/duplicate-block/")
        || relative_path.starts_with("packages/pug/test/eachOf/passing/")
        || relative_path.starts_with("packages/pug/test/extends-not-top-level/")
        || relative_path.starts_with("packages/pug-lexer/test/cases/")
        || relative_path.starts_with("packages/pug-filters/test/cases/")
        || relative_path.starts_with("packages/pug-linker/test/cases-src/")
        || relative_path.starts_with("packages/pug-linker/test/special-cases-src/")
    {
        return FixtureRole::Case;
    }

    FixtureRole::Support
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DocumentStats {
    pub statements: usize,
    pub structured_statements: usize,
    pub raw_statements: usize,
    pub comments: usize,
    pub text_lines: usize,
    pub raw_text_lines: usize,
}

pub fn document_stats(document: &ast::Document) -> DocumentStats {
    let mut stats = DocumentStats::default();
    accumulate_node_stats(&document.children, &mut stats);
    stats
}

fn accumulate_node_stats(nodes: &[ast::Node], stats: &mut DocumentStats) {
    for node in nodes {
        match node {
            ast::Node::Statement(statement) => {
                stats.statements += 1;
                match &statement.head {
                    ast::StatementHead::Raw(_) => stats.raw_statements += 1,
                    _ => stats.structured_statements += 1,
                }
                accumulate_node_stats(&statement.children, stats);
            }
            ast::Node::Comment(_) => stats.comments += 1,
            ast::Node::Text(_) => stats.text_lines += 1,
            ast::Node::RawText(_) => stats.raw_text_lines += 1,
        }
    }
}

pub fn bucket_counts(fixtures: &[UpstreamFixture]) -> Vec<(String, usize)> {
    let mut counts = BTreeMap::new();

    for fixture in fixtures {
        *counts.entry(fixture.bucket.clone()).or_insert(0usize) += 1;
    }

    counts.into_iter().collect()
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
