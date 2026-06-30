use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

use puggers_core::{
    CollapseSingleNestedMode, ConvertOptions, PugFormatOptions, QuoteStyle, TextWhitespaceMode,
    convert_html_to_pug,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mut allowed_attributes = BTreeSet::new();
    let mut trim_outer_document = false;
    let mut collapse_single_nested = CollapseSingleNestedMode::Off;
    let mut text_whitespace = TextWhitespaceMode::Collapse;
    let mut keep_comments = true;
    let mut indent_width = 2;
    let mut line_width = None;
    let mut use_tabs = false;
    let mut quote_style = QuoteStyle::Double;
    let mut input_path = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--allow-attr" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value after --allow-attr"))?;
                allowed_attributes.insert(value);
            }
            "--trim-outer-document" => trim_outer_document = true,
            "--collapse-single-nested" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value after --collapse-single-nested"))?;
                collapse_single_nested = parse_collapse_single_nested_mode(&value)?;
            }
            "--preserve-text-whitespace" => text_whitespace = TextWhitespaceMode::Preserve,
            "--drop-comments" => keep_comments = false,
            "--use-tabs" => use_tabs = true,
            "--quote-style" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value after --quote-style"))?;
                quote_style = match value.as_str() {
                    "double" => QuoteStyle::Double,
                    "single" => QuoteStyle::Single,
                    _ => {
                        return Err(format!(
                            "invalid --quote-style value {value}: expected double or single"
                        ));
                    }
                };
            }
            "--indent-width" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value after --indent-width"))?;
                indent_width = value
                    .parse::<usize>()
                    .map_err(|error| format!("invalid --indent-width value {value}: {error}"))?;
            }
            "--line-width" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value after --line-width"))?;
                line_width = Some(
                    value
                        .parse::<usize>()
                        .map_err(|error| format!("invalid --line-width value {value}: {error}"))?,
                );
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ if argument.starts_with('-') => {
                return Err(format!("unknown flag: {argument}"));
            }
            _ => {
                if input_path.is_some() {
                    return Err(String::from("only one input path is supported"));
                }
                input_path = Some(argument);
            }
        }
    }

    let input = if let Some(path) = input_path {
        fs::read_to_string(&path).map_err(|error| format!("failed to read {path}: {error}"))?
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|error| format!("failed to read stdin: {error}"))?;
        buffer
    };

    let output = convert_html_to_pug(
        &input,
        &ConvertOptions {
            allowed_attributes,
            trim_outer_document,
            collapse_single_nested,
            text_whitespace,
            keep_comments,
            formatting: PugFormatOptions {
                indent_width,
                line_width,
                use_tabs,
                quote_style,
            },
            ..Default::default()
        },
    );

    print!("{output}");
    Ok(())
}

fn print_help() {
    println!(
        "Usage: puggers [options] [path]\n\
     \n\
     If no path is provided, HTML is read from stdin.\n\
     \n\
     Options:\n\
       --allow-attr <name>          Keep an attribute during conversion\n\
       --indent-width <count>       Set the indentation width for space mode\n\
       --line-width <count>         Reflow prose and wrap long inline tag text\n\
       --quote-style <style>        Render attributes with double or single quotes\n\
       --trim-outer-document        Emit body children instead of html/head/body\n\
       --collapse-single-nested <mode>\n\
                                    Collapse single-child structural chains with off,\n\
                                    top-wins, bottom-wins, or best-tag-wins\n\
       --preserve-text-whitespace   Keep meaningful spaces around inline content\n\
       --drop-comments              Remove HTML comments\n\
       --use-tabs                   Indent with tabs instead of spaces\n\
       -h, --help                   Show this help text"
    );
}

fn parse_collapse_single_nested_mode(value: &str) -> Result<CollapseSingleNestedMode, String> {
    match value {
        "off" => Ok(CollapseSingleNestedMode::Off),
        "top-wins" => Ok(CollapseSingleNestedMode::TopWins),
        "bottom-wins" => Ok(CollapseSingleNestedMode::BottomWins),
        "best-tag-wins" => Ok(CollapseSingleNestedMode::BestTagWins),
        _ => Err(format!(
            "invalid --collapse-single-nested value {value}: expected off, top-wins, bottom-wins, or best-tag-wins"
        )),
    }
}
