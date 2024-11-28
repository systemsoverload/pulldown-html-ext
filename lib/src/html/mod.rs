//! HTML rendering functionality for Markdown content.
//!
//! This module provides configurable HTML rendering capabilities built on top
//! of pulldown-cmark's event model. It supports customized rendering of HTML
//! elements, attribute handling, and state management during rendering.

mod config;
mod default;
mod state;
mod writer;

use pulldown_cmark::{Event, Parser};
use pulldown_cmark_escape::{FmtWriter, IoWriter, StrWrite};
use std::iter::Peekable;

pub use self::config::{
    AttributeMappings, CodeBlockOptions, ElementOptions, HeadingOptions, HtmlConfig, HtmlOptions,
    LinkOptions,
};
pub use self::default::DefaultHtmlWriter;
pub use self::state::{HtmlState, ListContext, TableContext};
pub use self::writer::HtmlWriter;

/// Core renderer that processes Markdown events into HTML
use std::marker::PhantomData;

pub struct HtmlRenderer<W: StrWrite, H: HtmlWriter<W>> {
    writer: H,
    _phantom: PhantomData<W>,
}

impl<W: StrWrite, H: HtmlWriter<W>> HtmlRenderer<W, H> {
    pub fn new(writer: H) -> Self {
        Self {
            writer,
            _phantom: PhantomData,
        }
    }

    /// Process the event stream and generate HTML output
    pub fn run<'a, I>(&mut self, iter: I)
    where
        I: Iterator<Item = Event<'a>>,
    {
        let mut iter = iter.peekable();
        while let Some(event) = iter.next() {
            match event {
                Event::Start(tag) => self.handle_start(&mut iter, tag),
                Event::End(tag) => self.handle_end(tag),
                Event::Text(text) => self.writer.text(&text),
                Event::Code(text) => self.handle_inline_code(&text),
                Event::Html(html) => self.writer.write_str(&html),
                Event::SoftBreak => self.writer.soft_break(),
                Event::HardBreak => self.writer.hard_break(),
                Event::Rule => self.writer.horizontal_rule(),
                Event::FootnoteReference(name) => self.writer.footnote_reference(&name),
                Event::TaskListMarker(checked) => self.writer.task_list_item(checked),
            }
        }
    }

    /// Handle start tags with potential lookahead
    fn handle_start<'a, I>(&mut self, iter: &mut Peekable<I>, tag: pulldown_cmark::Tag<'a>)
    where
        I: Iterator<Item = Event<'a>>,
    {
        match tag {
            pulldown_cmark::Tag::Paragraph => self.writer.start_paragraph(),
            pulldown_cmark::Tag::Heading(level, id, classes) => {
                self.writer.start_heading(level, id, classes)
            }
            pulldown_cmark::Tag::BlockQuote => self.writer.start_blockquote(),
            pulldown_cmark::Tag::CodeBlock(kind) => self.writer.start_code_block(kind),
            pulldown_cmark::Tag::List(start) => self.writer.start_list(start),
            pulldown_cmark::Tag::Item => self.writer.start_list_item(),
            pulldown_cmark::Tag::FootnoteDefinition(name) => {
                self.writer.start_footnote_definition(&name)
            }
            pulldown_cmark::Tag::Table(alignments) => self.writer.start_table(alignments),
            pulldown_cmark::Tag::TableHead => self.writer.start_table_head(),
            pulldown_cmark::Tag::TableRow => self.writer.start_table_row(),
            pulldown_cmark::Tag::TableCell => self.writer.start_table_cell(),
            pulldown_cmark::Tag::Emphasis => self.writer.start_emphasis(),
            pulldown_cmark::Tag::Strong => self.writer.start_strong(),
            pulldown_cmark::Tag::Strikethrough => self.writer.start_strikethrough(),
            pulldown_cmark::Tag::Link(link_type, dest, title) => {
                self.writer.start_link(link_type, &dest, &title)
            }
            pulldown_cmark::Tag::Image(link_type, dest, title) => {
                self.writer.start_image(link_type, &dest, &title, iter)
            }
        }
    }

    /// Handle end tags
    fn handle_end(&mut self, tag: pulldown_cmark::Tag) {
        match tag {
            pulldown_cmark::Tag::Paragraph => self.writer.end_paragraph(),
            pulldown_cmark::Tag::Heading(level, ..) => self.writer.end_heading(level),
            pulldown_cmark::Tag::BlockQuote => self.writer.end_blockquote(),
            pulldown_cmark::Tag::CodeBlock(_) => self.writer.end_code_block(),
            pulldown_cmark::Tag::List(Some(_)) => self.writer.end_list(true),
            pulldown_cmark::Tag::List(None) => self.writer.end_list(false),
            pulldown_cmark::Tag::Item => self.writer.end_list_item(),
            pulldown_cmark::Tag::FootnoteDefinition(_) => self.writer.end_footnote_definition(),
            pulldown_cmark::Tag::Table(_) => self.writer.end_table(),
            pulldown_cmark::Tag::TableHead => self.writer.end_table_head(),
            pulldown_cmark::Tag::TableRow => self.writer.end_table_row(),
            pulldown_cmark::Tag::TableCell => self.writer.end_table_cell(),
            pulldown_cmark::Tag::Emphasis => self.writer.end_emphasis(),
            pulldown_cmark::Tag::Strong => self.writer.end_strong(),
            pulldown_cmark::Tag::Strikethrough => self.writer.end_strikethrough(),
            pulldown_cmark::Tag::Link(..) => self.writer.end_link(),
            pulldown_cmark::Tag::Image(..) => self.writer.end_image(),
        }
    }

    /// Handle inline code elements
    fn handle_inline_code(&mut self, text: &str) {
        self.writer.start_inline_code();
        self.writer.text(text);
        self.writer.end_inline_code();
    }
}

/// Convert Markdown to HTML using the default writer
pub fn push_html(markdown: &str, config: &HtmlConfig) -> String {
    let mut output = String::new();
    let parser = Parser::new(markdown);
    let writer = DefaultHtmlWriter::new(FmtWriter(&mut output), config);
    let mut renderer = HtmlRenderer::new(writer);
    renderer.run(parser); // TODO: Handle errors...
    output
}

/// Write HTML to a std::fmt::Write implementor
pub fn write_html_fmt<W>(writer: W, markdown: &str, config: &HtmlConfig)
where
    W: std::fmt::Write,
{
    let parser = Parser::new(markdown);
    let writer = DefaultHtmlWriter::new(FmtWriter(writer), config);
    let mut renderer = HtmlRenderer::new(writer);
    renderer.run(parser); // TODO: Handle errors...
}

/// Write HTML to a std::io::Write implementor
pub fn write_html_io<W>(writer: W, markdown: &str, config: &HtmlConfig)
where
    W: std::io::Write,
{
    let parser = Parser::new(markdown);
    let writer = DefaultHtmlWriter::new(IoWriter(writer), config);
    let mut renderer = HtmlRenderer::new(writer);
    renderer.run(parser)
    //.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)); // TODO: Handle errors...
}

/// Create a custom HTML renderer with a specific writer implementation
pub fn create_html_renderer<W: StrWrite, H: HtmlWriter<W>>(writer: H) -> HtmlRenderer<W, H> {
    HtmlRenderer::new(writer)
}

#[cfg(test)]
mod tests {

    use crate::html::{HtmlConfig, HtmlRenderer};
    use crate::DefaultHtmlWriter;
    use html_compare_rs::{assert_html_eq, presets::markdown};
    use pulldown_cmark::{Options, Parser};

    fn push_html_with_config(input: &str, config: &HtmlConfig) -> String {
        let mut output = String::new();
        let handler = DefaultHtmlWriter::new(&mut output, config);
        let mut renderer = HtmlRenderer::new(handler);
        renderer.run(Parser::new_ext(input, Options::all()));
        output
    }

    fn push_html(input: &str) -> String {
        push_html_with_config(input, &HtmlConfig::default())
    }

    #[test]
    fn test_basic_text_rendering() {
        assert_html_eq!(
            push_html("Hello, world!"),
            "<p>Hello, world!</p>",
            markdown()
        );
    }

    #[test]
    fn test_emphasis_and_strong() {
        assert_html_eq!(
            push_html("*italic* and **bold** text"),
            "<p><em>italic</em> and <strong>bold</strong> text</p>",
            markdown()
        );
    }

    #[test]
    fn test_nested_formatting() {
        assert_html_eq!(
            push_html("***bold italic*** and **bold *italic* mix**"),
             "<p><em><strong>bold italic</strong></em> and <strong>bold <em>italic</em> mix</strong></p>",
            markdown()
        );
    }

    #[test]
    fn test_headings() {
        let input = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
        assert_html_eq!(
            push_html(input),
            "<h1 id=\"heading-1\">H1</h1>\
             <h2 id=\"heading-2\">H2</h2>\
             <h3 id=\"heading-3\">H3</h3>\
             <h4 id=\"heading-4\">H4</h4>\
             <h5 id=\"heading-5\">H5</h5>\
             <h6 id=\"heading-6\">H6</h6>",
            markdown()
        );
    }

    #[test]
    fn test_lists() {
        let input = "- Item 1\n- Item 2\n  - Nested 1\n  - Nested 2\n- Item 3";
        assert_html_eq!(
            push_html(input),
            "<ul><li>Item 1</li>\
             <li>Item 2\
             <ul><li>Nested 1</li>\
             <li>Nested 2</li></ul></li>\
             <li>Item 3</li></ul>",
            markdown()
        );
    }

    #[test]
    fn test_ordered_lists() {
        let input = "1. First\n2. Second\n   1. Nested\n   2. Items\n3. Third";
        assert_html_eq!(
            push_html(input),
            "<ol><li>First</li>\
             <li>Second\
             <ol><li>Nested</li>\
             <li>Items</li></ol></li>\
             <li>Third</li></ol>",
            markdown()
        );
    }

    #[test]
    fn test_code_blocks() {
        let input = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        assert_html_eq!(
            push_html(input),
            "<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello\");\n}</code></pre>",
            markdown()
        );
    }

    #[test]
    fn test_inline_code() {
        assert_html_eq!(
            push_html("Use the `println!` macro"),
            "<p>Use the <code>println!</code> macro</p>",
            markdown()
        );
    }

    #[test]
    fn test_blockquotes() {
        let input = "> First level\n>> Second level\n\n> Back to first";
        assert_html_eq!(
            push_html(input),
            "<blockquote><p>First level</p><blockquote><p>Second level</p></blockquote></blockquote><blockquote><p>Back to first</p></blockquote>",
            markdown()
        );
    }

    #[test]
    fn test_links() {
        assert_html_eq!(
            push_html("[Example](https://example.com \"Title\")"),
            r#"<p><a href="https://example.com" title="Title" rel="nofollow" target="_blank">Example</a></p>"#,
            markdown()
        );
    }

    #[test]
    fn test_images() {
        assert_html_eq!(
            push_html("![Alt text](image.jpg \"Image title\")"),
            "<p><img src=\"image.jpg\" alt=\"Alt text\" title=\"Image title\"></p>",
            markdown()
        );
    }

    #[test]
    fn test_tables() {
        let input = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
        assert_html_eq!(
            push_html(input),
            "<table><thead><tr><th>Header 1</th><th>Header 2</th></tr></thead>\
             <tbody><tr><td>Cell 1</td><td>Cell 2</td></tr></tbody></table>",
            markdown()
        );
    }

    #[test]
    fn test_task_lists() {
        let input = "- [ ] Unchecked\n- [x] Checked";
        assert_html_eq!(
            push_html(input),
            "<ul><li><input type=\"checkbox\" disabled>Unchecked</li>\
             <li><input type=\"checkbox\" disabled checked>Checked</li></ul>",
            markdown()
        );
    }

    #[test]
    fn test_strikethrough() {
        assert_html_eq!(
            push_html("~~struck through~~"),
            "<p><del>struck through</del></p>",
            markdown()
        );
    }

    #[test]
    fn test_horizontal_rule() {
        assert_html_eq!(push_html("---"), "<hr>", markdown());
    }

    #[test]
    fn test_mixed_content() {
        let input = "# Title\n\
                     Some *formatted* text with `code`.\n\n\
                     > A quote with **bold**\n\n\
                     - List item 1\n\
                     - List item 2\n\n\
                     ```\nCode block\n```";

        assert_html_eq!(
            push_html(input),
            "<h1 id=\"heading-1\">Title</h1>\
             <p>Some <em>formatted</em> text with <code>code</code>.</p>\
             <blockquote><p>A quote with <strong>bold</strong></p></blockquote>\
             <ul><li>List item 1</li><li>List item 2</li></ul>\
             <pre><code>Code block</code></pre>",
            markdown()
        );
    }

    #[test]
    #[ignore = "Fix/implement escape_html option"]
    fn test_escaped_html() {
        let mut config = HtmlConfig::default();
        config.html.escape_html = true;

        assert_html_eq!(
            push_html_with_config("This is <em>HTML</em> content", &config),
            "<p>This is &lt;em&gt;HTML&lt;/em&gt; content</p>",
            markdown()
        );
    }

    #[test]
    fn test_footnotes() {
        let input = "Text with a footnote[^1].\n\n[^1]: Footnote content.";
        assert_html_eq!(
            push_html(input),
            "<p>Text with a footnote<sup class=\"footnote-reference\"><a href=\"#1\">1</a></sup>.</p>\
             <div class=\"footnote-definition\" id=\"1\"><sup class=\"footnote-definition-label\">1</sup>Footnote content.</div>",
            markdown()
        );
    }

    #[test]
    fn test_line_breaks() {
        assert_html_eq!(
            push_html("Line 1  \nLine 2"),
            "<p>Line 1<br>Line 2</p>",
            markdown()
        );
    }
}
