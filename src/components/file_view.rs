use crate::components::AutoComponent;
use crate::nodes::{node_to_item, NorgNode};
use crate::parse::parse_text;
use dioxus::prelude::*;

#[component]
pub fn FileView() -> Element {
    let text = use_context::<Signal<String>>();

    let tree = parse_text(&text());

    let root = tree.root_node();

    let root_node: NorgNode = node_to_item(root, &text()).unwrap();

    dbg!(root.to_sexp());

    rsx! {
        div {
            overflow: "auto",
            padding: "10px",

            div {
                max_width: "80ch",
                margin: "0 auto",
                AutoComponent {
                    node: root_node
                }
            }
        }
    }
}
