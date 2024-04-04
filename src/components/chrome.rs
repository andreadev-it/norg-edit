use crate::components::Hamburger;
use crate::components::Menu;
use crate::events::FileChosenEvent;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Chrome(current_file_name: String, on_file_chosen: EventHandler<FileChosenEvent>) -> Element {
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
                on_click: move |_| is_menu_open.set(true),
            },

            "File: {current_file_name}",

            Link { to: Route::EditView {}, "Edit" }
        },
        Menu {
            is_open: is_menu_open,
            on_file_chosen: move |evt| {
                is_menu_open.set(false);
                on_file_chosen(evt);
            }
        }
    }
}
