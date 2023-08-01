#[cfg(test)]
mod tests;

use std::{error::Error, fmt::Display};

/// Error parsing data from CSV file
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// No number value was given
    MissingValue,
    /// Value given is not a valid float
    ValueNotNumber,
    /// Too many cells in CSV row
    TooManyCells,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingValue => write!(f, "Missing value"),
            Self::ValueNotNumber => write!(f, "Value is not a number"),
            Self::TooManyCells => write!(f, "Too many cells in row"),
        }
    }
}

impl Error for ParseError {}

/// Data parsed from CSV file
///
///todo: Rename
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Csv {
    pub rows: Vec<CsvRow>,
}

impl TryFrom<&str> for Csv {
    type Error = ParseError;

    fn try_from(file: &str) -> Result<Self, Self::Error> {
        let mut rows = Vec::new();

        for line in file.lines() {
            if line.trim().is_empty() {
                continue;
            }
            rows.push(line.try_into()?);
        }

        Ok(Self { rows })
    }
}

impl Display for Csv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            row.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Csv {
    /// Alias for `self.to_string()`
    pub fn encode(&self) -> String {
        self.to_string()
    }

    /// Alias for `Csv::try_from(string)` or `string.try_into()`
    pub fn decode(file: &str) -> Result<Self, ParseError> {
        file.try_into()
    }

    /// Get total of all values added
    pub fn sum(&self) -> f32 {
        let sum: f32 = self.rows.iter().map(|row| row.value).sum();
        (sum * 100.0).round() / 100.0
    }

    /// Get total of all values added
    pub fn count(&self) -> usize {
        self.rows.len()
    }
}

/// Row parsed from CSV file
#[derive(Debug, Clone, PartialEq)]
pub struct CsvRow {
    /// Descriptive label of entry
    pub label: String,
    /// Number value of entry
    pub value: f32,
}

// Manual implementation of serialize
// impl Serialize for CsvRow {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("CsvRow", 2)?;
//         // This must be passed as a string, to not mess up float decimals in json conversion
//         state.serialize_field("value", &round_to_string(self.value))?;
//         state.serialize_field("label", &self.label)?;
//         state.end()
//     }
// }

impl Default for CsvRow {
    fn default() -> Self {
        Self {
            label: String::new(),
            value: 0.0,
        }
    }
}

impl TryFrom<&str> for CsvRow {
    type Error = ParseError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        // Split into cells, at each comma
        let mut cells = line.split(',');

        // Get label - First cell, or entire line if no first cell (impossible)
        let label = cells.next().unwrap_or(line).trim().to_string();

        // Get value as string - Second cell
        let Some(value) = cells.next() else {
            return Err(ParseError::MissingValue);
        };

        // Parse value as float
        let Ok(value) = value.trim().parse() else {
            return Err(ParseError::ValueNotNumber);
        };

        // Check there are no more cells
        if cells.next().is_some() {
            return Err(ParseError::TooManyCells);
        }

        Ok(Self { label, value })
    }
}

impl Display for CsvRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { label, value } = self;
        // Return string of label and value, separated with a comma
        write!(f, "{label},{value}")
    }
}
