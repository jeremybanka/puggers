use crate::ast::{Document, Node, RawTextNode, StatementHead, StatementNode, TagHead};
use crate::config::Configuration;

pub fn format(document: &Document, config: &Configuration) -> String {
    let mut output = String::new();

    for (index, node) in document.children.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        write_node(&mut output, node, 0, config);
    }

    if !output.ends_with('\n') {
        output.push('\n');
    }

    output
}

fn write_node(output: &mut String, node: &Node, depth: usize, config: &Configuration) {
    match node {
        Node::Statement(statement) => write_statement(output, statement, depth, config),
        Node::Comment(text) => {
            write_indent(output, depth, config.indent_width(), config.use_tabs());
            output.push_str("// ");
            output.push_str(text.trim());
        }
        Node::Text(text) => {
            write_indent(output, depth, config.indent_width(), config.use_tabs());
            output.push('|');
            output.push_str(&text.content);
        }
        Node::RawText(text) => write_raw_text(
            output,
            text,
            depth,
            config.indent_width(),
            config.use_tabs(),
        ),
    }
}

fn write_statement(
    output: &mut String,
    element: &StatementNode,
    depth: usize,
    config: &Configuration,
) {
    write_indent(output, depth, config.indent_width(), config.use_tabs());
    match &element.head {
        StatementHead::Tag(head) if should_wrap_attributes(head, depth, config) => {
            write_wrapped_tag_head(output, head, depth, config)
        }
        _ => output.push_str(&element.head.to_source(config)),
    }
    if element.text_block_kind.is_some() {
        output.push('.');
    }

    for child in &element.children {
        output.push('\n');
        write_node(output, child, depth + 1, config);
    }
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
    display_width(depth, config) + rendered.len() > line_width
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
            write_indent(output, depth + 1, config.indent_width(), config.use_tabs());
            output.push_str(&attribute.to_source(config.quote_style()));
        }
    }

    output.push('\n');
    write_indent(output, depth, config.indent_width(), config.use_tabs());
    output.push(')');

    if let (Some(inline_space), Some(inline_text)) = (&head.inline_space, &head.inline_text) {
        output.push_str(inline_space);
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

fn display_width(depth: usize, config: &Configuration) -> usize {
    if config.use_tabs() {
        depth
    } else {
        depth * config.indent_width()
    }
}

fn write_raw_text(
    output: &mut String,
    text: &RawTextNode,
    depth: usize,
    indent_width: usize,
    use_tabs: bool,
) {
    write_indent(output, depth, indent_width, use_tabs);
    for _ in 0..text.extra_indent {
        output.push(' ');
    }
    output.push_str(&text.content);
}

fn write_indent(output: &mut String, depth: usize, indent_width: usize, use_tabs: bool) {
    if depth == 0 {
        return;
    }

    if use_tabs {
        for _ in 0..depth {
            output.push('\t');
        }
        return;
    }

    if indent_width == 0 {
        return;
    }

    for _ in 0..depth {
        for _ in 0..indent_width {
            output.push(' ');
        }
    }
}
