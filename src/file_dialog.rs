use std::path::PathBuf;

/// Create simple file open/save dialog with `rfd`, for mgx files
pub fn mgx() -> rfd::FileDialog {
    any_filetype()
        .add_filter("MagicTax Encrypted File", &["mgx"])
        .set_file_name("report.mgx")
}

/// Create simple file open/save dialog with `rfd`, for html files
pub fn html() -> rfd::FileDialog {
    any_filetype()
        .add_filter("HTML", &["html"])
        .set_file_name("magictax-report.html")
}

/// Get default directory to open file open/save dialogs in
fn get_start_dir() -> Option<PathBuf> {
    if let Some(dir) = dirs_next::document_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::desktop_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::home_dir() {
        return Some(dir);
    }
    None
}

/// Create simple file open/save dialog with `rfd`, without filter or filename
fn any_filetype() -> rfd::FileDialog {
    let dialog = rfd::FileDialog::new();
    if let Some(dir) = get_start_dir() {
        dialog.set_directory(dir)
    } else {
        dialog
    }
}
