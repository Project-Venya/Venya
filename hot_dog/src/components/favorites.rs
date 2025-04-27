use dioxus::prelude::*;

use crate::backend::list_dogs;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_resource(list_dogs).suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (_id, url) in favorites().unwrap() {
                    // Render a div for each photo using the dog's ID as the list key
                    div {
                        key: _id,
                        class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
