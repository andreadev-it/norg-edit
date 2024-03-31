use crate::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(Italic);

#[component]
pub fn RenderItalic(node: Italic) -> Element {
    rsx! {
        i {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
