use crate::ast::{
    Attribute, AttributeValue, BlockHead, BlockMode, CodeHead, CodeKind, CommentKind, CommentNode,
    ControlFlowHead, ControlFlowKind, DoctypeHead, Document, ExtendsHead, FilterHead, IncludeHead,
    InlineText, InlineTextKind, MixinCallHead, MixinHead, Node, QuoteStyle, RawTextNode,
    StatementHead, StatementNode, TagHead, TextBlockKind, TextLineKind, TextLineNode,
};
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
                    preserve_base_indent: line.indent >= current_indent,
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
                preserve_base_indent: true,
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

        if let Some((kind, value)) = parse_comment_head(content) {
            let mut children = Vec::new();
            let mut next_index = index + 1;

            if next_index < lines.len() && lines[next_index].indent > current_indent {
                let (parsed_children, consumed_index) =
                    parse_raw_text_block(lines, next_index, lines[next_index].indent);
                children = parsed_children;
                next_index = consumed_index;
            }

            nodes.push(Node::Comment(CommentNode {
                kind,
                value,
                children,
            }));
            index = next_index;
            continue;
        }

        if let Some(text) = content.strip_prefix('|') {
            nodes.push(Node::Text(TextLineNode {
                kind: TextLineKind::Piped,
                content: text.to_string(),
            }));
            index += 1;
            continue;
        }

        let (statement_content, next_index) = collect_statement_lines(lines, index, current_indent);
        let (statement_content, has_text_block_suffix) =
            split_text_block_suffix(&statement_content);
        let head = parse_statement_head(statement_content);
        let text_block_kind = determine_text_block_kind(&head, has_text_block_suffix);

        let mut node = Node::Statement(StatementNode {
            head,
            text_block_kind,
            children: Vec::new(),
        });

        if next_index < lines.len() && lines[next_index].indent > current_indent {
            if let Node::Statement(statement) = &mut node {
                let next_mode = if statement.text_block_kind.is_some() {
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

fn collect_statement_lines(
    lines: &[LexedLine],
    start_index: usize,
    current_indent: usize,
) -> (String, usize) {
    let mut content = lines[start_index].content.trim_start().to_string();
    if !should_collect_multiline_statement(&content) {
        return (content, start_index + 1);
    }

    let mut index = start_index + 1;
    while index < lines.len() && has_unclosed_parenthesis(&content) {
        let line = &lines[index];
        if line.indent < current_indent {
            break;
        }

        content.push('\n');
        content.push_str(line.content.trim());
        index += 1;
    }

    (content, index)
}

fn should_collect_multiline_statement(content: &str) -> bool {
    starts_attribute_list_in_head(content) && has_unclosed_parenthesis(content)
}

fn has_unclosed_parenthesis(content: &str) -> bool {
    let mut in_quote = None;
    let mut escaped = false;
    let mut depth = 0isize;

    for ch in content.chars() {
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
            ')' => depth -= 1,
            _ => {}
        }
    }

    depth > 0
}

fn starts_attribute_list_in_head(content: &str) -> bool {
    let mut cursor = 0;

    if let Some((_, next_cursor)) = parse_tag_name(content, cursor) {
        cursor = next_cursor;
    }

    while let Some(marker) = content[cursor..].chars().next() {
        if marker != '#' && marker != '.' {
            break;
        }

        let segment_start = cursor + marker.len_utf8();
        let Some((_, next_cursor)) = parse_shorthand_value(content, segment_start) else {
            return false;
        };
        cursor = next_cursor;
    }

    content[cursor..].starts_with('(')
}

fn split_text_block_suffix(content: &str) -> (&str, bool) {
    let trimmed_end = content.trim_end_matches(char::is_whitespace);

    if trimmed_end == "." {
        return ("", true);
    }

    if matches!(trimmed_end, "" | "..") {
        return (content, false);
    }

    if let Some(without_dot) = trimmed_end.strip_suffix('.')
        && (without_dot.is_empty()
            || !without_dot
                .chars()
                .last()
                .is_some_and(|ch| ch.is_whitespace()))
    {
        return (without_dot, true);
    }

    (content, false)
}

fn parse_statement_head(content: &str) -> StatementHead {
    if let Some(head) = parse_doctype_head(content) {
        return StatementHead::Doctype(head);
    }

    if let Some(head) = parse_code_head(content) {
        return StatementHead::Code(head);
    }

    if let Some(head) = parse_control_flow_head(content) {
        return StatementHead::ControlFlow(head);
    }

    if let Some(head) = parse_filter_head(content) {
        return StatementHead::Filter(head);
    }

    if let Some(head) = parse_include_head(content) {
        return StatementHead::Include(head);
    }

    if let Some(head) = parse_extends_head(content) {
        return StatementHead::Extends(head);
    }

    if let Some(head) = parse_block_head(content) {
        return StatementHead::Block(head);
    }

    if let Some(head) = parse_mixin_head(content) {
        return StatementHead::Mixin(head);
    }

    if let Some(head) = parse_mixin_call_head(content) {
        return StatementHead::MixinCall(head);
    }

    if let Some(head) = parse_tag_head(content) {
        return StatementHead::Tag(head);
    }

    StatementHead::Raw(content.to_string())
}

fn determine_text_block_kind(
    head: &StatementHead,
    has_text_block_suffix: bool,
) -> Option<TextBlockKind> {
    if matches!(head, StatementHead::Filter(_)) {
        return Some(TextBlockKind::Raw);
    }

    has_text_block_suffix.then(|| classify_text_block_kind(head))
}

fn parse_comment_head(content: &str) -> Option<(CommentKind, Option<String>)> {
    if let Some(comment) = content.strip_prefix("//-") {
        return Some((CommentKind::Unbuffered, parse_optional_payload(comment)));
    }

    let comment = content.strip_prefix("//")?;
    Some((CommentKind::Buffered, parse_optional_payload(comment)))
}

fn parse_filter_head(content: &str) -> Option<FilterHead> {
    let name = content.strip_prefix(':')?;
    if name.is_empty() || name.chars().any(char::is_whitespace) {
        return None;
    }

    Some(FilterHead {
        name: name.to_string(),
    })
}

fn parse_code_head(content: &str) -> Option<CodeHead> {
    if let Some(suffix) = content.strip_prefix("!=") {
        return Some(CodeHead {
            kind: CodeKind::UnescapedBuffered,
            suffix: suffix.to_string(),
        });
    }

    if let Some(suffix) = content.strip_prefix('=') {
        return Some(CodeHead {
            kind: CodeKind::EscapedBuffered,
            suffix: suffix.to_string(),
        });
    }

    if let Some(suffix) = content.strip_prefix('-') {
        return Some(CodeHead {
            kind: CodeKind::Unbuffered,
            suffix: suffix.to_string(),
        });
    }

    None
}

fn parse_control_flow_head(content: &str) -> Option<ControlFlowHead> {
    const KEYWORDS: &[(ControlFlowKind, &str)] = &[
        (ControlFlowKind::ElseIf, "else if"),
        (ControlFlowKind::Else, "else"),
        (ControlFlowKind::If, "if"),
        (ControlFlowKind::Case, "case"),
        (ControlFlowKind::When, "when"),
        (ControlFlowKind::Default, "default"),
        (ControlFlowKind::Each, "each"),
        (ControlFlowKind::While, "while"),
    ];

    for (kind, keyword) in KEYWORDS {
        let Some(suffix) = content.strip_prefix(keyword) else {
            continue;
        };

        if !starts_control_flow_suffix(suffix) {
            continue;
        }

        return Some(ControlFlowHead {
            kind: *kind,
            suffix: suffix.to_string(),
        });
    }

    None
}

fn parse_include_head(content: &str) -> Option<IncludeHead> {
    let suffix = content.strip_prefix("include")?;
    if !starts_keyword_suffix(suffix) {
        return None;
    }

    Some(IncludeHead {
        suffix: suffix.to_string(),
    })
}

fn parse_extends_head(content: &str) -> Option<ExtendsHead> {
    let suffix = content.strip_prefix("extends")?;
    if !starts_keyword_suffix(suffix) {
        return None;
    }

    Some(ExtendsHead {
        suffix: suffix.to_string(),
    })
}

fn parse_block_head(content: &str) -> Option<BlockHead> {
    let suffix = content.strip_prefix("block")?;
    if !starts_keyword_suffix(suffix) {
        return None;
    }

    let trimmed = suffix.trim_start();
    let (mode, target) = if let Some(rest) = trimmed.strip_prefix("append")
        && starts_keyword_suffix(rest)
    {
        (Some(BlockMode::Append), parse_optional_payload(rest))
    } else if let Some(rest) = trimmed.strip_prefix("prepend")
        && starts_keyword_suffix(rest)
    {
        (Some(BlockMode::Prepend), parse_optional_payload(rest))
    } else {
        (None, parse_optional_payload(trimmed))
    };

    Some(BlockHead {
        mode,
        target,
        suffix: suffix.to_string(),
    })
}

fn parse_mixin_head(content: &str) -> Option<MixinHead> {
    let suffix = content.strip_prefix("mixin")?;
    if !starts_keyword_suffix(suffix) {
        return None;
    }

    Some(MixinHead {
        suffix: suffix.to_string(),
    })
}

fn parse_mixin_call_head(content: &str) -> Option<MixinCallHead> {
    let suffix = content.strip_prefix('+')?;
    if suffix.is_empty() {
        return None;
    }

    Some(MixinCallHead {
        suffix: suffix.to_string(),
    })
}

fn starts_keyword_suffix(suffix: &str) -> bool {
    suffix.is_empty() || suffix.chars().next().is_some_and(|ch| ch.is_whitespace())
}

fn parse_optional_payload(content: &str) -> Option<String> {
    let trimmed = content.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn parse_raw_text_block(
    lines: &[LexedLine],
    index: usize,
    current_indent: usize,
) -> (Vec<RawTextNode>, usize) {
    let (children, consumed_index) = parse_block(lines, index, current_indent, ParseMode::RawText);
    let raw_text = children
        .into_iter()
        .map(|node| match node {
            Node::RawText(text) => text,
            _ => unreachable!("raw text parse mode should only produce raw text nodes"),
        })
        .collect();

    (raw_text, consumed_index)
}

fn starts_control_flow_suffix(suffix: &str) -> bool {
    suffix.is_empty()
        || suffix
            .chars()
            .next()
            .is_some_and(|ch| ch.is_whitespace() || !is_identifier_continue(ch))
}

fn is_identifier_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
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
        attributes = Some(parse_attributes(&content[cursor + 1..end])?);
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

        if !text.is_empty() {
            inline_space = Some(spacing.to_string());
            inline_text = Some(InlineText {
                kind: classify_inline_text(text),
                content: text.to_string(),
            });
        }
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

fn classify_text_block_kind(head: &StatementHead) -> TextBlockKind {
    match head {
        StatementHead::Filter(_) => TextBlockKind::Raw,
        StatementHead::Tag(head)
            if head
                .tag_name
                .as_deref()
                .is_some_and(is_code_like_raw_text_tag) =>
        {
            TextBlockKind::Raw
        }
        _ => TextBlockKind::Prose,
    }
}

fn is_code_like_raw_text_tag(tag: &str) -> bool {
    matches!(tag, "pre" | "script" | "style" | "textarea")
}

fn classify_inline_text(text: &str) -> InlineTextKind {
    if text.trim_start().starts_with('<') {
        return InlineTextKind::LiteralHtml;
    }

    if text.contains("#[") || text.contains("#{") || text.contains("!{") {
        return InlineTextKind::Interpolated;
    }

    InlineTextKind::Plain
}

fn parse_attributes(content: &str) -> Option<Vec<Attribute>> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Some(Vec::new());
    }

    let mut attributes = Vec::new();
    for entry in split_top_level_attributes(trimmed) {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        attributes.push(parse_attribute(entry)?);
    }

    Some(attributes)
}

fn parse_attribute(content: &str) -> Option<Attribute> {
    let trimmed = content.trim();
    let Some(split_index) = find_top_level_equals(trimmed) else {
        return Some(Attribute {
            name: trimmed.to_string(),
            value: None,
        });
    };

    let name = trimmed[..split_index].trim();
    let value = trimmed[split_index + 1..].trim();

    if name.is_empty() || value.is_empty() {
        return None;
    }

    Some(Attribute {
        name: name.to_string(),
        value: Some(parse_attribute_value(value)),
    })
}

fn parse_attribute_value(content: &str) -> AttributeValue {
    if let Some((quote_style, value)) = parse_quoted_value(content) {
        return AttributeValue::Quoted { value, quote_style };
    }

    AttributeValue::Expression(content.to_string())
}

fn parse_quoted_value(content: &str) -> Option<(QuoteStyle, String)> {
    if content.len() < 2 {
        return None;
    }

    let mut chars = content.chars();
    let first = chars.next()?;
    let last = content.chars().last()?;

    let quote_style = match first {
        '"' if last == '"' => QuoteStyle::Double,
        '\'' if last == '\'' => QuoteStyle::Single,
        _ => return None,
    };

    if !is_wrapped_in_single_top_level_quote(content, first) {
        return None;
    }

    let inner = &content[first.len_utf8()..content.len() - last.len_utf8()];
    Some((quote_style, inner.to_string()))
}

fn is_wrapped_in_single_top_level_quote(content: &str, quote: char) -> bool {
    let mut escaped = false;
    let mut close_index = None;

    for (index, ch) in content.char_indices().skip(1) {
        if escaped {
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if ch == quote {
            close_index = Some(index);
            break;
        }
    }

    close_index == Some(content.len() - quote.len_utf8())
}

fn find_top_level_equals(content: &str) -> Option<usize> {
    let mut in_quote = None;
    let mut escaped = false;
    let mut paren_depth = 0isize;
    let mut bracket_depth = 0isize;
    let mut brace_depth = 0isize;

    for (index, ch) in content.char_indices() {
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
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            '{' => brace_depth += 1,
            '}' => brace_depth -= 1,
            '=' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                return Some(index);
            }
            _ => {}
        }
    }

    None
}

fn split_top_level_attributes(content: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_quote = None;
    let mut escaped = false;
    let mut paren_depth = 0isize;
    let mut bracket_depth = 0isize;
    let mut brace_depth = 0isize;

    for (index, ch) in content.char_indices() {
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
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            '{' => brace_depth += 1,
            '}' => brace_depth -= 1,
            ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                parts.push(&content[start..index]);
                start = index + ch.len_utf8();
            }
            '\n' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                parts.push(&content[start..index]);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    parts.push(&content[start..]);
    parts
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
