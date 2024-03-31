use crate::components::{Chrome, FileView};
use crate::utils::FileChosenEvent;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let mut current_file = use_signal(|| "".to_string());
    let mut text = use_signal(|| "Select a file in the menu".to_string());

    use_context_provider(|| text);

    let file_chosen = move |evt: FileChosenEvent| {
        let filepath = std::path::Path::new(&evt.filename);
        let filename = filepath.file_name().unwrap();

        *current_file.write() = filename.to_str().unwrap().to_string();

        *text.write() = evt.content;
    };

    rsx! {
        Chrome {
            current_file,
            on_file_chosen: file_chosen
        },
        FileView {}
    }
}
