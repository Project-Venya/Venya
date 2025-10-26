use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ComponentProps {
    /**
     * Size of the text.
     * @values 'base', 'small'
     * @default 'base'
     */
    #[props(default = "base".to_string())]
    size: String,
    /**
     * Color of the text.  Uses TailwindCSS text color classes (e.g., 'text-black', 'text-gray-500').
     * @default 'text-black'
     */
    #[props(default = "text-black".to_string())]
    color: String,
    /**
     * Custom CSS class(es) to apply to the text.
     * @default ''
     */
    #[props(default = "".to_string())]
    custom_class: String,
    /**
     * Determines if the text content should be rendered as HTML.
     * If true, the `htmlContent` prop will be used.
     * @default false
     */
    is_html: Option<bool>,
    /**
     * The HTML content to render if `isHtml` is true.
     * @default ''
     */
    #[props(default = "".to_string())]
    html_content: String,

    /**
     * The children element
     */
    children: Element,
}

#[component]
pub fn NormalText(props: ComponentProps) -> Element {
    let mut text_size = use_signal(|| "");

    // Set custom class
    use_effect(move || {
        if props.size == "base" {
            text_size.set("lg:text-sm mdlg:text-[12px] text-xs ");
        } else if props.size == "small" {
            text_size.set("text-xs ");
        }
    });

    rsx! {
        {
          if !props.is_html.unwrap_or(false) {
            rsx! {
                span {
                    class: "{props.color} {props.custom_class}",
                    {props.children}
                }
            }
        } else {
            rsx!{
                span {
                    class: "{props.color} {props.custom_class}",
                    dangerous_inner_html: "{props.html_content}"
                }
            }
        }
      }
    }
}
