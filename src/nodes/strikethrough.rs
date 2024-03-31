use crate::macros::attached_modifier::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(StrikeThrough);

#[component]
pub fn RenderStrikeThrough(node: StrikeThrough) -> Element {
    rsx! {
        s {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
