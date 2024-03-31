use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use anyhow::{Context, Result};
use dioxus::prelude::*;
use tree_sitter::Node;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListItem {
    pub is_ordered: bool,
    pub level: i8,
    children: Vec<NorgNode>,
}

impl NodeItem for ListItem {
    fn from_node<'a>(self: &mut Self, node: Node<'a>, source: &str) -> Result<()> {
        self.is_ordered = node.kind().starts_with("ordered");

        self.level = node
            .kind()
            .chars()
            .last()
            .unwrap()
            .to_string()
            .parse()
            .context("Failed converting list item level")?;

        let mut cur = node.walk();

        self.children = node
            .children(&mut cur)
            .filter_map(|n| node_to_item(n, source).ok())
            .collect();

        Ok(())
    }
}

#[component]
pub fn RenderListItem(node: ListItem) -> Element {
    let children = node
        .children
        .into_iter()
        .map(|item| rsx! { AutoComponent { node: item } });

    rsx! {
        li {
            {children}
        }
    }
}
