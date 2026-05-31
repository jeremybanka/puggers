#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexedLine {
  pub indent: usize,
  pub content: String,
}

pub fn lex(source: &str) -> Vec<LexedLine> {
  source
    .lines()
    .filter_map(|line| {
      if line.trim().is_empty() {
        return None;
      }

      let indent = line.chars().take_while(|ch| *ch == ' ').count();
      Some(LexedLine {
        indent,
        content: line.trim().to_string(),
      })
    })
    .collect()
}

