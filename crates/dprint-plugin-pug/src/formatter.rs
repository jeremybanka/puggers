use crate::ast::{Document, Node, RawTextNode, StatementNode};
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
            output.push_str(text);
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
    output.push_str(&element.head.to_source(config));
    if element.is_text_block {
        output.push('.');
    }

    for child in &element.children {
        output.push('\n');
        write_node(output, child, depth + 1, config);
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
