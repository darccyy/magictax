/// Private macros
#[macro_use]
mod macros;
/// Main app
mod app;
/// 'Attempt' something, such as close a file
mod attempt;
/// Wrapper for `Sender` and `Receiver` types in `std::sync::mpsc`
mod channel;
/// Handle CSV format for unencrypted files
mod csv;
/// Export (print) file information to html
mod export;
/// Handle file input/output and save state
mod file;
/// Create simple file open/save dialog with `rfd`
mod file_dialog;

pub use crate::app::App;
use crate::{attempt::Attempt, channel::Channel, file::File};

#[macro_export]
macro_rules! print_info {
    ( $($tt:tt)* ) => {
        print!("[info] ");
        println!( $($tt)* );
    }
}

/// Cryption key which every file uses
///
/// This is not very secure, but at least the file cannot be opened by any program
const KEY: &str = "super-secure-encryption-key";

/// Set window scale
///
/// Affects window zoom, position, and size
pub const GLOBAL_WINDOW_SCALE: f32 = 0.6;

/// Round a float to 2 decimal places and convert to string
fn round_to_string(number: f32) -> String {
    let rounded = (number * 100.0).round() / 100.0;
    rounded.to_string()
}
