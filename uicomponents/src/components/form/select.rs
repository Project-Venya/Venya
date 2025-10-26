#![allow(non_snake_case)] // Allow uppercase component names

use dioxus::prelude::*;

use crate::components::{
    badge::badge::{Badge, BadgeColor},
    form::{radio::Radio, text_field::TextField},
    icon::icon::Icon,
    image_loader::image_loader::ImageLoader,
    typography::normal_text::NormalText,
};
use rand::Rng;

/// Represents an option within the `Select` component.
#[derive(PartialEq, Clone, Debug)]
pub struct SelectOption {
    /// A unique key to identify the option.
    pub key: String,
    /// The primary value displayed for the option.
    pub value: String,
    /// An alternative value to display, if different from the primary value.
    pub alt_value: Option<String>,
    /// URL of an image to display with the option.
    pub image_url: Option<String>,
    /// Additional information about the option.
    pub extra_info: Option<String>,
    /// Whether the option is disabled.
    pub disabled: bool,
}

/// Represents the visual theme of the `Select` component.
#[derive(PartialEq, Clone, Debug)]
pub enum SelectTheme {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
}

impl Default for SelectTheme {
    fn default() -> Self {
        SelectTheme::Light
    }
}

impl std::fmt::Display for SelectTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectTheme::Light => write!(f, "light"),
            SelectTheme::Dark => write!(f, "dark"),
        }
    }
}

/// Properties for the `Select` component.
#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    /// Placeholder text for the select.
    #[props(default = "".to_string())]
    placeholder: String,

    /// The currently selected value (key from SelectOption).
    #[props(into)]
    value: Option<String>,

    /// Available options for the select.
    options: Vec<SelectOption>,

    /// Callback when the selection changes (passes the key of the selected option).
    on_change: EventHandler<String>,

    /// Callback when the selection changes for multiple select (passes the key of the selected option).
    on_multiple_items_change: EventHandler<Vec<String>>,

    /// Custom CSS class for the outer div.
    #[props(default = "".to_string())]
    custom_class: String,

    /// Whether multiple selections are allowed.
    #[props(default = false)]
    is_multiple: bool,

    /// Whether show value with key or not.
    #[props(default = false)]
    with_key: bool,

    /// CSS class for the default size.
    #[props(default = "w-full".to_string())]
    default_size: String,

    /// Whether the select should support auto-complete.
    #[props(default = false)]
    auto_complete: bool,

    /// Whether the select should use HTML for the displayed text.
    #[props(default = false)]
    use_html: bool,

    /// Whether the select is required.
    #[props(default = false)]
    required: bool,

    /// CSS class for show more icon size.
    #[props(default = "h-[8px] w-[14px]".to_string())]
    show_more_size: String,

    /// CSS class for input style.
    #[props(default = "".to_string())]
    input_style: String,

    #[props(default = SelectTheme::default())]
    theme: SelectTheme,

    /// Whether to enable search functionality
    #[props(default = false)]
    has_search: bool,

    /// Message to display when no search results are found
    #[props(default = "".to_string())]
    search_message: String,

    /// Whether search is loading
    #[props(default = false)]
    search_is_loading: bool,

    /// If the select is just a trigger element, and the dropdown is rendered elsewhere
    #[props(default = false)]
    is_wrapper: bool,

    #[props(default = false)]
    chip_like_options: bool,
    /// Title
    #[props(default = false)]
    has_title: bool,

    ///First Icon
    #[props(default = None)]
    first_icon: Option<String>,

    #[props(default = "py-4 px-3".to_string())]
    paddings: String,

    // /// Optional prefix content inside of the input
    // #[props(default = None)]
    // inner_prefix: Option<Element>,

    // /// Optional suffix content inside of the input
    // #[props(default = None)]
    // inner_suffix: Option<Element>,
    /// Event handler when an option is selected
    on_option_selected: EventHandler<SelectOption>,

    ///Event Handler when search value changes
    on_search: Option<EventHandler<String>>,

    /// Handle search
    handle_search: Option<EventHandler<String>>,

    ///Title children
    title_children: Option<Element>,

    ///Inner Prefix children
    inner_prefix_children: Option<Element>,

    ///Inner Suffix children
    inner_suffix_children: Option<Element>,

    wrapper_children: Option<Element>,
}

/// A customizable select component.
#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut show_select_modal = use_signal(|| false);
    let mut selected_key = use_signal(|| props.value.clone());
    let mut search_value = use_signal(|| "".to_string());
    let mut value_data = use_signal(|| "".to_string());
    let mut text_value = use_signal(|| "".to_string());
    let mut show_option = use_signal(|| false);
    let first_icon = use_signal(|| props.first_icon.clone());
    let mut is_focused = use_signal(|| false);
    // let mut validation_status = use_signal(|| false);
    let mut search_result: Signal<Vec<SelectOption>> = use_signal(|| Vec::new());
    let mut selected_items: Signal<Vec<String>> = use_signal(|| Vec::new());
    let select_options: Signal<Vec<SelectOption>> = use_signal(|| props.options.clone());

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();

    let tabIndex = use_signal(|| random_number);

    // Update selected_key when props.value changes from outside
    {
        let value = props.value.clone();
        use_effect(move || {
            selected_key.set(value.clone());
        });
    }

    // Set search result
    use_effect(move || {
        search_result.set(select_options());
    });

    // Use use_memo to efficiently filter options based on search value
    let filtered_options = use_memo(move || {
        let search_value = search_value.read();
        if search_value.is_empty() {
            props.options.clone()
        } else {
            props
                .options
                .iter()
                .filter(|option| {
                    option
                        .value
                        .to_lowercase()
                        .contains(&search_value.to_lowercase())
                })
                .cloned()
                .collect::<Vec<_>>()
        }
    });

    let item_is_selected =
        move |input_key: &str| -> bool { selected_items.read().contains(&input_key.to_string()) };

    let mut select_value = move |option: SelectOption| {
        selected_key.set(Some(option.clone().key));

        if props.auto_complete {
            props.on_option_selected.call(option.clone());

            is_focused.set(false);
            show_option.set(false);

            if props.with_key {
                value_data.set(option.clone().key);
            } else {
                value_data.set(option.clone().value);
                if option.alt_value.is_some() {
                    text_value.set(option.clone().alt_value.unwrap());
                } else {
                    text_value.set(option.clone().value);
                }
            }
        }

        if props.is_multiple {
            let option_clone = option.clone();
            if item_is_selected(option_clone.key.as_str()) {
                let items_not_selected: Vec<String> = selected_items
                    .read()
                    .iter()
                    .filter(|key| *key != &option_clone.key)
                    .cloned()
                    .collect();
                selected_items.set(items_not_selected);
                return;
            }
            selected_items.write().push(option_clone.key);
            props.on_multiple_items_change.call(selected_items())
        } else {
            let option_clone = option.clone();
            props.on_change.call(option_clone.key);
            props.on_option_selected.call(option.clone());

            if props.with_key {
                value_data.set(option.clone().key);
            } else {
                value_data.set(option.clone().value);
                if option.alt_value.is_some() {
                    text_value.set(option.clone().alt_value.unwrap());
                } else {
                    text_value.set(option.clone().value);
                }
                is_focused.set(false);
                show_option.set(false);
            }
        }
    };

    let get_selected_option = move |key_value: &str| -> Option<String> {
        let current_option = search_result
            .read()
            .iter()
            .find(|item| *item.key == key_value.to_string())
            .cloned();

        if current_option.is_some() {
            Some(current_option.unwrap().value)
        } else {
            None
        }
    };

    let get_selected_item_image = move |key_value: &str| -> Option<String> {
        let current_option = search_result
            .read()
            .iter()
            .find(|item| *item.key == key_value.to_string())
            .cloned();

        if current_option.is_some() {
            let current_option_data = current_option.unwrap();

            if current_option_data.image_url.is_some() {
                Some(current_option_data.image_url.unwrap())
            } else {
                None
            }
        } else {
            None
        }
    };

    let mut search_option = move || {
        if props.auto_complete && props.handle_search.is_none() {
            let search_response = search_options(&select_options(), search_value().as_str());
            search_result.set(search_response);
        }
    };

    // let check_validation = move || {
    //     if selected_key.read().is_some() {
    //         if !selected_key().unwrap().is_empty() {
    //             validation_status.set(true);
    //         }
    //     }
    // };

    // For search action
    use_effect(move || {
        if props.has_search && props.on_search.is_some() {
            props.on_search.unwrap().call(search_value.cloned());
        }

        if props.handle_search.is_some() {
            props.handle_search.unwrap().call(search_value.cloned());
        } else {
            search_option();
        }
    });

    rsx! {
        div {
            class: "flex flex-col space-y-2 {props.custom_class} {props.default_size}",
            if props.is_wrapper {
                {
                    rsx! {
                    div {
                        class: "w-full flex flex-col cursor-pointer",
                        onclick: move |_| show_select_modal.set(true),
                        {props.wrapper_children}
                    }
                }
             }
            } else {
                {
                rsx! {
                    if props.has_title && props.title_children.is_some() {
                        NormalText {
                            children: rsx! {
                                {props.title_children.unwrap()}
                                if props.required {
                                    span {
                                        class: "text-red-400 pl-[2px]",
                                        "*"
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex relative flex-row items-center space-x-1 justify-between cursor-pointer w-full rounded-lg bg-grey-50 border-[1px] border-grey-50 focus:border-primary-400 {props.paddings}",
                        id: "container{tabIndex}",
                        tabindex: "{tabIndex}",
                        onclick: move |_| show_select_modal.set(true),
                        onfocus: move |_| {
                            show_option.set(true);
                            is_focused.set(true)
                        },
                        onblur: move |_| {
                            show_option.set(false);
                            is_focused.set(false)
                        },

                        if get_selected_item_image(&selected_key().unwrap_or_else(|| "".to_string())).is_some() {
                            ImageLoader {
                                photo_url: get_selected_item_image(&selected_key().unwrap_or_else(|| "".to_string())).unwrap(),
                                custom_class: "h-[26px] w-[26px] rounded-full "
                            }
                        },

                        if first_icon.read().as_ref().is_some() {
                                 Icon  {
                                    name: "{first_icon().unwrap().clone()}",
                                    custom_class: "h-4 w-4 cursor-pointer"
                                }
                        }

                        if props.inner_prefix_children.is_some() {
                            {props.inner_prefix_children}
                        }

                        if !props.is_multiple {
                            if props.use_html {
                                input {
                                    value: if props.with_key {value_data()} else {text_value()},
                                    placeholder: props.placeholder,
                                    disabled: true,
                                    class: "flex-grow bg-transparent focus input w-full focus:outline-none !pt-[3px] placeholder-grey-400 cursor-pointer {props.input_style}"
                                }
                            } else {
                                NormalText {
                                    is_html: true,
                                    html_content: if props.with_key {value_data()} else {text_value()},
                                    custom_class: "whitespace-nowrap xs:!text-[11px] !text-[12px] text-black {props.input_style}"
                                }
                            }
                        } else {
                            div {
                                class: "w-full flex flex-row  flex-wrap items-center {props.input_style} cursor-pointer",
                                if selected_items().len() == 0 {
                                    NormalText {
                                        custom_class: "!text-grey-400 text-left",
                                        {props.placeholder}
                                    }
                                }

                                for (_index, item) in selected_items().iter().cloned().enumerate() {
                                    span {
                                        key: _index,
                                        class: "pr-1 pb-1",
                                        span {
                                            class: "px-2 py-[2px] border-[1px] border-grey-200 bg-white dark:bg-black text-center flex flex-row space-x-[2px] items-center rounded-[6px]",
                                            NormalText {
                                                custom_class: "!text-[10px]",
                                                {get_selected_option(&item.as_str())}
                                            }
                                        }

                                    }
                                }
                            }
                        }

                        Icon {
                            name: "chevron-down",
                            custom_class: "cursor-pointer {props.show_more_size}"
                        },

                        {props.inner_suffix_children}
                    }
                }
                }
            }

        }
        if *show_select_modal.read() {
               div {
                class: "fixed top-0 left-0 z-[99999999999999999] !font-inter bg-black !bg-opacity-30 dark:!bg-opacity-50 flex w-full h-full flex-row items-end justify-end mdlg:!items-center mdlg:!justify-center md:!justify-center md:!items-center",
                    onclick: move |_| show_select_modal.set(false),

                    div {
                        class: "w-full mdlg:!w-[60%] md:!w-[80%] grid grid-cols-12 h-full relative",
                        id: "modalContent",
                            // Left side
                        div {
                            class: "hidden col-span-3 md:!col-span-2 mdlg:!col-span-3 mdlg:!flex md:!flex flex-col sticky top-0"
                        }
                            // Main section
                        div {
                            class: "col-span-12 mdlg:!col-span-6 md:!col-span-8 relative h-full flex flex-col items-end justify-end mdlg:!items-center mdlg:!justify-center md:!justify-center md:!items-center",
                            div {
                                onclick: move |evt| evt.stop_propagation(),
                                class: "rounded-t-2xl mdlg:!rounded-[10px] md:!rounded-[10px] flex flex-col space-y-2 bg-white dark:!bg-black dark:border-[1px] dark:border-gray-100 w-full absolute mdlg:!relative md:!relative overflow-y-auto h-[50%] mdlg:!max-h-[500px] md:!max-h-[400px] xs:!bottom-0 sm:!bottom-0 left-0 pb-3 px-3 mdlg:!pb-4 md:!pb-4 lg:!text-sm mdlg:!text-[12px] text-xs",
                                div {
                                    class: "flex items-center justify-center sticky top-0 bg-white dark:!bg-black w-full pt-3",
                                    span {
                                        class: "bg-gray-500 dark:bg-gray-200 rounded-full w-[30px] h-[4px]"
                                    }
                                }
                                if props.auto_complete {
                                        div {
                                            class: "w-full pt-1 sticky top-[18px] bg-white dark:bg-black dark:black flex flex-row space-x-2 justify-between items-center",
                                            div {
                                                class: "flex flex-col w-full",
                                                TextField {
                                                    placeholder: "Search",
                                                    custom_class: "!border-primary",
                                                    on_change: move |event: String| {
                                                        search_value.set(event.clone());
                                                    }
                                                }
                                            }
                                            if props.is_multiple {
                                                    div {
                                                        class: "flex flex-col",
                                                        span {
                                                            class: "px-4 py-2 bg-primary-500 !text-white cursor-pointer rounded-[10px]",
                                                            onclick: move |_| show_select_modal.set(false),
                                                            "Close"
                                                        }
                                                    }
                                            }
                                        }
                                }

                                // Conditionally render single or multiselect options
                                if props.is_multiple {
                                        div {
                                            class: "w-full flex flex-col",
                                            for option in filtered_options.read().iter().cloned() {

                                                div {
                                                    key: "{option.key}",
                                                    class: "flex w-full flex-row space-x-2 items-center cursor-pointer py-3 border-b-[1px] border-gray-300 dark:border-grey-100 mb-[4px]",
                                                    onclick: move |_| {
                                                        props.on_change.call(option.key.clone());
                                                        props.on_option_selected.call(option.clone());
                                                    },
                                                    Icon {
                                                        custom_class: "h-[15px]",
                                                        name: if item_is_selected(&option.key) {"checked"} else {"not-checked"}
                                                    },
                                                    NormalText {
                                                        custom_class: "!font-semibold",
                                                        {option.alt_value.as_ref().unwrap_or(&option.value).clone()}
                                                    }
                                                }

                                            }
                                        }
                                } else {
                                        if !props.chip_like_options {
                                            Radio {
                                               options: search_result(),
                                               value: selected_key(),
                                               on_change: move |option: SelectOption| {
                                                   select_value(option);
                                               }
                                           }
                                        } else {
                                            div {
                                                class: "w-full flex flex-row justify-center items-center flex-wrap",
                                                 for (_index, option) in search_result().iter().cloned().enumerate() {
                                                     div {
                                                         key: _index,
                                                         class: "px-2 !py-3",
                                                         onclick: move |_| {
                                                             select_value(option.clone())
                                                         },
                                                         Badge {
                                                             color: if selected_key().unwrap_or("".to_string()) == option.clone().key { BadgeColor::Purple} else {BadgeColor::OutlinePurple},
                                                             {option.clone().value}
                                                         }
                                                     }
                                                 }
                                            }
                                        }
                                }

                                if search_result.read().is_empty() && props.has_search {
                                        div {
                                            class: "h-full w-full flex flex-row items-center justify-center",
                                            NormalText {
                                                custom_class: "text-center",
                                                color: "text-gray-400",
                                                {props.search_message}
                                            }
                                        }
                                }
                            }
                        }
                        // Right side
                        div {
                            class: "hidden col-span-3 md:!col-span-2 mdlg:!col-span-3 mdlg:!flex md:!flex flex-col sticky top-0"
                        }
                    }
                }
            }
    }
}

fn search_options(options: &[SelectOption], search_key: &str) -> Vec<SelectOption> {
    let search_key_lower = search_key.to_lowercase();

    options
        .iter()
        .filter(|opt| {
            opt.key.to_lowercase().contains(&search_key_lower)
                || opt.value.to_lowercase().contains(&search_key_lower)
                || opt
                    .alt_value
                    .as_ref()
                    .map(|v| v.to_lowercase().contains(&search_key_lower))
                    .unwrap_or(false)
                || opt
                    .extra_info
                    .as_ref()
                    .map(|v| v.to_lowercase().contains(&search_key_lower))
                    .unwrap_or(false)
        })
        .cloned()
        .collect()
}
