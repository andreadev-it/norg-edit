use dioxus::prelude::*;

#[component]
pub fn QuickButton(on_click: EventHandler<MouseEvent>, icon: String) -> Element {
    rsx! {
        button {
            class: "lighten-on-hover",
            border_radius: "100px",
            border: "solid 0px",
            width: "48px",
            height: "48px",
            box_sizing: "border-box",
            padding: "4px",
            background: "linear-gradient(135deg, #59b480, #4a3c95)",
            color: "#fff",
            font_size: "2em",
            font_weight: "bold",
            box_shadow: "4px 4px 8px rgba(0,0,0,0.5)",
            cursor: "pointer",
            display: "flex",
            justify_content: "center",
            align_items: "center",

            onclick: move |evt| on_click(evt),

            img {
                class: "white-icon",
                width: "32px",
                height: "32px",
                src: icon
            }
        }
    }
}

#[component]
pub fn QuickActionButton(
    on_click: EventHandler<MouseEvent>,
    icon: String,
    text: String,
) -> Element {
    rsx! {
        div {
            class: "lighten-on-hover",
            display: "flex",
            align_items: "center",
            gap: "10px",
            cursor: "pointer",

            onclick: move |evt| on_click(evt),

            button {
                border: "solid 0px",
                border_radius: "100px",
                width: "28px",
                height: "28px",
                padding: "4px",
                box_sizing: "border-box",
                background: "linear-gradient(135deg, #59b480, #4a3c95)",
                color: "#fff",
                box_shadow: "2px 2px 4px rgba(0,0,0,0.5)",
                display: "flex",
                justify_content: "center",
                align_items: "center",

                img {
                    cursor: "pointer",
                    class: "white-icon",
                    width: "20px",
                    height: "20px",
                    src: icon,
                }
            }

            {text}
        }
    }
}
