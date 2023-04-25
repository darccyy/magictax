#[cfg(test)]
mod tests;

use std::{error::Error, fmt::Display, fs, io};

use cocoon::Cocoon;

use crate::csv::{self, Csv};

type FileResult<T> = Result<T, FileError>;

#[derive(Debug)]
pub enum FileError {
    Crypto(cocoon::Error),
    CsvParse(csv::ParseError),
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use cocoon::Error::*;

        match self {
            FileError::CsvParse(error) => write!(f, "Failed to parse csv: {error}"),

            FileError::Crypto(error) => write!(
                f,
                "{}",
                match error {
                    Cryptography => {
                        "Invalid password for file. This file is not accessible with this program"
                    }

                    UnrecognizedFormat => "Unrecognized file type or format",

                    TooLarge => {
                        "File too large to decrypt. This most likely means it was not encrypted properly"
                    }

                    TooShort => {
                        "File too short to decrypt. This most likely means it was not encrypted properly"
                    }

                    Io(error) => match error.kind() {
                        io::ErrorKind::InvalidData => {
                            "Invalid data. This most likely means it was not encrypted properly"
                        }

                        io::ErrorKind::PermissionDenied => "Permission denied",

                        // ... more IO errors can be handled here
                        _ => "Unknown file error! Please try again",
                    },
                }
            ),
        }
    }
}

impl Error for FileError {}

/// Simple file handler API
#[derive(Clone, Default)]
pub struct File {
    /// Path to file
    ///
    /// `None` if file is not registered on file system (was never saved)
    path: Option<String>,
    /// Contents of file
    contents: Csv,
    /// Whether file is saved
    saved: bool,
}

impl File {
    /// Returns `true` if file does not have an associated filepath (was never saved)
    fn is_registered(&self) -> bool {
        self.path.is_some()
    }

    /// Returns `true` if file is registered and saved
    pub fn is_registered_and_saved(&self) -> bool {
        self.is_registered() && self.saved
    }

    /// Returns `true` if file is unregistered and unchanged (empty)
    pub fn is_unregistered_and_unchanged(&self) -> bool {
        !self.is_registered() && self.contents.rows.is_empty()
    }

    /// Returns `true` if:
    ///  - File is registered, and NOT saved
    ///  - File is not registered, and NOT empty
    pub fn is_changed(&self) -> bool {
        if self.is_registered() {
            !self.saved
        } else {
            !self.contents.rows.is_empty()
        }
    }

    /// Get file contents as reference
    pub fn contents(&self) -> &Csv {
        &self.contents
    }

    /// Get file contents as mutable reference
    pub fn contents_mut(&mut self) -> &mut Csv {
        &mut self.contents
    }

    /// Set save state to unsaved
    pub fn mark_as_unsaved(&mut self) {
        if self.is_registered_and_saved() {
            self.saved = false;
        }
    }

    /// Set save state to unsaved
    ///
    /// This should only be run after saving with `save_to_path`,
    ///     which did not register as saved
    pub fn force_set_saved(&mut self) {
        self.saved = true;
    }

    /// Get filepath as reference
    ///
    /// `None` if file is not registered on file system (was never saved)
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Set filepath
    pub fn set_path(&mut self, path: impl Into<String>) {
        self.path = Some(path.into())
    }

    /// Save encrypted file to given path
    ///
    /// Sets save state to saved
    pub fn save_to_path_encrypted(&mut self, path: &str, key: &str) -> FileResult<()> {
        // Create encryptor
        let cocoon = Cocoon::new(key.as_bytes());

        // Get content as bytes
        let bytes = self.contents.encode().into_bytes().to_vec();

        // Open file (creates new if not already existing)
        let mut file = fs::File::create(path).map_err(|error| {
            // Return an IO error if failed
            FileError::Crypto(cocoon::Error::Io(error))
        })?;

        // Write encrypted data to file
        cocoon
            .dump(bytes, &mut file)
            .map_err(FileError::Crypto)?;

        self.saved = true;
        Ok(())
    }

    /// Open encrypted file from given path
    ///
    /// Returns saved `File` with contents and associated path
    pub fn open_path_and_decrypt(path: impl Into<String>, key: &str) -> FileResult<Self> {
        let path = path.into();

        // Create decryptor
        let cocoon = Cocoon::new(key.as_bytes());

        // Open existing file
        let mut file = fs::File::open(&path).map_err(|error| {
            // Return an IO error if failed
            FileError::Crypto(cocoon::Error::Io(error))
        })?;

        // Decrypt data (bytes) from file
        let bytes = cocoon
            .parse(&mut file)
            .map_err( FileError::Crypto)?;

        // Convert bytes to string
        // This may fail, if bytes do not form a valid utf8 string
        let contents = String::from_utf8(bytes).map_err(|error| {
            // Bytes-to-string conversion failed
            // Return IO error of 'Invalid Data'
            FileError::Crypto(cocoon::Error::from(io::Error::new(
                io::ErrorKind::InvalidData,
                error,
            )))
        })?;

        // Parse contents from CSV format
        let contents = Csv::decode(&contents).map_err(FileError::CsvParse)?;

        Ok(Self {
            contents,
            path: Some(path),
            saved: true,
        })
    }
}
