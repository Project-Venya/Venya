use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    /// The color of the badge.
    color: BadgeColor,

    /// Optional custom CSS classes to apply.
    #[props(default = "".to_string())]
    custom_class: String,

    /// The content of the badge.
    children: Element,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BadgeColor {
    Green,
    Red,
    Yellow,
    Blue,
    Purple,
    OutlineGreen,
    OutlineRed,
    OutlineYellow,
    OutlineBlue,
    OutlinePurple,
    OutlineWhite,
    OutlineBlack,
}

impl std::fmt::Display for BadgeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeColor::Green => write!(f, "green"),
            BadgeColor::Red => write!(f, "red"),
            BadgeColor::Yellow => write!(f, "yellow"),
            BadgeColor::Blue => write!(f, "blue"),
            BadgeColor::Purple => write!(f, "purple"),
            BadgeColor::OutlineGreen => write!(f, "outline-green"),
            BadgeColor::OutlineRed => write!(f, "outline-red"),
            BadgeColor::OutlineYellow => write!(f, "outline-yellow"),
            BadgeColor::OutlineBlue => write!(f, "outline-blue"),
            BadgeColor::OutlinePurple => write!(f, "outline-purple"),
            BadgeColor::OutlineWhite => write!(f, "outline-white"),
            BadgeColor::OutlineBlack => write!(f, "outline-black"),
        }
    }
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let color_classes = match props.color {
        BadgeColor::Green => {
            "bg-success-50 dark:!bg-success-main dark:!text-success-50  border-success-50 text-success-500"
        }
        BadgeColor::Red => {
            "bg-error-50   border-error-50 text-error-500 dark:!bg-error-main dark:!text-error-50"
        }
        BadgeColor::Purple => {
            "bg-primary-50   border-primary-50 text-primary-500 dark:!bg-primary-main dark:!text-primary-50"
        }
        BadgeColor::Blue => {
            "!bg-blue-50   border-blue-50 !text-blue-500 dark:!bg-blue-main dark:!text-blue-50"
        }
        BadgeColor::Yellow => {
            "bg-secondary-50   border-secondary-50 text-secondary-500 dark:!bg-secondary-main dark:!text-secondary-50"
        }
        BadgeColor::OutlineGreen => "bg-transparent border-success-500 text-success-500",
        BadgeColor::OutlineRed => "bg-transparent border-error-500 text-error-500",
        BadgeColor::OutlineYellow => "bg-transparent border-secondary-500 text-secondary-500",
        BadgeColor::OutlineBlue => "bg-transparent border-blue-500 text-blue-500",
        BadgeColor::OutlinePurple => "bg-transparent border-primary-500 text-primary-500",
        BadgeColor::OutlineWhite => "bg-transparent   border-white text-white",
        BadgeColor::OutlineBlack => "bg-transparent   border-grey-700 text-grey-700",
    };

    let class = format!(
        "px-3 py-[2px] border-[1px] text-[11px] xs:text-[8px] !pt-[4px] whitespace-nowrap rounded-[28px] {} {}",
        color_classes, props.custom_class
    );

    rsx! {
        span {
            class: "{class}",
            {props.children}
        }
    }
}
