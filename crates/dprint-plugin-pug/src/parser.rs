use crate::ast::{
    Attribute, AttributeValue, BlockHead, BlockMode, CodeHead, CodeKind, CommentKind, CommentNode,
    ControlFlowHead, ControlFlowKind, DoctypeHead, Document, ExtendsHead, FilterHead, IncludeHead,
    InlineText, InlineTextKind, MixinCallHead, MixinHead, Node, QuoteStyle, RawTextNode,
    StatementHead, StatementNode, TagHead, TextBlockKind, TextLineKind, TextLineNode,
};
use crate::lexer::LexedLine;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseReport {
    pub document: Document,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn parse(lines: &[LexedLine]) -> Document {
    parse_with_diagnostics(lines).document
}

pub fn parse_with_diagnostics(lines: &[LexedLine]) -> ParseReport {
    let mut diagnostics = Vec::new();
    let (children, _) = parse_block(lines, 0, 0, ParseMode::Normal, &mut diagnostics);

    ParseReport {
        document: Document { children },
        diagnostics,
    }
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
    diagnostics: &mut Vec<Diagnostic>,
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

        let recovered_indent = line.indent > current_indent;
        if recovered_indent {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                line: index + 1,
                message: format!(
                    "Recovered inconsistent indentation: expected {} spaces in this block, found {}",
                    current_indent, line.indent
                ),
            });
        }

        let content = line.content.trim_start();
        let nesting_parent_indent = if recovered_indent {
            line.indent
        } else {
            current_indent
        };

        if let Some((kind, value)) = parse_comment_head(content) {
            let mut children = Vec::new();
            let mut next_index = index + 1;

            if block_has_children(lines, next_index, nesting_parent_indent) {
                let child_indent = determine_child_indent(lines, next_index, nesting_parent_indent);
                let (parsed_children, consumed_index) =
                    parse_raw_text_block(lines, next_index, child_indent, diagnostics);
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
        let head = parse_statement_head(statement_content, index + 1, diagnostics);
        validate_statement_context(&head, &nodes, index + 1, diagnostics);
        let text_block_kind = determine_text_block_kind(&head, has_text_block_suffix);

        let mut node = Node::Statement(StatementNode {
            head,
            text_block_kind,
            children: Vec::new(),
        });

        if block_has_children(lines, next_index, nesting_parent_indent) {
            if let Node::Statement(statement) = &mut node {
                let next_mode = if statement.text_block_kind.is_some() {
                    ParseMode::RawText
                } else {
                    ParseMode::Normal
                };
                let child_indent = determine_child_indent(lines, next_index, nesting_parent_indent);
                let (children, consumed_index) =
                    parse_block(lines, next_index, child_indent, next_mode, diagnostics);
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

    if mode == ParseMode::RawText {
        trim_blank_raw_text_edges(&mut nodes);
    }

    (nodes, index)
}

fn trim_blank_raw_text_edges(nodes: &mut Vec<Node>) {
    let leading_non_blank = nodes
        .iter()
        .position(|node| !matches!(node, Node::RawText(text) if text.content.is_empty()))
        .unwrap_or(nodes.len());
    if leading_non_blank > 0 {
        nodes.drain(..leading_non_blank);
    }

    let trailing_blank_count = nodes
        .iter()
        .rev()
        .take_while(|node| matches!(node, Node::RawText(text) if text.content.is_empty()))
        .count();
    if trailing_blank_count > 0 {
        let keep_len = nodes.len() - trailing_blank_count;
        nodes.truncate(keep_len);
    }
}

fn block_has_children(lines: &[LexedLine], start_index: usize, parent_indent: usize) -> bool {
    let mut index = start_index;

    while index < lines.len() {
        let line = &lines[index];
        if line.is_blank {
            index += 1;
            continue;
        }

        return line.indent > parent_indent;
    }

    false
}

fn determine_child_indent(lines: &[LexedLine], start_index: usize, parent_indent: usize) -> usize {
    let mut minimum_indent: Option<usize> = None;
    let mut index = start_index;

    while index < lines.len() {
        let line = &lines[index];

        if !line.is_blank && line.indent <= parent_indent {
            break;
        }

        if !line.is_blank && line.indent > parent_indent {
            minimum_indent = Some(match minimum_indent {
                Some(current_minimum) => current_minimum.min(line.indent),
                None => line.indent,
            });
        }

        index += 1;
    }

    minimum_indent.unwrap_or(lines[start_index].indent)
}

fn collect_statement_lines(
    lines: &[LexedLine],
    start_index: usize,
    current_indent: usize,
) -> (String, usize) {
    let mut content = lines[start_index].content.trim_start().to_string();
    let Some(collection_mode) = multiline_statement_mode(&content) else {
        return (content, start_index + 1);
    };

    let mut index = start_index + 1;
    while index < lines.len() && has_unclosed_parenthesis(&content) {
        let line = &lines[index];
        if line.indent < current_indent {
            break;
        }

        content.push('\n');
        match collection_mode {
            MultilineStatementMode::Normalized => content.push_str(line.content.trim()),
            MultilineStatementMode::PreserveLayout => {
                content.push_str(&" ".repeat(line.indent));
                content.push_str(&line.content);
            }
        }
        index += 1;
    }

    (content, index)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MultilineStatementMode {
    Normalized,
    PreserveLayout,
}

fn multiline_statement_mode(content: &str) -> Option<MultilineStatementMode> {
    if !has_unclosed_parenthesis(content) {
        return None;
    }

    if starts_attribute_list_in_head(content) {
        return Some(MultilineStatementMode::Normalized);
    }

    if content.starts_with('+') {
        return Some(MultilineStatementMode::PreserveLayout);
    }

    None
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

fn parse_statement_head(
    content: &str,
    line: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> StatementHead {
    if let Some(head) = parse_doctype_head(content) {
        return StatementHead::Doctype(head);
    }

    if let Some(head) = parse_code_head(content) {
        return StatementHead::Code(head);
    }

    if let Some(head) = parse_control_flow_head(content) {
        validate_control_flow_head(&head, line, diagnostics);
        return StatementHead::ControlFlow(head);
    }

    if let Some(head) = parse_filter_head(content) {
        return StatementHead::Filter(head);
    }

    if let Some(head) = parse_include_head(content) {
        validate_include_head(&head, line, diagnostics);
        return StatementHead::Include(head);
    }

    if let Some(head) = parse_extends_head(content) {
        validate_extends_head(&head, line, diagnostics);
        return StatementHead::Extends(head);
    }

    if let Some(head) = parse_block_head(content) {
        validate_block_head(&head, line, diagnostics);
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

fn validate_include_head(head: &IncludeHead, line: usize, diagnostics: &mut Vec<Diagnostic>) {
    if head.suffix.trim().is_empty() {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            line,
            message: String::from("Recovered `include` without a path"),
        });
    }
}

fn validate_extends_head(head: &ExtendsHead, line: usize, diagnostics: &mut Vec<Diagnostic>) {
    if head.suffix.trim().is_empty() {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            line,
            message: String::from("Recovered `extends` without a path"),
        });
    }
}

fn validate_block_head(head: &BlockHead, line: usize, diagnostics: &mut Vec<Diagnostic>) {
    if head.mode.is_some() && head.target.is_none() {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            line,
            message: String::from("Recovered `block append`/`block prepend` without a target"),
        });
    }
}

fn validate_control_flow_head(
    head: &ControlFlowHead,
    line: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let missing_payload = match head.kind {
        ControlFlowKind::If
        | ControlFlowKind::ElseIf
        | ControlFlowKind::Unless
        | ControlFlowKind::Case
        | ControlFlowKind::When
        | ControlFlowKind::Each
        | ControlFlowKind::While => head.suffix.trim().is_empty(),
        ControlFlowKind::Else | ControlFlowKind::Default => false,
    };

    if missing_payload {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            line,
            message: format!(
                "Recovered `{}` without the required expression",
                control_flow_keyword(head.kind)
            ),
        });
    }

    let unexpected_payload = match head.kind {
        ControlFlowKind::Else => !head.suffix.trim().is_empty(),
        ControlFlowKind::Default => {
            let trimmed = head.suffix.trim_start();
            !trimmed.is_empty() && !trimmed.starts_with(':')
        }
        ControlFlowKind::If
        | ControlFlowKind::ElseIf
        | ControlFlowKind::Unless
        | ControlFlowKind::Case
        | ControlFlowKind::When
        | ControlFlowKind::Each
        | ControlFlowKind::While => false,
    };

    if unexpected_payload {
        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            line,
            message: format!(
                "Recovered `{}` with unexpected trailing content",
                control_flow_keyword(head.kind)
            ),
        });
    }
}

fn validate_statement_context(
    head: &StatementHead,
    prior_nodes: &[Node],
    line: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let StatementHead::ControlFlow(head) = head else {
        return;
    };

    match head.kind {
        ControlFlowKind::Else => {
            if !previous_statement_head(prior_nodes).is_some_and(is_valid_else_predecessor) {
                diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Warning,
                    line,
                    message: String::from("Recovered orphaned `else` without a matching `if`"),
                });
            }
        }
        ControlFlowKind::Default
            if !matches!(
                previous_statement_head(prior_nodes),
                Some(StatementHead::ControlFlow(ControlFlowHead {
                    kind: ControlFlowKind::When,
                    ..
                }))
            ) =>
        {
            diagnostics.push(Diagnostic {
                severity: DiagnosticSeverity::Warning,
                line,
                message: String::from("Recovered orphaned `default` without a preceding `when`"),
            });
        }
        _ => {}
    }
}

fn previous_statement_head(nodes: &[Node]) -> Option<&StatementHead> {
    nodes.iter().rev().find_map(|node| match node {
        Node::Statement(statement) => Some(&statement.head),
        Node::Comment(_) | Node::Text(_) | Node::RawText(_) => None,
    })
}

fn is_valid_else_predecessor(head: &StatementHead) -> bool {
    matches!(
        head,
        StatementHead::ControlFlow(ControlFlowHead {
            kind: ControlFlowKind::If
                | ControlFlowKind::ElseIf
                | ControlFlowKind::Unless
                | ControlFlowKind::Each,
            ..
        })
    ) || is_for_loop_head(head)
}

fn is_for_loop_head(head: &StatementHead) -> bool {
    let StatementHead::Tag(head) = head else {
        return false;
    };

    head.tag_name.as_deref() == Some("for")
}

fn control_flow_keyword(kind: ControlFlowKind) -> &'static str {
    match kind {
        ControlFlowKind::If => "if",
        ControlFlowKind::ElseIf => "else if",
        ControlFlowKind::Else => "else",
        ControlFlowKind::Unless => "unless",
        ControlFlowKind::Case => "case",
        ControlFlowKind::When => "when",
        ControlFlowKind::Default => "default",
        ControlFlowKind::Each => "each",
        ControlFlowKind::While => "while",
    }
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
        (ControlFlowKind::Unless, "unless"),
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
    diagnostics: &mut Vec<Diagnostic>,
) -> (Vec<RawTextNode>, usize) {
    let (children, consumed_index) = parse_block(
        lines,
        index,
        current_indent,
        ParseMode::RawText,
        diagnostics,
    );
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
        if remainder
            .chars()
            .next()
            .is_some_and(|ch| ch.is_whitespace())
        {
            let separator_len = remainder
                .chars()
                .next()
                .map(char::len_utf8)
                .expect("whitespace remainder should have a first character");
            let separator = &remainder[..separator_len];
            let text = &remainder[separator_len..];

            if text.chars().any(|ch| !ch.is_whitespace()) {
                inline_space = Some(separator.to_string());
                inline_text = Some(InlineText {
                    kind: classify_inline_text(text),
                    content: text.to_string(),
                });
            }
        } else if attributes.is_some() {
            inline_space = Some(String::from(" "));
            inline_text = Some(InlineText {
                kind: classify_inline_text(remainder),
                content: remainder.to_string(),
            });
        } else {
            return None;
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
    let mut cursor = 0;

    while cursor < trimmed.len() {
        cursor = skip_attribute_separators(trimmed, cursor);
        if cursor >= trimmed.len() {
            break;
        }

        let (attribute, next_cursor) = parse_attribute_entry(trimmed, cursor)?;
        attributes.push(attribute);
        cursor = next_cursor;
    }

    Some(attributes)
}

fn skip_attribute_separators(content: &str, mut cursor: usize) -> usize {
    while cursor < content.len() {
        let ch = content[cursor..]
            .chars()
            .next()
            .expect("cursor should remain on a character boundary");
        if ch == ',' || ch.is_whitespace() {
            cursor += ch.len_utf8();
            continue;
        }
        break;
    }

    cursor
}

fn parse_attribute_entry(content: &str, start: usize) -> Option<(Attribute, usize)> {
    let name_end = consume_attribute_name_token(content, start)?;
    let name = &content[start..name_end];
    let mut cursor = skip_inline_attribute_whitespace(content, name_end);

    if cursor >= content.len() || !content[cursor..].starts_with('=') {
        return Some((
            Attribute {
                name: name.to_string(),
                value: None,
            },
            name_end,
        ));
    }

    cursor += '='.len_utf8();
    cursor = skip_inline_attribute_whitespace(content, cursor);
    let value_end = scan_attribute_value_end(content, cursor)?;
    let value = &content[cursor..value_end];
    if value.is_empty() {
        return None;
    }

    Some((
        Attribute {
            name: name.to_string(),
            value: Some(parse_attribute_value(value)),
        },
        value_end,
    ))
}

fn skip_inline_attribute_whitespace(content: &str, mut cursor: usize) -> usize {
    while cursor < content.len() {
        let ch = content[cursor..]
            .chars()
            .next()
            .expect("cursor should remain on a character boundary");
        if is_attribute_linebreak(ch) || !ch.is_whitespace() {
            break;
        }
        cursor += ch.len_utf8();
    }

    cursor
}

fn scan_attribute_value_end(content: &str, start: usize) -> Option<usize> {
    if start >= content.len() {
        return None;
    }

    let mut cursor = start;
    let mut in_quote = None;
    let mut escaped = false;
    let mut paren_depth = 0isize;
    let mut bracket_depth = 0isize;
    let mut brace_depth = 0isize;
    let mut can_terminate_without_separator = false;

    while cursor < content.len() {
        if can_terminate_without_separator && starts_attribute_candidate(content, cursor) {
            return Some(cursor);
        }

        let ch = content[cursor..]
            .chars()
            .next()
            .expect("cursor should remain on a character boundary");

        if let Some(quote) = in_quote {
            if escaped {
                escaped = false;
                cursor += ch.len_utf8();
                continue;
            }

            if ch == '\\' {
                escaped = true;
                cursor += ch.len_utf8();
                continue;
            }

            if ch == quote {
                in_quote = None;
                can_terminate_without_separator =
                    paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
            }

            cursor += ch.len_utf8();
            continue;
        }

        match ch {
            '\'' | '"' => {
                in_quote = Some(ch);
                can_terminate_without_separator = false;
                cursor += ch.len_utf8();
            }
            '(' => {
                paren_depth += 1;
                can_terminate_without_separator = false;
                cursor += ch.len_utf8();
            }
            ')' => {
                paren_depth -= 1;
                can_terminate_without_separator =
                    paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
                cursor += ch.len_utf8();
            }
            '[' => {
                bracket_depth += 1;
                can_terminate_without_separator = false;
                cursor += ch.len_utf8();
            }
            ']' => {
                bracket_depth -= 1;
                can_terminate_without_separator =
                    paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
                cursor += ch.len_utf8();
            }
            '{' => {
                brace_depth += 1;
                can_terminate_without_separator = false;
                cursor += ch.len_utf8();
            }
            '}' => {
                brace_depth -= 1;
                can_terminate_without_separator =
                    paren_depth == 0 && bracket_depth == 0 && brace_depth == 0;
                cursor += ch.len_utf8();
            }
            ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                return Some(cursor);
            }
            ch if is_attribute_linebreak(ch)
                && paren_depth == 0
                && bracket_depth == 0
                && brace_depth == 0 =>
            {
                return Some(cursor);
            }
            ch if ch.is_whitespace()
                && paren_depth == 0
                && bracket_depth == 0
                && brace_depth == 0 =>
            {
                let next_cursor = skip_horizontal_attribute_whitespace(content, cursor);
                if next_cursor >= content.len()
                    || content[next_cursor..].starts_with(',')
                    || content[next_cursor..]
                        .chars()
                        .next()
                        .is_some_and(is_attribute_linebreak)
                {
                    return Some(cursor);
                }
                if can_terminate_without_separator
                    && starts_attribute_candidate(content, next_cursor)
                {
                    return Some(cursor);
                }
                if looks_like_next_attribute(content, next_cursor) {
                    return Some(cursor);
                }
                can_terminate_without_separator = false;
                cursor = next_cursor;
            }
            _ => {
                can_terminate_without_separator = false;
                cursor += ch.len_utf8();
            }
        }
    }

    Some(cursor)
}

fn skip_horizontal_attribute_whitespace(content: &str, mut cursor: usize) -> usize {
    while cursor < content.len() {
        let ch = content[cursor..]
            .chars()
            .next()
            .expect("cursor should remain on a character boundary");
        if is_attribute_linebreak(ch) || !ch.is_whitespace() {
            break;
        }
        cursor += ch.len_utf8();
    }

    cursor
}

fn looks_like_next_attribute(content: &str, start: usize) -> bool {
    if start >= content.len() {
        return false;
    }

    let Some(name_end) = consume_attribute_name_token(content, start) else {
        return false;
    };

    let cursor = skip_inline_attribute_whitespace(content, name_end);
    cursor >= content.len()
        || content[cursor..].starts_with('=')
        || content[cursor..].starts_with(',')
        || content[cursor..]
            .chars()
            .next()
            .is_some_and(is_attribute_linebreak)
}

fn starts_attribute_candidate(content: &str, start: usize) -> bool {
    consume_attribute_name_token(content, start).is_some()
}

fn consume_attribute_name_token(content: &str, start: usize) -> Option<usize> {
    if start >= content.len() {
        return None;
    }

    let first = content[start..].chars().next()?;
    if matches!(first, '\'' | '"') {
        return consume_quoted_segment(content, start);
    }

    if !is_attribute_name_start(first) {
        return None;
    }

    let mut cursor = start + first.len_utf8();
    while cursor < content.len() {
        let ch = content[cursor..]
            .chars()
            .next()
            .expect("cursor should remain on a character boundary");
        if !is_attribute_name_continue(ch) {
            break;
        }
        cursor += ch.len_utf8();
    }

    Some(cursor)
}

fn consume_quoted_segment(content: &str, start: usize) -> Option<usize> {
    let quote = content[start..].chars().next()?;
    let mut cursor = start + quote.len_utf8();
    let mut escaped = false;

    while cursor < content.len() {
        let ch = content[cursor..].chars().next()?;
        if escaped {
            escaped = false;
            cursor += ch.len_utf8();
            continue;
        }

        if ch == '\\' {
            escaped = true;
            cursor += ch.len_utf8();
            continue;
        }

        cursor += ch.len_utf8();
        if ch == quote {
            return Some(cursor);
        }
    }

    None
}

fn is_attribute_name_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || matches!(ch, '_' | ':' | '@' | '[')
}

fn is_attribute_name_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | ':' | '@' | '.' | '$' | '[' | ']')
}

fn is_attribute_linebreak(ch: char) -> bool {
    matches!(ch, '\n' | '\r')
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
