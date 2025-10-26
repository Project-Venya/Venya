use dioxus::prelude::*;

use crate::components::{
    image_loader::image_loader::ImageLoader, typography::normal_text::NormalText,
};

use super::select::SelectOption;

/// Properties for the `Radio` component.
#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    /// List of options to display.
    options: Vec<SelectOption>,

    /// The key of the currently selected option.
    /// Use Option<String> if no selection is a valid state.
    /// If a selection is always expected, String can be used,
    /// and you might default to the first option's key if `value` is empty.
    #[props(into)] // Auto-convert String to Option<String> if needed etc.
    value: Option<String>,

    /// Event handler for when the selection changes.  Emits the key of the selected option.
    on_change: EventHandler<SelectOption>,
}

/// A radio button group component.
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let mut current_selected_key = use_signal(|| props.value.clone());

    let is_selected = move |option: SelectOption| -> bool {
        current_selected_key
            .read()
            .as_ref()
            .map_or(false, |key| key == &option.key)
    };

    rsx! {
        div {
            class: "flex w-full flex-col",

            for (index, option_item) in props.options.iter().cloned().enumerate() {
                {
                    let option_item_clone = option_item.clone();
                    rsx! {
                        div {
                            key: "{index}", // Use index as key for simple iteration. Better is the option.key if it's unique.
                            class: "flex w-full flex-row items-center justify-between cursor-pointer py-3 border-b-[1px] border-gray-300 dark:border-gray-600 mb-2",
                            onclick: move |_| {
                                current_selected_key.set(Some(option_item_clone.clone().key));
                                props.on_change.call(option_item_clone.clone());
                            },

                            div {
                                class: "flex flex-row space-x-2 items-center justify-between w-full",
                                // Left part: indicator, image, text
                                div {
                                    class: "flex flex-row space-x-2 items-center justify-start",
                                    // Radio indicator
                                    span {
                                        class: "h-full flex items-start justify-center",
                                        span {
                                            class: format!(
                                                "h-[11px] w-[11px] rounded-full {}",
                                                if is_selected(option_item.clone()) {
                                                    "!bg-secondary-500"
                                                } else {
                                                    "bg-[#D9D9D9]"
                                                }
                                            )
                                        }
                                    }

                                    // Image (optional)
                                    if let Some(url) = &option_item.image_url {
                                        ImageLoader { // replace with path to ImageLoader if needed
                                            photo_url: url.clone(),
                                            custom_class: "h-[26px] w-[26px] rounded-full".to_string(),
                                        }
                                    }

                                    // Option text
                                    NormalText {
                                        custom_class: "!font-[500] !text-left".to_string(),
                                        {
                                            option_item.alt_value.as_ref().unwrap_or(&option_item.value).clone()
                                        }
                                    }
                                }

                                // Right part: extra info (optional)
                                if let Some(info) = &option_item.extra_info {
                                    NormalText {
                                        custom_class: "!text-button-purple".to_string(),
                                        {info.clone()}
                                    }
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}
