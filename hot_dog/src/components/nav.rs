use dioxus::prelude::*;

use crate::Route;

#[derive(Clone)]
pub struct TitleState(pub String);

#[component]
pub fn NavBar() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div { id: "title",
            class: "pb-4",
            Link { to: Route::DogView,
            h1 { class: "text-3xl font-bold", "{title.0}! 🌭" }
            },
         Link { to: Route::Favorites, id: "heart", "♥️" }
         Link { to: Route::SpeechToText, id: "speech_to_text", class: "bg-white rounded-[5px] px-2 py-1 text-base", "🎙️" }
        }
        div {
            class: "w-full h-screen flex flex-col items-center justify-center overflow-y-auto",
            Outlet::<Route> {}
        }

    }
}
