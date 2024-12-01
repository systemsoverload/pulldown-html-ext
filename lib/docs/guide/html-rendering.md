# HTML Rendering

This guide explains how HTML rendering works in `pulldown-html-ext` and how to customize the rendering process.

## Core Concepts

The HTML rendering system consists of several key components:

1. `HtmlRenderer` - The core renderer that processes Markdown events
2. `HtmlWriter` - The trait that defines how elements are rendered
3. `HtmlState` - Maintains state during rendering
4. `DefaultHtmlWriter` - The default implementation of `HtmlWriter`

## Basic Rendering

The simplest way to render Markdown to HTML:

```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html};

let config = HtmlConfig::default();
let markdown = "# Hello\nThis is *markdown*";

let parser = Parser::new(markdown);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

## Working with the Renderer Directly

For more control, you can work with the renderer directly:

```rust
use pulldown_html_ext::{create_html_renderer, DefaultHtmlWriter};
use pulldown_cmark::{Parser, Event};
use pulldown_cmark_escape::FmtWriter;

let mut output = String::new();
let writer = DefaultHtmlWriter::new(FmtWriter(&mut output), &config);
let mut renderer = create_html_renderer(writer);

let parser = Parser::new(markdown);
renderer.run(parser)?;
```

## State Management

The renderer maintains state during processing:

```rust
pub struct HtmlState {
    pub numbers: Vec<u32>,              // For ordered lists
    pub table_state: TableContext,      // Current table state
    pub table_cell_index: usize,        // Current cell in table
    pub table_alignments: Vec<Alignment>, // Table column alignments
    pub list_stack: Vec<ListContext>,   // Nested list tracking
    pub link_stack: Vec<LinkType>,      // Nested link tracking
    pub heading_stack: Vec<String>,     // Header ID tracking
    pub currently_in_code_block: bool,  // Code block state
    pub currently_in_footnote: bool,    // Footnote state
}
```

## Event Handling

The renderer processes various Markdown events:

```rust
impl<W: StrWrite, H: HtmlWriter<W>> HtmlRenderer<W, H> {
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
}
```

## Element Handling Examples

### Headers
```rust
// Input Markdown
# Level 1
## Level 2

// Generated HTML
<h1 id="heading-1">Level 1</h1>
<h2 id="heading-2">Level 2</h2>
```

### Lists
```rust
// Input Markdown
1. First
2. Second
   * Nested
   * Items

// Generated HTML
<ol>
  <li>First</li>
  <li>Second
    <ul>
      <li>Nested</li>
      <li>Items</li>
    </ul>
  </li>
</ol>
```

### Tables
```rust
// Input Markdown
| Left | Center | Right |
|:-----|:------:|------:|
| 1    |   2    |     3 |

// Generated HTML
<table>
  <thead>
    <tr>
      <th style="text-align: left">Left</th>
      <th style="text-align: center">Center</th>
      <th style="text-align: right">Right</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align: left">1</td>
      <td style="text-align: center">2</td>
      <td style="text-align: right">3</td>
    </tr>
  </tbody>
</table>
```

## Error Handling

The renderer uses a custom error type:

```rust
pub enum HtmlError {
    Io(io::Error),
    Write(fmt::Error),
    Theme(String),
    Config(String),
    Render(String),
}
```

Handle errors appropriately:

```rust
fn render_markdown(markdown: &str) -> Result<String, HtmlError> {
    let config = HtmlConfig::default();
    let parser = Parser::new(markdown);
    let mut output = String::new();
    push_html(&mut output, parser, &config)?;
    Ok(output)
}
```

## Writing Output

The library supports different output methods:

```rust
// String output
let mut output = String::new();
push_html(&mut output, parser, &config)?;

// Write to formatter
write_html_fmt(&mut output, parser, &config)?;

// Write to IO
let file = File::create("output.html")?;
write_html_io(file, parser, &config)?;
```

## Best Practices

1. **State Management**
   - Reset state between documents
   - Check state before operations
   - Handle nested structures carefully

2. **Error Handling**
   - Always handle potential errors
   - Use appropriate error variants
   - Provide meaningful error messages

3. **Performance**
   - Create Parser instances appropriately
   - Reuse writers when processing multiple documents
   - Consider buffer size for large documents

4. **Memory Usage**
   - Clear state between documents
   - Be mindful of string allocations
   - Handle large documents efficiently

## Next Steps

- Learn about [Syntax Highlighting](syntax-highlighting.md)
- Implement [Custom Writers](custom-writers.md)
- See [Examples](../examples/basic-usage.md)
