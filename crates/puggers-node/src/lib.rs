use std::collections::BTreeSet;

use napi::{Error, Result, Status};
use napi_derive::napi;
use puggers_core::{
    CollapseSingleNestedMode, ConvertOptions, PugFormatOptions, QuoteStyle, RootSelection,
    TextWhitespaceMode, convert_html_to_pug,
};
use serde::Deserialize;

#[napi(js_name = "convertHtmlToPugNative")]
pub fn convert_html_to_pug_native(input: String, options_json: Option<String>) -> Result<String> {
    convert_html_to_pug_from_json(&input, options_json.as_deref()).map_err(to_napi_error)
}

fn convert_html_to_pug_from_json(input: &str, options_json: Option<&str>) -> Result<String> {
    let options = match options_json {
        Some(json) => {
            let js_options = serde_json::from_str::<JsConvertOptions>(json).map_err(|error| {
                Error::new(
                    Status::InvalidArg,
                    format!("invalid convertHtmlToPug options: {error}"),
                )
            })?;
            js_options.into_convert_options()?
        }
        None => ConvertOptions::default(),
    };

    convert_html_to_pug(input, &options)
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))
}

fn to_napi_error(error: Error) -> Error {
    error
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct JsConvertOptions {
    #[serde(default)]
    allowed_attributes: Option<Vec<String>>,
    #[serde(default)]
    preserve_id_and_class_shorthand: Option<bool>,
    #[serde(default)]
    root: Option<String>,
    #[serde(default)]
    collapse_single_nested: Option<JsCollapseSingleNestedMode>,
    #[serde(default)]
    text_whitespace: Option<JsTextWhitespaceMode>,
    #[serde(default)]
    keep_comments: Option<bool>,
    #[serde(default)]
    indent_width: Option<usize>,
    #[serde(default)]
    line_width: Option<usize>,
    #[serde(default)]
    use_tabs: Option<bool>,
    #[serde(default)]
    quote_style: Option<JsQuoteStyle>,
}

impl JsConvertOptions {
    fn into_convert_options(self) -> Result<ConvertOptions> {
        let mut options = ConvertOptions::default();

        if let Some(allowed_attributes) = self.allowed_attributes {
            options.allowed_attributes = allowed_attributes.into_iter().collect::<BTreeSet<_>>();
        }

        if let Some(preserve_id_and_class_shorthand) = self.preserve_id_and_class_shorthand {
            options.preserve_id_and_class_shorthand = preserve_id_and_class_shorthand;
        }

        if let Some(root) = self.root {
            options.root = Some(RootSelection::parse(&root).map_err(|error| {
                Error::new(
                    Status::InvalidArg,
                    format!("invalid convertHtmlToPug root option: {error}"),
                )
            })?);
        }

        if let Some(collapse_single_nested) = self.collapse_single_nested {
            options.collapse_single_nested = collapse_single_nested.into();
        }

        if let Some(text_whitespace) = self.text_whitespace {
            options.text_whitespace = text_whitespace.into();
        }

        if let Some(keep_comments) = self.keep_comments {
            options.keep_comments = keep_comments;
        }

        options.formatting = PugFormatOptions {
            indent_width: self.indent_width.unwrap_or(options.formatting.indent_width),
            line_width: self.line_width,
            use_tabs: self.use_tabs.unwrap_or(options.formatting.use_tabs),
            quote_style: self
                .quote_style
                .map(Into::into)
                .unwrap_or(options.formatting.quote_style),
        };

        Ok(options)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum JsCollapseSingleNestedMode {
    Off,
    TopWins,
    BottomWins,
    BestTagWins,
}

impl From<JsCollapseSingleNestedMode> for CollapseSingleNestedMode {
    fn from(value: JsCollapseSingleNestedMode) -> Self {
        match value {
            JsCollapseSingleNestedMode::Off => Self::Off,
            JsCollapseSingleNestedMode::TopWins => Self::TopWins,
            JsCollapseSingleNestedMode::BottomWins => Self::BottomWins,
            JsCollapseSingleNestedMode::BestTagWins => Self::BestTagWins,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum JsTextWhitespaceMode {
    Collapse,
    Preserve,
}

impl From<JsTextWhitespaceMode> for TextWhitespaceMode {
    fn from(value: JsTextWhitespaceMode) -> Self {
        match value {
            JsTextWhitespaceMode::Collapse => Self::Collapse,
            JsTextWhitespaceMode::Preserve => Self::Preserve,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum JsQuoteStyle {
    Double,
    Single,
}

impl From<JsQuoteStyle> for QuoteStyle {
    fn from(value: JsQuoteStyle) -> Self {
        match value {
            JsQuoteStyle::Double => Self::Double,
            JsQuoteStyle::Single => Self::Single,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_with_default_options() {
        let output = convert_html_to_pug_from_json("<main><h1>Hello</h1></main>", None)
            .expect("conversion should succeed");

        assert_eq!(output, "html\n  head\n  body\n    main\n      h1 Hello\n");
    }

    #[test]
    fn maps_javascript_options_to_core_options() {
        let output = convert_html_to_pug_from_json(
            r#"<main><a class="button" href="/docs">Docs</a></main>"#,
            Some(
                r#"{
                    "root": "main",
                    "allowedAttributes": ["class", "href"],
                    "quoteStyle": "single",
                    "indentWidth": 4
                }"#,
            ),
        )
        .expect("conversion should succeed");

        assert_eq!(output, "main\n    a.button(href='/docs') Docs\n");
    }

    #[test]
    fn reports_invalid_root_options() {
        let error = convert_html_to_pug_from_json("<main></main>", Some(r#"{"root": ">"}"#))
            .expect_err("invalid root should fail");

        assert!(error.to_string().contains("invalid root selection"));
    }
}
