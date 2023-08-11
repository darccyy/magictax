use magictax::App;
use magictax::CsvRow;

#[test]
#[ignore]
fn app_io_works() {
    let ctx = eframe::egui::Context::default();

    let mut app = App::default();

    section("New file and save as");

    app.file_new();

    let contents = app.file.contents_mut();
    contents.rows.push(CsvRow {
        label: "foo".to_string(),
        value: 69.0,
    });
    contents.rows.push(CsvRow {
        label: "bar".to_string(),
        value: 420.0,
    });

    pause("save file");
    app.file_save_as(&ctx);

    drop(app);

    section("Open existing file");

    let mut app = App::default();

    pause("open file that you just saved");
    app.file_open();

    let contents = app.file.contents();
    let mut rows = contents.rows.clone().into_iter();
    assert_eq!(
        rows.next().unwrap(),
        CsvRow {
            label: "foo".to_string(),
            value: 69.0
        }
    );
    assert_eq!(
        rows.next().unwrap(),
        CsvRow {
            label: "bar".to_string(),
            value: 420.0
        }
    );
    assert!(rows.next().is_none());

    pause("export (print) the file to html");
    app.file_export_html();

    println!("All good!");
}

/// Print title for section in test
fn section(title: &str) {
    println!();
    println!("====================");
    println!("{}", title);
    println!("====================");
}

/// Pause until stdin read
fn pause(action: &str) {
    // stderr so it shows before stdin read
    eprintln!();
    eprintln!("----------------");
    eprintln!(":: Manual action to {}", action);
    eprint!(":: [Press Enter] ");
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    println!()
}
