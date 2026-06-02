mod support;

pub use support::{ast, config, formatter, lexer, parser};

use std::collections::BTreeMap;

use config::Configuration;
use support::{FixtureBehavior, FixtureRole, FormatOutcome, StructureCoverage, UpstreamFixture};

#[test]
fn registers_all_vendored_upstream_pug_fixtures_by_bucket() {
    let fixtures = support::upstream_pug_sources();

    assert_eq!(
        fixtures.len(),
        459,
        "expected to register every vendored .pug fixture"
    );

    let expected = vec![
        (String::from("packages/pug-filters/test/cases"), 1),
        (String::from("packages/pug-lexer/test/cases"), 118),
        (String::from("packages/pug-lexer/test/errors"), 26),
        (String::from("packages/pug-linker/test/cases-src"), 40),
        (String::from("packages/pug-linker/test/errors-src"), 3),
        (String::from("packages/pug-linker/test/fixtures"), 17),
        (
            String::from("packages/pug-linker/test/special-cases-src"),
            3,
        ),
        (String::from("packages/pug/examples"), 19),
        (String::from("packages/pug/test/anti-cases"), 22),
        (String::from("packages/pug/test/browser"), 1),
        (String::from("packages/pug/test/cases"), 137),
        (String::from("packages/pug/test/cases-es2015"), 1),
        (String::from("packages/pug/test/dependencies"), 7),
        (String::from("packages/pug/test/duplicate-block"), 2),
        (String::from("packages/pug/test/eachOf/error"), 4),
        (String::from("packages/pug/test/eachOf/passing"), 2),
        (String::from("packages/pug/test/extends-not-top-level"), 3),
        (String::from("packages/pug/test/fixtures"), 39),
        (String::from("packages/pug/test/markdown-it"), 2),
        (String::from("packages/pug/test/regression-2436"), 6),
        (String::from("packages/pug/test/shadowed-block"), 3),
        (String::from("packages/pug/test/temp"), 3),
    ];

    assert_eq!(
        support::bucket_counts(&fixtures),
        expected,
        "upstream fixture bucket inventory changed"
    );
}

#[test]
fn formats_every_vendored_upstream_pug_fixture_without_panicking() {
    for fixture in support::upstream_pug_sources() {
        let formatted = support::format_source(&fixture.source, &Configuration::default());
        assert!(
            formatted.ends_with('\n'),
            "formatted output should end with a newline for {}",
            fixture.relative_path
        );
    }
}

#[test]
fn reports_current_upstream_case_and_anti_case_coverage() {
    let fixtures = support::upstream_pug_sources();
    let behaviors = fixtures
        .iter()
        .map(support::analyze_fixture)
        .collect::<Vec<_>>();

    let report = render_behavior_report(&fixtures, &behaviors);

    let total_by_role = summarize_by_role(&behaviors);
    assert_eq!(
        total_by_role,
        vec![
            (FixtureRole::Example, 19),
            (FixtureRole::Case, 308),
            (FixtureRole::AntiCase, 55),
            (FixtureRole::Support, 77),
        ],
        "fixture role inventory drifted\n{report}"
    );

    let format_by_role = summarize_format_outcomes(&behaviors);
    let structure_by_role = summarize_structure_coverage(&behaviors);

    assert_eq!(
        format_by_role.into_iter().collect::<Vec<_>>(),
        vec![
            ((FixtureRole::Example, FormatOutcome::Idempotent), 3),
            ((FixtureRole::Example, FormatOutcome::Rewritten), 16),
            ((FixtureRole::Case, FormatOutcome::Idempotent), 66),
            ((FixtureRole::Case, FormatOutcome::Rewritten), 242),
            ((FixtureRole::AntiCase, FormatOutcome::Idempotent), 14),
            ((FixtureRole::AntiCase, FormatOutcome::Rewritten), 41),
            ((FixtureRole::Support, FormatOutcome::Idempotent), 13),
            ((FixtureRole::Support, FormatOutcome::Rewritten), 64),
        ],
        "format outcome register drifted\n{report}"
    );

    assert_eq!(
        structure_by_role.into_iter().collect::<Vec<_>>(),
        vec![
            (
                (FixtureRole::Example, StructureCoverage::FullyStructured),
                10
            ),
            ((FixtureRole::Example, StructureCoverage::Mixed), 9),
            ((FixtureRole::Case, StructureCoverage::NoStatements), 4),
            ((FixtureRole::Case, StructureCoverage::FullyStructured), 225),
            ((FixtureRole::Case, StructureCoverage::Mixed), 75),
            ((FixtureRole::Case, StructureCoverage::RawOnly), 4),
            (
                (FixtureRole::AntiCase, StructureCoverage::FullyStructured),
                39
            ),
            ((FixtureRole::AntiCase, StructureCoverage::Mixed), 5),
            ((FixtureRole::AntiCase, StructureCoverage::RawOnly), 11),
            ((FixtureRole::Support, StructureCoverage::NoStatements), 1),
            (
                (FixtureRole::Support, StructureCoverage::FullyStructured),
                72
            ),
            ((FixtureRole::Support, StructureCoverage::Mixed), 2),
            ((FixtureRole::Support, StructureCoverage::RawOnly), 2),
        ],
        "structure coverage register drifted\n{report}"
    );

    println!("{report}");
}

#[test]
fn checked_in_upstream_register_report_is_current() {
    let fixtures = support::upstream_pug_sources();
    let behaviors = fixtures
        .iter()
        .map(support::analyze_fixture)
        .collect::<Vec<_>>();
    let report = render_behavior_report(&fixtures, &behaviors);
    let expected = std::fs::read_to_string(support::vendored_upstream_register_path())
        .expect("checked-in upstream register should exist");

    assert_eq!(report, expected, "checked-in upstream register drifted");
}

fn summarize_by_role(behaviors: &[FixtureBehavior]) -> Vec<(FixtureRole, usize)> {
    let mut counts = BTreeMap::new();

    for behavior in behaviors {
        *counts.entry(behavior.role).or_insert(0usize) += 1;
    }

    counts.into_iter().collect()
}

fn summarize_format_outcomes(
    behaviors: &[FixtureBehavior],
) -> BTreeMap<(FixtureRole, FormatOutcome), usize> {
    let mut counts = BTreeMap::new();

    for behavior in behaviors {
        *counts
            .entry((behavior.role, behavior.format_outcome))
            .or_insert(0usize) += 1;
    }

    counts
}

fn summarize_structure_coverage(
    behaviors: &[FixtureBehavior],
) -> BTreeMap<(FixtureRole, StructureCoverage), usize> {
    let mut counts = BTreeMap::new();

    for behavior in behaviors {
        *counts
            .entry((behavior.role, behavior.structure_coverage))
            .or_insert(0usize) += 1;
    }

    counts
}

fn render_behavior_report(fixtures: &[UpstreamFixture], behaviors: &[FixtureBehavior]) -> String {
    let mut output = String::new();

    output.push_str("Upstream fixture register\n");
    output.push_str("========================\n");
    output.push_str(&format!("total fixtures: {}\n", behaviors.len()));
    output.push('\n');

    output.push_str("By bucket\n");
    for (bucket, count) in support::bucket_counts(fixtures) {
        output.push_str(&format!("- {bucket}: {count}\n"));
    }
    output.push('\n');

    output.push_str("By role\n");
    for (role, count) in summarize_by_role(behaviors) {
        output.push_str(&format!("- {role}: {count}\n"));
    }
    output.push('\n');

    output.push_str("Format outcomes by role\n");
    for ((role, outcome), count) in summarize_format_outcomes(behaviors) {
        output.push_str(&format!("- {role} / {}: {count}\n", format_label(outcome)));
    }
    output.push('\n');

    output.push_str("Structure coverage by role\n");
    for ((role, coverage), count) in summarize_structure_coverage(behaviors) {
        output.push_str(&format!(
            "- {role} / {}: {count}\n",
            structure_label(coverage)
        ));
    }
    output.push('\n');

    output.push_str("Rewritten anti-cases\n");
    let rewritten_anti_cases = behaviors
        .iter()
        .filter(|behavior| {
            behavior.role == FixtureRole::AntiCase
                && behavior.format_outcome == FormatOutcome::Rewritten
        })
        .collect::<Vec<_>>();
    if rewritten_anti_cases.is_empty() {
        output.push_str("- none\n");
    } else {
        for behavior in rewritten_anti_cases {
            output.push_str(&format!(
                "- {} [{}]\n",
                behavior.relative_path, behavior.bucket
            ));
        }
    }
    output.push('\n');

    output.push_str("Most opaque case fixtures\n");
    let mut opaque_cases = behaviors
        .iter()
        .filter(|behavior| behavior.role == FixtureRole::Case)
        .collect::<Vec<_>>();
    opaque_cases.sort_by(|left, right| {
        right
            .stats
            .raw_statements
            .cmp(&left.stats.raw_statements)
            .then_with(|| left.relative_path.cmp(&right.relative_path))
    });

    for behavior in opaque_cases.into_iter().take(20) {
        output.push_str(&format!(
            "- raw {}/{} | {} | {}\n",
            behavior.stats.raw_statements,
            behavior.stats.statements,
            structure_label(behavior.structure_coverage),
            behavior.relative_path
        ));
    }

    output
}

fn format_label(outcome: FormatOutcome) -> &'static str {
    match outcome {
        FormatOutcome::Idempotent => "idempotent",
        FormatOutcome::Rewritten => "rewritten",
    }
}

fn structure_label(coverage: StructureCoverage) -> &'static str {
    match coverage {
        StructureCoverage::NoStatements => "no-statements",
        StructureCoverage::FullyStructured => "fully-structured",
        StructureCoverage::Mixed => "mixed",
        StructureCoverage::RawOnly => "raw-only",
    }
}
