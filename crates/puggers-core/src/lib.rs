use std::collections::BTreeSet;

use kuchikiki::traits::TendrilSink;
use kuchikiki::{NodeRef, parse_html};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertOptions {
    pub allowed_attributes: BTreeSet<String>,
    pub preserve_id_and_class_shorthand: bool,
    pub trim_outer_document: bool,
    pub collapse_single_nested: bool,
    pub text_whitespace: TextWhitespaceMode,
    pub keep_comments: bool,
    pub indent_width: usize,
    pub line_width: Option<usize>,
    pub use_tabs: bool,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            allowed_attributes: BTreeSet::new(),
            preserve_id_and_class_shorthand: true,
            trim_outer_document: false,
            collapse_single_nested: false,
            text_whitespace: TextWhitespaceMode::default(),
            keep_comments: true,
            indent_width: 2,
            line_width: None,
            use_tabs: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextWhitespaceMode {
    #[default]
    Collapse,
    Preserve,
}

pub fn convert_html_to_pug(input: &str, options: &ConvertOptions) -> String {
    let document = parse_html().one(input);
    let mut nodes = if options.trim_outer_document {
        root_nodes_from_body(&document, options)
    } else {
        nodes_from_children(&document, options)
    };

    if options.collapse_single_nested {
        nodes = nodes.into_iter().map(collapse_single_nested).collect();
    }

    let rendered = render_nodes(&nodes, 0, options);
    if rendered.is_empty() {
        String::new()
    } else {
        format!("{rendered}\n")
    }
}

fn root_nodes_from_body(document: &NodeRef, options: &ConvertOptions) -> Vec<Node> {
    document
        .select_first("body")
        .ok()
        .map(|body| nodes_from_children(body.as_node(), options))
        .filter(|children| !children.is_empty())
        .unwrap_or_else(|| nodes_from_children(document, options))
}

fn nodes_from_children(node: &NodeRef, options: &ConvertOptions) -> Vec<Node> {
    let children: Vec<_> = node.children().collect();

    children
        .iter()
        .enumerate()
        .filter_map(|(index, child)| {
            node_from_dom(
                child,
                options,
                text_boundary_context(&children, index, options),
            )
        })
        .collect()
}

fn node_from_dom(
    node: &NodeRef,
    options: &ConvertOptions,
    text_context: TextBoundaryContext,
) -> Option<Node> {
    if let Some(doctype) = node.as_doctype() {
        return Some(Node::Doctype(doctype.name.to_string()));
    }

    if let Some(comment) = node.as_comment() {
        if !options.keep_comments {
            return None;
        }

        let value = comment.borrow();
        return Some(Node::Comment(comment_from_html(&value)));
    }

    if let Some(text) = node.as_text() {
        let value = text.borrow();
        return normalize_text(&value, options.text_whitespace, text_context).map(Node::Text);
    }

    if let Some(element) = node.as_element() {
        let tag = element.name.local.to_string();
        let attributes = sanitize_attributes(&element.attributes.borrow(), options);
        let raw_text = if is_raw_text_tag(&tag) {
            collect_raw_text(node)
        } else {
            None
        };
        let children = if raw_text.is_some() {
            Vec::new()
        } else {
            nodes_from_children(node, options)
        };

        return Some(Node::Element(ElementNode {
            tag,
            attributes,
            raw_text,
            children,
        }));
    }

    None
}

fn sanitize_attributes(
    attributes: &kuchikiki::Attributes,
    options: &ConvertOptions,
) -> Vec<Attribute> {
    let mut filtered = Vec::new();

    for (name, attribute) in &attributes.map {
        let local_name = name.local.to_string();
        if !options.allowed_attributes.contains(&local_name) {
            continue;
        }

        let value = attribute.value.trim().to_string();
        let rendered_value = if value.is_empty() || value.eq_ignore_ascii_case(&local_name) {
            None
        } else {
            Some(value)
        };

        filtered.push(Attribute {
            name: local_name,
            value: rendered_value,
        });
    }

    filtered
}

#[derive(Clone, Copy, Default)]
struct TextBoundaryContext {
    has_previous_signal: bool,
    has_next_signal: bool,
}

fn text_boundary_context(
    siblings: &[NodeRef],
    index: usize,
    options: &ConvertOptions,
) -> TextBoundaryContext {
    TextBoundaryContext {
        has_previous_signal: siblings[..index]
            .iter()
            .rev()
            .any(|sibling| dom_node_is_signal(sibling, options)),
        has_next_signal: siblings[index + 1..]
            .iter()
            .any(|sibling| dom_node_is_signal(sibling, options)),
    }
}

fn dom_node_is_signal(node: &NodeRef, options: &ConvertOptions) -> bool {
    if node.as_doctype().is_some() || node.as_element().is_some() {
        return true;
    }

    if node.as_comment().is_some() {
        return options.keep_comments;
    }

    node.as_text()
        .is_some_and(|text| !text.borrow().trim().is_empty())
}

fn normalize_text(
    input: &str,
    mode: TextWhitespaceMode,
    context: TextBoundaryContext,
) -> Option<TextNode> {
    let collapsed = input.split_whitespace().collect::<Vec<_>>().join(" ");
    let prose_paragraphs = if !collapsed.is_empty() {
        vec![collapsed.clone()]
    } else {
        Vec::new()
    };

    match mode {
        TextWhitespaceMode::Collapse => (!collapsed.is_empty()).then_some(TextNode {
            content: collapsed,
            prose_paragraphs,
        }),
        TextWhitespaceMode::Preserve => {
            normalize_preserved_text(input, collapsed, context).map(|content| TextNode {
                content,
                prose_paragraphs: Vec::new(),
            })
        }
    }
}

fn normalize_preserved_text(
    input: &str,
    collapsed: String,
    context: TextBoundaryContext,
) -> Option<String> {
    if collapsed.is_empty() {
        return (context.has_previous_signal
            && context.has_next_signal
            && input.chars().any(char::is_whitespace))
        .then(|| String::from(" "));
    }

    let mut normalized = String::new();
    if context.has_previous_signal && starts_with_whitespace(input) {
        normalized.push(' ');
    }
    normalized.push_str(&collapsed);
    if context.has_next_signal && ends_with_whitespace(input) {
        normalized.push(' ');
    }

    Some(normalized)
}

fn starts_with_whitespace(input: &str) -> bool {
    input.chars().next().is_some_and(char::is_whitespace)
}

fn ends_with_whitespace(input: &str) -> bool {
    input.chars().next_back().is_some_and(char::is_whitespace)
}

fn collect_raw_text(node: &NodeRef) -> Option<String> {
    let mut text = String::new();

    for child in node.children() {
        if let Some(value) = child.as_text() {
            text.push_str(&value.borrow());
        }
    }

    let trimmed = text.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn comment_from_html(value: &str) -> CommentNode {
    if value.is_empty() {
        return CommentNode {
            inline_value: None,
            block_lines: Vec::new(),
        };
    }

    if !value.contains('\n') && value == value.trim() {
        return CommentNode {
            inline_value: Some(value.to_string()),
            block_lines: Vec::new(),
        };
    }

    CommentNode {
        inline_value: None,
        block_lines: value.split('\n').map(str::to_string).collect(),
    }
}

fn collapse_single_nested(node: Node) -> Node {
    match node {
        Node::Element(mut element) => {
            element.children = element
                .children
                .into_iter()
                .map(collapse_single_nested)
                .collect();

            while element.tag == "div"
                && element.attributes.is_empty()
                && element.children.len() == 1
                && matches!(&element.children[0], Node::Element(_))
            {
                match element.children.remove(0) {
                    Node::Element(child) => element = child,
                    other => return other,
                }
            }

            Node::Element(element)
        }
        other => other,
    }
}

fn render_nodes(nodes: &[Node], depth: usize, options: &ConvertOptions) -> String {
    nodes
        .iter()
        .map(|node| render_node(node, depth, options))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_node(node: &Node, depth: usize, options: &ConvertOptions) -> String {
    match node {
        Node::Doctype(name) if name.eq_ignore_ascii_case("html") => {
            format!("{}doctype html", indent(depth, options))
        }
        Node::Doctype(name) => format!("{}doctype {}", indent(depth, options), name.trim()),
        Node::Comment(comment) => render_comment(comment, depth, options),
        Node::Text(text) => format!("{}| {}", indent(depth, options), text.content),
        Node::Element(element) => render_element(element, depth, options),
    }
}

fn render_comment(comment: &CommentNode, depth: usize, options: &ConvertOptions) -> String {
    let mut output = format!("{}//", indent(depth, options));

    if let Some(value) = &comment.inline_value {
        output.push(' ');
        output.push_str(value);
        return output;
    }

    for line in &comment.block_lines {
        output.push('\n');
        output.push_str(&indent(depth + 1, options));
        output.push_str(line);
    }

    output
}

fn render_element(element: &ElementNode, depth: usize, options: &ConvertOptions) -> String {
    let mut line = format!("{}{}", indent(depth, options), element.tag);
    let mut trailing_attributes = Vec::new();

    for attribute in &element.attributes {
        if options.preserve_id_and_class_shorthand
            && attribute.name == "id"
            && let Some(value) = &attribute.value
            && is_shorthand_compatible(value)
        {
            line.push('#');
            line.push_str(value);
            continue;
        }

        if options.preserve_id_and_class_shorthand
            && attribute.name == "class"
            && let Some(value) = &attribute.value
        {
            let classes: Vec<_> = value.split_whitespace().collect();
            if !classes.is_empty()
                && classes
                    .iter()
                    .all(|class_name| is_shorthand_compatible(class_name))
            {
                for class_name in classes {
                    line.push('.');
                    line.push_str(class_name);
                }
                continue;
            }
        }

        trailing_attributes.push(render_attribute(attribute));
    }

    if !trailing_attributes.is_empty() {
        line.push('(');
        line.push_str(&trailing_attributes.join(", "));
        line.push(')');
    }

    if let Some(raw_text) = &element.raw_text {
        let mut output = line;
        output.push('.');
        for raw_line in raw_text.lines() {
            output.push('\n');
            output.push_str(&indent(depth + 1, options));
            output.push_str(raw_line);
        }
        return output;
    }

    if let [Node::Text(text)] = element.children.as_slice() {
        if should_render_prose_block(&line, text, depth, options) {
            return render_prose_block(line, text, depth, options);
        } else if should_wrap_inline_text(&line, text, depth, options) {
            let mut output = line;
            output.push('\n');
            output.push_str(&render_node(&Node::Text(text.clone()), depth + 1, options));
            return output;
        } else {
            line.push(' ');
            line.push_str(&text.content);
            return line;
        }
    }

    if element.children.is_empty() {
        return line;
    }

    let mut output = line;
    for child in &element.children {
        output.push('\n');
        output.push_str(&render_node(child, depth + 1, options));
    }
    output
}

fn should_wrap_inline_text(
    line_prefix: &str,
    text: &TextNode,
    depth: usize,
    options: &ConvertOptions,
) -> bool {
    let Some(line_width) = options.line_width else {
        return false;
    };

    display_width(depth, options) + line_prefix.trim_start().len() + 1 + text.content.len()
        > line_width
}

fn should_render_prose_block(
    line_prefix: &str,
    text: &TextNode,
    depth: usize,
    options: &ConvertOptions,
) -> bool {
    if options.line_width.is_none() || text.prose_paragraphs.is_empty() {
        return false;
    }

    if text.prose_paragraphs.len() > 1 {
        return true;
    }

    text.content.contains(' ') && should_wrap_inline_text(line_prefix, text, depth, options)
}

fn render_prose_block(
    mut line: String,
    text: &TextNode,
    depth: usize,
    options: &ConvertOptions,
) -> String {
    line.push('.');

    let available_width = options
        .line_width
        .and_then(|line_width| line_width.checked_sub(display_width(depth + 1, options)));

    for (index, paragraph) in text.prose_paragraphs.iter().enumerate() {
        if index > 0 {
            line.push('\n');
        }

        let wrapped_lines = wrap_prose_paragraph(paragraph, available_width);
        for wrapped_line in wrapped_lines {
            line.push('\n');
            line.push_str(&indent(depth + 1, options));
            line.push_str(&wrapped_line);
        }
    }

    line
}

fn wrap_prose_paragraph(paragraph: &str, available_width: Option<usize>) -> Vec<String> {
    let Some(available_width) = available_width else {
        return vec![paragraph.to_string()];
    };

    if paragraph.len() <= available_width {
        return vec![paragraph.to_string()];
    }

    wrap_words(
        &paragraph
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
        available_width,
    )
}

fn wrap_words(words: &[String], available_width: usize) -> Vec<String> {
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
            current = word.clone();
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

fn render_attribute(attribute: &Attribute) -> String {
    match &attribute.value {
        Some(value) => format!("{}=\"{}\"", attribute.name, escape_attr_value(value)),
        None => attribute.name.clone(),
    }
}

fn escape_attr_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn indent(depth: usize, options: &ConvertOptions) -> String {
    if options.use_tabs {
        "\t".repeat(depth)
    } else {
        " ".repeat(depth * options.indent_width)
    }
}

fn display_width(depth: usize, options: &ConvertOptions) -> usize {
    if options.use_tabs {
        depth
    } else {
        depth * options.indent_width
    }
}

fn is_shorthand_compatible(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
}

fn is_raw_text_tag(tag: &str) -> bool {
    matches!(tag, "pre" | "script" | "style" | "textarea")
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Doctype(String),
    Comment(CommentNode),
    Text(TextNode),
    Element(ElementNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextNode {
    content: String,
    prose_paragraphs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommentNode {
    inline_value: Option<String>,
    block_lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ElementNode {
    tag: String,
    attributes: Vec<Attribute>,
    raw_text: Option<String>,
    children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Attribute {
    name: String,
    value: Option<String>,
}
