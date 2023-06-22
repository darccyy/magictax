#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use magictax::{App, GLOBAL_WINDOW_SCALE};

// Start egui/eframe app
fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        // Change window size and position
        min_window_size: Some(egui::vec2(600.0, 400.0) * GLOBAL_WINDOW_SCALE),
        initial_window_size: Some(egui::vec2(1000.0, 600.0) * GLOBAL_WINDOW_SCALE),
        initial_window_pos: Some(egui::pos2(
            200.0 * GLOBAL_WINDOW_SCALE,
            100.0 * GLOBAL_WINDOW_SCALE,
        )),
        // Get icon from image
        icon_data: eframe::IconData::try_from_png_bytes(include_bytes!("../icon.png")).ok(),
        ..Default::default()
    };

    eframe::run_native("MagicTax", options, Box::new(|_cc| Box::<App>::default()))
}
