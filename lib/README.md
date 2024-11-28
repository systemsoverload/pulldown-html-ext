# pulldown-html-ext

A flexible Markdown to HTML renderer built on top of pulldown-cmark with support for custom styling, attributes, and rendering options.

## Features

- Configurable HTML rendering with support for custom attributes and classes
- Extensive options for headings, links, code blocks, and other elements
- Support for external link handling (nofollow, target="_blank")
- Custom ID generation for headings
- XHTML-style output option
- Table support with alignment
- Footnotes and task lists
- Line number support for code blocks

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pulldown-html-ext = "0.1.0"
```

### Basic Usage

For basic markdown rendering with default options (this should yield the same output as `pulldown-cmark` rendering):

```rust
use pulldown_html_ext::{HtmlConfig, push_html};

fn main() {
    let config = HtmlConfig::default();
    let markdown = "# Hello\nThis is *markdown*";
    let html = push_html(markdown, &config);
    println!("{}", html);
}
```

### Custom Configuration

You can customize the rendering behavior:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};
use std::collections::HashMap;

fn main() {
    let mut config = HtmlConfig::default();
    
    // Configure heading classes
    config.elements.headings.level_classes = {
        let mut map = HashMap::new();
        map.insert(1, "title".to_string());
        map.insert(2, "subtitle".to_string());
        map
    };
    

    // Configure external links
    config.elements.links.nofollow_external = true;
    config.elements.links.open_external_blank = true;
    
    // Configure HTML output
    config.html.escape_html = true;
    config.html.break_on_newline = false;
    config.html.xhtml_style = true;
    
    let markdown = r#"
# Main Title
## Subtitle

[External Link](https://example.com)
"#;
    
    let html = push_html(markdown, &config);
    println!("{}", html);
}
```

### Custom Rendering

For more control, you can implement your own HTML writer and override specific rendering methods:

```rust
use pulldown_html_ext::{HtmlConfig, HtmlWriter, HtmlState, create_html_renderer};
use pulldown_cmark::{HeadingLevel, Parser};

struct CustomWriter {
    config: HtmlConfig,
    output: String,
    state: HtmlState,
}

impl HtmlWriter for CustomWriter {
    fn get_config(&self) -> &HtmlConfig {
        &self.config
    }
    
    fn get_output(&mut self) -> &mut String {
        &mut self.output
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

fn main() {
    let writer = CustomWriter {
        config: HtmlConfig::default(),
        output: String::new(),
        state: HtmlState::new(),
    };
    let mut renderer = create_html_renderer(writer);
    
    let markdown = "# Main Title\n## Subtitle\n### Section";
    let parser = Parser::new(markdown);
    renderer.run(parser);
    
    // This will output:
    // <h1 class="fancy-heading level-1">🎯 Main Title </h1>
    // <h2 class="fancy-heading level-2">💫 Subtitle </h2>
    // <h3 class="fancy-heading level-3">✨ Section </h3>
    println!("{}", renderer.writer.output);
}
```


You can override any of the methods from the `HtmlWriter` trait to customize the rendering of different Markdown elements. Some commonly overridden methods include:
- `start_link`/`end_link` for custom link rendering
- `start_code_block`/`end_code_block` for custom code block formatting
- `start_list`/`end_list` for custom list rendering
- `text` for custom text processing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
