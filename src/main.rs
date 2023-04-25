#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use sumtotal::App;

// Start egui/eframe app
fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Encrypted CSV editor",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}
