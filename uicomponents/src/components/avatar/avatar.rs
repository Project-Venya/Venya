use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct AvatarProps {
    /**
     * Image source URL
     */
    #[props(default = "".to_string())]
    src: String,
    /**
     * Alternative text for the image
     */
    #[props(default = "".to_string())]
    alt: String,
    /**
     * Size of the avatar in pixels
     */
    #[props(default = 40)]
    size: i32,
    /**
     * Shape of the avatar - 'circle' or 'square'
     */
    #[props(default = "circle".to_string())]
    shape: String,
    /**
     * Name to generate initials from
     */
    #[props(default = "".to_string())]
    name: String,
    /**
     * Background color class (TailwindCSS Class)
     */
    #[props(default = "bg-gray-200".to_string())]
    bg_color: String,
    /**
     * Text color class (TailwindCSS Class)
     */
    #[props(default = "text-gray-700".to_string())]
    text_color: String,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let text_size = use_memo(move || {
        if props.size < 32 {
            "xs".to_string()
        } else if props.size < 48 {
            "sm".to_string()
        } else if props.size < 64 {
            "base".to_string()
        } else {
            "lg".to_string()
        }
    });

    let initials = use_memo(move || {
        if props.name.is_empty() {
            return "".to_string();
        }
        props
            .name
            .split(' ')
            .map(|word| word.chars().next().unwrap_or(' ').to_string())
            .collect::<String>()
            .to_uppercase()
            .chars()
            .take(2)
            .collect::<String>()
    });

    let shape_class = match props.shape.as_str() {
        "circle" => "rounded-full",
        "square" => "rounded-lg",
        _ => "rounded-full", // Default to circle
    };

    rsx! {
        div {
            class: "overflow-hidden {shape_class}",
            width: "{props.size}px",
            height: "{props.size}px",
            style: "width: {props.size}px; height: {props.size}px;",
            {
                if !props.src.is_empty() {
                    rsx! {
                        img {
                            src: "{props.src}",
                            alt: "{props.alt}",
                            class: "w-full h-full object-cover",
                            onerror: move |event| {
                                println!("Error loading image - {:?}", event);
                            }
                        }
                    }
                } else {
                    rsx! {
                        div {
                            class: "flex items-center justify-center w-full h-full {props.bg_color}",
                            span {
                                class: "text-{text_size} font-medium {props.text_color}",
                                {initials}
                            }
                        }
                    }
                }
            }
        }
    }
}
