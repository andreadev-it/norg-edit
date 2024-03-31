use crate::attached_modifier;
use crate::utils::RenderInterleavedTextAndChildren;
use dioxus::prelude::*;

attached_modifier!(Spoiler);

#[component]
pub fn RenderSpoiler(node: Spoiler) -> Element {
    rsx! {
        span {
            background_color: "currentcolor",

            RenderInterleavedTextAndChildren {
                text: node.text,
                children: node.children
            }
        }
    }
}
