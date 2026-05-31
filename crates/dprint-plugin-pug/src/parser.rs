use crate::ast::{Document, Node, RawTextNode, StatementNode};
use crate::lexer::LexedLine;

pub fn parse(lines: &[LexedLine]) -> Document {
    let (children, _) = parse_block(lines, 0, 0, ParseMode::Normal);
    Document { children }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseMode {
    Normal,
    RawText,
}

fn parse_block(
    lines: &[LexedLine],
    mut index: usize,
    current_indent: usize,
    mode: ParseMode,
) -> (Vec<Node>, usize) {
    let mut nodes = Vec::new();

    while index < lines.len() {
        let line = &lines[index];

        if line.is_blank {
            if mode == ParseMode::RawText {
                nodes.push(Node::RawText(RawTextNode {
                    extra_indent: line.indent.saturating_sub(current_indent),
                    content: String::new(),
                }));
            }
            index += 1;
            continue;
        }

        if line.indent < current_indent {
            break;
        }

        if mode == ParseMode::RawText {
            nodes.push(Node::RawText(RawTextNode {
                extra_indent: line.indent.saturating_sub(current_indent),
                content: line.content.clone(),
            }));
            index += 1;
            continue;
        }

        if line.indent > current_indent {
            index += 1;
            continue;
        }

        let content = line.content.trim_start();

        if let Some(comment) = content.strip_prefix("//") {
            nodes.push(Node::Comment(comment.trim().to_string()));
            index += 1;
            continue;
        }

        if let Some(text) = content.strip_prefix('|') {
            nodes.push(Node::Text(text.to_string()));
            index += 1;
            continue;
        }

        let mut node = Node::Statement(StatementNode {
            content: content.trim().to_string(),
            is_text_block: is_text_block(content),
            children: Vec::new(),
        });
        let next_index = index + 1;

        if next_index < lines.len() && lines[next_index].indent > current_indent {
            if let Node::Statement(statement) = &mut node {
                let next_mode = if statement.is_text_block {
                    ParseMode::RawText
                } else {
                    ParseMode::Normal
                };
                let (children, consumed_index) =
                    parse_block(lines, next_index, lines[next_index].indent, next_mode);
                statement.children = children;
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

fn is_text_block(content: &str) -> bool {
    content.ends_with('.') && !matches!(content, "." | "..")
}
