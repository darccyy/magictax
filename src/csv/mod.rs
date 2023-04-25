#[cfg(test)]
mod tests;

use std::fmt::Display;

/// Data parsed from CSV file
///
///todo: Rename
#[derive(Debug, PartialEq)]
pub struct Csv {
    pub rows: Vec<CsvRow>,
}

impl TryFrom<&str> for Csv {
    type Error = CsvParseError;

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
    #[allow(dead_code)]
    /// Returns struct with no rows
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    #[allow(dead_code)]
    /// Alias for `self.to_string()`
    pub fn encode(&self) -> String {
        self.to_string()
    }

    #[allow(dead_code)]
    /// Alias for `Csv::try_from(...)`
    pub fn decode<'a>(file: impl AsRef<&'a str>) -> Result<Self, CsvParseError> {
        let string: &str = file.as_ref();
        string.try_into()
    }
}

/// Row parsed from CSV file
#[derive(Debug, PartialEq)]
pub struct CsvRow {
    /// Descriptive label of entry
    pub label: String,
    /// Number value of entry
    pub value: f32,
}

impl TryFrom<&str> for CsvRow {
    type Error = CsvParseError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        // Split into cells, at each comma
        let mut cells = line.split(',');

        // Get label - First cell, or entire line if no first cell (impossible)
        let label = cells.next().unwrap_or(line).trim().to_string();

        // Get value as string - Second cell
        let Some(value) = cells.next() else {
            return Err(CsvParseError::MissingValue);
        };

        // Parse value as float
        let Ok(value) = value.trim().parse() else {
            return Err(CsvParseError::ValueParseError);
        };

        // Check there are no more cells
        if cells.next().is_some() {
            return Err(CsvParseError::TooManyCells);
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

/// Error parsing data from CSV file
#[derive(Debug, PartialEq)]
pub enum CsvParseError {
    /// No number value was given
    MissingValue,
    /// Value given is not a valid float
    ValueParseError,
    /// Too many cells in CSV row
    TooManyCells,
}
