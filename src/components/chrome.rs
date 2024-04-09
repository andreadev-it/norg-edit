use crate::components::AppState;
use crate::components::Hamburger;
use crate::components::Menu;
use dioxus::prelude::*;

#[component]
pub fn Chrome() -> Element {
    let mut is_menu_open = use_signal(|| false);
    let app_state = use_context::<Signal<AppState>>();

    rsx! {
        div {
            display: "grid",
            grid_template_columns: "40px 1fr 40px",
            justify_items: "center",
            align_items: "center",
            padding: "10px",
            background: "linear-gradient(-45deg, #59b480, #4a3c95)",
            color: "#fff",
            font_weight: "bold",

            Hamburger {
                on_click: move |_| is_menu_open.set(true),
            },

            "File: {app_state.read().current_file_name}",

        },
        Menu {
            is_open: is_menu_open,
        }
    }
}
