use crate::components::AppState;
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

    rsx! {
        textarea {
            oninput: move |evt| {
                edited_text.set(evt.value())
            },

            "{edited_text}"
        }
        button {
            onclick: save_changes,
            "Save"
        }
    }
}
