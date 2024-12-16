#[cfg(test)]
mod tests {
    use flashcard_desktop_app::utils::markdown::render_markdown_to_html;

    #[test]
    fn test_basic_markdown() {
        let markdown_input = "# Heading\n\nThis is a **bold** word and *italic* word.";
        let expected_html = "<h1>Heading</h1>\n<p>This is a <strong>bold</strong> word and <em>italic</em> word.</p>\n";

        let html_output = render_markdown_to_html(markdown_input);
        assert_eq!(html_output, expected_html);
    }

    #[test]
    fn test_markdown_with_extensions() {
        let markdown_input = "- [ ] Task 1\n- [x] Task 2\n\n~~Strikethrough~~";
        let expected_html = "<ul>\n<li><input disabled=\"\" type=\"checkbox\"/>\nTask 1</li>\n<li><input disabled=\"\" type=\"checkbox\" checked=\"\"/>\nTask 2</li>\n</ul>\n<p><del>Strikethrough</del></p>\n";
    
        let html_output = render_markdown_to_html(markdown_input);
        assert_eq!(html_output, expected_html);
    }
    
    #[test]
    fn test_markdown_with_tables() {
        let markdown_input = "| Column 1 | Column 2 |\n|----------|----------|\n| Value 1  | Value 2  |";
        let expected_html = "<table><thead><tr><th>Column 1</th><th>Column 2</th></tr></thead><tbody>\n<tr><td>Value 1</td><td>Value 2</td></tr>\n</tbody></table>\n";
    
        let html_output = render_markdown_to_html(markdown_input);
        assert_eq!(html_output, expected_html);
    }
}
