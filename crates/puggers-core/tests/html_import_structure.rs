use puggers_core::{CollapseSingleNestedMode, ConvertOptions, convert_html_to_pug};

#[test]
fn full_document_import_preserves_parsed_document_shell_by_default() {
    let output = convert_html_to_pug(
        "<!doctype html><html><head><title>Docs</title></head><body><main><h1>Hello</h1></main></body></html>",
        &ConvertOptions::default(),
    );

    assert_eq!(
        output,
        "doctype html\nhtml\n  head\n    title Docs\n  body\n    main\n      h1 Hello\n"
    );
}

#[test]
fn trim_outer_document_keeps_all_body_children_instead_of_selecting_main_content() {
    let output = convert_html_to_pug(
        "<!doctype html><html><body><header>Top</header><main><h1>Hello</h1></main><footer>End</footer></body></html>",
        &ConvertOptions {
            trim_outer_document: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "header Top\nmain\n  h1 Hello\nfooter End\n");
}

#[test]
fn collapse_mode_off_preserves_single_child_chain() {
    let output = convert_html_to_pug(
        "<div><section><article><p>Hello</p></article></section></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::Off,
            ..Default::default()
        },
    );

    assert_eq!(output, "div\n  section\n    article\n      p Hello\n");
}

#[test]
fn collapse_mode_top_wins_keeps_outermost_tag() {
    let output = convert_html_to_pug(
        "<div><section><article><p>Hello</p></article></section></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::TopWins,
            ..Default::default()
        },
    );

    assert_eq!(output, "div\n  p Hello\n");
}

#[test]
fn collapse_mode_bottom_wins_keeps_innermost_structural_tag() {
    let output = convert_html_to_pug(
        "<div><section><article><p>Hello</p></article></section></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::BottomWins,
            ..Default::default()
        },
    );

    assert_eq!(output, "article\n  p Hello\n");
}

#[test]
fn collapse_mode_best_tag_wins_prefers_section_over_div() {
    let output = convert_html_to_pug(
        "<div><section><div><p>Hello</p></div></section></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::BestTagWins,
            ..Default::default()
        },
    );

    assert_eq!(output, "section\n  p Hello\n");
}

#[test]
fn collapse_preserves_source_attributed_wrappers_even_when_attributes_are_filtered() {
    let output = convert_html_to_pug(
        "<div data-shell=\"true\"><section><p>Hello</p></section></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::BestTagWins,
            ..Default::default()
        },
    );

    assert_eq!(output, "div\n  section\n    p Hello\n");
}

#[test]
fn collapse_preserves_wrappers_with_multiple_element_children() {
    let output = convert_html_to_pug(
        "<div><section><p>Hello</p></section><aside><p>Related</p></aside></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::BestTagWins,
            ..Default::default()
        },
    );

    assert_eq!(
        output,
        "div\n  section\n    p Hello\n  aside\n    p Related\n"
    );
}

#[test]
fn collapse_preserves_comments_as_structure_when_comments_are_kept() {
    let output = convert_html_to_pug(
        "<div><!--marker--><section><p>Hello</p></section></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::BestTagWins,
            ..Default::default()
        },
    );

    assert_eq!(output, "div\n  // marker\n  section\n    p Hello\n");
}

#[test]
fn collapse_can_cross_comments_when_comments_are_dropped() {
    let output = convert_html_to_pug(
        "<div><!--marker--><section><p>Hello</p></section></div>",
        &ConvertOptions {
            trim_outer_document: true,
            collapse_single_nested: CollapseSingleNestedMode::BestTagWins,
            keep_comments: false,
            ..Default::default()
        },
    );

    assert_eq!(output, "section\n  p Hello\n");
}
