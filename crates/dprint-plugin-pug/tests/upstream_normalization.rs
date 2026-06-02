mod support;

pub use support::{ast, config, formatter, lexer, parser};

use std::collections::BTreeMap;

use support::{FixtureRole, FormatOutcome};

#[test]
fn upstream_normalization_inventory_is_explicit_by_role() {
    let mut counts = BTreeMap::new();

    for behavior in support::upstream_fixture_behaviors() {
        if behavior.format_outcome == FormatOutcome::Rewritten {
            *counts.entry(behavior.role).or_insert(0usize) += 1;
        }
    }

    assert_eq!(
        counts.into_iter().collect::<Vec<_>>(),
        vec![
            (FixtureRole::Example, 16),
            (FixtureRole::Case, 250),
            (FixtureRole::AntiCase, 43),
            (FixtureRole::Support, 64),
        ]
    );
}

#[test]
fn upstream_normalization_is_not_limited_to_a_single_corpus_tier() {
    let behaviors = support::upstream_fixture_behaviors();

    assert!(behaviors.iter().any(|behavior| {
        behavior.role == FixtureRole::Example && behavior.format_outcome == FormatOutcome::Rewritten
    }));
    assert!(behaviors.iter().any(|behavior| {
        behavior.role == FixtureRole::Case && behavior.format_outcome == FormatOutcome::Rewritten
    }));
    assert!(behaviors.iter().any(|behavior| {
        behavior.role == FixtureRole::Support && behavior.format_outcome == FormatOutcome::Rewritten
    }));
}
