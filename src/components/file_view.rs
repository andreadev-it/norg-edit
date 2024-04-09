use crate::components::buttons::QuickActionButton;
use crate::components::{AppState, AutoComponent, QuickActions};
use crate::nodes::{node_to_item, NorgNode};
use crate::parse::parse_text;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn FileView() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    let text = &app_state.read().text_content;
    let tree = parse_text(text);

    let root = tree.root_node();

    let root_node: NorgNode = node_to_item(root, text).unwrap();

    dbg!(root.to_sexp());

    let refresh = move |_| {
        let _ = app_state.write().read_current_file();
    };

    rsx! {
        div {
            overflow: "auto",
            padding: "10px",

            div {
                max_width: "80ch",
                margin: "0 auto",
                AutoComponent {
                    node: root_node
                }
            }

            QuickActions {
                QuickActionButton {
                    on_click: refresh,
                    icon: "../icons/refresh.svg",
                    text: "Refresh"
                },
                QuickActionButton {
                    on_click: move |_evt| {
                        router().push(Route::EditView {});
                    },
                    icon: "../icons/edit.svg",
                    text: "Edit"
                },
            }
        }
    }
}
