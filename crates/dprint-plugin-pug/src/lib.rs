#![cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    allow(dead_code)
)]

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
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let mut resolved = Configuration {
            indent_width: global_config.indent_width.map(|value| value as usize),
            line_width: global_config.line_width.map(|value| value as usize),
            quote_style: None,
            use_tabs: global_config.use_tabs,
        };

        if let Some(value) = config
            .get("indentWidth")
            .and_then(|value| value.as_number())
        {
            resolved.indent_width = Some(value as usize);
        }

        if let Some(value) = config.get("lineWidth").and_then(|value| value.as_number()) {
            resolved.line_width = Some(value as usize);
        }

        if let Some(value) = config.get("quoteStyle").and_then(|value| value.as_string()) {
            resolved.quote_style = match value.as_str() {
                "double" => Some(crate::ast::QuoteStyle::Double),
                "single" => Some(crate::ast::QuoteStyle::Single),
                _ => None,
            };
        }

        if let Some(value) = config.get("useTabs").and_then(|value| value.as_bool()) {
            resolved.use_tabs = Some(value);
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

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> Result<Vec<ConfigChange>, FormatError> {
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
