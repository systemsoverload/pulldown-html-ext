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
use pulldown_html_ext::{HtmlConfig, push_html, HtmlError};

fn main() -> Result<(), HtmlError> {
    let config = HtmlConfig::default();
    let markdown = "# Hello\nThis is *markdown*";
    let html = push_html(markdown, &config)?;
    println!("{}", html);
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
let html = push_html(markdown, &config)?;
```

### Write to Formatter
```rust
let mut output = String::new();
write_html_fmt(&mut output, markdown, &config)?;
```

### Write to IO
```rust
let file = File::create("output.html")?;
write_html_io(file, markdown, &config)?;
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
