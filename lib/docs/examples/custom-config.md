# Custom Configuration Examples

This guide provides practical examples of customizing the HTML output using various configuration options.

## Custom HTML Attributes

Add custom attributes to specific HTML elements:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();

    // Configure attributes for headings
    let mut h1_attrs = HashMap::new();
    h1_attrs.insert("class".to_string(), "title main-title".to_string());
    h1_attrs.insert("data-type".to_string(), "main-heading".to_string());
    
    let mut h2_attrs = HashMap::new();
    h2_attrs.insert("class".to_string(), "subtitle".to_string());
    
    config.attributes.element_attributes.insert("h1".to_string(), h1_attrs);
    config.attributes.element_attributes.insert("h2".to_string(), h2_attrs);

    let markdown = r#"
# Main Title
Some content

## Section Title
More content
    "#;

    let html = push_html(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Custom Heading IDs and Classes

Configure heading IDs and classes:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    
    // Configure heading options
    config.elements.headings.add_ids = true;
    config.elements.headings.id_prefix = "section-".to_string();
    
    // Add custom classes for different heading levels
    let mut level_classes = HashMap::new();
    level_classes.insert(1, "page-title text-4xl".to_string());
    level_classes.insert(2, "section-title text-2xl".to_string());
    level_classes.insert(3, "subsection-title text-xl".to_string());
    config.elements.headings.level_classes = level_classes;

    let markdown = r#"
# Main Title
## First Section
### Subsection
    "#;

    let html = push_html(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Link Configuration

Customize link handling:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    
    // Configure link options
    config.elements.links.nofollow_external = true;
    config.elements.links.open_external_blank = true;

    let markdown = r#"
# Links Example

- [Internal Link](/page)
- [External Link](https://example.com)
- [Another Internal](/about)
    "#;

    let html = push_html(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Code Block Configuration

Customize code block rendering:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    
    // Configure code block options
    config.elements.code_blocks.default_language = Some("text".to_string());
    config.elements.code_blocks.line_numbers = true;

    let markdown = r#"
# Code Examples

Unspecified language:
```
print("Hello")
```

Specified language:
```python
def greet(name):
    print(f"Hello, {name}!")
```
    "#;

    let html = push_html(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## HTML Output Options

Configure HTML output formatting:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    
    // Configure HTML options
    config.html.escape_html = true;
    config.html.break_on_newline = true;
    config.html.xhtml_style = true;
    config.html.pretty_print = true;

    let markdown = r#"
# HTML Example

This has <strong>HTML</strong> tags.

Line one
Line two

<img src="test.jpg" alt="Test">
    "#;

    let html = push_html(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Loading Configuration from TOML

Load configuration from a TOML file:

```rust
use pulldown_html_ext::{HtmlConfig, push_html};
use std::fs;
use toml;

fn load_config(path: &str) -> Result<HtmlConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: HtmlConfig = toml::from_str(&content)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example config.toml content:
    let config_toml = r#"
        [html]
        escape_html = true
        break_on_newline = true
        xhtml_style = false
        pretty_print = true

        [elements.headings]
        add_ids = true
        id_prefix = "section-"

        [elements.links]
        nofollow_external = true
        open_external_blank = true

        [elements.code_blocks]
        default_language = "text"
        line_numbers = true
    "#;

    // Save example config
    fs::write("config.toml", config_toml)?;

    // Load and use config
    let config = load_config("config.toml")?;
    let markdown = "# Test\nSome content";
    let html = push_html(markdown, &config)?;
    println!("{}", html);

    Ok(())
}
```

## Next Steps

- Check out [Syntax Highlighting Examples](syntax-highlighting.md)
- Learn about [Custom Writers Examples](custom-writers.md)
- Return to the [User Guide](../guide/getting-started.md)
