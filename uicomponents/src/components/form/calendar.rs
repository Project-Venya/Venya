use chrono::{Datelike, Local, NaiveDate, Weekday};
use dioxus::prelude::*;

/// Represents the different view types for the calendar.
#[derive(Clone, PartialEq)]
pub enum ViewType {
    /// Month view, displaying days of the month.
    Month,
    /// Year view, displaying months of the year.
    Year,
    /// Years12 view, displaying a range of 12 years.
    Years12,
}

/// Properties for the `AppCalendar` component.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// Padding for the calendar. Defaults to "py-2 px-4".
    #[props(default = "py-2 px-4".to_string())]
    pub padding: String,

    /// An optional event handler to close the modal.
    #[props(optional)]
    pub close_modal: Option<EventHandler<()>>,

    /// An optional default date string in "YYYY-MM-DD" format.
    #[props(optional)]
    pub default_date: Option<String>,

    /// If true, prevents the selection of dates in the past relative to minimum_date or today.
    #[props(default = false)]
    pub prevent_back_date: bool,

    /// An optional minimum date string in "YYYY-MM-DD" format. Dates before this date will be disabled if prevent_back_date is true.
    #[props(optional)]
    pub minimum_date: Option<String>,

    /// An optional event handler triggered when the selected date changes.  The new date is passed as a string in "YYYY-MM-DD" format.
    #[props(optional)]
    pub on_date_change: Option<EventHandler<String>>,
}

/// A calendar component for selecting dates.
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let mut current_month = use_signal(|| String::new());
    let mut current_selected_day = use_signal(|| None::<NaiveDate>);
    let mut month_days_count = use_signal(|| 31);
    let mut view_type = use_signal(|| ViewType::Month);
    let mut current_selected_month = use_signal(|| String::new());
    let mut first_day_position = use_signal(|| 0);
    let mut today_date = use_signal(|| 0);
    let minimum_date = use_signal(|| props.minimum_date);
    let mut years_12 = use_signal(|| Vec::<i32>::new());

    // Date utility functions
    let get_minimum_date = move || -> NaiveDate {
        if let Some(min_date) = &*minimum_date.read() {
            NaiveDate::parse_from_str(min_date, "%Y-%m-%d")
                .unwrap_or_else(|_| Local::now().date_naive())
        } else {
            Local::now().date_naive()
        }
    };

    let disable_date = move |date: NaiveDate| -> bool {
        if !props.prevent_back_date {
            return false;
        }

        let min_date = get_minimum_date();
        date < min_date
    };

    // Constants
    let days_headers = ["Mo.", "Tu.", "We.", "Th.", "Fr.", "Sa.", "Su."];
    let months_options = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Helper functions
    let mut set_current_12_years = move |start_year: i32, forward: bool| {
        let mut year_list = Vec::new();

        if forward {
            for i in 0..12 {
                year_list.push(start_year + i);
            }
        } else {
            for i in 0..12 {
                year_list.push(start_year - i);
            }
            year_list.reverse();
        }

        years_12.set(year_list);
    };

    let mut set_calendar_date = move |date: NaiveDate| {
        match *view_type.read() {
            ViewType::Month => {
                current_month.set(format!(
                    "{} {}",
                    months_options[date.month() as usize - 1],
                    date.year()
                ));
            }
            ViewType::Year => {
                current_month.set(date.year().to_string());
            }
            ViewType::Years12 => {
                let years = years_12.read();
                if !years.is_empty() {
                    current_month.set(format!(
                        "{} - {}",
                        years.first().unwrap(),
                        years.last().unwrap()
                    ));
                }
            }
        }

        // Calculate days in month
        let days_in_month = match date.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if date.year() % 4 == 0 && (date.year() % 100 != 0 || date.year() % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        };
        month_days_count.set(days_in_month);

        current_selected_month.set(format!("{:04}-{:02}-01", date.year(), date.month()));

        if current_selected_day.read().is_none() {
            current_selected_day.set(Some(date));
        }

        // Calculate first day position
        let first_day = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap_or(date);
        let weekday = first_day.weekday();
        let position = match weekday {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        };
        first_day_position.set(position);

        // Set today date if current month
        let today = Local::now().date_naive();
        if date.year() == today.year() && date.month() == today.month() {
            today_date.set(today.day() as usize);
        } else {
            today_date.set(0);
        }
    };

    // Initialize calendar
    use_effect(move || {
        let initial_date = if let Some(default_date) = &props.default_date {
            NaiveDate::parse_from_str(default_date, "%Y-%m-%d")
                .unwrap_or_else(|_| Local::now().date_naive())
        } else {
            Local::now().date_naive()
        };

        set_calendar_date(initial_date);
        set_current_12_years(initial_date.year(), true);
    });

    // Event handlers
    let mut select_year_month = move |month_index: usize| {
        let current_year = if let Some(parts) = current_month.read().split_whitespace().last() {
            parts.parse::<i32>().unwrap_or(Local::now().year())
        } else {
            Local::now().year()
        };

        if let Some(selected_month) = NaiveDate::from_ymd_opt(current_year, month_index as u32, 1) {
            set_calendar_date(selected_month);
            current_selected_day.set(Some(selected_month));
            view_type.set(ViewType::Month);
        }
    };

    let mut select_year = move |year: i32| {
        current_month.set(year.to_string());
        view_type.set(ViewType::Year);
    };

    let date_is_selected = move |day: usize| -> bool {
        if let Some(selected_day) = *current_selected_day.read() {
            let current_month_data = current_month.read();
            let current_parts: Vec<&str> = current_month_data.split_whitespace().collect();
            if current_parts.len() >= 2 {
                if let Ok(year) = current_parts[1].parse::<i32>() {
                    let month_name = current_parts[0];
                    if let Some(month_index) = months_options.iter().position(|&m| m == month_name)
                    {
                        if let Some(current_date) =
                            NaiveDate::from_ymd_opt(year, month_index as u32 + 1, day as u32)
                        {
                            return current_date == selected_day;
                        }
                    }
                }
            }
        }
        false
    };

    let mut select_date = move |day: usize| {
        let current_month_data = current_month.read();
        let current_parts: Vec<&str> = current_month_data.split_whitespace().collect();
        if current_parts.len() >= 2 {
            if let Ok(year) = current_parts[1].parse::<i32>() {
                let month_name = current_parts[0];
                if let Some(month_index) = months_options.iter().position(|&m| m == month_name) {
                    if let Some(selected_date) =
                        NaiveDate::from_ymd_opt(year, month_index as u32 + 1, day as u32)
                    {
                        if !disable_date(selected_date) {
                            current_selected_day.set(Some(selected_date));

                            // Emit the date change
                            if let Some(on_date_change) = &props.on_date_change {
                                on_date_change.call(selected_date.format("%Y-%m-%d").to_string());
                            }

                            // Close modal if provided
                            if let Some(close_modal) = &props.close_modal {
                                close_modal.call(());
                            }
                        }
                    }
                }
            }
        }
    };

    let select_view = move |_| match view_type() {
        ViewType::Month | ViewType::Year => {
            view_type.set(ViewType::Years12);
            let years = years_12.read();
            if !years.is_empty() {
                current_month.set(format!(
                    "{} - {}",
                    years.first().unwrap(),
                    years.last().unwrap()
                ));
            }
        }
        ViewType::Years12 => {}
    };

    let mut go_to_page = move |direction: &str| match direction {
        "prev" => match *view_type.read() {
            ViewType::Month => {
                let current_month_data = current_month.read();
                let current_parts: Vec<&str> = current_month_data.split_whitespace().collect();
                if current_parts.len() >= 2 {
                    if let Ok(year) = current_parts[1].parse::<i32>() {
                        let month_name = current_parts[0];
                        if let Some(month_index) =
                            months_options.iter().position(|&m| m == month_name)
                        {
                            let (new_year, new_month) = if month_index == 0 {
                                (year - 1, 12)
                            } else {
                                (year, month_index)
                            };

                            if let Some(new_date) =
                                NaiveDate::from_ymd_opt(new_year, new_month as u32, 1)
                            {
                                set_calendar_date(new_date);
                            }
                        }
                    }
                }
            }
            ViewType::Year => {
                if let Ok(year) = current_month.read().parse::<i32>() {
                    if let Some(new_date) = NaiveDate::from_ymd_opt(year - 1, 1, 1) {
                        set_calendar_date(new_date);
                    }
                }
            }
            ViewType::Years12 => {
                let years = years_12.read();
                if let Some(first_year) = years.first() {
                    set_current_12_years(first_year - 1, false);
                    let new_years = years_12.read();
                    current_month.set(format!(
                        "{} - {}",
                        new_years.first().unwrap(),
                        new_years.last().unwrap()
                    ));
                }
            }
        },
        "next" => match *view_type.read() {
            ViewType::Month => {
                let current_month_data = current_month.read();
                let current_parts: Vec<&str> = current_month_data.split_whitespace().collect();
                if current_parts.len() >= 2 {
                    if let Ok(year) = current_parts[1].parse::<i32>() {
                        let month_name = current_parts[0];
                        if let Some(month_index) =
                            months_options.iter().position(|&m| m == month_name)
                        {
                            let (new_year, new_month) = if month_index == 11 {
                                (year + 1, 1)
                            } else {
                                (year, month_index + 2)
                            };

                            if let Some(new_date) =
                                NaiveDate::from_ymd_opt(new_year, new_month as u32, 1)
                            {
                                set_calendar_date(new_date);
                            }
                        }
                    }
                }
            }
            ViewType::Year => {
                if let Ok(year) = current_month.read().parse::<i32>() {
                    if let Some(new_date) = NaiveDate::from_ymd_opt(year + 1, 1, 1) {
                        set_calendar_date(new_date);
                    }
                }
            }
            ViewType::Years12 => {
                let years = years_12.read();
                if let Some(last_year) = years.last() {
                    set_current_12_years(last_year + 1, true);
                    let new_years = years_12.read();
                    current_month.set(format!(
                        "{} - {}",
                        new_years.first().unwrap(),
                        new_years.last().unwrap()
                    ));
                }
            }
        },
        _ => {}
    };

    rsx! {
        div {
            class: "w-full col-span-full flex flex-col space-y-2",

            // Header with navigation
            div {
                class: "w-full items-center flex flex-row justify-between px-2",

                // Previous button
                div {
                    class: "h-[38px] w-[38px] rounded-md flex items-center justify-center !bg-grey-50 dark:!bg-gray-900 cursor-pointer",
                    onclick: move |_| go_to_page("prev"),

                    // Left arrow icon (you can replace with actual icon component)
                    span { "←" }
                }

                // Current month/year display
                span {
                    class: "!font-semibold underline cursor-pointer",
                    onclick: select_view,
                    "{current_month.read()}"
                }

                // Next button
                div {
                    class: "h-[38px] w-[38px] rounded-md flex items-center justify-center bg-grey-50 dark:!bg-gray-900 cursor-pointer",
                    onclick: move |_| go_to_page("next"),

                    // Right arrow icon (you can replace with actual icon component)
                    span { "→" }
                }
            }

            // Years 12 view
            if matches!(*view_type.read(), ViewType::Years12) {
                div {
                    class: "w-full grid grid-cols-3 gap-3",

                    for year in years_12.read().iter().cloned() {
                        div {
                            key: "{year}",
                            class: "py-4 flex flex-row items-center col-span-1 justify-center rounded-md hover:bg-grey-100 dark:!hover:bg-gray-700 cursor-pointer",
                            onclick: move |_| select_year(year),

                            span { "{year}" }
                        }
                    }
                }
            }

            // Year view (months)
            if matches!(*view_type.read(), ViewType::Year) {
                div {
                    class: "w-full grid grid-cols-3 gap-3",

                    for (index, month) in months_options.iter().enumerate() {
                        div {
                            key: "{index}",
                            class: "py-4 flex flex-row items-center col-span-1 justify-center rounded-md hover:bg-grey-100 dark:!hover:bg-gray-700 cursor-pointer",
                            onclick: move |_| select_year_month(index + 1),

                            span { "{month}" }
                        }
                    }
                }
            }

            // Month view (calendar)
            if matches!(*view_type.read(), ViewType::Month) {
                // Day headers
                div {
                    class: "w-full flex flex-row items-center justify-center",

                    for day in days_headers.iter() {
                        span {
                            class: "!font-semibold flex items-center justify-center !text-grey-800 !text-[11px] w-[14.2857142857%] dark:!text-gray-200",
                            "{day}"
                        }
                    }
                }

                // Calendar grid
                div {
                    class: "w-full flex flex-row flex-wrap items-center justify-start",

                    // Empty cells for first day position
                    for i in 0..*first_day_position.read() {
                        div {
                            key: "empty-{i}",
                            class: "w-[14.2857142857%] flex flex-col items-center justify-center py-[6px] px-[6px]",
                        }
                    }

                    // Calendar days
                    for day in 1..=*month_days_count.read() {
                        {
                            let current_month_data = current_month.read();
                            let current_parts: Vec<&str> = current_month_data.split_whitespace().collect();
                            let is_disabled = if current_parts.len() >= 2 {
                                if let Ok(year) = current_parts[1].parse::<i32>() {
                                    let month_name = current_parts[0];
                                    if let Some(month_index) = months_options.iter().position(|&m| m == month_name) {
                                        if let Some(date) = NaiveDate::from_ymd_opt(year, month_index as u32 + 1, day as u32) {
                                            disable_date(date)
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            };

                            rsx! {
                                div {
                                    key: "day-{day}",
                                    class: "w-[14.2857142857%] flex flex-col items-center justify-center py-[6px] px-[6px]",
                                    class: if is_disabled && props.prevent_back_date { "opacity-50" } else { "" },
                                    onclick: move |_| {
                                        if !(is_disabled && props.prevent_back_date) {
                                            select_date(day);
                                        }
                                    },

                                    span {
                                        class: "w-[36px] h-[36px] rounded-md flex justify-center items-center cursor-pointer",
                                        class: if date_is_selected(day) {
                                            "bg-gray-800 dark:!bg-gray-200"
                                        } else {
                                            "bg-gray-100 dark:!bg-gray-700"
                                        },

                                        span {
                                            class: if date_is_selected(day) {
                                                "!text-white dark:!text-black"
                                            } else {
                                                "!text-gray-800 dark:!text-gray-200"
                                            },
                                            "{day}"
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
}
