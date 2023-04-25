use super::*;

#[test]
fn parse_should_work() {
    let valid_file = "\
    foo bar,123.5
    something,0
    ,-1.0
    ";

    let csv: Csv = valid_file.try_into().expect("Should be valid");

    assert_eq!(
        csv,
        Csv {
            rows: vec![
                CsvRow {
                    label: "foo bar".to_string(),
                    value: 123.5,
                },
                CsvRow {
                    label: "something".to_string(),
                    value: 0.0,
                },
                CsvRow {
                    label: "".to_string(),
                    value: -1.0,
                }
            ]
        }
    );
}

#[test]
fn parse_should_fail() {
    let result: Result<CsvRow, _> = "".try_into();
    let error = result.expect_err("Should be invalid due to no label");
    assert_eq!(error, CsvParseError::MissingValue);

    let result: Result<CsvRow, _> = "no value,".try_into();
    let error = result.expect_err("Should be invalid due to no value");
    assert_eq!(error, CsvParseError::MissingValue);

    let result: Result<CsvRow, _> = "value not a number,foo".try_into();
    let error = result.expect_err("Should be invalid due to value not a number");
    assert_eq!(error, CsvParseError::ValueParseError);

    let result: Result<CsvRow, _> = "too many cells,123,unexpected cell".try_into();
    let error = result.expect_err("Should be invalid due to too many cells");
    assert_eq!(error, CsvParseError::TooManyCells);
}

#[test]
fn display_works() {
    let row = CsvRow {
        label: "foo bar".to_string(),
        value: 123.5,
    }
    .to_string();
    assert_eq!(row, "foo bar,123.5");

    let row = CsvRow {
        label: "something".to_string(),
        value: 0.0,
    }
    .to_string();
    assert_eq!(row, "something,0");

    let row = CsvRow {
        label: "".to_string(),
        value: -1.0,
    }
    .to_string();
    assert_eq!(row, ",-1");

    let csv = Csv {
        rows: vec![
            CsvRow {
                label: "foo bar".to_string(),
                value: 123.5,
            },
            CsvRow {
                label: "something".to_string(),
                value: 0.0,
            },
            CsvRow {
                label: "".to_string(),
                value: -1.0,
            },
        ],
    };

    let file = csv.to_string();

    assert_eq!(file, "foo bar,123.5\nsomething,0\n,-1\n");
}
