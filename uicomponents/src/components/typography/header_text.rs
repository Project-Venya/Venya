use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct HeaderTextProps {
    /**
     * Defines the size of the header text.
     *
     * @values lg, xl, base, xs
     * @default lg
     */
    #[props(default = "lg".to_string())]
    size: String,
    /**
     * Defines the color of the header text.
     * Accepts any valid tailwind color class e.g. `text-red-500`.
     * @default text-black
     */
    #[props(default = "text-black".to_string())]
    color: String,
    /**
     *  Allows to add custom classes to the header text.
     *  @default ""
     */
    #[props(default = "".to_string())]
    custom_class: String,

    /**
     * The headerText children
     */
    children: Element,
}

#[component]
pub fn HeaderText(props: HeaderTextProps) -> Element {
    let mut text_size = use_signal(|| "");

    // Set custom class
    use_effect(move || {
        if props.size == "lg" {
            text_size.set("lg:text-lg mdlg:text-lg text-lg");
        } else if props.size == "xl" {
            text_size.set("lg:text-lg mdlg:text-lg text-base");
        } else if props.size == "base" {
            text_size.set("lg:text-md mdlg:text-base text-sm");
        } else if props.size == "xs" {
            text_size.set("mdlg:text-[12px]! text-xs");
        }
    });

    rsx! {
        h3 {
            class: "font-semibold {text_size} {props.custom_class} {props.color}",
            {props.children}
        }
    }
}
