use pulldown_cmark::{Alignment, CodeBlockKind, HeadingLevel, LinkType};

/// Trait for handling Markdown tag rendering to HTML
pub trait TagHandler {
    /// Write a string directly to the output
    fn write_str(&mut self, s: &str);

    /// Write HTML attributes for a given element
    fn write_attributes(&mut self, element: &str);

    /// Check if a URL points to an external resource
    fn is_external_link(&self, url: &str) -> bool;

    /// Convert a HeadingLevel to a numeric level
    fn heading_level_to_u8(&self, level: HeadingLevel) -> u8;

    /// Generate an ID for a heading
    fn generate_heading_id(&self, level: HeadingLevel) -> String;

    // Text block handlers - these need custom implementations due to attributes/state
    fn start_paragraph(&mut self) {
        self.write_str("<p");
        self.write_attributes("p");
        self.write_str(">");
    }

    fn end_paragraph(&mut self) {
        self.write_str("</p>");
    }

    // Heading handlers need custom implementation for IDs and classes
    fn start_heading(&mut self, level: HeadingLevel, id: Option<&str>, classes: Vec<&str>);

    fn end_heading(&mut self, level: HeadingLevel) {
        self.write_str(&format!("</h{}>", self.heading_level_to_u8(level)));
    }

    fn start_blockquote(&mut self) {
        self.write_str("<blockquote");
        self.write_attributes("blockquote");
        self.write_str(">");
    }

    fn end_blockquote(&mut self) {
        self.write_str("</blockquote>");
    }

    // Code handlers - need custom implementation for language handling
    fn start_code_block(&mut self, kind: CodeBlockKind);
    fn end_code_block(&mut self) {
        self.write_str("</code></pre>");
    }

    fn start_inline_code(&mut self) {
        self.write_str("<code");
        self.write_attributes("code");
        self.write_str(">");
    }

    fn end_inline_code(&mut self) {
        self.write_str("</code>");
    }

    // List handlers - need custom implementation for ordered/unordered handling
    fn start_list(&mut self, first_number: Option<u64>);
    fn end_list(&mut self, ordered: bool) {
        self.write_str(if ordered { "</ol>" } else { "</ul>" });
    }

    fn start_list_item(&mut self) {
        self.write_str("<li");
        self.write_attributes("li");
        self.write_str(">");
    }

    fn end_list_item(&mut self) {
        self.write_str("</li>");
    }

    // Table handlers - need custom implementation for alignment/state
    fn start_table(&mut self, alignments: Vec<Alignment>);
    fn end_table(&mut self) {
        self.write_str("</tbody></table>");
    }

    fn start_table_head(&mut self);
    fn end_table_head(&mut self) {
        self.write_str("</tr></thead><tbody>");
    }

    fn start_table_row(&mut self) {
        self.write_str("<tr>");
    }

    fn end_table_row(&mut self) {
        self.write_str("</tr>");
    }

    fn start_table_cell(&mut self); // Needs custom implementation for th/td
    fn end_table_cell(&mut self) {
        self.write_str("</td>");
    }

    // Inline format handlers - simple HTML tags
    fn start_emphasis(&mut self) {
        self.write_str("<em");
        self.write_attributes("em");
        self.write_str(">");
    }

    fn end_emphasis(&mut self) {
        self.write_str("</em>");
    }

    fn start_strong(&mut self) {
        self.write_str("<strong");
        self.write_attributes("strong");
        self.write_str(">");
    }

    fn end_strong(&mut self) {
        self.write_str("</strong>");
    }

    fn start_strikethrough(&mut self) {
        self.write_str("<del");
        self.write_attributes("del");
        self.write_str(">");
    }

    fn end_strikethrough(&mut self) {
        self.write_str("</del>");
    }

    // Link and media handlers - need custom implementation for external link handling
    fn start_link(&mut self, link_type: LinkType, dest: &str, title: &str);

    fn end_link(&mut self) {
        self.write_str("</a>");
    }

    // Image doesn't need an end tag, but the start needs custom implementation
    fn start_image(&mut self, link_type: LinkType, dest: &str, title: &str);
    fn end_image(&mut self) {}

    // Footnote handlers - need custom implementation for state/ID handling
    fn start_footnote_reference(&mut self, name: &str);
    fn end_footnote_reference(&mut self) {
        self.write_str("</sup>");
    }

    fn start_footnote_definition(&mut self, name: &str) {
        self.write_str("<div class=\"footnote-definition\" id=\"");
        self.write_str(name);
        self.write_str("\"><sup class=\"footnote-definition-label\">");
        self.write_str(name);
        self.write_str("</sup>");
    }
    fn end_footnote_definition(&mut self) {
        self.write_str("</div>");
    }

    // Task list handlers
    fn start_task_list_item(&mut self, checked: bool) {
        self.write_str("<input type=\"checkbox\" disabled");
        if checked {
            self.write_str(" checked");
        }
        self.write_str(">");
    }

    fn end_task_list_item(&mut self) {}

    // Special elements - simple HTML
    fn horizontal_rule(&mut self) {
        self.write_str("<hr>");
    }

    fn soft_break(&mut self) {
        self.write_str("\n");
    }

    fn hard_break(&mut self) {
        self.write_str("<br>");
    }

    fn text(&mut self, text: &str) {
        self.write_str(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal implementation just for testing default methods
    struct TestHandler {
        output: String,
    }

    impl TestHandler {
        fn new() -> Self {
            Self {
                output: String::new(),
            }
        }

        fn get_output(&self) -> &str {
            &self.output
        }
    }

    impl TagHandler for TestHandler {
        fn write_str(&mut self, s: &str) {
            self.output.push_str(s);
        }

        fn write_attributes(&mut self, _element: &str) {
            // No attributes for testing
        }

        fn is_external_link(&self, url: &str) -> bool {
            url.starts_with("http://") || url.starts_with("https://")
        }

        fn heading_level_to_u8(&self, level: HeadingLevel) -> u8 {
            match level {
                HeadingLevel::H1 => 1,
                HeadingLevel::H2 => 2,
                HeadingLevel::H3 => 3,
                HeadingLevel::H4 => 4,
                HeadingLevel::H5 => 5,
                HeadingLevel::H6 => 6,
            }
        }

        fn generate_heading_id(&self, level: HeadingLevel) -> String {
            format!("heading-{}", self.heading_level_to_u8(level))
        }

        // Required methods that need custom implementation
        fn start_heading(&mut self, level: HeadingLevel, _id: Option<&str>, _classes: Vec<&str>) {
            self.write_str(&format!("<h{}>", self.heading_level_to_u8(level)));
        }

        fn start_code_block(&mut self, _kind: CodeBlockKind) {
            self.write_str("<pre><code>");
        }

        fn start_list(&mut self, first_number: Option<u64>) {
            match first_number {
                Some(_) => self.write_str("<ol>"),
                None => self.write_str("<ul>"),
            }
        }

        fn start_table(&mut self, _alignments: Vec<Alignment>) {
            self.write_str("<table>");
        }

        fn start_table_head(&mut self) {
            self.write_str("<thead><tr>");
        }

        fn start_table_cell(&mut self) {
            self.write_str("<td>");
        }

        fn start_link(&mut self, _link_type: LinkType, dest: &str, title: &str) {
            self.write_str("<a href=\"");
            self.write_str(dest);
            if !title.is_empty() {
                self.write_str("\" title=\"");
                self.write_str(title);
            }
            self.write_str("\">");
        }

        fn start_image(&mut self, _link_type: LinkType, dest: &str, title: &str) {
            self.write_str("<img src=\"");
            self.write_str(dest);
            self.write_str("\" alt=\"");
            if !title.is_empty() {
                self.write_str("\" title=\"");
                self.write_str(title);
            }
            self.write_str("\">");
        }

        fn start_footnote_reference(&mut self, name: &str) {
            self.write_str("<sup class=\"footnote-reference\"><a href=\"#");
            self.write_str(name);
            self.write_str("\">");
            self.write_str(name);
            self.write_str("</a></sup>");
        }
    }

    #[test]
    fn test_paragraph() {
        let mut handler = TestHandler::new();
        handler.start_paragraph();
        handler.text("Hello world");
        handler.end_paragraph();
        assert_eq!(handler.get_output(), "<p>Hello world</p>");
    }

    #[test]
    fn test_blockquote() {
        let mut handler = TestHandler::new();
        handler.start_blockquote();
        handler.text("Quote");
        handler.end_blockquote();
        assert_eq!(handler.get_output(), "<blockquote>Quote</blockquote>");
    }

    #[test]
    fn test_emphasis() {
        let mut handler = TestHandler::new();
        handler.start_emphasis();
        handler.text("emphasized");
        handler.end_emphasis();
        assert_eq!(handler.get_output(), "<em>emphasized</em>");
    }

    #[test]
    fn test_strong() {
        let mut handler = TestHandler::new();
        handler.start_strong();
        handler.text("bold");
        handler.end_strong();
        assert_eq!(handler.get_output(), "<strong>bold</strong>");
    }

    #[test]
    fn test_strikethrough() {
        let mut handler = TestHandler::new();
        handler.start_strikethrough();
        handler.text("strike");
        handler.end_strikethrough();
        assert_eq!(handler.get_output(), "<del>strike</del>");
    }

    #[test]
    fn test_inline_code() {
        let mut handler = TestHandler::new();
        handler.start_inline_code();
        handler.text("code");
        handler.end_inline_code();
        assert_eq!(handler.get_output(), "<code>code</code>");
    }

    #[test]
    fn test_line_breaks() {
        let mut handler = TestHandler::new();
        handler.soft_break();
        handler.hard_break();
        assert_eq!(handler.get_output(), "\n<br>");
    }

    #[test]
    fn test_horizontal_rule() {
        let mut handler = TestHandler::new();
        handler.horizontal_rule();
        assert_eq!(handler.get_output(), "<hr>");
    }

    #[test]
    fn test_task_list() {
        let mut handler = TestHandler::new();
        handler.start_task_list_item(true);
        handler.text("Done");
        handler.end_task_list_item();
        assert_eq!(
            handler.get_output(),
            "<input type=\"checkbox\" disabled checked>Done"
        );

        let mut handler = TestHandler::new();
        handler.start_task_list_item(false);
        handler.text("Todo");
        handler.end_task_list_item();
        assert_eq!(
            handler.get_output(),
            "<input type=\"checkbox\" disabled>Todo"
        );
    }

    #[test]
    fn test_footnote_definition() {
        let mut handler = TestHandler::new();
        handler.start_footnote_definition("1");
        handler.text("Footnote content");
        handler.end_footnote_definition();
        assert_eq!(
            handler.get_output(),
            "<div class=\"footnote-definition\" id=\"1\">\
             <sup class=\"footnote-definition-label\">1</sup>\
             Footnote content</div>"
        );
    }

    #[test]
    fn test_list_endings() {
        let mut handler = TestHandler::new();
        handler.end_list(true);
        assert_eq!(handler.get_output(), "</ol>");

        let mut handler = TestHandler::new();
        handler.end_list(false);
        assert_eq!(handler.get_output(), "</ul>");
    }

    #[test]
    fn test_table_structure() {
        let mut handler = TestHandler::new();
        handler.end_table_head();
        handler.end_table_row();
        handler.end_table_cell();
        handler.end_table();
        assert_eq!(
            handler.get_output(),
            "</tr></thead><tbody></tr></td></tbody></table>"
        );
    }
}
