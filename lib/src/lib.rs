//! A configurable Markdown to HTML renderer built on top of pulldown-cmark.
//!
//! This crate provides a flexible HTML renderer with support for custom styling,
//! attributes, and rendering options. It extends pulldown-cmark's capabilities
//! while maintaining a clean, safe API.
//!
//! # Examples
//!
//! Basic usage with default options:
//! ```rust
//! use pulldown_html_ext::{HtmlConfig, push_html};
//!
//! let config = HtmlConfig::default();
//! let markdown = "# Hello\nThis is *markdown*";
//! let html = push_html(markdown, &config);
//! assert!(html.contains("<h1"));
//! ```
//!
//! Custom rendering with a custom writer:
//! ```rust
//! use pulldown_html_ext::{HtmlConfig, HtmlWriter, HtmlState, create_html_renderer};
//!
//! struct CustomWriter {
//!     config: HtmlConfig,
//!     output: String,
//!     state: HtmlState,
//! }
//!
//! impl HtmlWriter for CustomWriter {
//!     // Implement required methods...
//! #    fn get_config(&self) -> &HtmlConfig { &self.config }
//! #    fn get_output(&mut self) -> &mut String { &mut self.output }
//! #    fn get_state(&mut self) -> &mut HtmlState { &mut self.state }
//! }
//!
//! let writer = CustomWriter {
//!     config: HtmlConfig::default(),
//!     output: String::new(),
//!     state: HtmlState::new(),
//! };
//! let renderer = create_html_renderer(writer);
//! ```

mod html;
pub mod utils;

pub use html::{
    create_html_renderer, push_html, AttributeMappings, CodeBlockOptions, DefaultHtmlWriter,
    ElementOptions, HeadingOptions, HtmlConfig, HtmlOptions, HtmlRenderer, HtmlState, HtmlWriter,
    LinkOptions,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_basic_markdown() {
        let config = HtmlConfig::default();
        let markdown = "# Hello\nThis is a test.";
        let html = push_html(markdown, &config);
        assert!(html.contains("<h1"));
        assert!(html.contains("Hello"));
        assert!(html.contains("<p>"));
        assert!(html.contains("This is a test."));
    }

    #[test]
    fn test_custom_heading_classes() {
        let mut config = HtmlConfig::default();
        config.elements.headings.level_classes = {
            let mut map = HashMap::new();
            map.insert(1, "title".to_string());
            map.insert(2, "subtitle".to_string());
            map
        };

        let markdown = "# Main Title\n## Subtitle";
        let html = push_html(markdown, &config);
        assert!(html.contains(r#"<h1 id="heading-1" class="title""#));
        assert!(html.contains(r#"<h2 id="heading-2" class="subtitle""#));
    }

    #[test]
    fn test_code_blocks() {
        let mut config = HtmlConfig::default();
        config.elements.code_blocks.default_language = Some("text".to_string());

        let markdown = "```python\nprint('hello')\n```";
        let html = push_html(markdown, &config);
        assert!(html.contains(r#"<code class="language-python">"#));

        let markdown = "```\nplain text\n```";
        let html = push_html(markdown, &config);
        assert!(html.contains(r#"<code class="language-text">"#));
    }

    #[test]
    fn test_external_links() {
        let mut config = HtmlConfig::default();
        config.elements.links.nofollow_external = true;
        config.elements.links.open_external_blank = true;

        let markdown = "[External](https://example.com)";
        let html = push_html(markdown, &config);
        assert!(html.contains(r#"rel="nofollow""#));
        assert!(html.contains(r#"target="_blank""#));

        let markdown = "[Internal](/local)";
        let html = push_html(markdown, &config);
        assert!(!html.contains(r#"rel="nofollow""#));
        assert!(!html.contains(r#"target="_blank""#));
    }

    #[test]
    fn test_html_options() {
        let mut config = HtmlConfig::default();
        config.html.escape_html = true;
        config.html.break_on_newline = false;
        config.html.xhtml_style = true;

        let markdown = "Test & test\nNew line";
        let html = push_html(markdown, &config);
        assert!(html.contains("&amp;"));
        assert!(!html.contains("<br"));
    }
}
