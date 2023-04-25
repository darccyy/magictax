use std::f32::INFINITY;

use eframe::{
    egui,
    emath::Align2,
};
use egui::Grid;

use crate::csv::CsvRow;

use super::{App, CloseFileAction, ConcurrentMessage};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Close window, if action was triggered in a previous frame
        //      (from a method that did not have `frame`)
        if self.close_window_on_next_frame {
            frame.close();
        }

        // * Handle concurrent messages

        if let Ok(msg) = self.channel.receiver.try_recv() {
            match msg {
                ConcurrentMessage::FinishConcurrentSave => {
                    println!("Save finished!");
                    self.file.force_set_saved();

                    if self.attempting_file_close.is_attempting() {
                        self.call_close_action();
                    }
                }
            }
        }

        // * Render main window

        // Whether the file is currently writing on a different thread
        let concurrently_writing = *self.writing.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Encrypted text editor");

            // * Top bar

            // File actions and status
            ui.horizontal(|ui| {
                /// Create new action, with button and keybind
                macro_rules! action_button_and_keybind {
                    (
                        // Title of button
                        $title: expr,
                        // Keybind
                        ($($keybind:tt)*),
                        // Condition for button and keybind to be enabled
                        if $condition: expr =>
                        // Action to run
                        $($action:tt)*
                    ) => {{
                        // Condition also requires that file is not writing on another thread
                        let condition = $condition && !concurrently_writing;

                        // Create button with title, that is only enabled if `condition` is true
                        let button = ui.add_enabled(condition, egui::Button::new($title));

                        // Create keybind, if condition is true
                        // See `keys!` macro
                        let keybind_active = (keys!(ui: $($keybind)*) && condition);

                        // If button is clicked, or keybind is active, run condition
                        if button.clicked() || keybind_active {
                            $($action)*
                        }
                    }};
                }

                // Add row at bottom
                if ui.button("+").clicked() {
                    self.file.contents_mut().rows.push(CsvRow::default());
                }

                ui.separator();

                // Create actions from macro
                action_button_and_keybind!( "Save", (CTRL + S), if !self.file.is_registered_and_saved() => {
                    self.file_save_or_save_as(ctx);
                });
                action_button_and_keybind!( "Save As", (CTRL + SHIFT + S), if true => {
                    self.file_save_as(ctx);
                });
                action_button_and_keybind!( "Open", (CTRL + O), if true => {
                    self.file_open();
                });
                action_button_and_keybind!( "New", (CTRL + N), if !self.file.is_unregistered_and_unchanged() => {
                    self.file_new();
                });
                
                // Show filepath if file is registered
                if let Some(path) = self.file.path() {
                    ui.monospace(path);
                }

                // Save state
                ui.label(if concurrently_writing {
                    // File is currently being written to
                    "Writing..."
                } else if self.file.is_registered_and_saved() {
                    // File is registered and saved
                    "Saved"
                } else if self.file.is_changed() {
                    // File has changed
                    "UNSAVED"
                } else {
                    // File is unregistered
                    ""
                });
            });

            // * Rows

            Grid::new("rows").num_columns(3).striped(true).show(ui, |ui|{
                for i in 0..self.file.contents().rows.len() {
                    /// Returns `true` if index is still in bounds
                    macro_rules! this_row_still_exists {
                        () => {
                            i < self.file.contents().rows.len()
                        };
                    }

                    // Break loop if index out of bounds
                    // Needed due to `.remove()` call inside loop
                    if !this_row_still_exists!() {
                        break;
                    }

                    /// Get mutable reference to this row
                    /// 
                    /// Returns from current function if index out of bounds
                    macro_rules! this_row {
                        () => {
                            match self.file.contents_mut().rows.get_mut(i) {
                                Some(row) => row,
                                None => return,
                            }
                        };
                    }

                    // Editable value
                    ui.horizontal(|ui|{
                        let value = &mut this_row!().value;

                        // Number value
                        let value_element = ui.add(
                            egui::DragValue::new(value)
                                .prefix("$")
                                .max_decimals(2)
                                .clamp_range(0.0..=INFINITY)
                                .speed(0.01),
                        );

                        // Mark as unsaved if label or number was changed
                        if value_element.changed() {
                            self.file.mark_as_unsaved();
                        }
                    });

                    // Editable label
                    ui.horizontal(|ui|{
                        let label = &mut this_row!().label;

                        let label_element = ui.text_edit_singleline(label);

                        // Mark as unsaved if label or number was changed
                        if label_element.changed() {
                            self.file.mark_as_unsaved();
                        }

                        ui.separator();
                    });

                    // Action buttons
                    if this_row_still_exists!() {
                        ui.horizontal(|ui| {
                            // New entry after this one
                            if ui.button("+").clicked() {
                                self.file.contents_mut().rows.insert(i + 1, CsvRow::default());
                                self.file.mark_as_unsaved();
                            }

                            // Remove this entry
                            if ui.button("-").clicked() {
                                self.file.contents_mut().rows.remove(i);
                                self.file.mark_as_unsaved();
                            }
                        });
                    }

                    // Keybinds

                    // Add new row
                    if keys!(ui: Enter) {
                        if this_row_still_exists!() {
                            self.file.contents_mut().rows.insert(i + 1, CsvRow::default());
                        } else {
                            self.file.contents_mut().rows.push(CsvRow::default());
                        }
                        self.file.mark_as_unsaved();
                    }

                    // Next row of grid
                    ui.end_row();
                }
            });
        });

        // * Render popup windows

        // Attempting to close file
        // Create custom window dialog if necessary
        if self.attempting_file_close.is_attempting() {
            if concurrently_writing {
                // Wait for file to finish writing
                // This cannot be overridden with a button,
                //      because it would only ever need to be closed while the file is writing
                //      if the program has frozen, and in that case it can be closed with task manager
                dialog_window("Waiting for file to save...").show(ctx, |ui| {
                    ui.label("File may corrupt if not saved properly.");
                });
            } else if !self.file.is_registered_and_saved() {
                // Closing unsaved file
                dialog_window("Do you want to save your changes?").show(ctx, |ui| {
                    ui.label("Your changes will be lost if you don't save them.");

                    // Actions
                    ui.horizontal(|ui| {
                        // Close file without saving
                        if ui.button("Don't save").clicked() {
                            // Override close condition
                            self.attempting_file_close.override_condition();

                            // Try action again
                            self.call_close_action();
                        }

                        // Cancel attempt, returning to current file
                        // Button and keybind
                        if ui.button("Cancel").clicked() || keys!(ui: Escape) {
                            // Stop attempting close file
                            self.reset_close_action();
                        }

                        // Save file and close
                        if ui.button("Save").clicked() {
                            // Save (concurrently)
                            // This will show 'wait for file to save' until save completes
                            self.file_save_or_save_as(ctx);

                            // Try action again
                            self.call_close_action();
                        }
                    });
                });
            }
        }

        // Error message popup
        if let Some(error_msg) = self.get_error_message() {
            dialog_window("Error").show(ctx, |ui| {
                ui.heading("An error occurred!");

                ui.label(error_msg);

                if ui.button("Ok").clicked() {
                    self.clear_error_message();
                }
            });
        }
    }

    // Program was closed
    // ALT+F4, Close button, ect.
    fn on_close_event(&mut self) -> bool {
        // Set file close action to quit app
        self.attempting_file_close
            .set_action(CloseFileAction::CloseWindow);
        // Returns true if file is allowed to close
        self.file_can_close()
    }
}

/// Create a simple reusable popup dialog window
fn dialog_window(title: &str) -> egui::Window {
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
}
