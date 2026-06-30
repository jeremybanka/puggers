pub use puggers_core::QuoteStyle;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub indent_width: Option<usize>,
    pub line_width: Option<usize>,
    pub quote_style: Option<QuoteStyle>,
    pub use_tabs: Option<bool>,
}

impl Configuration {
    pub fn indent_width(&self) -> usize {
        self.indent_width.unwrap_or(2)
    }

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style.unwrap_or(QuoteStyle::Double)
    }

    pub fn line_width(&self) -> Option<usize> {
        self.line_width
    }

    pub fn use_tabs(&self) -> bool {
        self.use_tabs.unwrap_or(false)
    }

    pub fn format_options(&self) -> puggers_core::PugFormatOptions {
        puggers_core::PugFormatOptions {
            indent_width: self.indent_width(),
            line_width: self.line_width(),
            use_tabs: self.use_tabs(),
            quote_style: self.quote_style(),
        }
    }
}
