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
#[derive(Debug, Clone, PartialEq)]
pub struct Csv {
    pub rows: Vec<CsvRow>,
}

impl Default for Csv {
    fn default() -> Self {
        Self { rows: Vec::new() }
    }
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
            write!(f, "\n")?;
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
}

/// Row parsed from CSV file
#[derive(Debug, Clone, PartialEq)]
pub struct CsvRow {
    /// Descriptive label of entry
    pub label: String,
    /// Number value of entry
    pub value: f32,
}

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
