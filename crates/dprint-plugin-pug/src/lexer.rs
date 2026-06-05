#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexedLine {
    pub indent: usize,
    pub content: String,
    pub is_blank: bool,
}

pub fn lex(source: &str) -> Vec<LexedLine> {
    let mut lines = source.split('\n').collect::<Vec<_>>();
    if source.ends_with('\n') {
        lines.pop();
    }

    lines
        .into_iter()
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
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
