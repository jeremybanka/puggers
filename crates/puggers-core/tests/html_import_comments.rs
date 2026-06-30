use puggers_core::{ConvertOptions, RootSelection, convert_html_to_pug};

fn convert(input: &str, options: &ConvertOptions) -> String {
    convert_html_to_pug(input, options).expect("conversion should succeed")
}

fn body_root() -> RootSelection {
    RootSelection::parse("html>body").expect("root path should parse")
}

#[test]
fn keeps_empty_comments_by_default() {
    let output = convert(
        "<!doctype html><html><body><!-- --><p>Hello</p></body></html>",
        &ConvertOptions {
            root: Some(body_root()),
            ..Default::default()
        },
    );

    assert_eq!(output, "body\n  //\n     \n  p Hello\n");
}

#[test]
fn preserves_spaced_comment_payloads_without_trimming_them() {
    let output = convert(
        "<!doctype html><html><body><!--  note  --></body></html>",
        &ConvertOptions {
            root: Some(body_root()),
            ..Default::default()
        },
    );

    assert_eq!(output, "body\n  //\n      note  \n");
}

#[test]
fn preserves_multiline_comment_payloads_as_pipeless_blocks() {
    let output = convert(
        "<!doctype html><html><body><!--first line\n  second line--></body></html>",
        &ConvertOptions {
            root: Some(body_root()),
            ..Default::default()
        },
    );

    assert_eq!(output, "body\n  //\n    first line\n      second line\n");
}
