use crate::components::{Chrome, EditView, FileView};
use crate::events::FileChosenEvent;
use dioxus::prelude::*;
use std::fs;
use std::path::Path;

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
    pub current_file_name: String,
    pub text_content: String,
    pub should_save_file: bool,
}

impl AppState {
    pub fn read_current_file(&mut self) -> Result<(), ()> {
        if let Some(file) = &self.current_file {
            let file_path = Path::new(&file);
            if let Ok(text) = fs::read_to_string(file_path) {
                self.text_content = text;
                self.should_save_file = false;
            }
        }

        Ok(())
    }

    pub fn read_file(&mut self, file: &String) -> Result<(), ()> {
        let file_path = Path::new(&file);

        if let Ok(text) = fs::read_to_string(file_path) {
            self.current_file = Some(file.clone());
            self.current_file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
            self.text_content = text;
            self.should_save_file = false;
        }

        Ok(())
    }
}

#[component]
pub fn App() -> Element {
    let state = use_signal(|| AppState {
        current_file: None,
        current_file_name: "".to_string(),
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
    let app_state = use_context::<Signal<AppState>>();

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
        Chrome {},
        Outlet::<Route> {}
    }
}
