use crate::components::buttons::QuickButton;
use dioxus::prelude::*;

#[component]
pub fn QuickActions(children: Element) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            position: "absolute",
            right: "20px",
            bottom: "20px",

            QuickButton {
                on_click: move |_evt| is_open.set(!is_open()),
                icon: if is_open() {
                    "../icons/close.svg"
                }
                else {
                    "../icons/more.svg"
                }
            }

            div {
                position: "absolute",
                bottom: "60px",
                right: "0px",
                display: "flex",
                flex_direction: "column",
                gap: "10px",
                transition: "transform .5s, opacity .5s",

                pointer_events: if is_open() {
                    "auto"
                } else {
                    "none"
                },

                transform: if is_open() {
                    "translateY(0px)"
                } else {
                    "translateY(30px)"
                },

                opacity: if is_open() {
                    "1"
                } else {
                    "0"
                },

                {children}
            }
        }
    }
}
