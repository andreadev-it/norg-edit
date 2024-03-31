use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use anyhow::Result;
use dioxus::prelude::*;
use tree_sitter::Node;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Document {
    children: Vec<NorgNode>,
}

impl Document {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

impl NodeItem for Document {
    fn from_node<'a>(self: &mut Self, node: Node<'a>, source: &str) -> Result<()> {
        let mut cur = node.walk();
        self.children = node
            .children(&mut cur)
            .filter_map(|child| node_to_item(child, source).ok())
            .collect();

        Ok(())
    }
}

#[component]
pub fn RenderDocument(node: Document) -> Element {
    rsx! {
        {node.children.into_iter().map(|c| rsx! { AutoComponent { node: c } })}
    }
}
