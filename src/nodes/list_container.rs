use crate::components::AutoComponent;
use crate::nodes::List;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListContainer {
    lists: Vec<List>,
}

impl NodeItem for ListContainer {
    fn from_node<'a>(&mut self, node: tree_sitter::Node<'a>, source: &str) -> anyhow::Result<()> {
        let mut cur = node.walk();

        let mut lists: Vec<List> = vec![];
        let mut is_first = true;
        let mut last_child_level = 1;
        let mut last_child_was_ordered = true;
        let mut cur_parent = List::default();

        for child in node.children(&mut cur) {
            let item = node_to_item(child, source)?;

            if let NorgNode::ListItem(list_item) = item {
                if is_first {
                    cur_parent.is_ordered = list_item.is_ordered;
                    is_first = false;
                } else {
                    if last_child_level != list_item.level
                        || last_child_was_ordered != list_item.is_ordered
                    {
                        lists.push(cur_parent);
                        cur_parent = List::default();
                        cur_parent.is_ordered = list_item.is_ordered;
                    }
                }

                last_child_level = list_item.level;
                last_child_was_ordered = list_item.is_ordered;
                cur_parent.children.push(NorgNode::ListItem(list_item));
            }
        }
        lists.push(cur_parent);

        self.lists = lists;

        Ok(())
    }
}

#[component]
pub fn RenderListContainer(node: ListContainer) -> Element {
    let lists = node
        .lists
        .into_iter()
        .map(|l| NorgNode::List(l))
        .map(|c| rsx! { AutoComponent { node: c } });

    rsx! {
        {lists}
    }
}
