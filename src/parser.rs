use crate::ast::{Document, ElementNode, Node};
use crate::lexer::LexedLine;

pub fn parse(lines: &[LexedLine]) -> Document {
  let (children, _) = parse_block(lines, 0, 0);
  Document { children }
}

fn parse_block(lines: &[LexedLine], mut index: usize, current_indent: usize) -> (Vec<Node>, usize) {
  let mut nodes = Vec::new();

  while index < lines.len() {
    let line = &lines[index];

    if line.indent < current_indent {
      break;
    }

    if line.indent > current_indent {
      index += 1;
      continue;
    }

    if let Some(comment) = line.content.strip_prefix("//") {
      nodes.push(Node::Comment(comment.trim().to_string()));
      index += 1;
      continue;
    }

    let mut node = parse_node(&line.content);
    let next_index = index + 1;

    if next_index < lines.len() && lines[next_index].indent > current_indent {
      if let Node::Element(element) = &mut node {
        let (children, consumed_index) = parse_block(lines, next_index, lines[next_index].indent);
        element.children = children;
        index = consumed_index;
      } else {
        index = next_index;
      }
    } else {
      index = next_index;
    }

    nodes.push(node);
  }

  (nodes, index)
}

fn parse_node(content: &str) -> Node {
  let first = content.chars().next();
  if matches!(first, Some('|')) {
    return Node::Text(content[1..].trim().to_string());
  }

  let mut tag = String::from("div");
  let mut id = None;
  let mut classes = Vec::new();
  let mut text = None;
  let mut chars = content.char_indices().peekable();
  let mut saw_head = false;
  let mut text_start = None;

  while let Some((index, ch)) = chars.next() {
    match ch {
      '#' => {
        let value = consume_ident(content, &mut chars);
        if !value.is_empty() {
          id = Some(value);
        }
      }
      '.' => {
        let value = consume_ident(content, &mut chars);
        if !value.is_empty() {
          classes.push(value);
        }
      }
      ' ' | '\t' => {
        text_start = Some(index + ch.len_utf8());
        break;
      }
      _ => {
        if !saw_head && is_ident_char(ch) {
          let mut end = index + ch.len_utf8();
          while let Some((next_index, next_char)) = chars.peek().copied() {
            if !is_ident_char(next_char) {
              break;
            }
            end = next_index + next_char.len_utf8();
            chars.next();
          }
          tag = content[index..end].to_string();
          saw_head = true;
        }
      }
    }
  }

  if let Some(start) = text_start {
    let value = content[start..].trim();
    if !value.is_empty() {
      text = Some(value.to_string());
    }
  }

  Node::Element(ElementNode {
    tag,
    id,
    classes,
    text,
    children: Vec::new(),
  })
}

fn consume_ident(
  source: &str,
  chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
) -> String {
  let mut start = None;
  let mut end = None;

  while let Some((index, ch)) = chars.peek().copied() {
    if !is_ident_char(ch) {
      break;
    }
    start.get_or_insert(index);
    end = Some(index + ch.len_utf8());
    chars.next();
  }

  match (start, end) {
    (Some(start), Some(end)) => source[start..end].to_string(),
    _ => String::new(),
  }
}

fn is_ident_char(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_')
}

