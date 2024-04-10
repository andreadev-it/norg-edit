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
pub fn QuickMultiButton(open_icon: String, close_icon: String, children: Element) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            QuickButton {
                on_click: move |_evt| is_open.set(!is_open()),
                icon: if is_open() {
                    close_icon
                }
                else {
                    open_icon
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
