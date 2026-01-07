use dioxus::prelude::*;

use sdk::app::components::AppProvider;

use crate::constants::{FAVICON_ICO, STYLE_CSS};
use crate::routes::Routes;

#[component]
pub fn App() -> Element {
    let mut is_starting = use_signal(|| true);

    use_effect(move || {
        is_starting.set(false);
    });

    rsx! {
        document::Link { rel: "manifest", href: "/manifest.json" }
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        AppProvider { is_starting, Router::<Routes> {} }
    }
}
