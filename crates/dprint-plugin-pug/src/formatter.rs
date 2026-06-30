use crate::ast::{
    CommentKind, CommentNode, Document, Node, RawTextNode, StatementHead, StatementNode, TagHead,
    TextBlockKind,
};
use crate::config::Configuration;
use puggers_core::{PugFormatOptions, formatting};

pub fn format(document: &Document, config: &Configuration) -> String {
    let mut output = String::new();

    for (index, node) in document.children.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        write_node(&mut output, node, 0, config);
    }

    collapse_trailing_blank_lines(&mut output);
    output
}

fn write_node(output: &mut String, node: &Node, depth: usize, config: &Configuration) {
    match node {
        Node::Statement(statement) => write_statement(output, statement, depth, config),
        Node::Comment(comment) => write_comment(output, comment, depth, config),
        Node::Text(text) => {
            formatting::write_indent(output, depth, &config.format_options());
            output.push('|');
            output.push_str(&text.content);
        }
        Node::RawText(text) => write_raw_text(output, text, depth, &config.format_options()),
    }
}

fn write_statement(
    output: &mut String,
    element: &StatementNode,
    depth: usize,
    config: &Configuration,
) {
    formatting::write_indent(output, depth, &config.format_options());
    match &element.head {
        StatementHead::Tag(head) if should_wrap_attributes(head, depth, config) => {
            write_wrapped_tag_head(output, head, depth, config)
        }
        _ => output.push_str(&element.head.to_source(config)),
    }
    if element.text_block_kind.is_some() && !matches!(&element.head, StatementHead::Filter(_)) {
        output.push('.');
    }

    if let Some(lines) = reflowed_text_block_lines(element, depth, config) {
        for line in lines {
            output.push('\n');
            formatting::write_indent(output, depth + 1, &config.format_options());
            output.push_str(&line);
        }
        return;
    }

    for child in &element.children {
        output.push('\n');
        write_node(output, child, depth + 1, config);
    }
}

fn write_comment(output: &mut String, comment: &CommentNode, depth: usize, config: &Configuration) {
    formatting::write_indent(output, depth, &config.format_options());
    match comment.kind {
        CommentKind::Buffered => output.push_str("//"),
        CommentKind::Unbuffered => output.push_str("//-"),
    }

    if let Some(value) = &comment.value {
        output.push(' ');
        output.push_str(value.trim());
    }

    for child in &comment.children {
        output.push('\n');
        write_raw_text(output, child, depth + 1, &config.format_options());
    }
}

fn reflowed_text_block_lines(
    element: &StatementNode,
    depth: usize,
    config: &Configuration,
) -> Option<Vec<String>> {
    if element.text_block_kind != Some(TextBlockKind::Prose) {
        return None;
    }

    let line_width = config.line_width()?;
    let format_options = config.format_options();
    let available_width =
        line_width.checked_sub(formatting::display_width(depth + 1, &format_options))?;
    if available_width == 0 {
        return None;
    }

    let segments = plain_prose_segments(&element.children)?;
    let normalized_line = segments.join(" ");
    if normalized_line.len() <= available_width {
        return None;
    }

    Some(formatting::wrap_words(
        segments
            .iter()
            .flat_map(|segment| segment.split_whitespace()),
        available_width,
    ))
}

fn should_wrap_attributes(head: &TagHead, depth: usize, config: &Configuration) -> bool {
    let Some(line_width) = config.line_width() else {
        return false;
    };

    let Some(attributes) = &head.attributes else {
        return false;
    };

    if attributes.len() < 2 {
        return false;
    }

    let rendered = head.to_source(config);
    formatting::display_width(depth, &config.format_options()) + rendered.len() > line_width
}

fn write_wrapped_tag_head(
    output: &mut String,
    head: &TagHead,
    depth: usize,
    config: &Configuration,
) {
    output.push_str(&tag_head_prefix(head));
    output.push('(');

    if let Some(attributes) = &head.attributes {
        for attribute in attributes {
            output.push('\n');
            formatting::write_indent(output, depth + 1, &config.format_options());
            output.push_str(&attribute.to_source(config.quote_style()));
        }
    }

    output.push('\n');
    formatting::write_indent(output, depth, &config.format_options());
    output.push(')');

    if head.inline_space.is_some() && head.inline_text.is_some() {
        output.push(' ');
    }

    if let Some(inline_text) = &head.inline_text {
        output.push_str(&inline_text.content);
    }
}

fn tag_head_prefix(head: &TagHead) -> String {
    let mut output = String::new();

    if let Some(tag_name) = &head.tag_name {
        output.push_str(tag_name);
    }

    if let Some(shorthand_id) = &head.shorthand_id {
        output.push('#');
        output.push_str(shorthand_id);
    }

    for shorthand_class in &head.shorthand_classes {
        output.push('.');
        output.push_str(shorthand_class);
    }

    output
}

fn plain_prose_segments(children: &[Node]) -> Option<Vec<String>> {
    if children.is_empty() {
        return None;
    }

    let mut segments = Vec::with_capacity(children.len());

    for child in children {
        let Node::RawText(text) = child else {
            return None;
        };

        if text.extra_indent != 0 || text.content.is_empty() {
            return None;
        }

        if text.content.trim() != text.content {
            return None;
        }

        if text.content.contains("  ") || text.content.contains("#[") || text.content.contains('<')
        {
            return None;
        }

        segments.push(text.content.clone());
    }

    Some(segments)
}

fn write_raw_text(
    output: &mut String,
    text: &RawTextNode,
    depth: usize,
    options: &PugFormatOptions,
) {
    if text.content.is_empty() && !text.preserve_base_indent && text.extra_indent == 0 {
        return;
    }

    formatting::write_indent(output, depth, options);
    for _ in 0..text.extra_indent {
        output.push(' ');
    }
    output.push_str(&text.content);
}

fn collapse_trailing_blank_lines(output: &mut String) {
    if !output.ends_with('\n') {
        output.push('\n');
    }

    while let Some(last_newline) = output.rfind('\n') {
        let prefix = &output[..last_newline];
        let Some(line_start) = prefix.rfind('\n').map(|index| index + 1) else {
            break;
        };
        if !output[line_start..last_newline]
            .chars()
            .all(char::is_whitespace)
        {
            break;
        }
        output.truncate(line_start);
    }
}
