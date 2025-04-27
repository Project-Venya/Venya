mod backend;
mod components;

use components::{
    favorites::Favorites,
    nav::{NavBar, TitleState},
    speech_to_text::SpeechToText,
    view::DogView,
};
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,

    #[route("/speech-to-text")]
    SpeechToText,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDog".to_string()));
    rsx! {
        document::Stylesheet { href: CSS }
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    }
}
