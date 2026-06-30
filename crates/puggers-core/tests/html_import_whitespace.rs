use puggers_core::{ConvertOptions, RootSelection, TextWhitespaceMode, convert_html_to_pug};

fn convert(input: &str, options: &ConvertOptions) -> String {
    convert_html_to_pug(input, options).expect("conversion should succeed")
}

fn root(value: &str) -> RootSelection {
    RootSelection::parse(value).expect("root path should parse")
}

#[test]
fn collapses_normal_text_whitespace_by_default() {
    let output = convert(
        "<p>  Hello   there  </p>",
        &ConvertOptions {
            root: Some(root("p")),
            ..Default::default()
        },
    );

    assert_eq!(output, "p Hello there\n");
}

#[test]
fn preserve_mode_keeps_meaningful_spaces_around_inline_tags() {
    let output = convert(
        "<p>\n  If I do, whitespace is <strong>respected</strong> and <em>everybody</em> is happy.\n</p>",
        &ConvertOptions {
            root: Some(root("p")),
            text_whitespace: TextWhitespaceMode::Preserve,
            ..Default::default()
        },
    );

    assert_eq!(
        output,
        "p\n  | If I do, whitespace is \n  strong respected\n  |  and \n  em everybody\n  |  is happy.\n"
    );
}

#[test]
fn preserve_mode_keeps_whitespace_only_separators_between_inline_elements() {
    let output = convert(
        "<p><strong>Hello</strong> <em>there</em></p>",
        &ConvertOptions {
            root: Some(root("p")),
            text_whitespace: TextWhitespaceMode::Preserve,
            ..Default::default()
        },
    );

    assert_eq!(output, "p\n  strong Hello\n  |  \n  em there\n");
}
