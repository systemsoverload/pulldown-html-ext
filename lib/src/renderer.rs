use crate::config::RendererConfig;
use crate::default_handler::DefaultTagHandler;
use crate::tag_handler::TagHandler;
use pulldown_cmark::{Event, Tag};

pub struct Renderer<H: TagHandler> {
    handler: H,
}
impl<H: TagHandler> Renderer<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    pub fn run<'a, I>(&mut self, iter: I)
    where
        I: Iterator<Item = Event<'a>>,
    {
        for event in iter {
            match event {
                Event::Start(tag) => self.handle_start(tag),
                Event::End(tag) => self.handle_end(tag),
                Event::Text(text) => self.text(&text),
                Event::Code(text) => self.inline_code(&text),
                Event::Html(html) => self.raw_html(&html),
                Event::SoftBreak => self.soft_break(),
                Event::HardBreak => self.hard_break(),
                Event::Rule => self.horizontal_rule(),
                Event::FootnoteReference(name) => self.footnote_reference(&name),
                Event::TaskListMarker(checked) => self.task_list_marker(checked),
            }
        }
    }

    fn handle_start(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.handler.start_paragraph(),
            Tag::Heading(level, id, classes) => self.handler.start_heading(level, id, classes),
            Tag::BlockQuote => self.handler.start_blockquote(),
            Tag::CodeBlock(kind) => self.handler.start_code_block(kind),
            Tag::List(first_number) => self.handler.start_list(first_number),
            Tag::Item => self.handler.start_list_item(),
            Tag::FootnoteDefinition(name) => self.handler.start_footnote_definition(&name),
            Tag::Table(alignments) => self.handler.start_table(alignments),
            Tag::TableHead => self.handler.start_table_head(),
            Tag::TableRow => self.handler.start_table_row(),
            Tag::TableCell => self.handler.start_table_cell(),
            Tag::Emphasis => self.handler.start_emphasis(),
            Tag::Strong => self.handler.start_strong(),
            Tag::Strikethrough => self.handler.start_strikethrough(),
            Tag::Link(link_type, dest, title) => self.handler.start_link(link_type, &dest, &title),
            Tag::Image(link_type, dest, title) => {
                self.handler.start_image(link_type, &dest, &title)
            }
        }
    }

    fn handle_end(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.handler.write_str("</p>"),
            Tag::Heading(level, ..) => self.handler.write_str(&format!("</h{}>", level)),
            Tag::BlockQuote => self.handler.write_str("</blockquote>"),
            Tag::CodeBlock(_) => {
                self.handler.write_str("</code></pre>");
            }
            Tag::List(Some(_)) => self.handler.write_str("</ol>"),
            Tag::List(None) => self.handler.write_str("</ul>"),
            Tag::Item => self.handler.write_str("</li>"),
            Tag::FootnoteDefinition(_) => self.handler.write_str("</div>"),
            Tag::Table(_) => {
                self.handler.write_str("</tbody></table>");
            }
            Tag::TableHead => {
                self.handler.write_str("</tr></thead><tbody>");
            }
            Tag::TableRow => self.handler.write_str("</tr>"),
            Tag::TableCell => {
                // We determine the closing tag based on the current table state
                self.handler.write_str("</td>");
            }
            Tag::Emphasis => self.handler.write_str("</em>"),
            Tag::Strong => self.handler.write_str("</strong>"),
            Tag::Strikethrough => self.handler.write_str("</del>"),
            Tag::Link(..) => self.handler.write_str("</a>"),
            Tag::Image(..) => {} // No end tag needed for images
        }
    }

    fn text(&mut self, text: &str) {
        self.handler.write_str(text);
    }

    fn inline_code(&mut self, text: &str) {
        self.handler.write_str("<code>");
        self.handler.write_str(text);
        self.handler.write_str("</code>");
    }

    fn raw_html(&mut self, html: &str) {
        self.handler.write_str(html);
    }

    fn soft_break(&mut self) {
        self.handler.write_str("\n");
    }

    fn hard_break(&mut self) {
        self.handler.write_str("<br>");
    }

    fn horizontal_rule(&mut self) {
        self.handler.write_str("<hr>");
    }

    fn footnote_reference(&mut self, name: &str) {
        self.handler
            .write_str("<sup class=\"footnote-reference\"><a href=\"#");
        self.handler.write_str(name);
        self.handler.write_str("\">");
        self.handler.write_str(name);
        self.handler.write_str("</a></sup>");
    }

    fn task_list_marker(&mut self, checked: bool) {
        self.handler.write_str("<input type=\"checkbox\" disabled");
        if checked {
            self.handler.write_str(" checked");
        }
        self.handler.write_str(">");
    }
}

// Helper function to create a renderer with the default handler
pub fn create_renderer<'a>(
    output: &'a mut String,
    config: &'a RendererConfig,
) -> Renderer<DefaultTagHandler<'a>> {
    let handler = DefaultTagHandler::new(output, config);
    Renderer::new(handler)
}
