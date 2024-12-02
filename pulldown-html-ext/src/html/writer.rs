use super::{ListContext, TableContext};
use crate::html::state::HtmlState;
use crate::html::HtmlError;
use crate::HtmlConfig;

use pulldown_cmark::{
    Alignment, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, MetadataBlockKind,
};
use pulldown_cmark_escape::{escape_href, escape_html, escape_html_body_text, StrWrite};
use std::iter::Peekable;

/// Trait for handling Markdown tag rendering to HTML
pub trait HtmlWriter<W: StrWrite> {
    /// Write a string directly to the output
    fn write_str(&mut self, s: &str) -> Result<(), HtmlError> {
        self.get_writer()
            .write_str(s)
            .map_err(|_| HtmlError::Write(std::fmt::Error))
    }

    /// Write HTML attributes for a given element
    fn write_attributes(&mut self, element: &str) -> Result<(), HtmlError> {
        let mut attrs_string = String::new();

        if let Some(attrs) = self.get_config().attributes.element_attributes.get(element) {
            for (key, value) in attrs {
                attrs_string.push_str(&format!(" {}=\"{}\"", key, value));
            }
        }

        if !attrs_string.is_empty() {
            self.write_str(&attrs_string)?;
        }
        Ok(())
    }

    fn get_config(&self) -> &HtmlConfig;

    fn get_writer(&mut self) -> &mut W;

    fn get_state(&mut self) -> &mut HtmlState;

    /// Check if a URL points to an external resource
    fn is_external_link(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    fn start_paragraph(&mut self) -> Result<(), HtmlError> {
        if !self.get_state().currently_in_footnote {
            self.write_str("<p")?;
            self.write_attributes("p")?;
            self.write_str(">")?;
        }
        Ok(())
    }

    fn end_paragraph(&mut self) -> Result<(), HtmlError> {
        if !self.get_state().currently_in_footnote {
            self.write_str("</p>")?;
        }
        Ok(())
    }

    fn start_heading(
        &mut self,
        level: HeadingLevel,
        id: Option<&str>,
        classes: &[CowStr],
        attrs: &Vec<(CowStr, Option<CowStr>)>,
    ) -> Result<(), HtmlError> {
        // Get all config values up front
        let level_num = level as u8;
        let add_ids = self.get_config().elements.headings.add_ids;
        let id_prefix = self.get_config().elements.headings.id_prefix.clone();
        let level_classes = self
            .get_config()
            .elements
            .headings
            .level_classes
            .get(&level_num)
            .cloned();

        // Start the heading tag
        self.write_str(&format!("<h{}", level_num))?;

        // Handle ID attribute
        if add_ids {
            let heading_id =
                id.map_or_else(|| format!("{}{}", id_prefix, level_num), |s| s.to_string());
            self.write_str(" id=\"")?;
            escape_html(self.get_writer(), &heading_id)
                .map_err(|_| HtmlError::Write(std::fmt::Error))?;
            self.write_str("\"")?;
            self.get_state().heading_stack.push(heading_id);
        }

        // Combine and handle classes
        let mut all_classes = Vec::new();
        if let Some(level_class) = level_classes {
            all_classes.push(level_class);
        }
        all_classes.extend(classes.iter().map(|s| s.to_string()));

        if !all_classes.is_empty() {
            self.write_str(" class=\"")?;
            escape_html(self.get_writer(), &all_classes.join(" "))
                .map_err(|_| HtmlError::Write(std::fmt::Error))?;
            self.write_str("\"")?;
        }

        // Handle additional attributes
        for (key, value) in attrs {
            self.write_str(" ")?;
            escape_html(self.get_writer(), key).map_err(|_| HtmlError::Write(std::fmt::Error))?;
            if let Some(val) = value {
                self.write_str("=\"")?;
                escape_html(self.get_writer(), val)
                    .map_err(|_| HtmlError::Write(std::fmt::Error))?;
                self.write_str("\"")?;
            }
        }

        // Add any configured element attributes
        self.write_attributes(&format!("h{}", level_num))?;

        // Close the opening tag
        self.write_str(">")
    }
    fn end_heading(&mut self, level: HeadingLevel) -> Result<(), HtmlError> {
        self.write_str(&format!("</{}>", level))
    }

    fn start_blockquote(&mut self) -> Result<(), HtmlError> {
        self.write_str("<blockquote")?;
        self.write_attributes("blockquote")?;
        self.write_str(">")?;
        Ok(())
    }

    fn end_blockquote(&mut self) -> Result<(), HtmlError> {
        self.write_str("</blockquote>")
    }

    fn start_code_block(&mut self, kind: CodeBlockKind) -> Result<(), HtmlError> {
        self.get_state().currently_in_code_block = true;
        self.write_str("<pre")?;
        self.write_attributes("pre")?;
        self.write_str("><code")?;

        match kind {
            CodeBlockKind::Fenced(info) => {
                let lang = if info.is_empty() {
                    self.get_config()
                        .elements
                        .code_blocks
                        .default_language
                        .as_deref()
                } else {
                    Some(&*info)
                };

                if let Some(lang) = lang {
                    self.write_str(&format!(" class=\"language-{}\"", lang))?;
                }
            }
            CodeBlockKind::Indented => {
                if let Some(lang) = &self.get_config().elements.code_blocks.default_language {
                    self.write_str(&format!(" class=\"language-{}\"", lang))?;
                }
            }
        }

        self.write_attributes("code")?;
        self.write_str(">")?;
        Ok(())
    }

    fn end_code_block(&mut self) -> Result<(), HtmlError> {
        self.write_str("</code></pre>")
    }

    fn start_inline_code(&mut self) -> Result<(), HtmlError> {
        self.write_str("<code")?;
        self.write_attributes("code")?;
        self.write_str(">")?;
        Ok(())
    }

    fn end_inline_code(&mut self) -> Result<(), HtmlError> {
        self.write_str("</code>")
    }

    fn start_list(&mut self, first_number: Option<u64>) -> Result<(), HtmlError> {
        match first_number {
            Some(n) => {
                self.get_state().numbers.push(n.try_into().unwrap());
                self.get_state()
                    .list_stack
                    .push(ListContext::Ordered(n.try_into().unwrap()));
                self.write_str("<ol")?;
                if n != 1 {
                    self.write_str(&format!(" start=\"{}\"", n))?;
                }
                self.write_attributes("ol")?;
                self.write_str(">")?;
            }
            None => {
                self.get_state().list_stack.push(ListContext::Unordered);
                self.write_str("<ul")?;
                self.write_attributes("ul")?;
                self.write_str(">")?;
            }
        }
        Ok(())
    }

    fn end_list(&mut self, ordered: bool) -> Result<(), HtmlError> {
        self.write_str(if ordered { "</ol>" } else { "</ul>" })
    }

    fn start_list_item(&mut self) -> Result<(), HtmlError> {
        self.write_str("<li")?;
        self.write_attributes("li")?;
        self.write_str(">")
    }

    fn end_list_item(&mut self) -> Result<(), HtmlError> {
        self.write_str("</li>")
    }

    fn start_table(&mut self, alignments: Vec<Alignment>) -> Result<(), HtmlError> {
        self.get_state().table_state = TableContext::InHeader;
        self.get_state().table_alignments = alignments;
        self.write_str("<table")?;
        self.write_attributes("table")?;
        self.write_str(">")
    }

    fn end_table(&mut self) -> Result<(), HtmlError> {
        self.write_str("</tbody></table>")
    }

    fn start_table_head(&mut self) -> Result<(), HtmlError> {
        self.get_state().table_cell_index = 0;
        self.write_str("<thead><tr>")
    }

    fn end_table_head(&mut self) -> Result<(), HtmlError> {
        self.write_str("</tr></thead><tbody>")
    }

    fn start_table_row(&mut self) -> Result<(), HtmlError> {
        self.get_state().table_cell_index = 0;
        if self.get_state().table_state == TableContext::InHeader {
            self.get_state().table_state = TableContext::InBody;
        }
        self.write_str("<tr>")
    }

    fn end_table_row(&mut self) -> Result<(), HtmlError> {
        self.write_str("</tr>")
    }

    fn start_table_cell(&mut self) -> Result<(), HtmlError> {
        let tag = match self.get_state().table_state {
            TableContext::InHeader => "th",
            _ => "td",
        };

        self.write_str("<")?;
        self.write_str(tag)?;
        let idx = self.get_state().table_cell_index;
        if let Some(alignment) = self.get_state().table_alignments.get(idx) {
            match alignment {
                Alignment::Left => self.write_str(" style=\"text-align: left\"")?,
                Alignment::Center => self.write_str(" style=\"text-align: center\"")?,
                Alignment::Right => self.write_str(" style=\"text-align: right\"")?,
                Alignment::None => {}
            }
        }

        self.write_attributes(tag)?;
        self.write_str(">")?;

        self.get_state().table_cell_index += 1;
        Ok(())
    }

    fn end_table_cell(&mut self) -> Result<(), HtmlError> {
        self.write_str("</td>")
    }

    fn start_emphasis(&mut self) -> Result<(), HtmlError> {
        self.write_str("<em")?;
        self.write_attributes("em")?;
        self.write_str(">")
    }

    fn end_emphasis(&mut self) -> Result<(), HtmlError> {
        self.write_str("</em>")
    }

    fn start_strong(&mut self) -> Result<(), HtmlError> {
        self.write_str("<strong")?;
        self.write_attributes("strong")?;
        self.write_str(">")
    }

    fn end_strong(&mut self) -> Result<(), HtmlError> {
        self.write_str("</strong>")
    }

    fn start_strikethrough(&mut self) -> Result<(), HtmlError> {
        self.write_str("<del")?;
        self.write_attributes("del")?;
        self.write_str(">")
    }

    fn end_strikethrough(&mut self) -> Result<(), HtmlError> {
        self.write_str("</del>")
    }

    fn start_link(
        &mut self,
        _link_type: LinkType,
        dest: &str,
        title: &str,
    ) -> Result<(), HtmlError> {
        self.write_str("<a href=\"")?;
        escape_href(self.get_writer(), dest).map_err(|_| HtmlError::Write(std::fmt::Error))?;

        if !title.is_empty() {
            self.write_str("\" title=\"")?;
            escape_html(self.get_writer(), title).map_err(|_| HtmlError::Write(std::fmt::Error))?;
        }

        if self.is_external_link(dest) {
            if self.get_config().elements.links.nofollow_external {
                self.write_str("\" rel=\"nofollow")?;
            }
            if self.get_config().elements.links.open_external_blank {
                self.write_str("\" target=\"_blank")?;
            }
        }

        self.write_str("\"")?;
        self.write_attributes("a")?;
        self.write_str(">")
    }

    fn end_link(&mut self) -> Result<(), HtmlError> {
        self.write_str("</a>")
    }

    fn start_image<'a, I>(
        &mut self,
        _link_type: LinkType,
        dest: &str,
        title: &str,
        iter: &mut Peekable<I>,
    ) -> Result<(), HtmlError>
    where
        I: Iterator<Item = Event<'a>>,
    {
        self.write_str("<img src=\"")?;
        escape_href(self.get_writer(), dest).map_err(|_| HtmlError::Write(std::fmt::Error))?;
        self.write_str("\" alt=\"")?;

        let alt_text = self.collect_alt_text(iter);
        escape_html(self.get_writer(), &alt_text).map_err(|_| HtmlError::Write(std::fmt::Error))?;
        self.write_str("\"")?;

        if !title.is_empty() {
            self.write_str(" title=\"")?;
            escape_html(self.get_writer(), title).map_err(|_| HtmlError::Write(std::fmt::Error))?;
            self.write_str("\"")?;
        }

        self.write_attributes("img")?;

        if self.get_config().html.xhtml_style {
            self.write_str(" />")?;
        } else {
            self.write_str(">")?;
        }
        Ok(())
    }

    fn end_image(&mut self) -> Result<(), HtmlError> {
        Ok(())
    }

    fn footnote_reference(&mut self, name: &str) -> Result<(), HtmlError> {
        self.write_str("<sup class=\"footnote-reference\"><a href=\"#")?;
        self.write_str(name)?;
        self.write_str("\">")?;
        self.write_str(name)?;
        self.write_str("</a></sup>")
    }

    fn start_footnote_definition(&mut self, name: &str) -> Result<(), HtmlError> {
        self.write_str("<div class=\"footnote-definition\" id=\"")?;
        self.write_str(name)?;
        self.write_str("\"><sup class=\"footnote-definition-label\">")?;
        self.write_str(name)?;
        self.get_state().currently_in_footnote = true;
        self.write_str("</sup>")?;

        Ok(())
    }
    fn end_footnote_definition(&mut self) -> Result<(), HtmlError> {
        self.write_str("</div>")?;
        self.get_state().currently_in_footnote = false;
        Ok(())
    }

    // Task list handlers
    fn task_list_item(&mut self, checked: bool) -> Result<(), HtmlError> {
        self.write_str("<input type=\"checkbox\" disabled")?;
        if checked {
            self.write_str(" checked")?;
        }
        self.write_str(">")
    }

    // Special elements - simple HTML
    fn horizontal_rule(&mut self) -> Result<(), HtmlError> {
        self.write_str("<hr>")
    }

    fn soft_break(&mut self) -> Result<(), HtmlError> {
        if self.get_config().html.break_on_newline {
            self.write_str("<br>")
        } else {
            self.write_str("\n")
        }
    }

    fn hard_break(&mut self) -> Result<(), HtmlError> {
        self.write_str("<br>")
    }

    fn text(&mut self, text: &str) -> Result<(), HtmlError> {
        if self.get_config().html.escape_html {
            escape_html_body_text(self.get_writer(), text)
                .map_err(|_| HtmlError::Write(std::fmt::Error))?;
        } else {
            self.write_str(text)?;
        }
        Ok(())
    }

    fn start_definition_list(&mut self) -> Result<(), HtmlError> {
        self.write_str("<dl")?;
        self.write_attributes("dl")?;
        self.write_str(">")
    }

    fn end_definition_list(&mut self) -> Result<(), HtmlError> {
        self.write_str("</dl>")
    }

    fn start_definition_list_title(&mut self) -> Result<(), HtmlError> {
        self.write_str("<dt")?;
        self.write_attributes("dt")?;
        self.write_str(">")
    }

    fn end_definition_list_title(&mut self) -> Result<(), HtmlError> {
        self.write_str("</dt>")
    }

    fn start_definition_list_definition(&mut self) -> Result<(), HtmlError> {
        self.write_str("<dd")?;
        self.write_attributes("dd")?;
        self.write_str(">")
    }

    fn end_definition_list_definition(&mut self) -> Result<(), HtmlError> {
        self.write_str("</dd>")
    }

    fn start_metadata_block(
        &mut self,
        _metadata_type: &MetadataBlockKind,
    ) -> Result<(), HtmlError> {
        // TODO - implement this
        //self.get_state().in_non_writing_block = true
        Ok(())
    }
    fn end_metadata_block(&mut self) -> Result<(), HtmlError> {
        // TODO - implement this
        //self.get_state().in_non_writing_block = false
        Ok(())
    }

    fn html_raw(&mut self, html: &CowStr) -> Result<(), HtmlError> {
        self.write_str(html)
    }

    fn collect_alt_text<'a, I>(&self, iter: &mut Peekable<I>) -> String
    where
        I: Iterator<Item = Event<'a>>,
    {
        let mut alt = String::new();
        let mut nest = 0;

        for event in iter.by_ref() {
            match event {
                Event::Start(_) => nest += 1,
                Event::End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Event::Text(text) => {
                    alt.push_str(&text);
                }
                Event::Code(text) => {
                    alt.push_str(&text);
                }
                Event::SoftBreak | Event::HardBreak => {
                    alt.push(' ');
                }
                _ => {}
            }
        }
        alt
    }
}

// Default bases to derive from, implements the default getter methods.
pub struct HtmlWriterBase<W: StrWrite> {
    writer: W,
    config: HtmlConfig,
    state: HtmlState,
}

impl<W: StrWrite> HtmlWriterBase<W> {
    pub fn new(writer: W, config: HtmlConfig) -> Self {
        Self {
            writer,
            config,
            state: HtmlState::new(),
        }
    }
}

impl<W: StrWrite> HtmlWriter<W> for HtmlWriterBase<W> {
    fn get_writer(&mut self) -> &mut W {
        &mut self.writer
    }

    fn get_config(&self) -> &HtmlConfig {
        &self.config
    }

    fn get_state(&mut self) -> &mut HtmlState {
        &mut self.state
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pulldown_cmark_escape::FmtWriter;

    struct TestHandler<W: StrWrite> {
        writer: W,
        config: HtmlConfig,
        state: HtmlState,
    }

    impl<W: StrWrite> TestHandler<W> {
        fn new(writer: W) -> Self {
            let mut config = HtmlConfig::default();
            config.html.break_on_newline = false;
            Self {
                writer,
                config,
                state: HtmlState::new(),
            }
        }
    }

    impl<W: StrWrite> HtmlWriter<W> for TestHandler<W> {
        fn get_writer(&mut self) -> &mut W {
            &mut self.writer
        }
        fn get_config(&self) -> &HtmlConfig {
            &self.config
        }

        fn get_state(&mut self) -> &mut HtmlState {
            &mut self.state
        }
    }

    #[test]
    fn test_paragraph() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.start_paragraph().unwrap();
        handler.text("Hello world").unwrap();
        handler.end_paragraph().unwrap();
        assert_eq!(output, "<p>Hello world</p>");
    }

    #[test]
    fn test_blockquote() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.start_blockquote().unwrap();
        handler.text("Quote").unwrap();
        handler.end_blockquote().unwrap();
        assert_eq!(output, "<blockquote>Quote</blockquote>");
    }

    #[test]
    fn test_emphasis() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.start_emphasis().unwrap();
        handler.text("emphasized").unwrap();
        handler.end_emphasis().unwrap();
        assert_eq!(output, "<em>emphasized</em>");
    }

    #[test]
    fn test_strong() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.start_strong().unwrap();
        handler.text("bold").unwrap();
        handler.end_strong().unwrap();
        assert_eq!(output, "<strong>bold</strong>");
    }

    #[test]
    fn test_strikethrough() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.start_strikethrough().unwrap();
        handler.text("strike").unwrap();
        handler.end_strikethrough().unwrap();
        assert_eq!(output, "<del>strike</del>");
    }

    #[test]
    fn test_inline_code() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.start_inline_code().unwrap();
        handler.text("code").unwrap();
        handler.end_inline_code().unwrap();
        assert_eq!(output, "<code>code</code>");
    }

    #[test]
    fn test_line_breaks() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.soft_break().unwrap();
        handler.hard_break().unwrap();
        assert_eq!(output, "\n<br>");
    }

    #[test]
    fn test_horizontal_rule() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.horizontal_rule().unwrap();
        assert_eq!(output, "<hr>");
    }

    #[test]
    fn test_task_list() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.task_list_item(true).unwrap();
        handler.text("Done").unwrap();

        assert_eq!(output, "<input type=\"checkbox\" disabled checked>Done");

        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.task_list_item(false).unwrap();
        handler.text("Todo").unwrap();

        assert_eq!(output, "<input type=\"checkbox\" disabled>Todo");
    }

    #[test]
    fn test_footnote_definition() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.start_footnote_definition("1").unwrap();
        handler.text("Footnote content").unwrap();
        handler.end_footnote_definition().unwrap();
        assert_eq!(
            output,
            "<div class=\"footnote-definition\" id=\"1\">\
             <sup class=\"footnote-definition-label\">1</sup>\
             Footnote content</div>"
        );
    }

    #[test]
    fn test_list_endings() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.end_list(true).unwrap();
        assert_eq!(output, "</ol>");

        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.end_list(false).unwrap();
        assert_eq!(output, "</ul>");
    }

    #[test]
    fn test_table_structure() {
        let mut output = String::new();
        let mut handler = TestHandler::new(FmtWriter(&mut output));
        handler.end_table_head().unwrap();
        handler.end_table_row().unwrap();
        handler.end_table_cell().unwrap();
        handler.end_table().unwrap();
        assert_eq!(output, "</tr></thead><tbody></tr></td></tbody></table>");
    }
}
