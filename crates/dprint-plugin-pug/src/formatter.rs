use crate::ast::{Document, Node, RawTextNode, StatementNode};
use crate::config::Configuration;

pub fn format(document: &Document, config: &Configuration) -> String {
  let mut output = String::new();

  for (index, node) in document.children.iter().enumerate() {
    if index > 0 {
      output.push('\n');
    }
    write_node(&mut output, node, 0, config.indent_width());
  }

  if !output.ends_with('\n') {
    output.push('\n');
  }

  output
}

fn write_node(output: &mut String, node: &Node, depth: usize, indent_width: usize) {
  match node {
    Node::Statement(statement) => write_statement(output, statement, depth, indent_width),
    Node::Comment(text) => {
      write_indent(output, depth, indent_width);
      output.push_str("// ");
      output.push_str(text.trim());
    }
    Node::Text(text) => {
      write_indent(output, depth, indent_width);
      output.push('|');
      output.push_str(text);
    }
    Node::RawText(text) => write_raw_text(output, text, depth, indent_width),
  }
}

fn write_statement(output: &mut String, element: &StatementNode, depth: usize, indent_width: usize) {
  write_indent(output, depth, indent_width);
  output.push_str(element.content.trim());

  for child in &element.children {
    output.push('\n');
    write_node(output, child, depth + 1, indent_width);
  }
}

fn write_raw_text(output: &mut String, text: &RawTextNode, depth: usize, indent_width: usize) {
  write_indent(output, depth, indent_width);
  for _ in 0..text.extra_indent {
    output.push(' ');
  }
  output.push_str(&text.content);
}

fn write_indent(output: &mut String, depth: usize, indent_width: usize) {
  for _ in 0..(depth * indent_width) {
    output.push(' ');
  }
}
