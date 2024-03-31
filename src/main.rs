#![allow(non_snake_case)]
use crate::components::App;
use dioxus::prelude::*;
use dioxus_desktop::Config;

mod components;
mod macros;
mod nodes;
mod parse;
mod utils;

fn main() {
    // Popup-like window setup
    //
    // dioxus_desktop ::launch_cfg(App, Config::default()
    //     .with_window(WindowBuilder::new()
    //         .with_transparent(true)
    //         .with_resizable(false)
    //         .with_decorations(false)
    //     )
    //     .with_custom_head("<link href=\"../public/base.css\" rel=\"stylesheet\">".to_string())
    // );

    LaunchBuilder::desktop()
        .with_cfg(
            Config::default().with_custom_head(
                "<link href=\"../public/base.css\" rel=\"stylesheet\">".to_string(),
            ),
        )
        .launch(App);
}
