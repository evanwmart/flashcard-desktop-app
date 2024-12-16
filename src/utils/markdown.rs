use pulldown_cmark::{html, Options, Parser};

/// Converts Markdown content to HTML.
///
/// # Arguments
///
/// * `markdown` - A string slice containing the Markdown content.
///
/// # Returns
///
/// A `String` containing the rendered HTML.
pub fn render_markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
