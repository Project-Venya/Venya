use dioxus::prelude::*;

use crate::backend::save_dog;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let skip_action = move |_event| {
        img_src.restart();
    };

    let save_action = move |_event| async move {
        let current = img_src.cloned().unwrap();
        img_src.restart();
        _ = save_dog(current).await;
    };

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div {
            id: "buttons",
            button { onclick: skip_action, class:"!text-xl cursor-pointer", id: "skip", "skip"},
            button { onclick: save_action, class:"!text-xl cursor-pointer", id: "save", "save!" }
        }
    }
}
