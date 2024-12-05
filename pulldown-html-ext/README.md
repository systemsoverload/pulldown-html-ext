# pulldown-html-ext

A configurable Markdown to HTML renderer that extends [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark). This library provides a flexible HTML rendering system with extensive configuration options, custom styling support, and attribute handling capabilities.

## Features

- Configurable HTML rendering with extensive options
- Custom attribute mapping for HTML elements
- Support for heading IDs and custom classes
- Customizable code block rendering
- External link handling with `nofollow` and `target="_blank"` options
- Table support with alignment controls
- Footnote rendering
- Task list support
- XHTML-style output option
- Pretty printing support
- Syntect-based syntax highlighting for code blocks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pulldown-html-ext = "0.1.0"
```

## Quick Start

Here's a simple example of converting Markdown to HTML using default settings:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};
use pulldown_cmark::Parser;

let config = HtmlConfig::default();
let markdown = "# Hello\nThis is *markdown*";
let mut output = String::new();
let parser = Parser::new(markdown);
let html = push_html(&mut output, parser, &config);
```

## Configuration

The library provides extensive configuration options through the `HtmlConfig` struct:

```rust
let mut config = HtmlConfig::default();

// Configure HTML options
config.html.escape_html = true;
config.html.break_on_newline = true;
config.html.xhtml_style = false;
config.html.pretty_print = true;

// Configure heading options
config.elements.headings.add_ids = true;
config.elements.headings.id_prefix = "heading-".to_string();

// Configure link options
config.elements.links.nofollow_external = true;
config.elements.links.open_external_blank = true;

// Configure code block options
config.elements.code_blocks.default_language = Some("rust".to_string());
config.elements.code_blocks.line_numbers = false;

// Configure syntax highlighting ( If feature is enabled )
config.syntect = Some(SyntectConfigStyle {
    theme: "base16-ocean.dark".to_string(),
    class_style: ClassStyle::Spaced,
    inject_css: true,
});
```

## Custom Attribute Mapping

You can add custom attributes to HTML elements:

```rust
use std::collections::HashMap;

let mut config = HtmlConfig::default();
let mut attrs = HashMap::new();
attrs.insert("class".to_string(), "custom-paragraph".to_string());
config.attributes.element_attributes.insert("p".to_string(), attrs);
```

## Custom Writers

Create custom HTML writers by implementing the `HtmlWriter` trait. This allows you to customize how specific Markdown elements are rendered to HTML:

```rust
use pulldown_html_ext::{HtmlConfig, HtmlWriter, HtmlState, HtmlRenderer};
use pulldown_cmark_escape::{StrWrite, FmtWriter};
use pulldown_cmark::{HeadingLevel, Parser};

struct CustomWriter<W: StrWrite> {
    writer: W,
    config: HtmlConfig,
    state: HtmlState,
}

impl<W: StrWrite> CustomWriter<W> {
    fn new(writer: W, config: HtmlConfig) -> Self {
        Self {
            writer,
            config,
            state: HtmlState::new(),
        }
    }
}

impl<W: StrWrite> HtmlWriter<W> for CustomWriter<W> {
    fn get_writer(&mut self) -> &mut W {
        &mut self.writer
    }

    fn get_config(&self) -> &HtmlConfig {
        &self.config
    }

    fn get_state(&mut self) -> &mut HtmlState {
        &mut self.state
    }

    // Override heading rendering to add emoji markers and custom classes
    fn start_heading(&mut self, level: HeadingLevel, _id: Option<&str>, classes: Vec<&str>) {
        let level_num = self.heading_level_to_u8(level);
        let emoji = match level_num {
            1 => "🎯",
            2 => "💫",
            _ => "✨",
        };
        
        self.write_str(&format!("<h{} class=\"fancy-heading level-{}", level_num, level_num));
        if !classes.is_empty() {
            self.write_str(" ");
            self.write_str(&classes.join(" "));
        }
        self.write_str("\">");
        self.write_str(emoji);
        self.write_str(" ");
    }

    fn end_heading(&mut self, level: HeadingLevel) {
        let level_num = self.heading_level_to_u8(level);
        self.write_str(&format!(" </h{}>", level_num));
    }
}

// Usage example:
fn main() {
    let mut output = String::new();
    let writer = CustomWriter::new(FmtWriter(&mut output), HtmlConfig::default());
    let mut renderer = HtmlRenderer::new(writer);
    
    let markdown = "# Main Title\n## Subtitle\n### Section";
    let parser = Parser::new(markdown);
    renderer.run(parser);
    
    println!("{}", output);
    // Output:
    // <h1 class="fancy-heading level-1">🎯 Main Title </h1>
    // <h2 class="fancy-heading level-2">💫 Subtitle </h2>
    // <h3 class="fancy-heading level-3">✨ Section </h3>
}
```

## Syntect-based Syntax Highlighting

The library provides an optional feature to enable syntax highlighting for code blocks using the Syntect library. To use this, you can enable the `syntect` feature in your `Cargo.toml`:

```toml
[dependencies]
pulldown-html-ext = { version = "0.1.0", features = ["syntect"] }
```

Then, you can configure the syntax highlighting options in your `HtmlConfig`:

```rust
let mut config = HtmlConfig::default();
config.syntect = Some(SyntectConfigStyle {
    theme: "base16-ocean.dark".to_string(),
    class_style: ClassStyle::Spaced,
    inject_css: true,
});
```

This will add syntax highlighting to your code blocks using the specified theme and class style.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
