//! pulldown-html-ext
//!
//! A configurable Markdown to HTML renderer built on top of pulldown-cmark.
//!
//! # Documentation
//!
//! - [API Documentation](https://docs.rs/pulldown-html-ext)
//! - [User Guide](https://systemsoverload.github.io/pulldown-html-ext)
//! - [Examples](https://systemsoverload.github.io/pulldown-html-ext/examples)
//!
//! # Quick Start
//!
//! ```rust
//! use pulldown_html_ext::{HtmlConfig, push_html};
//! use pulldown_cmark::Parser;
//!
//! let config = HtmlConfig::default();
//! let markdown = "# Hello\nThis is *markdown*";
//! let parser = Parser::new(markdown);
//! let mut output = String::new();
//! let html = push_html(&mut output, parser, &config).unwrap();
//! ```
//!
//! Custom rendering with a custom writer:
//! ```rust
//! use pulldown_html_ext::{HtmlConfig, HtmlWriter, HtmlState, create_html_renderer};
//! use pulldown_cmark_escape::{StrWrite, FmtWriter};
//!
//! struct CustomWriter<W: StrWrite> {
//!     writer: W,
//!     config: HtmlConfig,
//!     state: HtmlState,
//! }
//!
//! impl<W: StrWrite> CustomWriter<W> {
//!     fn new(writer: W, config: HtmlConfig) -> Self {
//!         Self {
//!             writer,
//!             config,
//!             state: HtmlState::new(),
//!         }
//!     }
//! }
//!
//! impl<W: StrWrite> HtmlWriter<W> for CustomWriter<W> {
//!     fn get_writer(&mut self) -> &mut W {
//!         &mut self.writer
//!     }
//!
//!     fn get_config(&self) -> &HtmlConfig {
//!         &self.config
//!     }
//!
//!     fn get_state(&mut self) -> &mut HtmlState {
//!         &mut self.state
//!     }
//! }
//!
//! let mut output = String::new();
//! let writer = CustomWriter::new(
//!     FmtWriter(&mut output),
//!     HtmlConfig::default()
//! );
//! let mut renderer = create_html_renderer(writer);
//!
//! // Use the renderer with a parser
//! use pulldown_cmark::Parser;
//! let markdown = "# Hello\nThis is *markdown*";
//! let parser = Parser::new(markdown);
//! renderer.run(parser);
//!
//! assert!(output.contains("<h1"));
//! ```

mod html;
pub mod utils;

pub use html::{
    create_html_renderer, push_html, push_html_with_highlighting, write_html_fmt, write_html_io,
    AttributeMappings, CodeBlockOptions, DefaultHtmlWriter, ElementOptions, HeadingOptions,
    HtmlConfig, HtmlError, HtmlOptions, HtmlRenderer, HtmlState, HtmlWriter, LinkOptions,
    SyntectConfig, SyntectConfigStyle, SyntectWriter,
};

#[cfg(test)]
mod tests_lib {
    use super::*;
    use pulldown_cmark::Parser;
    use std::collections::HashMap;

    #[test]
    fn test_basic_markdown() {
        let config = HtmlConfig::default();
        let markdown = "# Hello\nThis is a test.";
        let parser = Parser::new(markdown);
        let mut output = String::new();

        push_html(&mut output, parser, &config).unwrap();

        assert!(output.contains("<h1"));
        assert!(output.contains("Hello"));
        assert!(output.contains("<p>"));
        assert!(output.contains("This is a test."));
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
        let parser = Parser::new(markdown);
        let mut output = String::new();

        push_html(&mut output, parser, &config).unwrap();

        assert!(output.contains(r#"<h1 id="heading-1" class="title""#));
        assert!(output.contains(r#"<h2 id="heading-2" class="subtitle""#));
    }

    #[test]
    fn test_code_blocks() {
        let mut config = HtmlConfig::default();
        config.elements.code_blocks.default_language = Some("text".to_string());

        let markdown = "```python\nprint('hello')\n```";
        let parser = Parser::new(markdown);
        let mut output = String::new();

        push_html(&mut output, parser, &config).unwrap();
        assert!(output.contains(r#"<code class="language-python">"#));

        let markdown = "```\nplain text\n```";
        let parser = Parser::new(markdown);
        let mut output = String::new();

        push_html(&mut output, parser, &config).unwrap();
        assert!(output.contains(r#"<code class="language-text">"#));
    }

    #[test]
    fn test_external_links() {
        let mut config = HtmlConfig::default();
        config.elements.links.nofollow_external = true;
        config.elements.links.open_external_blank = true;

        let markdown = "[External](https://example.com)";
        let parser = Parser::new(markdown);
        let mut output = String::new();

        push_html(&mut output, parser, &config).unwrap();
        assert!(output.contains(r#"rel="nofollow""#));
        assert!(output.contains(r#"target="_blank""#));

        let markdown = "[Internal](/local)";
        let parser = Parser::new(markdown);
        let mut output = String::new();

        push_html(&mut output, parser, &config).unwrap();
        assert!(!output.contains(r#"rel="nofollow""#));
        assert!(!output.contains(r#"target="_blank""#));
    }

    #[test]
    fn test_html_options() {
        let mut config = HtmlConfig::default();
        config.html.escape_html = true;
        config.html.break_on_newline = false;
        config.html.xhtml_style = true;

        let markdown = "Test & test\nNew line";
        let parser = Parser::new(markdown);
        let mut output = String::new();

        push_html(&mut output, parser, &config).unwrap();
        assert!(output.contains("&amp;"));
        assert!(!output.contains("<br"));
    }

    #[test]
    fn test_custom_parser_options() {
        use pulldown_cmark::{Options, Parser};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);

        let markdown = "~~strikethrough~~";
        let parser = Parser::new_ext(markdown, options);
        let mut output = String::new();
        let config = HtmlConfig::default();

        push_html(&mut output, parser, &config).unwrap();
        assert!(output.contains("<del>"));
        assert!(output.contains("</del>"));
    }

    #[test]
    fn test_streaming_parser() {
        let config = HtmlConfig::default();
        let mut output = String::new();

        // Simulate streaming input by creating multiple parsers
        let chunk1 = "# Title\n";
        let chunk2 = "Paragraph 1\n";
        let chunk3 = "* List item";

        let parser1 = Parser::new(chunk1);
        push_html(&mut output, parser1, &config).unwrap();

        let parser2 = Parser::new(chunk2);
        push_html(&mut output, parser2, &config).unwrap();

        let parser3 = Parser::new(chunk3);
        push_html(&mut output, parser3, &config).unwrap();

        assert!(output.contains("<h1"));
        assert!(output.contains("<p>"));
        assert!(output.contains("<li>"));
    }
}
