use crate::nodes::{node_to_item, NodeItem, NorgNode};
use crate::utils::{get_node_splitted_text, Range, RenderInterleavedTextAndChildren};
use anyhow::Result;
use dioxus::prelude::*;
use tree_sitter::Node;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParagraphSegment {
    text: Vec<String>,
    children: Vec<NorgNode>,
    textrange: Range,
}

impl NodeItem for ParagraphSegment {
    fn from_node<'a>(self: &mut Self, node: Node<'a>, source: &str) -> Result<()> {
        let mut cur = node.walk();

        self.children = node
            .children(&mut cur)
            .filter_map(|n| node_to_item(n, source).ok())
            .collect();

        self.textrange = Range {
            start: node.start_byte(),
            end: node.end_byte(),
        };

        self.text = get_node_splitted_text(&self.textrange, &self.children, source, true);
        // Make sure that paragraph segments are separated when
        // displayed on the same line
        self.text.push(" ".to_string());

        Ok(())
    }
}

#[component]
pub fn RenderParagraphSegment(node: ParagraphSegment) -> Element {
    rsx! {
        span {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
