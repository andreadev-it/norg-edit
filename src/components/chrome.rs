use crate::components::Hamburger;
use crate::components::Menu;
use crate::events::FileChosenEvent;
use dioxus::prelude::*;

#[component]
pub fn Chrome(current_file: String, on_file_chosen: EventHandler<FileChosenEvent>) -> Element {
    // I want it to be a bool, but I can't make it work
    let mut is_menu_open = use_signal(|| false);

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
                on_click: move |_| *is_menu_open.write() = true,
            },

            "{current_file}",

            button {
                "edit"
            }
        },
        Menu {
            is_open: is_menu_open,
            on_file_chosen: move |evt| {
                *is_menu_open.write() = false;
                on_file_chosen(evt);
            }
        }
    }
}
