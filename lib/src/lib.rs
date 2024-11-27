//! A configurable Markdown to HTML renderer built on top of pulldown-cmark.
//!
//! This crate provides a flexible HTML renderer with support for custom styling,
//! attributes, and rendering options. It extends pulldown-cmark's capabilities
//! while maintaining a clean, safe API.
//!
//! # Example
//! ```rust
//! use pulldown_html_ext::{RendererConfig, render_markdown};
//!
//! let config = RendererConfig::default();
//! let markdown = "# Hello\nThis is *markdown*";
//! let html = render_markdown(markdown, &config);
//! assert!(html.contains("<h1"));
//! ```

mod config;
mod default_handler;
mod renderer;
mod renderer_state;
mod tag_handler;
pub mod utils;

pub use config::{
    AttributeMappings, CodeBlockOptions, ElementOptions, HeadingOptions, HtmlOptions, LinkOptions,
    RendererConfig,
};
pub use default_handler::DefaultTagHandler;
pub use renderer::Renderer;
pub use tag_handler::TagHandler;

use pulldown_cmark::Parser;

/// Convert markdown to HTML with the given configuration
///
/// # Arguments
///
/// * `markdown` - The markdown text to convert
/// * `config` - Configuration options for the renderer
///
/// # Example
///
/// ```rust
/// use pulldown_html_ext::{RendererConfig, render_markdown};
///
/// let config = RendererConfig::default();
/// let markdown = "# Title\nHello *world*!";
/// let html = render_markdown(markdown, &config);
/// ```
pub fn render_markdown(markdown: &str, config: &RendererConfig) -> String {
    let mut output = String::new();
    let parser = Parser::new(markdown);
    let handler = default_handler::DefaultTagHandler::new(&mut output, config);
    let mut renderer = renderer::Renderer::new(handler);
    renderer.run(parser);
    output
}

/// Create a custom renderer with a specific tag handler
///
/// # Arguments
///
/// * `handler` - The custom tag handler implementation
///
/// # Example
///
/// ```rust
/// use pulldown_html_ext::{TagHandler, Renderer};
///
/// struct CustomHandler;
/// impl TagHandler for CustomHandler {
///     // Implement required methods...
/// #    fn write_str(&mut self, _s: &str) {}
/// #    fn write_attributes(&mut self, _element: &str) {}
/// #    fn is_external_link(&self, _url: &str) -> bool { false }
/// #    fn heading_level_to_u8(&self, _level: pulldown_cmark::HeadingLevel) -> u8 { 1 }
/// #    fn generate_heading_id(&self, _level: pulldown_cmark::HeadingLevel) -> String { String::new() }
/// #    fn start_heading(&mut self, _level: pulldown_cmark::HeadingLevel, _id: Option<&str>, _classes: Vec<&str>) {}
/// #    fn start_code_block(&mut self, _kind: pulldown_cmark::CodeBlockKind) {}
/// #    fn start_list(&mut self, _first_number: Option<u64>) {}
/// #    fn start_list_item(&mut self) {}
/// #    fn start_table(&mut self, _alignments: Vec<pulldown_cmark::Alignment>) {}
/// #    fn start_table_head(&mut self) {}
/// #    fn start_table_row(&mut self) {}
/// #    fn start_table_cell(&mut self) {}
/// #    fn start_link(&mut self, _link_type: pulldown_cmark::LinkType, _dest: &str, _title: &str) {}
/// #    fn start_image(&mut self, _link_type: pulldown_cmark::LinkType, _dest: &str, _title: &str) {}
/// }
///
/// let handler = CustomHandler;
/// let renderer = Renderer::new(handler);
/// ```
pub fn create_renderer<H: TagHandler>(handler: H) -> Renderer<H> {
    Renderer::new(handler)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_basic_markdown() {
        let config = RendererConfig::default();
        let markdown = "# Hello\nThis is a test.";
        let html = render_markdown(markdown, &config);
        assert!(html.contains("<h1"));
        assert!(html.contains("Hello"));
        assert!(html.contains("<p>"));
        assert!(html.contains("This is a test."));
    }

    #[test]
    fn test_custom_heading_classes() {
        let mut config = RendererConfig::default();
        config.elements.headings.level_classes = {
            let mut map = HashMap::new();
            map.insert(1, "title".to_string());
            map.insert(2, "subtitle".to_string());
            map
        };

        let markdown = "# Main Title\n## Subtitle";
        let html = render_markdown(markdown, &config);
        assert!(html.contains(r#"<h1 id="heading-1" class="title""#));
        assert!(html.contains(r#"<h2 id="heading-2" class="subtitle""#));
    }

    #[test]
    fn test_code_blocks() {
        let mut config = RendererConfig::default();
        config.elements.code_blocks.default_language = Some("text".to_string());

        let markdown = "```python\nprint('hello')\n```";
        let html = render_markdown(markdown, &config);
        assert!(html.contains(r#"<code class="language-python">"#));

        let markdown = "```\nplain text\n```";
        let html = render_markdown(markdown, &config);
        assert!(html.contains(r#"<code class="language-text">"#));
    }

    #[test]
    fn test_external_links() {
        let mut config = RendererConfig::default();
        config.elements.links.nofollow_external = true;
        config.elements.links.open_external_blank = true;

        let markdown = "[External](https://example.com)";
        let html = render_markdown(markdown, &config);
        assert!(html.contains(r#"rel="nofollow""#));
        assert!(html.contains(r#"target="_blank""#));

        let markdown = "[Internal](/local)";
        let html = render_markdown(markdown, &config);
        assert!(!html.contains(r#"rel="nofollow""#));
        assert!(!html.contains(r#"target="_blank""#));
    }
}
