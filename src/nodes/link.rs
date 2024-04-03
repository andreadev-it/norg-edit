use crate::utils::Range;
use anyhow::{anyhow, Result};
use dioxus::prelude::*;

use super::NodeItem;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Link {
    pub target: String,
    pub text: String,
    pub textrange: Range,
}

impl NodeItem for Link {
    fn from_node<'a>(&mut self, node: tree_sitter::Node<'a>, source: &str) -> Result<()> {
        let mut cur = node.walk();
        let mut location = "".to_string();
        let mut description = "".to_string();

        for child in node.children(&mut cur) {
            match child.kind() {
                "link_location" => {
                    let start = child.start_byte() + 1;
                    let end = child.end_byte() - 1;
                    let text = source[start..end].to_string();
                    location = text;

                    println!("Link location: {location}");
                }
                "link_description" => {
                    let start = child.start_byte() + 1;
                    let end = child.end_byte() - 1;
                    let text = source[start..end].to_string();
                    description = text;

                    println!("Link description: {description}");
                }
                _ => {}
            }
        }

        if description.is_empty() && location.is_empty() {
            return Err(anyhow!("Could not understand this link"));
        }

        self.text = if description.is_empty() {
            location.clone()
        } else {
            description
        };
        self.target = location;

        self.textrange = Range {
            start: node.start_byte(),
            end: node.end_byte(),
        };

        Ok(())
    }
}

#[component]
pub fn RenderLink(node: Link) -> Element {
    rsx! {
        a {
            href: node.target,

            "{node.text}"
        }
    }
}
