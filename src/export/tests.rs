use super::*;
use crate::csv::CsvRow;

#[test]
fn json_convert_works() {
    let csv = Csv {
        rows: vec![
            CsvRow {
                label: "income example".to_owned(),
                value: 100.0,
            },
            CsvRow {
                label: "expense example".to_owned(),
                value: -100.0,
            },
            CsvRow {
                label: "zero example".to_owned(),
                value: 0.0,
            },
            CsvRow {
                label: "".to_owned(), // no label, without value
                value: 0.0,
            },
            CsvRow {
                label: "".to_owned(), // no label, with value
                value: 50.0,
            },
        ],
    };

    let rows = csv_report(&csv);

    for (i, row) in rows.into_iter().enumerate() {
        match i {
            0 => assert_eq!(
                row,
                ReportRow {
                    name: Some("income example".to_owned()),
                    income: Some("100".to_owned()),
                    expense: None,
                }
            ),
            1 => assert_eq!(
                row,
                ReportRow {
                    name: Some("expense example".to_owned()),
                    income: None,
                    expense: Some("100".to_owned()),
                }
            ),
            2 => assert_eq!(
                row,
                ReportRow {
                    name: Some("zero example".to_owned()),
                    income: None,
                    expense: None,
                }
            ),
            3 => assert_eq!(
                row,
                ReportRow {
                    name: None,
                    income: Some("50".to_owned()),
                    expense: None,
                }
            ),
            _ => panic!("row {i} should not exist"),
        }
    }
}
