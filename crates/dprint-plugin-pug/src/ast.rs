use crate::config;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Statement(StatementNode),
    Comment(String),
    Text(TextLineNode),
    RawText(RawTextNode),
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
    Raw(String),
}

impl StatementHead {
    pub fn to_source(&self, config: &config::Configuration) -> String {
        match self {
            StatementHead::Tag(head) => head.to_source(config),
            StatementHead::Doctype(head) => head.to_source(),
            StatementHead::Code(head) => head.to_source(),
            StatementHead::ControlFlow(head) => head.to_source(),
            StatementHead::Raw(content) => content.clone(),
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
        format!("{}{}", self.kind.operator(), self.suffix)
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
        format!("{}{}", self.kind.keyword(), self.suffix)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlowKind {
    If,
    ElseIf,
    Else,
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
            ControlFlowKind::Case => "case",
            ControlFlowKind::When => "when",
            ControlFlowKind::Default => "default",
            ControlFlowKind::Each => "each",
            ControlFlowKind::While => "while",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuoteStyle {
    Double,
    Single,
}

impl QuoteStyle {
    fn delimiter(self) -> char {
        match self {
            QuoteStyle::Double => '"',
            QuoteStyle::Single => '\'',
        }
    }

    fn escape_quoted_value(self, value: &str) -> String {
        let delimiter = self.delimiter();
        let mut escaped = String::new();
        let mut chars = value.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\'
                && let Some(next) = chars.peek().copied()
                && next == delimiter
            {
                escaped.push(ch);
                escaped.push(next);
                chars.next();
                continue;
            }

            if ch == delimiter {
                escaped.push('\\');
            }

            escaped.push(ch);
        }

        escaped
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
                    output.push_str(", ");
                }
                output.push_str(&attribute.to_source(config.quote_style()));
            }
            output.push(')');
        }

        if let (Some(inline_space), Some(inline_text)) = (&self.inline_space, &self.inline_text) {
            output.push_str(inline_space);
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
                let delimiter = quote_style.delimiter();
                let escaped = quote_style.escape_quoted_value(value);
                format!("{delimiter}{escaped}{delimiter}")
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
        let mut output = String::from("doctype");

        if let Some(spacing) = &self.spacing {
            output.push_str(spacing);
        }

        if let Some(value) = &self.value {
            output.push_str(value);
        }

        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawTextNode {
    pub extra_indent: usize,
    pub content: String,
}
