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

    // Text block handlers
    fn start_paragraph(&mut self) {
        self.write_str("<p");
        self.write_attributes("p");
        self.write_str(">");
    }

    fn start_heading(&mut self, level: HeadingLevel, id: Option<&str>, classes: Vec<&str>);

    fn start_blockquote(&mut self) {
        self.write_str("<blockquote");
        self.write_attributes("blockquote");
        self.write_str(">");
    }

    // Code handlers
    fn start_code_block(&mut self, kind: CodeBlockKind);

    // List handlers
    fn start_list(&mut self, first_number: Option<u64>);
    fn start_list_item(&mut self);

    // Table handlers
    fn start_table(&mut self, alignments: Vec<Alignment>);
    fn start_table_head(&mut self);
    fn start_table_row(&mut self);
    fn start_table_cell(&mut self);

    // Inline format handlers
    fn start_emphasis(&mut self) {
        self.write_str("<em");
        self.write_attributes("em");
        self.write_str(">");
    }

    fn start_strong(&mut self) {
        self.write_str("<strong");
        self.write_attributes("strong");
        self.write_str(">");
    }

    fn start_strikethrough(&mut self) {
        self.write_str("<del");
        self.write_attributes("del");
        self.write_str(">");
    }

    // Link and media handlers
    fn start_link(&mut self, link_type: LinkType, dest: &str, title: &str);
    fn start_image(&mut self, link_type: LinkType, dest: &str, title: &str);

    // Footnote handlers
    fn start_footnote_definition(&mut self, name: &str) {
        self.write_str("<div class=\"footnote-definition\" id=\"");
        escape_html(&name);
        self.write_str("\"><sup class=\"footnote-definition-label\">");
        escape_html(&name);
        self.write_str("</sup>");
    }
}

// Helper function for HTML escaping
fn escape_html(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '&' => "&amp;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
