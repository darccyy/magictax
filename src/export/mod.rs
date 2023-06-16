use handlebars::Handlebars;
use serde_json::json;

use crate::csv::Csv;

pub fn export_html(csv: &Csv) -> Result<String, ()> {
    // Get templates from files
    let template = include_str!("template/index.hbs");
    let style = include_str!("template/style.css");

    // Create json object to pass to template
    let json = json!({
        "table": csv,
        "style": style,
    });

    println!("{:#?}", json);

    // Create handlebars interface
    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(true);

    // Render template with handlebars
    let html = hbs
        .render_template(&template, &json)
        .expect("Failed to render template");

    // Minify html
    Ok(minify(html))
}

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
