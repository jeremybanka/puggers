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
  pub content: String,
  pub is_text_block: bool,
  pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawTextNode {
  pub extra_indent: usize,
  pub content: String,
}
