use dioxus::prelude::*;

use crate::components::{form::calendar::Calendar, icon::icon::Icon};

/// Represents the type of input field.
#[derive(Clone, PartialEq)]
pub enum FieldType {
    /// A standard text input field.
    Text,
    /// A password input field.
    Password,
    /// An email input field.
    Email,
    /// A telephone number input field.
    Tel,
    /// A number input field.
    Number,
    /// A date input field.
    Date,
}

/// Represents a validation rule that can be applied to a field.
#[derive(Clone, PartialEq)]
pub enum ValidationRule {
    /// Field is required and cannot be empty.
    IsRequired,
    /// Field value must be greater than the specified number.
    IsGreaterThan(usize),
    /// Field value must be less than the specified number.
    IsLessThan(usize),
    /// Field value must be equal to the specified number.
    IsEqualsTo(usize),
    /// Field value must be greater than or equal to the specified number.
    IsGreaterThanOrEqualsTo(usize),
    /// Field value must be less than or equal to the specified number.
    IsLessThanOrEqualsTo(usize),
    /// Field value must match the provided regex pattern.
    /// (pattern, error_message)
    IsRegex(String, String),
    /// Field value must satisfy the given condition.
    /// (condition, error_message)
    IsCondition(bool, String),
}

/// Represents a rule for formatting content within the input field.
#[derive(Clone, PartialEq)]
pub struct FormContentRule {
    /// The maximum length of the input.
    pub max: usize,
    /// Add a character after this number of characters.
    pub add_after_count: usize,
    /// The character to add.
    pub character_to_add: String,
}

/// Properties for the `TextField` component.
#[derive(Props, Clone, PartialEq)]
pub struct TextFieldProps {
    // Basic props
    /// The value of the input field.
    #[props(default = "".to_string())]
    pub model_value: String,

    /// The placeholder text for the input field.
    #[props(default = "".to_string())]
    pub placeholder: String,

    /// The name of the input field.
    #[props(default = "".to_string())]
    pub name: String,

    /// The type of the input field.
    #[props(default = FieldType::Text)]
    pub field_type: FieldType,

    // Styling props
    /// The padding of the input field.
    #[props(default = "py-3 px-3".to_string())]
    pub padding: String,

    /// The border color when the input is focused.
    #[props(default = "border-purple-500".to_string())]
    pub focus_border: String,

    /// Custom styling for the input.
    #[props(default = "".to_string())]
    pub input_style: String,

    /// Custom CSS class for the input.
    #[props(default = "".to_string())]
    pub custom_class: String,

    // Boolean flags
    /// Determines if the input is a textarea.
    #[props(default = false)]
    pub is_textarea: bool,

    /// Determines if the input is required.
    #[props(default = false)]
    pub required: bool,

    /// Determines if the input has a title.
    #[props(default = false)]
    pub has_title: bool,

    /// Determines if the input is disabled.
    #[props(default = false)]
    pub disabled: bool,

    /// Determines if the input is formatted.
    #[props(default = false)]
    pub is_formatted: bool,

    /// Determines if back dates are prevented on datepickers.
    #[props(default = false)]
    pub prevent_back_date: bool,

    // Validation and rules
    /// The list of validation rules to apply to the input.
    #[props(default = vec![])]
    pub rules: Vec<ValidationRule>,

    /// The rule for formatting content.
    #[props(optional)]
    pub content_rule: Option<FormContentRule>,

    // Limits and sizing
    /// The maximum number of characters allowed in the input.
    #[props(default = 0)]
    pub max_character: usize,

    /// The number of rows for the textarea.
    #[props(default = 5)]
    pub textarea_rows: usize,

    // Date specific
    /// The minimum date allowed for date input.
    #[props(default = "".to_string())]
    pub minimum_date: String,

    // Event handlers
    /// Callback for when the input value changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,

    /// Callback for when the input is focused.
    #[props(optional)]
    pub on_focus: Option<EventHandler<()>>,

    /// Callback for when the input loses focus.
    #[props(optional)]
    pub on_blur: Option<EventHandler<()>>,

    /// Callback for when a key is pressed in the input.
    #[props(optional)]
    pub on_key_pressed: Option<EventHandler<String>>,

    /// Callback for custom actions.
    #[props(optional)]
    pub action: Option<EventHandler<()>>,

    // Slots
    /// Slot for content to be rendered before the input element, outside the containing div.
    pub outer_prefix: Option<Element>,

    /// Slot for content to be rendered before the input element, inside the containing div.
    pub inner_prefix: Option<Element>,

    /// Slot for content to be rendered after the input element, outside the containing div.
    pub outer_suffix: Option<Element>,

    /// Slot for content to be rendered after the input element, inside the containing div.
    pub inner_suffix: Option<Element>,
}

/// A text input component with customizable validation and formatting.
#[component]
pub fn TextField(props: TextFieldProps) -> Element {
    let mut content = use_signal(|| props.model_value.clone());
    let mut field_type = use_signal(|| props.field_type.clone());
    let mut is_focused = use_signal(|| false);
    let mut show_calendar_modal = use_signal(|| false);
    let mut validation_status = use_signal(|| true);
    let mut error_message = use_signal(|| String::new());
    let _content_character_stop = use_signal(|| 0);

    // Update content when model_value changes
    use_effect(move || {
        if props.model_value != *content.read() {
            content.set(props.model_value.clone());
        }
    });

    // Validation functions
    use_effect(move || {
        let current_content = content.read().clone();
        let mut all_valid = true;
        let mut first_error = String::new();

        if props.rules.is_empty() {
            validation_status.set(true);
            error_message.set(String::new());
            return ();
        }

        for rule in &props.rules {
            let (is_valid, err_msg) = match rule {
                ValidationRule::IsRequired => {
                    if current_content.is_empty() {
                        (false, format!("{} is required", props.name))
                    } else {
                        (true, String::new())
                    }
                }
                ValidationRule::IsGreaterThan(count) => {
                    if current_content.len() > *count {
                        (true, String::new())
                    } else {
                        (
                            false,
                            format!("{} must be more than {} characters", props.name, count),
                        )
                    }
                }
                ValidationRule::IsLessThan(count) => {
                    if current_content.len() < *count {
                        (true, String::new())
                    } else {
                        (
                            false,
                            format!("{} must be less than {} characters", props.name, count),
                        )
                    }
                }
                ValidationRule::IsEqualsTo(count) => {
                    if current_content.len() == *count {
                        (true, String::new())
                    } else {
                        (
                            false,
                            format!("{} must be {} characters", props.name, count),
                        )
                    }
                }
                ValidationRule::IsGreaterThanOrEqualsTo(count) => {
                    if current_content.len() >= *count {
                        (true, String::new())
                    } else {
                        (
                            false,
                            format!("{} must be more than {} characters", props.name, count - 1),
                        )
                    }
                }
                ValidationRule::IsLessThanOrEqualsTo(count) => {
                    if current_content.len() <= *count {
                        (true, String::new())
                    } else {
                        (
                            false,
                            format!("{} must be less than {} characters", props.name, count + 1),
                        )
                    }
                }
                ValidationRule::IsRegex(pattern, err_msg) => {
                    let re = regex::Regex::new(pattern).expect("Invalid regex pattern");
                    if re.is_match(&current_content) {
                        (true, String::new())
                    } else {
                        (false, err_msg.clone())
                    }
                }
                ValidationRule::IsCondition(condition, err_msg) => {
                    if *condition {
                        (true, String::new())
                    } else {
                        (false, err_msg.clone())
                    }
                }
            };

            if !is_valid && all_valid {
                all_valid = false;
                first_error = err_msg;
            }
        }

        validation_status.set(all_valid);
        error_message.set(first_error);
    });

    // Handle input change
    let handle_input_change = move |evt: Event<FormData>| {
        let value = evt.value();
        content.set(value.clone());

        if let Some(on_change) = &props.on_change {
            on_change.call(value.clone());
        }

        if let Some(on_key_pressed) = &props.on_key_pressed {
            on_key_pressed.call(value);
        }
    };

    // Handle focus
    let handle_focus = move |_evt: Event<FocusData>| {
        is_focused.set(true);
        if let Some(on_focus) = &props.on_focus {
            on_focus.call(());
        }
    };

    // Handle blur
    let handle_blur = move |_evt: Event<FocusData>| {
        is_focused.set(false);
        if let Some(on_blur) = &props.on_blur {
            on_blur.call(());
        }
    };

    // Toggle password visibility
    let toggle_password = move |_| {
        let current_type = field_type.read().clone();
        match current_type {
            FieldType::Password => field_type.set(FieldType::Text),
            FieldType::Text if field_type.read().clone() == FieldType::Password => {
                field_type.set(FieldType::Password)
            }
            _ => {}
        }
    };

    // Handle click actions
    let handle_click = move |_| {
        if let Some(action) = &props.action {
            action.call(());
        } else if matches!(field_type.read().clone(), FieldType::Date) {
            show_calendar_modal.set(true);
        }
    };

    let handle_close_calendar = move |_| {
        show_calendar_modal.set(false);
    };

    // Get input type string
    let input_type = match *field_type.read() {
        FieldType::Text => "text",
        FieldType::Password => "password",
        FieldType::Email => "email",
        FieldType::Tel => "tel",
        FieldType::Number => "number",
        FieldType::Date => "text",
    };

    let focus_class = if *is_focused.read() {
        format!("{} border-[1px]", props.focus_border)
    } else {
        String::new()
    };

    rsx! {
        div {
            class: "flex w-full flex-col relative",

            div {
                class: "w-full flex flex-row items-center",
                tabindex: "0",
                onfocusin: handle_focus,
                onfocusout: handle_blur,
                onclick: handle_click,

                // Outer prefix slot would go here
                {props.outer_prefix.unwrap_or_else(|| rsx!{})}

                div {
                    class: "flew-grow w-full space-x-2 flex-row flex items-center focus:border-primary-400 focus:dark:border-primary-200 justify-between rounded-[8px] border-[1px] bg-white border-[#AD8EE9] dark:!bg-grey-900 relative {props.padding} {props.custom_class} {focus_class}",
                    onclick: handle_click,

                    // Title
                    if props.has_title {
                        span {
                            class: "!absolute left-4 top-[-23%] px-1 py-[2px] bg-white !text-light-text z-10 !text-[12px]",
                            // Title content would be passed as children
                        }
                    }

                    // Inner prefix slot would go here
                     {props.inner_prefix.unwrap_or_else(|| rsx!{})}

                    // Date field display
                    if matches!(*field_type.read(), FieldType::Date) {
                        span {
                            class: "text-left {props.input_style}",
                            if content.read().is_empty() {
                                span {
                                    class: "text-grey-400 {props.input_style}",
                                    "{props.placeholder}"
                                }
                            } else {
                                "{content.read()}"
                            }
                        }
                    } else if props.is_textarea {
                        // Textarea
                        textarea {
                            class: "text-[#7743DB] dark:!text-white flex-grow bg-transparent placeholder-grey-400 dark:placeholder-grey-200 focus input w-full focus:outline-none {props.input_style}",
                            placeholder: "{props.placeholder}",
                            rows: "{props.textarea_rows}",
                            disabled: props.disabled,
                            value: "{content.read()}",
                            oninput: handle_input_change,
                            onfocusin: handle_focus,
                            onfocusout: handle_blur,
                        }
                    } else {
                        // Regular input
                        input {
                            class: "text-[#7743DB] dark:!text-white flex-grow bg-transparent placeholder-grey-400 dark:!placeholder-grey-200 focus input w-full dark:disabled:!placeholder-white !pt-[2px] focus:outline-none {props.input_style}",
                            r#type: "{input_type}",
                            placeholder: "{props.placeholder}",
                            disabled: props.disabled || matches!(*field_type.read(), FieldType::Date),
                            value: "{content.read()}",
                            oninput: handle_input_change,
                            onfocusin: handle_focus,
                            onfocusout: handle_blur,
                            onclick: handle_click,
                        }
                    }

                    // Inner suffix slot would go here
                    {props.inner_suffix.unwrap_or_else(|| rsx!{})}

                    // Password toggle icon
                    if props.field_type == FieldType::Password {
                        Icon {
                            name:
                                if *field_type.read() == FieldType::Password { "eye" } else { "eye-off" }
                            ,
                            custom_class: "h-[18px] cursor-pointer",
                            onclick: toggle_password,
                        }
                    }
                }

                // Outer suffix slot would go here
                {props.outer_suffix.unwrap_or_else(|| rsx!{})}
            }

            // Validation and character count
            if !*validation_status.read() || props.max_character > 0 {
                div {
                    class: "w-full flex flex-row pt-1 justify-between items-center",

                    span {
                        class: "!text-error-500 dark:!text-error-400",
                        class: if *validation_status.read() { "invisible" } else { "" },
                        "{error_message.read()}"
                    }

                    if props.max_character > 0 {
                        span {
                            class: "!text-[12px] text-gray-600",
                            "{content.read().len()}/{props.max_character}"
                        }
                    }
                }
            }

            // Calendar Modal (simplified - you'd need a proper calendar component)
            if *show_calendar_modal.read() {
                div {
                    class: "fixed top-0 left-0 z-[99999999999999999] bg-black !bg-opacity-30 dark:!bg-opacity-50 flex w-full h-full flex-row items-end justify-end mdlg:!items-center mdlg:!justify-center md:!justify-center md:!items-center",
                    onclick: move |_| show_calendar_modal.set(false),

                    div {
                        class: "w-full mdlg:!w-[60%] md:!w-[80%] grid grid-cols-12 h-full relative",

                        div { class: "hidden col-span-3 md:!col-span-2 mdlg:!col-span-3 mdlg:!flex md:!flex flex-col sticky top-0" }

                        div {
                            class: "col-span-12 mdlg:!col-span-6 md:!col-span-8 relative h-full flex flex-col items-end justify-end mdlg:!items-center mdlg:!justify-center md:!justify-center md:!items-center",

                            div {
                                class: "rounded-t-2xl mdlg:!rounded-[10px] md:!rounded-[10px] flex flex-col space-y-2 bg-white dark:!bg-black dark:border-[1px] dark:border-gray-100 w-full absolute mdlg:!relative md:!relative overflow-y-auto h-auto max-h-auto bottom-0 left-0 pb-3 px-3 mdlg:!pb-4 md:!pb-4 lg:!text-sm mdlg:!text-[12px] text-xs",
                                onclick: move |e| e.stop_propagation(),

                                div {
                                    class: "flex items-center justify-center sticky top-0 bg-white dark:!bg-black w-full pt-3",
                                    span { class: "bg-gray-500 dark:bg-gray-200 rounded-full w-[30px] h-[4px]" }
                                }

                                div {
                                    class: "flex items-center justify-center sticky top-0 flex-col bg-white w-full",
                                    span {
                                        class: "!text-xs font-semibold w-full text-left py-2",
                                        "{props.placeholder}"
                                    }
                                }

                                // Calendar component would go here
                                Calendar {
                                    close_modal: handle_close_calendar,
                                    default_date: Some(content()),
                                    prevent_back_date: props.prevent_back_date,
                                    minimum_date: props.minimum_date
                                }
                            }
                        }

                        div { class: "hidden col-span-3 md:!col-span-2 mdlg:!col-span-3 mdlg:!flex md:!flex flex-col sticky top-0" }
                    }
                }
            }
        }
    }
}

/// Formats a string as money.
///
/// This is a placeholder function and should be replaced with actual money formatting logic.
pub fn format_money(value: &str) -> String {
    // Placeholder for money formatting logic
    value.to_string()
}

/// Checks if a key code corresponds to a number key.
pub fn is_number_key(key_code: u32) -> bool {
    (key_code >= 48 && key_code <= 57)
        || key_code == 46
        || key_code == 8
        || key_code == 37
        || key_code == 39
}
