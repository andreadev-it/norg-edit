use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use anyhow::Result;
use dioxus::prelude::*;
use tree_sitter::Node;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct List {
    pub is_ordered: bool,
    pub children: Vec<NorgNode>,
}

impl NodeItem for List {
    fn from_node(self: &mut Self, node: Node, source: &str) -> Result<()> {
        let mut cur = node.walk();

        let list_items = node
            .children(&mut cur)
            .filter_map(|n| node_to_item(n, source).ok())
            .collect();

        // TODO: Analyze the items list to group into different
        // lists by consecutive element type (not so simple, actually)

        self.children = list_items;

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
