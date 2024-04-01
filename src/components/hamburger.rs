use dioxus::prelude::*;

#[component]
pub fn Hamburger(on_click: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div {
            width: "20px",
            padding: "4px",
            cursor: "pointer",

            onclick: move |evt| on_click(evt),

            div {
                border_bottom: "solid 3px #fff",
                margin_bottom: "3px"
            }
            div {
                border_bottom: "solid 3px #fff",
                margin_bottom: "3px"
            }
            div {
                border_bottom: "solid 3px #fff"
            }
        }
    }
}
