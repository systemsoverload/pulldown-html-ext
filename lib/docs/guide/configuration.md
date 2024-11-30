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
let mut config = HtmlConfig::default();

// Basic HTML options
config.html.escape_html = true;      // Escape HTML in the input
config.html.break_on_newline = true; // Convert newlines to <br> tags
config.html.xhtml_style = false;     // Use XHTML-style self-closing tags
config.html.pretty_print = true;     // Add newlines for prettier output
```

## Element Options

### Heading Configuration
```rust
// Configure headings
config.elements.headings.add_ids = true;
config.elements.headings.id_prefix = "heading-".to_string();

// Add custom classes for different heading levels
config.elements.headings.level_classes.insert(1, "title".to_string());
config.elements.headings.level_classes.insert(2, "subtitle".to_string());
```

### Link Configuration
```rust
// Configure links
config.elements.links.nofollow_external = true;     // Add rel="nofollow"
config.elements.links.open_external_blank = true;   // Add target="_blank"
```

### Code Block Configuration
```rust
// Configure code blocks
config.elements.code_blocks.default_language = Some("rust".to_string());
config.elements.code_blocks.line_numbers = true;
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
```

## Syntax Highlighting Configuration

Enable syntax highlighting with Syntect:

```rust
use pulldown_html_ext::SyntectConfigStyle;

let style = SyntectConfigStyle {
    theme: "base16-ocean.dark".to_string(),
    class_style: ClassStyle::Spaced,
    inject_css: true,
};
config.syntect = Some(style);
```

## Using TOML Configuration

You can also load configuration from a TOML file:

```toml
[html]
escape_html = true
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
line_numbers = true

[syntect]
theme = "base16-ocean.dark"
class_style = "spaced"
inject_css = true
```

Load it in your code:

```rust
use std::fs;
use toml;

fn load_config(path: &str) -> Result<HtmlConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: HtmlConfig = toml::from_str(&content)?;
    Ok(config)
}
```

## Next Steps

- Learn about [HTML Rendering](html-rendering.md)
- Explore [Custom Writers](custom-writers.md)
- See [Examples](../examples/custom-config.md)
