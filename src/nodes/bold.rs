use crate::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(Bold);

#[component]
pub fn RenderBold(node: Bold) -> Element {
    rsx! {
        b {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
