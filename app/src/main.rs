use dioxus::prelude::*;
use window_settings::configure_window;

pub mod window_settings;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        configure_window();
    });

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        "Venya AI"
    }
}
