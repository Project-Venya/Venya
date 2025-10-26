use dioxus::prelude::*;

use crate::components::icon::icon_map::get_icon;

#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    /**
     * The name of the icon file (without the extension).
     * The component will attempt to load `/images/icons/{name}.svg`.
     */
    pub name: String,
    /**
     * Custom CSS classes to apply to the `<img>` element.
     * Useful for styling the icon's size, color, etc.
     */
    #[props(default = "w-5 h-5".to_string())]
    pub custom_class: String,
    /**
     * The file extension of the icon.
     */
    #[props(default = "svg".to_string())]
    pub extension: String,

    /**
     * Optional onclick handler.
     */
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let icon_src = get_icon(&props.name);

    rsx! {
        img {
            src: "{icon_src}",
            class: "{props.custom_class}",
            alt: "{props.name} icon",
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            onerror: move |event| {
                println!("Error loading icon: {} - {:?}", icon_src, event);
            }
        }
    }
}
