/// Custom implementations for `App`
mod methods;
/// Render `App` with `eframe::App` implementation
mod render;

use std::sync::{Arc, Mutex};

use crate::{Attempt, Channel, File};

/// Possible messages between threads
enum ConcurrentMessage {
    /// Save has succeeded
    FinishConcurrentSave,
}

/// Actions to allow after close attempt passes
enum CloseFileAction {
    NewFile,
    OpenFile,
    CloseWindow,
}

/// Main app state
#[derive(Default)]
pub struct App {
    /// Current file opened
    pub file: File,

    /// Whether file is currently writing
    writing: Arc<Mutex<bool>>,

    /// Attempt to close file (See `Attempt`)
    attempting_file_close: Attempt<CloseFileAction>,

    /// Whether program window should be closed on next frame render
    close_window_on_next_frame: bool,

    /// Row to focus on next frame
    ///
    /// Index of row, and kind of element in row
    ///
    /// `None` if no row needs to gain focus
    focus_row_on_next_frame: Option<(usize, RowElement)>,

    /// Whether to focus new element on next frame (such as dialog window button)
    focus_new_element_on_next_frame: bool,

    /// Send messages between threads
    channel: Channel<ConcurrentMessage>,

    /// Display any error message
    error_message: Arc<Mutex<Option<String>>>,
}

#[derive(Clone, Copy, PartialEq)]
enum RowElement {
    Value,
    Label,
    InsertButton,
    RemoveButton,
}

impl RowElement {
    pub fn previous(&self) -> Self {
        use RowElement::*;

        match self {
            Value => Value,
            Label => Value,
            InsertButton => Label,
            RemoveButton => InsertButton,
        }
    }

    pub fn next(&self) -> Self {
        use RowElement::*;

        match self {
            Value => Label,
            Label => InsertButton,
            InsertButton => RemoveButton,
            RemoveButton => RemoveButton,
        }
    }
}
