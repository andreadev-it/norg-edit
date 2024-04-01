use crate::components::AutoComponent;
use crate::nodes::NorgNode;
use anyhow::anyhow;
use anyhow::Result;
use dioxus::prelude::*;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new() -> Self {
        Self { start: 0, end: 0 }
    }
}

// Easily extract ranges from NorgNodes (only works
// with AttachedModifiers)
pub fn get_node_range(node: &NorgNode) -> Result<Range> {
    match node {
        NorgNode::Bold(n) => Ok(n.textrange),
        NorgNode::Italic(n) => Ok(n.textrange),
        NorgNode::Underline(n) => Ok(n.textrange),
        NorgNode::StrikeThrough(n) => Ok(n.textrange),
        NorgNode::Superscript(n) => Ok(n.textrange),
        NorgNode::Subscript(n) => Ok(n.textrange),
        NorgNode::Spoiler(n) => Ok(n.textrange),
        NorgNode::InlineCode(n) => Ok(n.textrange),
        _ => Err(anyhow!("This is not an attached modifier")),
    }
}

// Get list of text splitted when a child node was
// found
pub fn get_node_splitted_text(
    textrange: &Range,
    children: &Vec<NorgNode>,
    source: &str,
    inclusive: bool,
) -> Vec<String> {
    let mut splitted_text = vec![];

    let start = if inclusive {
        textrange.start
    } else {
        textrange.start + 1
    };

    let end = if inclusive {
        textrange.end
    } else {
        textrange.end - 1
    };

    let mut curbyte = start;
    for child in children {
        let range = get_node_range(child);
        if range.is_err() {
            continue;
        }

        let range = range.unwrap();

        splitted_text.push(source[curbyte..range.start].to_string());

        curbyte = range.end;
    }

    if curbyte < end {
        splitted_text.push(source[curbyte..end].to_string());
    }

    splitted_text
}

// TODO: Fix this component so that it doesn't
// require me to always pass `node.children.iter().collect()`.
#[component]
pub fn RenderInterleavedTextAndChildren(text: Vec<String>, children: Vec<NorgNode>) -> Element {
    let text_rsx = text.iter().map(|t| rsx! { {t.as_str()} });

    let child_rsx = children
        .into_iter()
        .map(|c| rsx! { AutoComponent { node: c } });

    let both = text_rsx.interleave(child_rsx);

    rsx! {
        {both}
    }
}
