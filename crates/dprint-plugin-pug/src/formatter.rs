use crate::ast::{Document, ElementNode, Node};
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
    Node::Element(element) => write_element(output, element, depth, indent_width),
    Node::Comment(text) => {
      write_indent(output, depth, indent_width);
      output.push_str("// ");
      output.push_str(text.trim());
    }
    Node::Text(text) => {
      write_indent(output, depth, indent_width);
      output.push_str(text.trim());
    }
  }
}

fn write_element(output: &mut String, element: &ElementNode, depth: usize, indent_width: usize) {
  write_indent(output, depth, indent_width);
  output.push_str(&element.tag);

  if let Some(id) = &element.id {
    output.push('#');
    output.push_str(id);
  }

  for class_name in &element.classes {
    output.push('.');
    output.push_str(class_name);
  }

  if let Some(text) = &element.text {
    let trimmed = text.trim();
    if !trimmed.is_empty() {
      output.push(' ');
      output.push_str(trimmed);
    }
  }

  for child in &element.children {
    output.push('\n');
    write_node(output, child, depth + 1, indent_width);
  }
}

fn write_indent(output: &mut String, depth: usize, indent_width: usize) {
  for _ in 0..(depth * indent_width) {
    output.push(' ');
  }
}

