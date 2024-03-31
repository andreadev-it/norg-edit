use crate::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(Superscript);

#[component]
pub fn RenderSuperscript(node: Superscript) -> Element {
    rsx! {
        sup {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
