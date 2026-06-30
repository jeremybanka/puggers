#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuoteStyle {
    #[default]
    Double,
    Single,
}

impl QuoteStyle {
    pub fn delimiter(self) -> char {
        match self {
            QuoteStyle::Double => '"',
            QuoteStyle::Single => '\'',
        }
    }

    pub fn escape_quoted_value(self, value: &str) -> String {
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

    pub fn render_quoted_value(self, value: &str) -> String {
        let delimiter = self.delimiter();
        let escaped = self.escape_quoted_value(value);
        format!("{delimiter}{escaped}{delimiter}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PugFormatOptions {
    pub indent_width: usize,
    pub line_width: Option<usize>,
    pub use_tabs: bool,
    pub quote_style: QuoteStyle,
}

impl Default for PugFormatOptions {
    fn default() -> Self {
        Self {
            indent_width: 2,
            line_width: None,
            use_tabs: false,
            quote_style: QuoteStyle::default(),
        }
    }
}

pub fn indent(depth: usize, options: &PugFormatOptions) -> String {
    if options.use_tabs {
        "\t".repeat(depth)
    } else {
        " ".repeat(depth * options.indent_width)
    }
}

pub fn write_indent(output: &mut String, depth: usize, options: &PugFormatOptions) {
    output.push_str(&indent(depth, options));
}

pub fn display_width(depth: usize, options: &PugFormatOptions) -> usize {
    if options.use_tabs {
        depth
    } else {
        depth * options.indent_width
    }
}

pub fn wrap_words<'a>(
    words: impl IntoIterator<Item = &'a str>,
    available_width: usize,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in words {
        let next_len = if current.is_empty() {
            word.len()
        } else {
            current.len() + 1 + word.len()
        };

        if !current.is_empty() && next_len > available_width {
            lines.push(current);
            current = word.to_string();
            continue;
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

pub fn render_attribute_value(value: &str, quote_style: QuoteStyle) -> String {
    quote_style.render_quoted_value(value)
}
