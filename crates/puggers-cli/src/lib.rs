use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, Read, Write};

use puggers_core::{
    CollapseSingleNestedMode, ConvertOptions, PugFormatOptions, QuoteStyle, RootSelection,
    TextWhitespaceMode, convert_html_to_pug,
};

pub fn run_from_env() -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    run(env::args().skip(1), stdin.lock(), stdout.lock())
}

pub fn run<I, S, R, W>(arguments: I, mut stdin: R, mut stdout: W) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
    R: Read,
    W: Write,
{
    let mut args = arguments.into_iter().map(Into::into);
    let mut allowed_attributes = BTreeSet::new();
    let mut root = None;
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
            "--root" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value after --root"))?;
                root = Some(RootSelection::parse(&value).map_err(|error| error.to_string())?);
            }
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
                print_help(&mut stdout)?;
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
        stdin
            .read_to_string(&mut buffer)
            .map_err(|error| format!("failed to read stdin: {error}"))?;
        buffer
    };

    let output = convert_html_to_pug(
        &input,
        &ConvertOptions {
            allowed_attributes,
            root,
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
    )
    .map_err(|error| error.to_string())?;

    write!(stdout, "{output}").map_err(|error| format!("failed to write stdout: {error}"))?;
    Ok(())
}

fn print_help(mut output: impl Write) -> Result<(), String> {
    writeln!(
        output,
        "Usage: puggers [options] [path]\n\
     \n\
     If no path is provided, HTML is read from stdin.\n\
     \n\
     Options:\n\
       --allow-attr <name>          Keep an attribute during conversion\n\
       --indent-width <count>       Set the indentation width for space mode\n\
       --line-width <count>         Reflow prose and wrap long inline tag text\n\
       --quote-style <style>        Render attributes with double or single quotes\n\
       --root <path>                Emit the first root matching a path like html>body article\n\
       --collapse-single-nested <mode>\n\
                                    Collapse single-child structural chains with off,\n\
                                    top-wins, bottom-wins, or best-tag-wins\n\
       --preserve-text-whitespace   Keep meaningful spaces around inline content\n\
       --drop-comments              Remove HTML comments\n\
       --use-tabs                   Indent with tabs instead of spaces\n\
       -h, --help                   Show this help text"
    )
    .map_err(|error| format!("failed to write help: {error}"))
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
