use dioxus::desktop::{use_window, LogicalPosition, LogicalSize};

pub fn configure_window() {
    let current_window = use_window();

    if let Some(primary_monitor) = current_window.current_monitor() {
        let screen_size = primary_monitor.size();
        let scale_factor = primary_monitor.scale_factor();

        println!(
            "screen_size - {:?}, scale_factor - {:?}",
            screen_size, scale_factor
        );

        let logical_size = screen_size.to_logical::<f64>(scale_factor);

        let width = (logical_size.width as f64 * 0.8).round();
        let height = (logical_size.height as f64 * 0.8).round();
        let x = ((logical_size.width as f64 - width) / 2.0).round();
        let y = ((logical_size.height as f64 - height) / 2.0).round();

        current_window.set_inner_size(LogicalSize::new(width, height));
        current_window.set_outer_position(LogicalPosition::new(x, y));
    }
}
