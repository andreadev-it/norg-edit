use crate::components::buttons::QuickButton;
use crate::components::{AppState, QuickActions};
use dioxus::prelude::*;

#[component]
pub fn EditView() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();
    let text = &app_state.read().text_content;

    let mut edited_text = use_signal(|| text.clone());

    let save_changes = move |_: MouseEvent| {
        println!("Saving...");
        app_state.write().text_content = edited_text().clone();
        app_state.write().should_save_file = true;

        router().push("/");
    };

    let cancel_changes = move |_: MouseEvent| {
        router().push("/");
    };

    rsx! {
        textarea {
            oninput: move |evt| {
                edited_text.set(evt.value())
            },

            "{edited_text}"
        }

        QuickActions {
            QuickButton {
                on_click: save_changes,
                icon: "../icons/save.svg"
            }
            QuickButton {
                on_click: cancel_changes,
                icon: "../icons/close.svg"
            }
        }
    }
}
