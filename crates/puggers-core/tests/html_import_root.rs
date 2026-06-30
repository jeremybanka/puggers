use puggers_core::{ConvertError, ConvertOptions, RootSelection, convert_html_to_pug};

fn convert_with_root(input: &str, root: &str) -> String {
    convert_html_to_pug(
        input,
        &ConvertOptions {
            root: Some(RootSelection::parse(root).expect("root path should parse")),
            ..Default::default()
        },
    )
    .expect("root should match")
}

#[test]
fn root_selection_targets_first_descendant_match_after_direct_child_steps() {
    let output = convert_with_root(
        "<!doctype html><html><body><main><section><article><h1>First</h1></article></section><article><h1>Second</h1></article></main></body></html>",
        "html>body article",
    );

    assert_eq!(output, "article\n  h1 First\n");
}

#[test]
fn root_selection_direct_child_steps_do_not_match_deeper_descendants() {
    let output = convert_with_root(
        "<!doctype html><html><body><main><article><h1>Nested</h1></article></main><article><h1>Direct</h1></article></body></html>",
        "html>body>article",
    );

    assert_eq!(output, "article\n  h1 Direct\n");
}

#[test]
fn root_selection_reports_no_match_as_a_typed_conversion_error() {
    let error = convert_html_to_pug(
        "<!doctype html><html><body><main><article><h1>Nested</h1></article></main></body></html>",
        &ConvertOptions {
            root: Some(RootSelection::parse("html>body>article").expect("root path should parse")),
            ..Default::default()
        },
    )
    .expect_err("direct child root path should not match nested article");

    assert_eq!(
        error,
        ConvertError::RootNotFound {
            root: String::from("html>body>article")
        }
    );
}
