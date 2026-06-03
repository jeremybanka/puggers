use puggers_core::{ConvertOptions, TextWhitespaceMode, convert_html_to_pug};

#[test]
fn collapses_normal_text_whitespace_by_default() {
    let output = convert_html_to_pug(
        "<p>  Hello   there  </p>",
        &ConvertOptions {
            trim_outer_document: true,
            ..Default::default()
        },
    );

    assert_eq!(output, "p Hello there\n");
}

#[test]
fn preserve_mode_keeps_meaningful_spaces_around_inline_tags() {
    let output = convert_html_to_pug(
        "<p>\n  If I do, whitespace is <strong>respected</strong> and <em>everybody</em> is happy.\n</p>",
        &ConvertOptions {
            trim_outer_document: true,
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
    let output = convert_html_to_pug(
        "<p><strong>Hello</strong> <em>there</em></p>",
        &ConvertOptions {
            trim_outer_document: true,
            text_whitespace: TextWhitespaceMode::Preserve,
            ..Default::default()
        },
    );

    assert_eq!(output, "p\n  strong Hello\n  |  \n  em there\n");
}
