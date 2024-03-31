use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NodeItem, NorgNode};
use anyhow::Result;
use dioxus::prelude::*;
use tree_sitter::Node;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Heading {
    level: usize,
    text: String,
    children: Vec<NorgNode>,
}

impl NodeItem for Heading {
    fn from_node<'a>(self: &mut Self, node: Node<'a>, source: &str) -> Result<()> {
        let mut cur = node.walk();

        let mut children = node.children(&mut cur);

        if let Some(prefix_node) = children.next() {
            self.level = prefix_node.utf8_text(source.as_bytes())?.chars().count();
        }

        if let Some(title_node) = children.next() {
            self.text = title_node.utf8_text(source.as_bytes())?.to_string();
        }

        // TODO: this currently just ignore items that generate errors
        self.children = children
            .filter_map(|child| node_to_item(child, source).ok())
            .collect();

        Ok(())
    }
}

#[component]
pub fn RenderHeading(node: Heading) -> Element {
    let children = node
        .children
        .into_iter()
        .map(|c| rsx! { AutoComponent { node: c } });

    let text = node.text.as_str();

    let heading = match node.level {
        1 => rsx! {
            h1 {
                user_select: "none",
                {text}
            }
        },
        2 => rsx! {
            h2 {
                user_select: "none",
                {text}
            }
        },
        3 => rsx! {
            h3 {
                user_select: "none",
                {text}
            }
        },
        4 => rsx! {
            h4 {
                user_select: "none",
                {text}
            }
        },
        5 => rsx! {
            h5 {
                user_select: "none",
                {text}
            }
        },
        6 => rsx! {
            h6 {
                user_select: "none",
                {text}
            }
        },
        _ => rsx! {
            h6 {
                user_select: "none",
                {text}
            }
        },
    };

    rsx! {
        details {
            open: true,
            class: "heading",
            summary {
                {heading}
            }
            {children}
        }
    }
}

mod tests {
    use super::*;
    use crate::parse::parse_text;

    #[test]
    fn parse_heading() {
        let text = r"* Heading 1";
        let tree = parse_text(&text);
        let root = tree.root_node();

        let mut heading = Heading::default();
        heading.from_node(root, &text).unwrap();
    }
}
