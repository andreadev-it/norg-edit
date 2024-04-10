use crate::components::buttons::QuickButton;
use dioxus::prelude::*;

#[component]
pub fn QuickActions(children: Element) -> Element {
    rsx! {
        div {
            position: "absolute",
            right: "20px",
            bottom: "20px",
            display: "flex",
            gap: "10px",

            {children}
        }
    }
}
