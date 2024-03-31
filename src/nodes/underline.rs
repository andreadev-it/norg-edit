use crate::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(Underline);

#[component]
pub fn RenderUnderline(node: Underline) -> Element {
    rsx! {
        u {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
