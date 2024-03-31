use crate::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(InlineCode);

#[component]
pub fn RenderInlineCode(node: InlineCode) -> Element {
    rsx! {
        code {
            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
