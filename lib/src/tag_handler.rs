use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, LinkType};
use std::iter::Peekable;

use crate::renderer_state::{ListType, RendererState};
use crate::utils::{escape_href, escape_html};
use crate::HtmlConfig;

/// Trait for handling Markdown tag rendering to HTML
pub trait HtmlWriter {
    /// Write a string directly to the output
    fn write_str(&mut self, s: &str) {
        self.get_output().push_str(s);
    }

    /// Write HTML attributes for a given element
    fn write_attributes(&mut self, element: &str) {
        let mut attrs_string = String::new();

        if let Some(attrs) = self.get_config().attributes.element_attributes.get(element) {
            for (key, value) in attrs {
                attrs_string.push_str(&format!(" {}=\"{}\"", key, value));
            }
        }

        if !attrs_string.is_empty() {
            self.write_str(&attrs_string);
        }
    }

    fn get_config(&self) -> &HtmlConfig;

    fn get_output(&mut self) -> &mut String;

    fn get_state(&mut self) -> &mut RendererState;

    /// Check if a URL points to an external resource
    fn is_external_link(&self, url: &str) -> bool {
        // TODO - This is probably incorrect
        url.starts_with("http://") || url.starts_with("https://")
    }

    /// Convert a HeadingLevel to a numeric level
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
        let level_num = self.heading_level_to_u8(level);
        format!(
            "{}{}",
            self.get_config().elements.headings.id_prefix,
            level_num
        )
    }

    fn start_paragraph(&mut self) {
        // Don't create paragraph tags inside footnote definitions
        if !self.get_state().currently_in_footnote {
            self.write_str("<p");
            self.write_attributes("p");
            self.write_str(">");
        }
    }

    fn end_paragraph(&mut self) {
        if !self.get_state().currently_in_footnote {
            self.write_str("</p>");
        }
    }

    // Heading handlers need custom implementation for IDs and classes
    fn start_heading(&mut self, level: HeadingLevel, id: Option<&str>, classes: Vec<&str>) {
        let tag = format!("h{}", self.heading_level_to_u8(level));
        self.write_str(&format!("<{}", tag));

        if self.get_config().elements.headings.add_ids {
            let heading_id = match id {
                Some(id) => id.to_string(),
                None => self.generate_heading_id(level),
            };
            self.write_str(&format!(" id=\"{}\"", heading_id));
            self.get_state().heading_stack.push(heading_id);
        }

        let mut all_classes = Vec::new();
        let level_num = self.heading_level_to_u8(level);
        if let Some(level_class) = self
            .get_config()
            .elements
            .headings
            .level_classes
            .get(&level_num)
        {
            all_classes.push(level_class.clone());
        }
        all_classes.extend(classes.into_iter().map(|s| s.to_string()));

        if !all_classes.is_empty() {
            self.write_str(" class=\"");
            self.write_str(&all_classes.join(" "));
            self.write_str("\"");
        }

        self.write_attributes(&tag);
        self.write_str(">");
    }

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

    fn start_code_block(&mut self, kind: CodeBlockKind) {
        self.get_state().currently_in_code_block = true;
        self.write_str("<pre");
        self.write_attributes("pre");
        self.write_str("><code");

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
                    self.write_str(&format!(" class=\"language-{}\"", lang));
                }
            }
            CodeBlockKind::Indented => {
                if let Some(lang) = &self.get_config().elements.code_blocks.default_language {
                    self.write_str(&format!(" class=\"language-{}\"", lang));
                }
            }
        }

        self.write_attributes("code");
        self.write_str(">");
    }
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
    fn start_list(&mut self, first_number: Option<u64>) {
        match first_number {
            Some(n) => {
                self.get_state().numbers.push(n.try_into().unwrap());
                self.get_state()
                    .list_stack
                    .push(ListType::Ordered(n.try_into().unwrap()));
                self.write_str("<ol");
                if n != 1 {
                    self.write_str(&format!(" start=\"{}\"", n));
                }
                self.write_attributes("ol");
                self.write_str(">");
            }
            None => {
                self.get_state().list_stack.push(ListType::Unordered);
                self.write_str("<ul");
                self.write_attributes("ul");
                self.write_str(">");
            }
        }
    }
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

    fn start_table(&mut self, alignments: Vec<Alignment>) {
        self.get_state().table_state = super::renderer_state::TableState::InHeader;
        self.get_state().table_alignments = alignments;
        self.write_str("<table");
        self.write_attributes("table");
        self.write_str(">");
    }

    fn end_table(&mut self) {
        self.write_str("</tbody></table>");
    }

    fn start_table_head(&mut self) {
        self.get_state().table_cell_index = 0;
        self.write_str("<thead><tr>");
    }

    fn end_table_head(&mut self) {
        self.write_str("</tr></thead><tbody>");
    }

    fn start_table_row(&mut self) {
        self.get_state().table_cell_index = 0;
        // When starting a row after the header, we're in the body
        if self.get_state().table_state == super::renderer_state::TableState::InHeader {
            self.get_state().table_state = super::renderer_state::TableState::InBody;
        }
        self.write_str("<tr>");
    }

    fn end_table_row(&mut self) {
        self.write_str("</tr>");
    }

    fn start_table_cell(&mut self) {
        let tag = match self.get_state().table_state {
            super::renderer_state::TableState::InHeader => "th",
            _ => "td",
        };

        self.write_str("<");
        self.write_str(tag);
        let idx = self.get_state().table_cell_index;
        if let Some(alignment) = self.get_state().table_alignments.get(idx) {
            match alignment {
                Alignment::Left => self.write_str(" style=\"text-align: left\""),
                Alignment::Center => self.write_str(" style=\"text-align: center\""),
                Alignment::Right => self.write_str(" style=\"text-align: right\""),
                Alignment::None => {}
            }
        }

        self.write_attributes(tag);
        self.write_str(">");

        self.get_state().table_cell_index += 1;
    }
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
    fn start_link(&mut self, _link_type: LinkType, dest: &str, title: &str) {
        // FIXME
        // self.state.link_stack.push(link_type);

        self.write_str("<a href=\"");
        escape_href(self.get_output(), dest);
        self.write_str("\"");

        if !title.is_empty() {
            self.write_str(" title=\"");
            escape_html(self.get_output(), title);
            self.write_str("\"");
        }

        // Apply link options from config for external links
        if self.is_external_link(dest) {
            if self.get_config().elements.links.nofollow_external {
                self.write_str(" rel=\"nofollow\"");
            }
            if self.get_config().elements.links.open_external_blank {
                self.write_str(" target=\"_blank\"");
            }
        }

        self.write_attributes("a");
        self.write_str(">");
    }

    fn end_link(&mut self) {
        self.write_str("</a>");
    }

    // Image parsing requires lookahead to capture events that need to be parsed into `alt` attr
    fn start_image<'b, I>(
        &mut self,
        _link_type: LinkType,
        dest: &str,
        title: &str,
        iter: &mut Peekable<I>,
    ) where
        I: Iterator<Item = Event<'b>>,
    {
        // Start an image tag by writing the opening <img> element. Unlike other tag handlers,
        // this requires access to the parser's event iterator because the alt text content
        // appears as Text events *after* the Image tag rather than between Start/End events.
        // This means we need to peek ahead in the event stream to collect the alt text before
        // writing the tag.
        self.write_str("<img src=\"");
        escape_href(self.get_output(), dest);
        self.write_str("\"");

        self.write_str(r#" alt=""#);
        // Collect and write the alt text
        let alt_text = self.collect_alt_text(iter);
        escape_html(self.get_output(), &alt_text);
        self.write_str("\"");

        if !title.is_empty() {
            self.write_str(r#" title=""#);
            escape_html(self.get_output(), title);
            self.write_str("\"");
        }

        self.write_attributes("img");

        if self.get_config().html.xhtml_style {
            self.write_str(" />");
        } else {
            self.write_str(">");
        }
    }
    fn end_image(&mut self) {}

    fn start_footnote_reference(&mut self, name: &str) {
        self.write_str("<sup class=\"footnote-reference\"><a href=\"#");
        self.write_str(name);
        self.write_str("\">");
        self.write_str(name);
        self.write_str("</a></sup>");
    }

    fn end_footnote_reference(&mut self) {
        self.write_str("</sup>");
    }

    fn start_footnote_definition(&mut self, name: &str) {
        self.write_str("<div class=\"footnote-definition\" id=\"");
        self.write_str(name);
        self.write_str("\"><sup class=\"footnote-definition-label\">");
        self.write_str(name);
        self.write_str("</sup>");
        self.get_state().currently_in_footnote = true;
    }

    fn end_footnote_definition(&mut self) {
        self.write_str("</div>");
        self.get_state().currently_in_footnote = false;
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
        if self.get_config().html.break_on_newline {
            self.write_str("<br>");
        } else {
            self.write_str("\n");
        }
    }

    fn hard_break(&mut self) {
        self.write_str("<br>");
    }

    fn text(&mut self, text: &str) {
        if self.get_config().html.escape_html {
            let mut escaped = String::new();
            super::utils::escape_html(&mut escaped, text);
            self.write_str(&escaped);
        } else {
            self.write_str(text);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        output: String,
        config: HtmlConfig,
        state: RendererState,
    }

    impl TestHandler {
        fn new() -> Self {
            let mut config = HtmlConfig::default();
            config.html.break_on_newline = false;
            let state = RendererState::new();
            Self {
                output: String::new(),
                config,
                state,
            }
        }

        fn get_output(&self) -> &str {
            &self.output
        }
    }

    impl HtmlWriter for TestHandler {
        fn get_config(&self) -> &HtmlConfig {
            &self.config
        }

        fn get_output(&mut self) -> &mut String {
            &mut self.output
        }

        fn get_state(&mut self) -> &mut RendererState {
            &mut self.state
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
