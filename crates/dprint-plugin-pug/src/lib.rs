#![cfg_attr(not(all(target_arch = "wasm32", target_os = "unknown")), allow(dead_code))]

mod ast;
mod config;
mod formatter;
mod lexer;
mod parser;

use config::Configuration;
use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::{
  CheckConfigUpdatesMessage, ConfigChange, FileMatchingInfo, PluginInfo,
  PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

struct PugPluginHandler;

impl SyncPluginHandler<Configuration> for PugPluginHandler {
  fn plugin_info(&mut self) -> PluginInfo {
    PluginInfo {
      name: String::from("pug"),
      version: env!("CARGO_PKG_VERSION").to_string(),
      config_key: String::from("pug"),
      help_url: String::from("https://pugjs.org/"),
      config_schema_url: String::new(),
      update_url: None,
    }
  }

  fn resolve_config(
    &mut self,
    config: ConfigKeyMap,
    _global_config: &GlobalConfiguration,
  ) -> PluginResolveConfigurationResult<Configuration> {
    let mut resolved = Configuration::default();

    if let Some(value) = config.get("indentWidth").and_then(|value| value.as_number()) {
      resolved.indent_width = Some(value as usize);
    }

    PluginResolveConfigurationResult {
      config: resolved,
      diagnostics: Vec::new(),
      file_matching: FileMatchingInfo {
        file_extensions: vec![String::from("pug")],
        file_names: Vec::new(),
      },
    }
  }

  fn license_text(&mut self) -> String {
    String::from("MIT")
  }

  fn check_config_updates(&self, _message: CheckConfigUpdatesMessage) -> anyhow::Result<Vec<ConfigChange>> {
    Ok(Vec::new())
  }

  fn format(
    &mut self,
    request: SyncFormatRequest<Configuration>,
    _format_with_host: impl FnMut(SyncHostFormatRequest) -> dprint_core::plugins::FormatResult,
  ) -> dprint_core::plugins::FormatResult {
    let file_text = String::from_utf8(request.file_bytes)?;
    let lexed = lexer::lex(&file_text);
    let document = parser::parse(&lexed);
    let formatted = formatter::format(&document, request.config);

    if formatted == file_text {
      Ok(None)
    } else {
      Ok(Some(formatted.into_bytes()))
    }
  }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
generate_plugin_code!(PugPluginHandler, PugPluginHandler);

#[cfg(test)]
mod tests {
  use super::{formatter, lexer, parser};
  use crate::config::Configuration;

  #[test]
  fn formats_nested_tags_consistently() {
    let source = "div#app.main\n    p   hello world\n      span.label  neat\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);
    let formatted = formatter::format(&document, &Configuration::default());

    assert_eq!(formatted, "div#app.main\n  p hello world\n    span.label neat\n");
  }

  #[test]
  fn preserves_comments_and_text_lines() {
    let source = "//note\n|  hello\n";
    let lexed = lexer::lex(source);
    let document = parser::parse(&lexed);
    let formatted = formatter::format(&document, &Configuration::default());

    assert_eq!(formatted, "// note\nhello\n");
  }
}
