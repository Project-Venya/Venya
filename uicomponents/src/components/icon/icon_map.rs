use dioxus::prelude::*;

pub fn get_icon(name: &str) -> Asset {
    match name {
        "search" => asset!("/assets/images/icons/search.svg"),
        "eye" => asset!("/assets/images/icons/eye.svg"),
        "eye-off" => asset!("/assets/images/icons/eye-off.svg"),
        "chevron-down" => asset!("/assets/images/icons/chevron-down.svg"),
        "checked" => asset!("/assets/images/icons/checked.svg"),
        "not-checked" => asset!("/assets/images/icons/not-checked.svg"),
        _ => panic!("Icon not found"),
    }
}
