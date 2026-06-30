use std::collections::BTreeSet;
use std::fmt;

pub mod formatting;

use kuchikiki::traits::TendrilSink;
use kuchikiki::{NodeRef, parse_html};

pub use formatting::{PugFormatOptions, QuoteStyle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertOptions {
    pub allowed_attributes: BTreeSet<String>,
    pub preserve_id_and_class_shorthand: bool,
    pub root: Option<RootSelection>,
    pub collapse_single_nested: CollapseSingleNestedMode,
    pub text_whitespace: TextWhitespaceMode,
    pub keep_comments: bool,
    pub formatting: PugFormatOptions,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            allowed_attributes: BTreeSet::new(),
            preserve_id_and_class_shorthand: true,
            root: None,
            collapse_single_nested: CollapseSingleNestedMode::default(),
            text_whitespace: TextWhitespaceMode::default(),
            keep_comments: true,
            formatting: PugFormatOptions::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootSelection {
    source: String,
    steps: Vec<RootSelectionStep>,
}

impl RootSelection {
    pub fn parse(input: &str) -> Result<Self, RootSelectionParseError> {
        let mut steps = Vec::new();
        let mut index = 0;
        let mut pending_relation = RootSelectionRelation::Descendant;
        let mut expecting_tag = true;
        let chars: Vec<_> = input.char_indices().collect();

        while index < input.len() {
            let Some((next_index, ch)) = chars
                .iter()
                .copied()
                .find(|(char_index, _)| *char_index >= index)
            else {
                break;
            };

            if ch.is_whitespace() {
                index = next_index + ch.len_utf8();
                continue;
            }

            if ch == '>' {
                if expecting_tag {
                    return Err(RootSelectionParseError::new(input));
                }

                pending_relation = RootSelectionRelation::DirectChild;
                expecting_tag = true;
                index = next_index + ch.len_utf8();
                continue;
            }

            let tag_start = next_index;
            let mut tag_end = tag_start;
            for (char_index, tag_ch) in input[tag_start..].char_indices() {
                if tag_ch.is_whitespace() || tag_ch == '>' {
                    break;
                }

                tag_end = tag_start + char_index + tag_ch.len_utf8();
            }

            let tag = input[tag_start..tag_end].to_ascii_lowercase();
            if tag.is_empty()
                || !tag
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
            {
                return Err(RootSelectionParseError::new(input));
            }

            let relation = if steps.is_empty() {
                RootSelectionRelation::Descendant
            } else {
                pending_relation
            };
            steps.push(RootSelectionStep { relation, tag });
            pending_relation = RootSelectionRelation::Descendant;
            expecting_tag = false;
            index = tag_end;
        }

        if steps.is_empty() || expecting_tag {
            return Err(RootSelectionParseError::new(input));
        }

        Ok(Self {
            source: input.trim().to_string(),
            steps,
        })
    }

    fn source(&self) -> &str {
        &self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootSelectionParseError {
    input: String,
}

impl RootSelectionParseError {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }
}

impl fmt::Display for RootSelectionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid root selection: {}", self.input)
    }
}

impl std::error::Error for RootSelectionParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertError {
    RootNotFound { root: String },
}

impl fmt::Display for ConvertError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootNotFound { root } => write!(formatter, "root not found: {root}"),
        }
    }
}

impl std::error::Error for ConvertError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RootSelectionStep {
    relation: RootSelectionRelation,
    tag: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootSelectionRelation {
    DirectChild,
    Descendant,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CollapseSingleNestedMode {
    #[default]
    Off,
    TopWins,
    BottomWins,
    BestTagWins,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextWhitespaceMode {
    #[default]
    Collapse,
    Preserve,
}

pub fn convert_html_to_pug(input: &str, options: &ConvertOptions) -> Result<String, ConvertError> {
    let document = parse_html().one(input);
    let mut nodes = if let Some(root) = &options.root {
        let root_node =
            find_root_selection(&document, root).ok_or_else(|| ConvertError::RootNotFound {
                root: root.source().to_string(),
            })?;

        node_from_dom(&root_node, options, TextBoundaryContext::default())
            .map(|node| vec![node])
            .unwrap_or_default()
    } else {
        nodes_from_children(&document, options)
    };

    if options.collapse_single_nested != CollapseSingleNestedMode::Off {
        nodes = nodes
            .into_iter()
            .map(|node| collapse_single_nested(node, options.collapse_single_nested))
            .collect();
    }

    let rendered = render_nodes(&nodes, 0, options);
    if rendered.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("{rendered}\n"))
    }
}

fn find_root_selection(document: &NodeRef, root: &RootSelection) -> Option<NodeRef> {
    find_root_step_matches(document, root, 0)
}

fn find_root_step_matches(
    current: &NodeRef,
    root: &RootSelection,
    step_index: usize,
) -> Option<NodeRef> {
    if step_index >= root.steps.len() {
        return Some(current.clone());
    }

    let step = &root.steps[step_index];
    let candidates = match step.relation {
        RootSelectionRelation::DirectChild => direct_child_elements_matching(current, &step.tag),
        RootSelectionRelation::Descendant => descendant_elements_matching(current, &step.tag),
    };

    for candidate in candidates {
        if let Some(matched) = find_root_step_matches(&candidate, root, step_index + 1) {
            return Some(matched);
        }
    }

    None
}

fn direct_child_elements_matching(node: &NodeRef, tag: &str) -> Vec<NodeRef> {
    node.children()
        .filter(|child| element_tag_matches(child, tag))
        .collect()
}

fn descendant_elements_matching(node: &NodeRef, tag: &str) -> Vec<NodeRef> {
    let mut matches = Vec::new();
    collect_descendant_elements_matching(node, tag, &mut matches);
    matches
}

fn collect_descendant_elements_matching(node: &NodeRef, tag: &str, matches: &mut Vec<NodeRef>) {
    for child in node.children() {
        if element_tag_matches(&child, tag) {
            matches.push(child.clone());
        }

        collect_descendant_elements_matching(&child, tag, matches);
    }
}

fn element_tag_matches(node: &NodeRef, expected: &str) -> bool {
    node.as_element()
        .is_some_and(|element| element.name.local.as_ref().eq_ignore_ascii_case(expected))
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
        let source_attributes = element.attributes.borrow();
        let has_source_attributes = !source_attributes.map.is_empty();
        let attributes = sanitize_attributes(&source_attributes, options);
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
            has_source_attributes,
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

fn collapse_single_nested(node: Node, mode: CollapseSingleNestedMode) -> Node {
    match node {
        Node::Element(mut element) => {
            element.children = element
                .children
                .into_iter()
                .map(|child| collapse_single_nested(child, mode))
                .collect();

            collapse_single_nested_element(element, mode)
        }
        other => other,
    }
}

fn collapse_single_nested_element(element: ElementNode, mode: CollapseSingleNestedMode) -> Node {
    let Some((chain, terminal_children)) = collect_single_nested_chain(&element) else {
        return Node::Element(element);
    };

    let winner_index = match mode {
        CollapseSingleNestedMode::Off => return Node::Element(element),
        CollapseSingleNestedMode::TopWins => 0,
        CollapseSingleNestedMode::BottomWins => chain.len() - 1,
        CollapseSingleNestedMode::BestTagWins => chain
            .iter()
            .enumerate()
            .min_by_key(|(index, element)| (best_tag_rank(&element.tag), *index))
            .map(|(index, _)| index)
            .unwrap_or(0),
    };

    let mut winner = chain[winner_index].clone();
    winner.children = terminal_children;
    Node::Element(winner)
}

fn collect_single_nested_chain(element: &ElementNode) -> Option<(Vec<ElementNode>, Vec<Node>)> {
    if !is_single_nested_chain_link(element) {
        return None;
    }

    let mut chain = Vec::new();
    let mut current = element;

    loop {
        let [Node::Element(child)] = current.children.as_slice() else {
            return None;
        };

        let mut link = current.clone();
        link.children = Vec::new();
        chain.push(link);

        if !is_single_nested_chain_link(child) {
            return (chain.len() >= 2).then(|| (chain, current.children.clone()));
        }

        current = child;
    }
}

fn is_single_nested_chain_link(element: &ElementNode) -> bool {
    !element.has_source_attributes
        && element.raw_text.is_none()
        && matches!(element.children.as_slice(), [Node::Element(_)])
}

const BEST_TAG_HIERARCHY: &[&str] = &[
    "main",
    "article",
    "section",
    "nav",
    "aside",
    "header",
    "footer",
    "form",
    "table",
    "ul",
    "ol",
    "dl",
    "figure",
    "blockquote",
];

fn best_tag_rank(tag: &str) -> usize {
    BEST_TAG_HIERARCHY
        .iter()
        .position(|candidate| *candidate == tag)
        .unwrap_or_else(|| {
            if tag == "div" {
                BEST_TAG_HIERARCHY.len() + 1
            } else {
                BEST_TAG_HIERARCHY.len()
            }
        })
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
            format!(
                "{}doctype html",
                formatting::indent(depth, &options.formatting)
            )
        }
        Node::Doctype(name) => format!(
            "{}doctype {}",
            formatting::indent(depth, &options.formatting),
            name.trim()
        ),
        Node::Comment(comment) => render_comment(comment, depth, options),
        Node::Text(text) => format!(
            "{}| {}",
            formatting::indent(depth, &options.formatting),
            text.content
        ),
        Node::Element(element) => render_element(element, depth, options),
    }
}

fn render_comment(comment: &CommentNode, depth: usize, options: &ConvertOptions) -> String {
    let mut output = format!("{}//", formatting::indent(depth, &options.formatting));

    if let Some(value) = &comment.inline_value {
        output.push(' ');
        output.push_str(value);
        return output;
    }

    for line in &comment.block_lines {
        output.push('\n');
        output.push_str(&formatting::indent(depth + 1, &options.formatting));
        output.push_str(line);
    }

    output
}

fn render_element(element: &ElementNode, depth: usize, options: &ConvertOptions) -> String {
    let mut line = format!(
        "{}{}",
        formatting::indent(depth, &options.formatting),
        element.tag
    );
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

        trailing_attributes.push(render_attribute(attribute, options));
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
            output.push_str(&formatting::indent(depth + 1, &options.formatting));
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
    let Some(line_width) = options.formatting.line_width else {
        return false;
    };

    formatting::display_width(depth, &options.formatting)
        + line_prefix.trim_start().len()
        + 1
        + text.content.len()
        > line_width
}

fn should_render_prose_block(
    line_prefix: &str,
    text: &TextNode,
    depth: usize,
    options: &ConvertOptions,
) -> bool {
    if options.formatting.line_width.is_none() || text.prose_paragraphs.is_empty() {
        return false;
    };

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

    let available_width = options.formatting.line_width.and_then(|line_width| {
        line_width.checked_sub(formatting::display_width(depth + 1, &options.formatting))
    });

    for (index, paragraph) in text.prose_paragraphs.iter().enumerate() {
        if index > 0 {
            line.push('\n');
        }

        let wrapped_lines = wrap_prose_paragraph(paragraph, available_width);
        for wrapped_line in wrapped_lines {
            line.push('\n');
            line.push_str(&formatting::indent(depth + 1, &options.formatting));
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

    formatting::wrap_words(paragraph.split_whitespace(), available_width)
}

fn render_attribute(attribute: &Attribute, options: &ConvertOptions) -> String {
    match &attribute.value {
        Some(value) => format!(
            "{}={}",
            attribute.name,
            formatting::render_attribute_value(value, options.formatting.quote_style)
        ),
        None => attribute.name.clone(),
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
    has_source_attributes: bool,
    raw_text: Option<String>,
    children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Attribute {
    name: String,
    value: Option<String>,
}
