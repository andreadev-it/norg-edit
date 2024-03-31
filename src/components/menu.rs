use dioxus::prelude::*;

use crate::components::Hamburger;
use crate::utils::FileChosenEvent;

#[component]
pub fn Menu(is_open: Signal<bool>, on_file_chosen: EventHandler<FileChosenEvent>) -> Element {
    let filepicker_changed = move |evt: FormEvent| async move {
        if let Some(file_engine) = &evt.files() {
            let files = file_engine.files();
            let selected = files.first();

            if let Some(selected) = selected {
                if let Some(file) = file_engine.read_file_to_string(selected).await {
                    let file_event = FileChosenEvent {
                        filename: selected.clone(),
                        content: file,
                    };

                    on_file_chosen(file_event);
                }
            }
        }
    };

    rsx! {
        div {
            position: "absolute",
            display: "flex",
            flex_direction: "column",
            gap: "10px",
            top: "0px",
            left: "0px",
            background_color: "#eee",
            height: "100%",
            min_width: "200px",
            padding: "10px",
            box_sizing: "border-box",
            transform: if !is_open() {"translateX(-100%)"} else {"translateX(0)"},
            transition: "transform .3s",

            button {
                onclick: move |_| *is_open.write() = false,

                "Close"
            },

            label {
                border: "solid 1px #aaa",
                background_color: "#ededed",

                input {
                    r#type: "file",
                    accept: ".norg",
                    display: "none",

                    onchange: move |evt| filepicker_changed(evt)
                }
                "Open file"
            },

            "Menu"
        },
    }
}
