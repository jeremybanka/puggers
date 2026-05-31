#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
  pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
  Element(ElementNode),
  Comment(String),
  Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementNode {
  pub tag: String,
  pub id: Option<String>,
  pub classes: Vec<String>,
  pub text: Option<String>,
  pub children: Vec<Node>,
}

