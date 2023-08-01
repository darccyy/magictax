#[cfg(test)]
mod tests;

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use chrono::Local;

use crate::{csv::Csv, round_to_string};

pub fn export_html(csv: &Csv) -> Result<String, handlebars::RenderError> {
    // Get templates from files
    let template = include_str!("template/index.hbs");
    let style = include_str!("template/style.css");

    // Create json object to pass to template
    let json = json!({
        "style": style,
        "table": csv_report(&csv),
        "total": round_to_string(csv.sum()),
        "date": get_today_date(),
    });

    // Create handlebars interface
    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(false);

    // Render template with handlebars
    let html = hbs.render_template(&template, &json)?;

    // Minify html
    Ok(minify(html))
}

/// Get today's date as a string
fn get_today_date() -> String {
     let today = Local::now();
    let date_string = today.format("%Y-%m-%d").to_string();
    date_string
}

/// Object passed into template, all values stringified
#[derive(Debug, PartialEq, Serialize)]
struct ReportRow {
    name: String,
    income: Option<String>,
    expense: Option<String>,
}

/// Convert csv rows to stringified values, for template
fn csv_report(csv: &Csv) -> Vec<ReportRow> {
    let mut report = Vec::new();

    for row in &csv.rows {
        let value = row.value;

        // Set income or expense, depending on sign of number value
        let (income, expense) = if value > 0.0 {
            (Some(round_to_string(value)), None)
        } else if value < 0.0 {
            (None, Some(round_to_string(0.0 - value)))
        } else {
            (None, None)
        };

        report.push(ReportRow {
            name: row.label.to_owned(),
            income,
            expense,
        })
    }

    report
}

/// Minify html document
fn minify(html: String) -> String {
    let config = minify_html::Cfg {
        do_not_minify_doctype: true,
        keep_comments: true,
        keep_html_and_head_opening_tags: true,
        keep_closing_tags: true,
        minify_css: true,
        minify_js: true,
        ..minify_html::Cfg::default()
    };

    // Minify bytes
    let html = minify_html::minify(&html.as_bytes(), &config);
    // Convert back to string
    String::from_utf8_lossy(&html).to_string()
}
