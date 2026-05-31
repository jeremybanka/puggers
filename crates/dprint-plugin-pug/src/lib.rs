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
            use_tabs: global_config.use_tabs,
        };

        if let Some(value) = config
            .get("indentWidth")
            .and_then(|value| value.as_number())
        {
            resolved.indent_width = Some(value as usize);
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
    ) -> anyhow::Result<Vec<ConfigChange>> {
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
    use std::fs;
    use std::path::PathBuf;

    use super::{formatter, lexer, parser};
    use crate::ast::{Node, StatementHead};
    use crate::config::Configuration;

    #[test]
    fn formats_nested_tags_consistently() {
        let source = "div#app.main\n    p   hello world\n      span.label  neat\n";
        let lexed = lexer::lex(source);
        let document = parser::parse(&lexed);
        let formatted = formatter::format(&document, &Configuration::default());

        assert_eq!(
            formatted,
            "div#app.main\n  p   hello world\n    span.label  neat\n"
        );
    }

    #[test]
    fn parses_statement_heads_into_structured_ast() {
        let source = "doctype html\n#app.shell(data-mode=\"demo\")\np   hello world\n";
        let lexed = lexer::lex(source);
        let document = parser::parse(&lexed);

        assert!(matches!(
            &document.children[0],
            Node::Statement(statement)
                if matches!(
                    &statement.head,
                    StatementHead::Doctype(head)
                        if head.spacing.as_deref() == Some(" ")
                            && head.value.as_deref() == Some("html")
                )
        ));

        assert!(matches!(
            &document.children[1],
            Node::Statement(statement)
                if matches!(
                    &statement.head,
                    StatementHead::Tag(head)
                        if head.tag_name.is_none()
                            && head.shorthand_id.as_deref() == Some("app")
                            && head.shorthand_classes == vec![String::from("shell")]
                            && head.attributes.as_deref() == Some("data-mode=\"demo\"")
                )
        ));

        assert!(matches!(
            &document.children[2],
            Node::Statement(statement)
                if matches!(
                    &statement.head,
                    StatementHead::Tag(head)
                        if head.tag_name.as_deref() == Some("p")
                            && head.inline_space.as_deref() == Some("   ")
                            && head.inline_text.as_deref() == Some("hello world")
                )
        ));
    }

    #[test]
    fn preserves_raw_statement_heads_for_unsupported_syntax() {
        let source = "a: img\n- var title = \"hello\"\nfoo/\n";
        let lexed = lexer::lex(source);
        let document = parser::parse(&lexed);

        for node in &document.children {
            assert!(matches!(
                node,
                Node::Statement(statement) if matches!(&statement.head, StatementHead::Raw(_))
            ));
        }

        let formatted = formatter::format(&document, &Configuration::default());
        assert_eq!(formatted, source);
    }

    #[test]
    fn formats_with_tabs_when_requested() {
        let source = "div\n  span hello\n";
        let lexed = lexer::lex(source);
        let document = parser::parse(&lexed);
        let formatted = formatter::format(
            &document,
            &Configuration {
                use_tabs: Some(true),
                indent_width: Some(2),
            },
        );

        assert_eq!(formatted, "div\n\tspan hello\n");
    }

    #[test]
    fn preserves_comments_and_text_lines() {
        let source = "//note\n|  hello\n";
        let lexed = lexer::lex(source);
        let document = parser::parse(&lexed);
        let formatted = formatter::format(&document, &Configuration::default());

        assert_eq!(formatted, "// note\n|  hello\n");
    }

    #[test]
    fn preserves_attributes_and_text_blocks() {
        let source = "doctype html\nhtml(lang=\"en\")\n  body\n    textarea(data-x=\"1\").\n      line one\n        line two\n    a.link(href=\"/docs\") Docs\n";
        let lexed = lexer::lex(source);
        let document = parser::parse(&lexed);
        let formatted = formatter::format(&document, &Configuration::default());

        assert_eq!(
            formatted,
            "doctype html\nhtml(lang=\"en\")\n  body\n    textarea(data-x=\"1\").\n      line one\n        line two\n    a.link(href=\"/docs\") Docs\n"
        );
    }

    #[test]
    fn formats_generated_docs_idempotently() {
        let docs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/pug/2026-05-31");

        for entry in fs::read_dir(&docs_dir).expect("docs directory should exist") {
            let entry = entry.expect("directory entry should load");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("pug") {
                continue;
            }

            let source = fs::read_to_string(&path).expect("pug file should read");
            let lexed = lexer::lex(&source);
            let document = parser::parse(&lexed);
            let formatted = formatter::format(&document, &Configuration::default());

            assert_same_text(
                &formatted,
                &source,
                &format!("formatter changed {}", path.display()),
            );
        }
    }

    fn assert_same_text(actual: &str, expected: &str, context: &str) {
        if actual == expected {
            return;
        }

        let mismatch = actual
            .chars()
            .zip(expected.chars())
            .position(|(left, right)| left != right)
            .unwrap_or_else(|| actual.len().min(expected.len()));

        let actual_snippet: String = actual.chars().skip(mismatch).take(120).collect();
        let expected_snippet: String = expected.chars().skip(mismatch).take(120).collect();

        panic!(
            "{context}\nfirst mismatch at char {mismatch}\nactual:   {:?}\nexpected: {:?}",
            actual_snippet, expected_snippet
        );
    }
}
