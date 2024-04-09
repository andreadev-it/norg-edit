use crate::components::AppState;
use dioxus::prelude::*;

#[component]
pub fn Menu(is_open: Signal<bool>) -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let filepicker_changed = move |evt: FormEvent| async move {
        if let Some(file_engine) = &evt.files() {
            let files = file_engine.files();
            let selected = files.first();

            if let Some(selected) = selected {
                let _ = app_state.write().read_file(selected);
                is_open.set(false);
            }
        }
    };

    rsx! {
        div {
            position: "absolute",
            display: "flex",
            flex_direction: "column",
            top: "0px",
            left: "0px",
            background_color: "#eee",
            height: "100%",
            min_width: "200px",
            padding: "0px",
            box_sizing: "border-box",
            transform: if !is_open() {"translateX(-100%)"} else {"translateX(0)"},
            transition: "transform .3s",

            button {
                display: "flex",
                justify_content: "right",
                color: "#fff",
                font_weight: "bold",
                border: "0px",
                padding: "14px",
                cursor: "pointer",
                background: "linear-gradient(-45deg, #59b480, #4a3c95)",
                onclick: move |_| is_open.set(false),

                "Close"
            },

            label {
                padding: "10px",
                background_color: "#ededed",
                border_bottom: "solid 1px #aaa",
                cursor: "pointer",

                input {
                    r#type: "file",
                    accept: ".norg",
                    display: "none",

                    onchange: move |evt| filepicker_changed(evt)
                }
                "Open file"
            },
        },
    }
}
