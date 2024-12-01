# Getting Started with pulldown-html-ext

This guide will help you get started with `pulldown-html-ext`, a flexible Markdown to HTML renderer built on top of pulldown-cmark.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pulldown-html-ext = "0.1.0"
```

or:

```bash
cargo add pulldown-html-ext
```

## Basic Usage

The simplest way to convert Markdown to HTML is using the default configuration:

```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = HtmlConfig::default();
    
    // Your markdown content
    let markdown = "# Hello\nThis is *markdown*";
    
    // Create parser
    let parser = Parser::new(markdown);
    
    // Convert to HTML
    let mut output = String::new();
    push_html(&mut output, parser, &config)?;
    
    println!("{}", output);
    Ok(())
}
```

This will generate HTML with sensible defaults:
- Headers get IDs for linking
- External links open in new tabs
- Code blocks support syntax highlighting
- HTML is escaped by default

## Writing to Different Outputs

The library provides multiple ways to write HTML output:

### String Output
```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html};

let config = HtmlConfig::default();
let parser = Parser::new(markdown);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

### Write to Formatter
```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, write_html_fmt};

let config = HtmlConfig::default();
let parser = Parser::new(markdown);
let mut output = String::new();
write_html_fmt(&mut output, parser, &config)?;
```

### Write to IO
```rust
use std::fs::File;
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, write_html_io};

let file = File::create("output.html")?;
let config = HtmlConfig::default();
let parser = Parser::new(markdown);
write_html_io(file, parser, &config)?;
```

## Error Handling

The library provides comprehensive error handling through the `HtmlError` type:

```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html, HtmlError};

fn convert_markdown(markdown: &str) -> Result<String, HtmlError> {
    let config = HtmlConfig::default();
    let parser = Parser::new(markdown);
    let mut output = String::new();
    
    push_html(&mut output, parser, &config)?;
    Ok(output)
}

fn main() {
    match convert_markdown("# Test\nContent") {
        Ok(html) => println!("Converted HTML: {}", html),
        Err(HtmlError::Config(e)) => eprintln!("Configuration error: {}", e),
        Err(HtmlError::Render(e)) => eprintln!("Rendering error: {}", e),
        Err(HtmlError::Theme(e)) => eprintln!("Theme error: {}", e),
        Err(e) => eprintln!("Other error: {}", e),
    }
}
```

## Custom Parser Options

You can customize the parser behavior using pulldown-cmark's options:

```rust
use pulldown_cmark::{Parser, Options};
use pulldown_html_ext::{HtmlConfig, push_html};

// Create parser with custom options
let mut options = Options::empty();
options.insert(Options::ENABLE_STRIKETHROUGH);
options.insert(Options::ENABLE_TABLES);

let parser = Parser::new_ext(markdown, options);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

## Working with Custom Writers

For advanced customization, you can create your own HTML writer:

```rust
use pulldown_html_ext::{HtmlWriter, HtmlConfig, HtmlState, create_html_renderer};
use pulldown_cmark_escape::StrWrite;
use pulldown_cmark::Parser;

struct CustomWriter<W: StrWrite> {
    writer: W,
    config: HtmlConfig,
    state: HtmlState,
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

    // Implement handlers for any desired tag here.
}
```

## Next Steps

- Learn about [Configuration](configuration.md) options
- Explore [HTML Rendering](html-rendering.md) capabilities
- Set up [Syntax Highlighting](syntax-highlighting.md)
- Create [Custom Writers](custom-writers.md)

## Additional Resources

- [API Documentation](https://docs.rs/pulldown-html-ext)
- [GitHub Repository](https://github.com/systemsoverload/pulldown-html-ext)
- [Examples](../examples/basic-usage.md)
