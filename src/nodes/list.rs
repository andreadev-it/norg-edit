use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use anyhow::Result;
use dioxus::prelude::*;
use tree_sitter::Node;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct List {
    pub is_ordered: bool,
    children: Vec<NorgNode>,
}

impl NodeItem for List {
    fn from_node(self: &mut Self, node: Node, source: &str) -> Result<()> {
        let mut cur = node.walk();

        self.children = node
            .children(&mut cur)
            .filter_map(|n| node_to_item(n, source).ok())
            .collect();

        // Look into the children to see if the
        // list is ordered. Defaults to unordered
        self.is_ordered = self
            .children
            .first()
            .map(|li| match li {
                NorgNode::ListItem(li) => li.is_ordered,
                _ => false,
            })
            .unwrap_or(false);

        Ok(())
    }
}

#[component]
pub fn RenderList(node: List) -> Element {
    let children = node
        .children
        .into_iter()
        .map(|c| rsx! { AutoComponent { node: c } });

    match node.is_ordered {
        true => rsx! {
            ol { {children} }
        },
        false => rsx! {
            ul { {children} }
        },
    }
}
