use crate::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(Subscript);

#[component]
pub fn RenderSubscript(node: Subscript) -> Element {
    rsx! {
        sub {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
