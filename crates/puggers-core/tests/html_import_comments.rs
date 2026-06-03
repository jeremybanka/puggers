use puggers_core::{ConvertOptions, convert_html_to_pug};

#[test]
fn keeps_empty_comments_by_default() {
    let output = convert_html_to_pug(
        "<!-- --><p>Hello</p>",
        &ConvertOptions {
            trim_outer_document: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "//\np Hello\n");
}

#[test]
fn preserves_spaced_comment_payloads_without_trimming_them() {
    let output = convert_html_to_pug(
        "<!doctype html><html><body><!--  note  --></body></html>",
        &ConvertOptions {
            trim_outer_document: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "//\n    note  \n");
}

#[test]
fn preserves_multiline_comment_payloads_as_pipeless_blocks() {
    let output = convert_html_to_pug(
        "<!doctype html><html><body><!--first line\n  second line--></body></html>",
        &ConvertOptions {
            trim_outer_document: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "//\n  first line\n    second line\n");
}
