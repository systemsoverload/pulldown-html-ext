//! HTML rendering functionality for Markdown content.
//!
//! This module provides configurable HTML rendering capabilities built on top
//! of pulldown-cmark's event model. It supports customized rendering of HTML
//! elements, attribute handling, and state management during rendering.

mod config;
mod default;
mod error;
mod state;
mod writer;

#[cfg(feature = "syntect")]
mod syntect;
#[cfg(feature = "syntect")]
pub use self::syntect::{
    push_html_with_highlighting, SyntectConfig, SyntectConfigStyle, SyntectWriter,
};
use pulldown_cmark::{Event, Tag, TagEnd};
use pulldown_cmark_escape::{FmtWriter, IoWriter, StrWrite};
use std::iter::Peekable;

pub use self::config::{
    AttributeMappings, CodeBlockOptions, ElementOptions, HeadingOptions, HtmlConfig, HtmlOptions,
    LinkOptions,
};
pub use self::default::DefaultHtmlWriter;
pub use self::error::HtmlError;
pub use self::state::{HtmlState, ListContext, TableContext};
pub use self::writer::HtmlWriter;

pub type Result<T> = std::result::Result<T, HtmlError>;

/// Core renderer that processes Markdown events into HTML
use std::marker::PhantomData;

pub struct HtmlRenderer<W: StrWrite, H: HtmlWriter<W>> {
    pub(crate) writer: H,
    _phantom: PhantomData<W>,
}

impl<W: StrWrite, H: HtmlWriter<W>> HtmlRenderer<W, H> {
    pub fn new(writer: H) -> Self {
        Self {
            writer,
            _phantom: PhantomData,
        }
    }

    pub fn run<'a, I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Event<'a>>,
    {
        let mut iter = iter.peekable();
        while let Some(event) = iter.next() {
            match event {
                Event::Start(tag) => self.handle_start(&mut iter, tag)?,
                Event::End(tag) => self.handle_end(tag)?,
                Event::Text(text) => self.writer.text(&text)?,
                Event::Code(text) => self.handle_inline_code(&text)?,
                Event::Html(html) => self.writer.write_str(&html)?,
                Event::SoftBreak => self.writer.soft_break()?,
                Event::HardBreak => self.writer.hard_break()?,
                Event::Rule => self.writer.horizontal_rule()?,
                Event::FootnoteReference(name) => self.writer.footnote_reference(&name)?,
                Event::TaskListMarker(checked) => self.writer.task_list_item(checked)?,
                Event::InlineMath(_) | Event::DisplayMath(_) | Event::InlineHtml(_) => todo!(),
            }
        }
        Ok(())
    }

    fn handle_start<'a, I>(
        &mut self,
        iter: &mut Peekable<I>,
        tag: pulldown_cmark::Tag<'a>,
    ) -> Result<()>
    where
        I: Iterator<Item = Event<'a>>,
    {
        match tag {
            Tag::Paragraph => self.writer.start_paragraph()?,
            Tag::Heading {
                level,
                id,
                classes,
                attrs,
            } => self
                .writer
                .start_heading(level, id.as_deref(), &classes, &attrs)?,
            Tag::BlockQuote(_) => self.writer.start_blockquote()?,
            Tag::CodeBlock(kind) => self.writer.start_code_block(kind)?,
            Tag::List(start) => self.writer.start_list(start)?,
            Tag::Item => self.writer.start_list_item()?,
            Tag::FootnoteDefinition(name) => self.writer.start_footnote_definition(&name)?,
            Tag::Table(alignments) => self.writer.start_table(alignments)?,
            Tag::TableHead => self.writer.start_table_head()?,
            Tag::TableRow => self.writer.start_table_row()?,
            Tag::TableCell => self.writer.start_table_cell()?,
            Tag::Emphasis => self.writer.start_emphasis()?,
            Tag::Strong => self.writer.start_strong()?,
            Tag::Strikethrough => self.writer.start_strikethrough()?,
            Tag::Link {
                link_type,
                dest_url,
                title,
                id: _,
            } => self.writer.start_link(link_type, &dest_url, &title)?,
            Tag::Image {
                link_type,
                dest_url,
                title,
                id: _,
            } => self
                .writer
                .start_image(link_type, &dest_url, &title, iter)?,

            Tag::DefinitionList => self.writer.start_definition_list()?,
            Tag::DefinitionListTitle => self.writer.start_definition_list_title()?,
            Tag::DefinitionListDefinition => self.writer.start_definition_list_definition()?,

            Tag::MetadataBlock(kind) => self.writer.start_metadata_block(&kind)?,
            Tag::HtmlBlock => (),
        }
        Ok(())
    }

    fn handle_end(&mut self, tag: TagEnd) -> Result<()> {
        match tag {
            TagEnd::Paragraph => self.writer.end_paragraph()?,
            TagEnd::Heading(level) => self.writer.end_heading(level)?,
            TagEnd::BlockQuote(_) => self.writer.end_blockquote()?,
            TagEnd::CodeBlock => self.writer.end_code_block()?,
            TagEnd::List(b) => self.writer.end_list(b)?,
            // TagEnd::List(None) => self.writer.end_list(false)?,
            TagEnd::Item => self.writer.end_list_item()?,
            TagEnd::FootnoteDefinition => self.writer.end_footnote_definition()?,
            TagEnd::Table => self.writer.end_table()?,
            TagEnd::TableHead => self.writer.end_table_head()?,
            TagEnd::TableRow => self.writer.end_table_row()?,
            TagEnd::TableCell => self.writer.end_table_cell()?,
            TagEnd::Emphasis => self.writer.end_emphasis()?,
            TagEnd::Strong => self.writer.end_strong()?,
            TagEnd::Strikethrough => self.writer.end_strikethrough()?,
            TagEnd::Link {} => self.writer.end_link()?,
            TagEnd::Image {} => self.writer.end_image()?,
            TagEnd::DefinitionList => self.writer.end_definition_list()?,
            TagEnd::DefinitionListTitle => self.writer.end_definition_list_title()?,
            TagEnd::DefinitionListDefinition => self.writer.end_definition_list_title()?,

            TagEnd::MetadataBlock(_) => self.writer.end_metadata_block()?,
            TagEnd::HtmlBlock => (),
        }
        Ok(())
    }

    fn handle_inline_code(&mut self, text: &str) -> Result<()> {
        self.writer.start_inline_code()?;
        self.writer.text(text)?;
        self.writer.end_inline_code()?;
        Ok(())
    }
}

/// Renders markdown events to HTML and appends to the provided string
///
/// # Arguments
///
/// * `output` - String buffer to append the HTML output to
/// * `iter` - Iterator of markdown events to process
/// * `config` - Configuration for HTML rendering
///
/// # Example
///
/// ```rust
/// use pulldown_cmark::Parser;
/// use pulldown_html_ext::{HtmlConfig, push_html};
///
/// let markdown = "# Hello\n* Item 1\n* Item 2";
/// let parser = Parser::new(markdown);
/// let mut output = String::new();
/// let config = HtmlConfig::default();
///
/// push_html(&mut output, parser, &config).unwrap();
/// assert!(output.contains("<h1"));
/// ```
pub fn push_html<'a, I>(output: &mut String, iter: I, config: &HtmlConfig) -> Result<()>
where
    I: Iterator<Item = Event<'a>>,
{
    write_html_fmt(output, iter, config)
}

/// Renders markdown events to HTML using a fmt::Write implementation
///
/// # Arguments
///
/// * `writer` - Any type implementing fmt::Write
/// * `iter` - Iterator of markdown events to process
/// * `config` - Configuration for HTML rendering
pub fn write_html_fmt<'a, W, I>(writer: W, iter: I, config: &HtmlConfig) -> Result<()>
where
    W: std::fmt::Write,
    I: Iterator<Item = Event<'a>>,
{
    let writer = DefaultHtmlWriter::new(FmtWriter(writer), config);
    let mut renderer = HtmlRenderer::new(writer);
    renderer.run(iter)
}

/// Renders markdown events to HTML using an io::Write implementation
///
/// # Arguments
///
/// * `writer` - Any type implementing io::Write
/// * `iter` - Iterator of markdown events to process
/// * `config` - Configuration for HTML rendering
pub fn write_html_io<'a, W, I>(writer: W, iter: I, config: &HtmlConfig) -> Result<()>
where
    W: std::io::Write,
    I: Iterator<Item = Event<'a>>,
{
    let writer = DefaultHtmlWriter::new(IoWriter(writer), config);
    let mut renderer = HtmlRenderer::new(writer);
    renderer.run(iter)
}

pub fn create_html_renderer<W: StrWrite, H: HtmlWriter<W>>(writer: H) -> HtmlRenderer<W, H> {
    HtmlRenderer::new(writer)
}

#[cfg(test)]
mod tests_mod {
    use super::*;
    use html_compare_rs::assert_html_eq;
    use pulldown_cmark::Parser;

    #[test]
    fn test_push_html() {
        let markdown = "# Hello\n\nThis is a test.";
        let parser = Parser::new(markdown);
        let mut output = String::new();
        let config = HtmlConfig::default();

        push_html(&mut output, parser, &config).unwrap();

        assert_html_eq!(
            output,
            r#"<h1 id="heading-1">Hello</h1><p>This is a test.</p>"#
        );
    }

    #[test]
    fn test_write_html_fmt() {
        let markdown = "# Test\n* Item 1\n* Item 2";
        let parser = Parser::new(markdown);
        let mut output = String::new();
        let config = HtmlConfig::default();

        write_html_fmt(&mut output, parser, &config).unwrap();

        assert_html_eq!(
            output,
            r#"<h1 id="heading-1">Test</h1><ul><li>Item 1</li><li>Item 2</li></ul>"#
        );
    }

    #[test]
    fn test_write_html_io() {
        let markdown = "# Test";
        let parser = Parser::new(markdown);
        let mut output = Vec::new();
        let config = HtmlConfig::default();

        write_html_io(&mut output, parser, &config).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_html_eq!(result, r#"<h1 id="heading-1">Test</h1>"#);
    }

    #[test]
    fn test_with_syntax_highlighting() {
        let markdown = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let parser = Parser::new(markdown);
        let mut output = String::new();
        let config = HtmlConfig::default();

        push_html(&mut output, parser, &config).unwrap();

        assert!(output.contains(r#"<code class="language-rust">"#));
        assert!(output.contains("println"));
    }
}
