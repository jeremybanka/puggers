use crate::ast::{DoctypeHead, Document, Node, RawTextNode, StatementHead, StatementNode, TagHead};
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

        let trimmed = content.trim();
        let (statement_content, is_text_block) = split_text_block_suffix(trimmed);

        let mut node = Node::Statement(StatementNode {
            head: parse_statement_head(statement_content),
            is_text_block,
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

fn split_text_block_suffix(content: &str) -> (&str, bool) {
    if content.ends_with('.') && !matches!(content, "." | "..") {
        (&content[..content.len() - 1], true)
    } else {
        (content, false)
    }
}

fn parse_statement_head(content: &str) -> StatementHead {
    if let Some(head) = parse_doctype_head(content) {
        return StatementHead::Doctype(head);
    }

    if let Some(head) = parse_tag_head(content) {
        return StatementHead::Tag(head);
    }

    StatementHead::Raw(content.to_string())
}

fn parse_doctype_head(content: &str) -> Option<DoctypeHead> {
    if content == "doctype" {
        return Some(DoctypeHead {
            spacing: None,
            value: None,
        });
    }

    let suffix = content.strip_prefix("doctype")?;
    if suffix.is_empty() || !suffix.chars().next().is_some_and(|ch| ch.is_whitespace()) {
        return None;
    }

    let spacing_len = suffix
        .chars()
        .take_while(|ch| ch.is_whitespace())
        .map(char::len_utf8)
        .sum();

    let spacing = &suffix[..spacing_len];
    let value = &suffix[spacing_len..];

    Some(DoctypeHead {
        spacing: Some(spacing.to_string()),
        value: Some(value.to_string()),
    })
}

fn parse_tag_head(content: &str) -> Option<TagHead> {
    let mut cursor = 0;
    let mut tag_name = None;
    let mut shorthand_id = None;
    let mut shorthand_classes = Vec::new();

    if let Some((name, next_cursor)) = parse_tag_name(content, cursor) {
        tag_name = Some(name.to_string());
        cursor = next_cursor;
    }

    while let Some(marker) = content[cursor..].chars().next() {
        if marker != '#' && marker != '.' {
            break;
        }

        let segment_start = cursor + marker.len_utf8();
        let (value, next_cursor) = parse_shorthand_value(content, segment_start)?;

        if marker == '#' {
            if shorthand_id.is_some() {
                return None;
            }
            shorthand_id = Some(value.to_string());
        } else {
            shorthand_classes.push(value.to_string());
        }

        cursor = next_cursor;
    }

    if tag_name.is_none() && shorthand_id.is_none() && shorthand_classes.is_empty() {
        return None;
    }

    let mut attributes = None;
    if content[cursor..].starts_with('(') {
        let end = find_matching_paren(content, cursor)?;
        attributes = Some(content[cursor + 1..end].to_string());
        cursor = end + 1;
    }

    let mut inline_space = None;
    let mut inline_text = None;
    if cursor < content.len() {
        let remainder = &content[cursor..];
        if !remainder
            .chars()
            .next()
            .is_some_and(|ch| ch.is_whitespace())
        {
            return None;
        }

        let spacing_len = remainder
            .chars()
            .take_while(|ch| ch.is_whitespace())
            .map(char::len_utf8)
            .sum();
        let spacing = &remainder[..spacing_len];
        let text = &remainder[spacing_len..];

        if text.is_empty() {
            return None;
        }

        inline_space = Some(spacing.to_string());
        inline_text = Some(text.to_string());
    }

    Some(TagHead {
        tag_name,
        shorthand_id,
        shorthand_classes,
        attributes,
        inline_space,
        inline_text,
    })
}

fn parse_tag_name(content: &str, start: usize) -> Option<(&str, usize)> {
    let mut chars = content[start..].char_indices();
    let (_, first) = chars.next()?;
    if !is_tag_name_start(first) {
        return None;
    }

    let mut end = start + first.len_utf8();
    for (offset, ch) in chars {
        if !is_tag_name_continue(ch) {
            break;
        }
        end = start + offset + ch.len_utf8();
    }

    Some((&content[start..end], end))
}

fn parse_shorthand_value(content: &str, start: usize) -> Option<(&str, usize)> {
    let mut chars = content[start..].char_indices();
    let (_, first) = chars.next()?;
    if !is_shorthand_char(first) {
        return None;
    }

    let mut end = start + first.len_utf8();
    for (offset, ch) in chars {
        if !is_shorthand_char(ch) {
            break;
        }
        end = start + offset + ch.len_utf8();
    }

    Some((&content[start..end], end))
}

fn find_matching_paren(content: &str, open_index: usize) -> Option<usize> {
    let mut depth = 0;
    let mut in_quote = None;
    let mut escaped = false;

    for (offset, ch) in content[open_index..].char_indices() {
        if let Some(quote) = in_quote {
            if escaped {
                escaped = false;
                continue;
            }

            if ch == '\\' {
                escaped = true;
                continue;
            }

            if ch == quote {
                in_quote = None;
            }
            continue;
        }

        match ch {
            '\'' | '"' => in_quote = Some(ch),
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open_index + offset);
                }
            }
            _ => {}
        }
    }

    None
}

fn is_tag_name_start(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_tag_name_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_')
}

fn is_shorthand_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_')
}
