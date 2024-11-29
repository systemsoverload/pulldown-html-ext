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

use pulldown_cmark::{Event, Parser};
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
            pulldown_cmark::Tag::Paragraph => self.writer.start_paragraph()?,
            pulldown_cmark::Tag::Heading(level, id, classes) => {
                self.writer.start_heading(level, id, classes)?
            }
            pulldown_cmark::Tag::BlockQuote => self.writer.start_blockquote()?,
            pulldown_cmark::Tag::CodeBlock(kind) => self.writer.start_code_block(kind)?,
            pulldown_cmark::Tag::List(start) => self.writer.start_list(start)?,
            pulldown_cmark::Tag::Item => self.writer.start_list_item()?,
            pulldown_cmark::Tag::FootnoteDefinition(name) => {
                self.writer.start_footnote_definition(&name)?
            }
            pulldown_cmark::Tag::Table(alignments) => self.writer.start_table(alignments)?,
            pulldown_cmark::Tag::TableHead => self.writer.start_table_head()?,
            pulldown_cmark::Tag::TableRow => self.writer.start_table_row()?,
            pulldown_cmark::Tag::TableCell => self.writer.start_table_cell()?,
            pulldown_cmark::Tag::Emphasis => self.writer.start_emphasis()?,
            pulldown_cmark::Tag::Strong => self.writer.start_strong()?,
            pulldown_cmark::Tag::Strikethrough => self.writer.start_strikethrough()?,
            pulldown_cmark::Tag::Link(link_type, dest, title) => {
                self.writer.start_link(link_type, &dest, &title)?
            }
            pulldown_cmark::Tag::Image(link_type, dest, title) => {
                self.writer.start_image(link_type, &dest, &title, iter)?
            }
        }
        Ok(())
    }

    fn handle_end(&mut self, tag: pulldown_cmark::Tag) -> Result<()> {
        match tag {
            pulldown_cmark::Tag::Paragraph => self.writer.end_paragraph()?,
            pulldown_cmark::Tag::Heading(level, ..) => self.writer.end_heading(level)?,
            pulldown_cmark::Tag::BlockQuote => self.writer.end_blockquote()?,
            pulldown_cmark::Tag::CodeBlock(_) => self.writer.end_code_block()?,
            pulldown_cmark::Tag::List(Some(_)) => self.writer.end_list(true)?,
            pulldown_cmark::Tag::List(None) => self.writer.end_list(false)?,
            pulldown_cmark::Tag::Item => self.writer.end_list_item()?,
            pulldown_cmark::Tag::FootnoteDefinition(_) => self.writer.end_footnote_definition()?,
            pulldown_cmark::Tag::Table(_) => self.writer.end_table()?,
            pulldown_cmark::Tag::TableHead => self.writer.end_table_head()?,
            pulldown_cmark::Tag::TableRow => self.writer.end_table_row()?,
            pulldown_cmark::Tag::TableCell => self.writer.end_table_cell()?,
            pulldown_cmark::Tag::Emphasis => self.writer.end_emphasis()?,
            pulldown_cmark::Tag::Strong => self.writer.end_strong()?,
            pulldown_cmark::Tag::Strikethrough => self.writer.end_strikethrough()?,
            pulldown_cmark::Tag::Link(..) => self.writer.end_link()?,
            pulldown_cmark::Tag::Image(..) => self.writer.end_image()?,
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

pub fn push_html(markdown: &str, config: &HtmlConfig) -> Result<String> {
    let mut output = String::new();
    write_html_fmt(&mut output, markdown, config)?;
    Ok(output)
}

pub fn write_html_fmt<W>(writer: W, markdown: &str, config: &HtmlConfig) -> Result<()>
where
    W: std::fmt::Write,
{
    let parser = Parser::new(markdown);
    let writer = DefaultHtmlWriter::new(FmtWriter(writer), config);
    let mut renderer = HtmlRenderer::new(writer);
    renderer.run(parser)
}

pub fn write_html_io<W>(writer: W, markdown: &str, config: &HtmlConfig) -> Result<()>
where
    W: std::io::Write,
{
    let parser = Parser::new(markdown);
    let writer = DefaultHtmlWriter::new(IoWriter(writer), config);
    let mut renderer = HtmlRenderer::new(writer);
    renderer.run(parser)
}

pub fn create_html_renderer<W: StrWrite, H: HtmlWriter<W>>(writer: H) -> HtmlRenderer<W, H> {
    HtmlRenderer::new(writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_compare_rs::assert_html_eq;

    #[test]
    fn test_write_html_fmt() {
        let config = HtmlConfig::default();
        let mut output = String::new();
        write_html_fmt(&mut output, "# Test", &config).unwrap();
        assert_html_eq!(output, r#"<h1 id="heading-1">Test</h1>"#);
    }

    #[test]
    fn test_write_html_io() {
        let config = HtmlConfig::default();
        let mut output = Vec::new();
        write_html_io(&mut output, "# Test", &config).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_html_eq!(result, r#"<h1 id="heading-1">Test</h1>"#);
    }
}
