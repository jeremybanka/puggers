mod support;

pub use support::{ast, config, formatter, lexer, parser};

use config::Configuration;
use support::{assert_same_text, pug_doc_sources};

#[test]
fn formats_generated_docs_idempotently() {
    for (path, source) in pug_doc_sources() {
        let formatted = support::format_source(&source, &Configuration::default());

        assert_same_text(
            &formatted,
            &source,
            &format!("formatter changed {}", path.display()),
        );
    }
}
