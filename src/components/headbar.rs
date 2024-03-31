use crate::components::Hamburger;
use crate::{components::Menu, utils::FileChosenEvent};
use dioxus::prelude::*;

#[component]
pub fn Chrome(current_file: String, on_file_chosen: EventHandler<FileChosenEvent>) -> Element {
    // I want it to be a bool, but I can't make it work
    let mut is_menu_open = use_signal(|| false);

    rsx! {
        div {
            display: "flex",
            justify_content: "space-between",
            align_items: "center",
            padding: "10px",
            background_color: "#eee",

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
            on_file_chosen
        }
    }
}
