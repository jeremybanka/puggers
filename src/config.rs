use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub indent_width: Option<usize>,
}

impl Configuration {
  pub fn indent_width(&self) -> usize {
    self.indent_width.unwrap_or(2)
  }
}

