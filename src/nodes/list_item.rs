use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use anyhow::{Context, Result};
use dioxus::prelude::*;
use tree_sitter::Node;

use super::list::List;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListItem {
    pub is_ordered: bool,
    pub level: i8,
    pub children: Vec<NorgNode>,
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
        let mut cur_parent: Option<List> = None;

        // This too wil just ignore errors in the children
        for child in node.children(&mut cur) {
            let item_res = node_to_item(child, source);

            if let Ok(item) = item_res {
                match item {
                    NorgNode::ListItem(li) => {
                        if cur_parent.is_none() {
                            let mut list = List::default();
                            list.is_ordered = li.is_ordered;
                            cur_parent = Some(list);
                        }

                        if let Some(l) = cur_parent {
                            // If the li found is different from the one before (ordered/unordered)
                            // - Push the current list as a children
                            // - Create a new list that matches the li type
                            // - Set that as the new parent
                            // The `li` will be added just after this code
                            if l.is_ordered != li.is_ordered {
                                self.children.push(NorgNode::List(l));

                                let mut list = List::default();
                                list.is_ordered = li.is_ordered;
                                cur_parent = Some(list);
                            } else {
                                cur_parent = Some(l);
                            }
                        }

                        if let Some(l) = &mut cur_parent {
                            l.children.push(NorgNode::ListItem(li));
                        }
                    }
                    _ => {
                        if let Some(l) = cur_parent {
                            self.children.push(NorgNode::List(l));
                        }
                        cur_parent = None;
                        self.children.push(item);
                    }
                }
            }
        }
        if let Some(l) = cur_parent {
            self.children.push(NorgNode::List(l));
        }

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
