use crate::components::{Chrome, EditView, FileView};
use crate::events::FileChosenEvent;
use dioxus::prelude::*;
use std::fs;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        FileView {},

        #[route("/edit")]
        EditView {},
}

#[derive(Default)]
pub struct AppState {
    pub current_file: Option<String>,
    pub text_content: String,
    pub should_save_file: bool,
}

#[component]
pub fn App() -> Element {
    let state = use_signal(|| AppState {
        current_file: None,
        text_content: "Select a file in the menu".to_string(),
        should_save_file: false,
    });

    use_context_provider(|| state);

    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Layout() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let mut current_file_name = use_signal(|| "".to_string());

    // This function will put the updated file into the state
    let file_chosen = move |evt: FileChosenEvent| {
        let filepath = std::path::Path::new(&evt.filename);
        let filename = filepath.file_name().unwrap();

        current_file_name.set(filename.to_str().unwrap().to_string());

        app_state.write().current_file = Some(filepath.to_str().unwrap().to_string());
        app_state.write().text_content = evt.content;
    };

    // This effect is used for saving the file
    use_effect(move || {
        if app_state.read().should_save_file {
            if let Some(file) = &app_state.read().current_file {
                // TODO: Handle result
                let _ = fs::write(file, &app_state.read().text_content);
            }
        }
    });

    rsx! {
        Chrome {
            current_file_name,
            on_file_chosen: file_chosen
        },
        Outlet::<Route> {}
    }
}
