#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Statement(StatementNode),
    Comment(String),
    Text(String),
    RawText(RawTextNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementNode {
    pub head: StatementHead,
    pub is_text_block: bool,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementHead {
    Tag(TagHead),
    Doctype(DoctypeHead),
    Raw(String),
}

impl StatementHead {
    pub fn to_source(&self) -> String {
        match self {
            StatementHead::Tag(head) => head.to_source(),
            StatementHead::Doctype(head) => head.to_source(),
            StatementHead::Raw(content) => content.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagHead {
    pub tag_name: Option<String>,
    pub shorthand_id: Option<String>,
    pub shorthand_classes: Vec<String>,
    pub attributes: Option<String>,
    pub inline_space: Option<String>,
    pub inline_text: Option<String>,
}

impl TagHead {
    pub fn to_source(&self) -> String {
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
            output.push_str(attributes);
            output.push(')');
        }

        if let (Some(inline_space), Some(inline_text)) = (&self.inline_space, &self.inline_text) {
            output.push_str(inline_space);
            output.push_str(inline_text);
        }

        output
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
