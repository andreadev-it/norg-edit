use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use anyhow::Result;
use dioxus::prelude::*;
use tree_sitter::Node;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Paragraph {
    children: Vec<NorgNode>,
}

impl NodeItem for Paragraph {
    fn from_node<'a>(self: &mut Self, node: Node<'a>, source: &str) -> Result<()> {
        let mut cur = node.walk();

        self.children = node
            .children(&mut cur)
            .filter_map(|n| node_to_item(n, source).ok())
            .collect();

        Ok(())
    }
}

#[component]
pub fn RenderParagraph(node: Paragraph) -> Element {
    rsx! {
        p {
            {node.children
                .into_iter()
                .map(|c| rsx! { AutoComponent { node: c } })}
        }
    }
}
