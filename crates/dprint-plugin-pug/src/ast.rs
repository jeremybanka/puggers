use crate::config;
pub use puggers_core::QuoteStyle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Statement(StatementNode),
    Comment(CommentNode),
    Text(TextLineNode),
    RawText(RawTextNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentNode {
    pub kind: CommentKind,
    pub value: Option<String>,
    pub children: Vec<RawTextNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    Buffered,
    Unbuffered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementNode {
    pub head: StatementHead,
    pub text_block_kind: Option<TextBlockKind>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBlockKind {
    Prose,
    Raw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextLineNode {
    pub kind: TextLineKind,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextLineKind {
    Piped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementHead {
    Tag(TagHead),
    Doctype(DoctypeHead),
    Code(CodeHead),
    ControlFlow(ControlFlowHead),
    Filter(FilterHead),
    Include(IncludeHead),
    Extends(ExtendsHead),
    Block(BlockHead),
    Mixin(MixinHead),
    MixinCall(MixinCallHead),
    Raw(String),
}

impl StatementHead {
    pub fn to_source(&self, config: &config::Configuration) -> String {
        match self {
            StatementHead::Tag(head) => head.to_source(config),
            StatementHead::Doctype(head) => head.to_source(),
            StatementHead::Code(head) => head.to_source(),
            StatementHead::ControlFlow(head) => head.to_source(),
            StatementHead::Filter(head) => head.to_source(),
            StatementHead::Include(head) => head.to_source(),
            StatementHead::Extends(head) => head.to_source(),
            StatementHead::Block(head) => head.to_source(),
            StatementHead::Mixin(head) => head.to_source(),
            StatementHead::MixinCall(head) => head.to_source(),
            StatementHead::Raw(content) => content.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterHead {
    pub name: String,
}

impl FilterHead {
    pub fn to_source(&self) -> String {
        format!(":{}", self.name)
    }
}

fn render_keyword_head(keyword: &str, suffix: &str) -> String {
    match normalize_structural_suffix(suffix) {
        Some(payload) if payload.starts_with(':') => format!("{keyword}{payload}"),
        Some(payload) => format!("{keyword} {payload}"),
        None => keyword.to_string(),
    }
}

fn render_operator_head(operator: &str, suffix: &str) -> String {
    match normalize_structural_suffix(suffix) {
        Some(payload) => format!("{operator} {payload}"),
        None => operator.to_string(),
    }
}

fn render_attached_head(prefix: &str, suffix: &str) -> String {
    match normalize_structural_suffix(suffix) {
        Some(payload) => format!("{prefix}{payload}"),
        None => prefix.to_string(),
    }
}

fn normalize_structural_suffix(suffix: &str) -> Option<String> {
    let trimmed = suffix.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut output = String::with_capacity(trimmed.len());
    let mut pending_space = false;
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut in_template_literal = false;
    let mut escaped = false;

    for ch in trimmed.chars() {
        if escaped {
            output.push(ch);
            escaped = false;
            continue;
        }

        if in_single_quote {
            output.push(ch);
            if ch == '\\' {
                escaped = true;
            } else if ch == '\'' {
                in_single_quote = false;
            }
            continue;
        }

        if in_double_quote {
            output.push(ch);
            if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double_quote = false;
            }
            continue;
        }

        if in_template_literal {
            output.push(ch);
            if ch == '\\' {
                escaped = true;
            } else if ch == '`' {
                in_template_literal = false;
            }
            continue;
        }

        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }

        if pending_space && !output.is_empty() {
            output.push(' ');
        }
        pending_space = false;
        output.push(ch);

        match ch {
            '\'' => in_single_quote = true,
            '"' => in_double_quote = true,
            '`' => in_template_literal = true,
            _ => {}
        }
    }

    Some(output)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeHead {
    pub suffix: String,
}

impl IncludeHead {
    pub fn to_source(&self) -> String {
        render_keyword_head("include", &self.suffix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendsHead {
    pub suffix: String,
}

impl ExtendsHead {
    pub fn to_source(&self) -> String {
        render_keyword_head("extends", &self.suffix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHead {
    pub mode: Option<BlockMode>,
    pub target: Option<String>,
    pub suffix: String,
}

impl BlockHead {
    pub fn to_source(&self) -> String {
        render_keyword_head("block", &self.suffix)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockMode {
    Append,
    Prepend,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixinHead {
    pub suffix: String,
}

impl MixinHead {
    pub fn to_source(&self) -> String {
        render_keyword_head("mixin", &self.suffix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixinCallHead {
    pub suffix: String,
}

impl MixinCallHead {
    pub fn to_source(&self) -> String {
        if self.suffix.contains('\n') {
            format!("+{}", self.suffix.trim_end())
        } else {
            render_attached_head("+", &self.suffix)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeHead {
    pub kind: CodeKind,
    pub suffix: String,
}

impl CodeHead {
    pub fn to_source(&self) -> String {
        render_operator_head(self.kind.operator(), &self.suffix)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeKind {
    Unbuffered,
    EscapedBuffered,
    UnescapedBuffered,
}

impl CodeKind {
    fn operator(self) -> &'static str {
        match self {
            CodeKind::Unbuffered => "-",
            CodeKind::EscapedBuffered => "=",
            CodeKind::UnescapedBuffered => "!=",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFlowHead {
    pub kind: ControlFlowKind,
    pub suffix: String,
}

impl ControlFlowHead {
    pub fn to_source(&self) -> String {
        render_keyword_head(self.kind.keyword(), &self.suffix)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlowKind {
    If,
    ElseIf,
    Else,
    Unless,
    Case,
    When,
    Default,
    Each,
    While,
}

impl ControlFlowKind {
    fn keyword(self) -> &'static str {
        match self {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: Option<AttributeValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    Quoted {
        value: String,
        quote_style: QuoteStyle,
    },
    Expression(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagHead {
    pub tag_name: Option<String>,
    pub shorthand_id: Option<String>,
    pub shorthand_classes: Vec<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub inline_space: Option<String>,
    pub inline_text: Option<InlineText>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineText {
    pub kind: InlineTextKind,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineTextKind {
    Plain,
    Interpolated,
    LiteralHtml,
}

impl TagHead {
    pub fn to_source(&self, config: &config::Configuration) -> String {
        let mut output = String::new();

        if let Some(tag_name) = &self.tag_name {
            output.push_str(tag_name);
        }

        if let Some(shorthand_id) = &self.shorthand_id {
            output.push('#');
            output.push_str(shorthand_id);
        }

        for shorthand_class in &self.shorthand_classes {
            output.push('.');
            output.push_str(shorthand_class);
        }

        if let Some(attributes) = &self.attributes {
            output.push('(');
            for (index, attribute) in attributes.iter().enumerate() {
                if index > 0 {
                    output.push(' ');
                }
                output.push_str(&attribute.to_source(config.quote_style()));
            }
            output.push(')');
        }

        if self.inline_space.is_some() && self.inline_text.is_some() {
            output.push(' ');
        }

        if let Some(inline_text) = &self.inline_text {
            output.push_str(&inline_text.content);
        }

        output
    }
}

impl Attribute {
    pub(crate) fn to_source(&self, quote_style: QuoteStyle) -> String {
        let mut output = self.name.clone();

        if let Some(value) = &self.value {
            output.push('=');
            output.push_str(&value.to_source(quote_style));
        }

        output
    }
}

impl AttributeValue {
    fn to_source(&self, quote_style: QuoteStyle) -> String {
        match self {
            AttributeValue::Quoted { value, .. } => {
                puggers_core::formatting::render_attribute_value(value, quote_style)
            }
            AttributeValue::Expression(value) => value.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctypeHead {
    pub spacing: Option<String>,
    pub value: Option<String>,
}

impl DoctypeHead {
    pub fn to_source(&self) -> String {
        match self.value.as_deref().and_then(normalize_structural_suffix) {
            Some(value) => format!("doctype {value}"),
            None => String::from("doctype"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawTextNode {
    pub preserve_base_indent: bool,
    pub extra_indent: usize,
    pub content: String,
}
