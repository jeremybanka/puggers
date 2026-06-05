mod support;

pub use support::{ast, config, formatter, lexer, parser};

use std::collections::BTreeMap;

use support::{FixtureRole, FormatOutcome};

#[test]
fn upstream_idempotence_inventory_is_explicit_by_role() {
    let mut counts = BTreeMap::new();

    for behavior in support::upstream_fixture_behaviors() {
        if behavior.format_outcome == FormatOutcome::Idempotent {
            *counts.entry(behavior.role).or_insert(0usize) += 1;
        }
    }

    assert_eq!(
        counts.into_iter().collect::<Vec<_>>(),
        vec![
            (FixtureRole::Example, 3),
            (FixtureRole::Case, 66),
            (FixtureRole::AntiCase, 13),
            (FixtureRole::Support, 12),
        ]
    );
}

#[test]
fn upstream_idempotence_covers_multiple_positive_corpus_buckets() {
    let mut counts = BTreeMap::new();

    for behavior in support::upstream_fixture_behaviors() {
        if behavior.format_outcome == FormatOutcome::Idempotent {
            *counts.entry(behavior.bucket).or_insert(0usize) += 1;
        }
    }

    assert!(counts.contains_key("packages/pug/examples"));
    assert!(counts.contains_key("packages/pug/test/cases"));
    assert!(counts.contains_key("packages/pug-lexer/test/cases"));
    assert!(counts.contains_key("packages/pug/test/fixtures"));
}
