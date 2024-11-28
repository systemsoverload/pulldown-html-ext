// use crate::tag_handler::HtmlWriter;
// use pulldown_cmark::{Event, Tag};
// use std::iter::Peekable;

// pub struct Renderer<H: HtmlWriter> {
//     handler: H,
// }

// impl<H: HtmlWriter> Renderer<H> {
//     pub fn new(handler: H) -> Self {
//         Self { handler }
//     }

//     pub fn run<'a, I>(&mut self, iter: I)
//     where
//         I: Iterator<Item = Event<'a>>,
//     {
//         let mut iter = iter.peekable();
//         while let Some(event) = iter.next() {
//             // TODO - verbose logging options to monitor event stream?
//             match event {
//                 Event::Start(tag) => self.handle_start(&mut iter, tag),
//                 Event::End(tag) => self.handle_end(tag),
//                 Event::Text(text) => self.handler.text(&text),
//                 Event::Code(text) => self.inline_code(&text),
//                 Event::Html(html) => self.handler.write_str(&html),
//                 Event::SoftBreak => self.handler.soft_break(),
//                 Event::HardBreak => self.handler.hard_break(),
//                 Event::Rule => self.handler.horizontal_rule(),
//                 Event::FootnoteReference(name) => self.handler.start_footnote_reference(&name),
//                 Event::TaskListMarker(checked) => self.handler.start_task_list_item(checked),
//             }
//         }
//     }

//     fn handle_start<'a, I>(&mut self, iter: &mut Peekable<I>, tag: Tag)
//     where
//         I: Iterator<Item = Event<'a>>,
//     {
//         match tag {
//             Tag::Paragraph => self.handler.start_paragraph(),
//             Tag::Heading(level, id, classes) => self.handler.start_heading(level, id, classes),
//             Tag::BlockQuote => self.handler.start_blockquote(),
//             Tag::CodeBlock(kind) => self.handler.start_code_block(kind),
//             Tag::List(first_number) => self.handler.start_list(first_number),
//             Tag::Item => self.handler.start_list_item(),
//             Tag::FootnoteDefinition(name) => self.handler.start_footnote_definition(&name),
//             Tag::Table(alignments) => self.handler.start_table(alignments),
//             Tag::TableHead => self.handler.start_table_head(),
//             Tag::TableRow => self.handler.start_table_row(),
//             Tag::TableCell => self.handler.start_table_cell(),
//             Tag::Emphasis => self.handler.start_emphasis(),
//             Tag::Strong => self.handler.start_strong(),
//             Tag::Strikethrough => self.handler.start_strikethrough(),
//             Tag::Link(link_type, dest, title) => self.handler.start_link(link_type, &dest, &title),
//             Tag::Image(link_type, dest, title) => {
//                 self.handler.start_image(link_type, &dest, &title, iter);
//             }
//         }
//     }

//     fn handle_end(&mut self, tag: Tag) {
//         println!("End tag: {:?}", tag);
//         match tag {
//             Tag::Paragraph => self.handler.end_paragraph(),
//             Tag::Heading(level, ..) => self.handler.end_heading(level),
//             Tag::BlockQuote => self.handler.end_blockquote(),
//             Tag::CodeBlock(_) => self.handler.end_code_block(),
//             Tag::List(Some(_)) => self.handler.end_list(true),
//             Tag::List(None) => self.handler.end_list(false),
//             Tag::Item => self.handler.end_list_item(),
//             Tag::FootnoteDefinition(_) => self.handler.end_footnote_definition(),
//             Tag::Table(_) => self.handler.end_table(),
//             Tag::TableHead => self.handler.end_table_head(),
//             Tag::TableRow => self.handler.end_table_row(),
//             Tag::TableCell => self.handler.end_table_cell(),
//             Tag::Emphasis => self.handler.end_emphasis(),
//             Tag::Strong => self.handler.end_strong(),
//             Tag::Strikethrough => self.handler.end_strikethrough(),
//             Tag::Link(..) => self.handler.end_link(),
//             Tag::Image(..) => self.handler.end_image(),
//         }
//     }

//     // TODO - move to `handle_code` in HtmlWriter trait
//     fn inline_code(&mut self, text: &str) {
//         self.handler.start_inline_code();
//         self.handler.text(text);
//         self.handler.end_inline_code();
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::config::HtmlConfig;
//     use crate::DefaultHtmlWriter;
//     use html_compare_rs::{assert_html_eq, presets::markdown};
//     use pulldown_cmark::{Options, Parser};

//     fn push_html_with_config(input: &str, config: &HtmlConfig) -> String {
//         let mut output = String::new();
//         let handler = DefaultHtmlWriter::new(&mut output, config);
//         let mut renderer = Renderer::new(handler);
//         renderer.run(Parser::new_ext(input, Options::all()));
//         output
//     }

//     fn push_html(input: &str) -> String {
//         push_html_with_config(input, &HtmlConfig::default())
//     }

//     #[test]
//     fn test_basic_text_rendering() {
//         assert_html_eq!(
//             push_html("Hello, world!"),
//             "<p>Hello, world!</p>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_emphasis_and_strong() {
//         assert_html_eq!(
//             push_html("*italic* and **bold** text"),
//             "<p><em>italic</em> and <strong>bold</strong> text</p>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_nested_formatting() {
//         assert_html_eq!(
//             push_html("***bold italic*** and **bold *italic* mix**"),
//              "<p><em><strong>bold italic</strong></em> and <strong>bold <em>italic</em> mix</strong></p>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_headings() {
//         let input = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
//         assert_html_eq!(
//             push_html(input),
//             "<h1 id=\"heading-1\">H1</h1>\
//              <h2 id=\"heading-2\">H2</h2>\
//              <h3 id=\"heading-3\">H3</h3>\
//              <h4 id=\"heading-4\">H4</h4>\
//              <h5 id=\"heading-5\">H5</h5>\
//              <h6 id=\"heading-6\">H6</h6>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_lists() {
//         let input = "- Item 1\n- Item 2\n  - Nested 1\n  - Nested 2\n- Item 3";
//         assert_html_eq!(
//             push_html(input),
//             "<ul><li>Item 1</li>\
//              <li>Item 2\
//              <ul><li>Nested 1</li>\
//              <li>Nested 2</li></ul></li>\
//              <li>Item 3</li></ul>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_ordered_lists() {
//         let input = "1. First\n2. Second\n   1. Nested\n   2. Items\n3. Third";
//         assert_html_eq!(
//             push_html(input),
//             "<ol><li>First</li>\
//              <li>Second\
//              <ol><li>Nested</li>\
//              <li>Items</li></ol></li>\
//              <li>Third</li></ol>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_code_blocks() {
//         let input = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
//         assert_html_eq!(
//             push_html(input),
//             "<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello\");\n}</code></pre>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_inline_code() {
//         assert_html_eq!(
//             push_html("Use the `println!` macro"),
//             "<p>Use the <code>println!</code> macro</p>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_blockquotes() {
//         let input = "> First level\n>> Second level\n\n> Back to first";
//         assert_html_eq!(
//             push_html(input),
//             "<blockquote><p>First level</p><blockquote><p>Second level</p></blockquote></blockquote><blockquote><p>Back to first</p></blockquote>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_links() {
//         assert_html_eq!(
//             push_html("[Example](https://example.com \"Title\")"),
//             r#"<p><a href="https://example.com" title="Title" rel="nofollow" target="_blank">Example</a></p>"#,
//             markdown()
//         );
//     }

//     #[test]
//     fn test_images() {
//         assert_html_eq!(
//             push_html("![Alt text](image.jpg \"Image title\")"),
//             "<p><img src=\"image.jpg\" alt=\"Alt text\" title=\"Image title\"></p>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_tables() {
//         let input = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
//         assert_html_eq!(
//             push_html(input),
//             "<table><thead><tr><th>Header 1</th><th>Header 2</th></tr></thead>\
//              <tbody><tr><td>Cell 1</td><td>Cell 2</td></tr></tbody></table>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_task_lists() {
//         let input = "- [ ] Unchecked\n- [x] Checked";
//         assert_html_eq!(
//             push_html(input),
//             "<ul><li><input type=\"checkbox\" disabled>Unchecked</li>\
//              <li><input type=\"checkbox\" disabled checked>Checked</li></ul>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_strikethrough() {
//         assert_html_eq!(
//             push_html("~~struck through~~"),
//             "<p><del>struck through</del></p>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_horizontal_rule() {
//         assert_html_eq!(push_html("---"), "<hr>", markdown());
//     }

//     #[test]
//     fn test_mixed_content() {
//         let input = "# Title\n\
//                      Some *formatted* text with `code`.\n\n\
//                      > A quote with **bold**\n\n\
//                      - List item 1\n\
//                      - List item 2\n\n\
//                      ```\nCode block\n```";

//         assert_html_eq!(
//             push_html(input),
//             "<h1 id=\"heading-1\">Title</h1>\
//              <p>Some <em>formatted</em> text with <code>code</code>.</p>\
//              <blockquote><p>A quote with <strong>bold</strong></p></blockquote>\
//              <ul><li>List item 1</li><li>List item 2</li></ul>\
//              <pre><code>Code block</code></pre>",
//             markdown()
//         );
//     }

//     #[test]
//     #[ignore = "Fix/implement escape_html option"]
//     fn test_escaped_html() {
//         let mut config = HtmlConfig::default();
//         config.html.escape_html = true;

//         assert_html_eq!(
//             push_html_with_config("This is <em>HTML</em> content", &config),
//             "<p>This is &lt;em&gt;HTML&lt;/em&gt; content</p>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_footnotes() {
//         let input = "Text with a footnote[^1].\n\n[^1]: Footnote content.";
//         assert_html_eq!(
//             push_html(input),
//             "<p>Text with a footnote<sup class=\"footnote-reference\"><a href=\"#1\">1</a></sup>.</p>\
//              <div class=\"footnote-definition\" id=\"1\"><sup class=\"footnote-definition-label\">1</sup>Footnote content.</div>",
//             markdown()
//         );
//     }

//     #[test]
//     fn test_line_breaks() {
//         assert_html_eq!(
//             push_html("Line 1  \nLine 2"),
//             "<p>Line 1<br>Line 2</p>",
//             markdown()
//         );
//     }
// }
