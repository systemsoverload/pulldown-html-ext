# Configuration

`pulldown-html-ext` provides extensive configuration options through the `HtmlConfig` struct. This guide covers all available configuration options and their effects.

## Basic Configuration Structure

The configuration is organized into several sections:

```rust
pub struct HtmlConfig {
    pub html: HtmlOptions,
    pub elements: ElementOptions,
    pub attributes: AttributeMappings,
    pub syntect: Option<SyntectConfigStyle>,
}
```

## HTML Options

Control basic HTML rendering behavior:

```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html};

let mut config = HtmlConfig::default();

// Basic HTML options
config.html.escape_html = false;      // Whether to escape HTML in the input
config.html.break_on_newline = true;  // Convert newlines to <br> tags
config.html.xhtml_style = false;      // Use XHTML-style self-closing tags
config.html.pretty_print = true;      // Add newlines for prettier output

// Convert some markdown
let markdown = "<div>Test</div>\nNew line";
let parser = Parser::new(markdown);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

## Element Options

### Heading Configuration
```rust
use std::collections::HashMap;

// Configure headings
config.elements.headings.add_ids = true;
config.elements.headings.id_prefix = "heading-".to_string();

// Add custom classes for different heading levels
let mut level_classes = HashMap::new();
level_classes.insert(1, "title".to_string());
level_classes.insert(2, "subtitle".to_string());
config.elements.headings.level_classes = level_classes;

// Test the configuration
let markdown = "# Main Title\n## Subtitle";
let parser = Parser::new(markdown);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

### Link Configuration
```rust
// Configure links
config.elements.links.nofollow_external = true;     // Add rel="nofollow"
config.elements.links.open_external_blank = true;   // Add target="_blank"

let markdown = "[External Link](https://example.com)";
let parser = Parser::new(markdown);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

### Code Block Configuration
```rust
// Configure code blocks
config.elements.code_blocks.default_language = Some("rust".to_string());
config.elements.code_blocks.line_numbers = false;

let markdown = "```\nfn main() {\n    println!(\"Hello\");\n}\n```";
let parser = Parser::new(markdown);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

## Custom Attributes

Add custom attributes to any HTML element:

```rust
use std::collections::HashMap;

// Add custom attributes to paragraphs
let mut p_attrs = HashMap::new();
p_attrs.insert("class".to_string(), "content".to_string());
p_attrs.insert("data-type".to_string(), "paragraph".to_string());
config.attributes.element_attributes.insert("p".to_string(), p_attrs);

// Add custom attributes to blockquotes
let mut blockquote_attrs = HashMap::new();
blockquote_attrs.insert("class".to_string(), "quote".to_string());
config.attributes.element_attributes.insert("blockquote".to_string(), blockquote_attrs);

let markdown = "This is a paragraph\n\n> This is a quote";
let parser = Parser::new(markdown);
let mut output = String::new();
push_html(&mut output, parser, &config)?;
```

## Syntax Highlighting Configuration

Enable syntax highlighting with Syntect:

```rust
use pulldown_html_ext::{SyntectConfigStyle, push_html_with_highlighting};
use syntect::html::ClassStyle;

let mut config = HtmlConfig::default();
let style = SyntectConfigStyle {
    theme: "base16-ocean.dark".to_string(),
    class_style: ClassStyle::Spaced,
    inject_css: true,
};
config.syntect = Some(style);

let markdown = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
let parser = Parser::new(markdown);
let mut output = String::new();
push_html_with_highlighting(&mut output, parser, &config)?;
```

## Using TOML Configuration

You can also load configuration from a TOML file:

```toml
[html]
escape_html = false
break_on_newline = true
xhtml_style = false
pretty_print = true

[elements.headings]
add_ids = true
id_prefix = "heading-"

[elements.links]
nofollow_external = true
open_external_blank = true

[elements.code_blocks]
default_language = "rust"
line_numbers = false

[syntect]
theme = "base16-ocean.dark"
class_style = "spaced"
inject_css = true
```

Load it in your code:

```rust
use std::fs;
use toml;
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html};

fn load_config(path: &str) -> Result<HtmlConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: HtmlConfig = toml::from_str(&content)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.toml")?;
    let markdown = "# Test\nSome content";
    let parser = Parser::new(markdown);
    let mut output = String::new();
    push_html(&mut output, parser, &config)?;
    println!("{}", output);
    Ok(())
}
```

## Error Handling

The configuration system includes error handling for invalid settings:

```rust
use pulldown_html_ext::HtmlError;

match config.elements.headings.level_classes.insert(7, "invalid".to_string()) {
    Ok(_) => println!("Configuration updated"),
    Err(HtmlError::Config(e)) => eprintln!("Invalid configuration: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Next Steps

- Learn about [HTML Rendering](html-rendering.md)
- Explore [Custom Writers](custom-writers.md)
- See [Examples](../examples/custom-config.md)
