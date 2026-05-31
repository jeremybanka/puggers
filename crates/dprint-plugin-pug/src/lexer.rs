#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexedLine {
  pub indent: usize,
  pub content: String,
  pub is_blank: bool,
}

pub fn lex(source: &str) -> Vec<LexedLine> {
  source
    .lines()
    .map(|line| {
      let indent = line.chars().take_while(|ch| *ch == ' ').count();
      LexedLine {
        indent,
        content: line[indent..].to_string(),
        is_blank: line.trim().is_empty(),
      }
    })
    .collect()
}
