mod support;

pub use support::{ast, config, formatter, lexer, parser};

use std::collections::BTreeMap;

use support::{FixtureRole, FormatOutcome, StructureCoverage};

#[test]
fn upstream_failure_mode_inventory_is_explicit() {
    let anti_cases = support::upstream_fixture_behaviors()
        .into_iter()
        .filter(|behavior| behavior.role == FixtureRole::AntiCase)
        .collect::<Vec<_>>();

    assert_eq!(anti_cases.len(), 55);

    let mut bucket_counts = BTreeMap::new();
    for behavior in &anti_cases {
        *bucket_counts
            .entry(behavior.bucket.clone())
            .or_insert(0usize) += 1;
    }

    assert_eq!(
        bucket_counts.into_iter().collect::<Vec<_>>(),
        vec![
            (String::from("packages/pug-lexer/test/errors"), 26),
            (String::from("packages/pug-linker/test/errors-src"), 3),
            (String::from("packages/pug/test/anti-cases"), 22),
            (String::from("packages/pug/test/eachOf/error"), 4),
        ]
    );
}

#[test]
fn upstream_failure_modes_have_a_pinned_current_behavior_split() {
    let anti_cases = support::upstream_fixture_behaviors()
        .into_iter()
        .filter(|behavior| behavior.role == FixtureRole::AntiCase)
        .collect::<Vec<_>>();

    let mut format_counts = BTreeMap::new();
    let mut structure_counts = BTreeMap::new();

    for behavior in anti_cases {
        *format_counts
            .entry(behavior.format_outcome)
            .or_insert(0usize) += 1;
        *structure_counts
            .entry(behavior.structure_coverage)
            .or_insert(0usize) += 1;
    }

    assert_eq!(
        format_counts.into_iter().collect::<Vec<_>>(),
        vec![
            (FormatOutcome::Idempotent, 12),
            (FormatOutcome::Rewritten, 43),
        ]
    );

    assert_eq!(
        structure_counts.into_iter().collect::<Vec<_>>(),
        vec![
            (StructureCoverage::FullyStructured, 34),
            (StructureCoverage::Mixed, 9),
            (StructureCoverage::RawOnly, 12),
        ]
    );
}
