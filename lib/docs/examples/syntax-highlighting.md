# Syntax Highlighting Examples

This guide provides practical examples of using syntax highlighting in `pulldown-html-ext`.

## Basic Syntax Highlighting

Enable basic syntax highlighting with default settings:

```rust
use pulldown_html_ext::{HtmlConfig, push_html_with_highlighting};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = HtmlConfig::default();
    
    let markdown = r#"
# Code Examples

```rust
fn main() {
    println!("Hello, world!");
}
```

```python
def greet(name):
    print(f"Hello, {name}!")
```
    "#;

    let html = push_html_with_highlighting(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Custom Theme Configuration

Use different themes for syntax highlighting:

```rust
use pulldown_html_ext::{HtmlConfig, SyntectConfigStyle, push_html_with_highlighting};
use syntect::html::ClassStyle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    
    // Configure dark theme
    config.syntect = Some(SyntectConfigStyle {
        theme: "base16-ocean.dark".to_string(),
        class_style: ClassStyle::Spaced,
        inject_css: true,
    });

    let markdown = r#"
```javascript
function calculateTotal(items) {
    return items.reduce((sum, item) => sum + item.price, 0);
}
```
    "#;

    println!("Dark theme:");
    let dark_html = push_html_with_highlighting(markdown, &config)?;
    println!("{}", dark_html);

    // Switch to light theme
    config.syntect = Some(SyntectConfigStyle {
        theme: "InspiredGitHub".to_string(),
        class_style: ClassStyle::Spaced,
        inject_css: true,
    });

    println!("\nLight theme:");
    let light_html = push_html_with_highlighting(markdown, &config)?;
    println!("{}", light_html);

    Ok(())
}
```

## Custom CSS Classes

Customize CSS class generation:

```rust
use pulldown_html_ext::{HtmlConfig, SyntectConfigStyle, push_html_with_highlighting};
use syntect::html::ClassStyle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    
    // Use prefixed classes
    config.syntect = Some(SyntectConfigStyle {
        theme: "Solarized-dark".to_string(),
        class_style: ClassStyle::SpacedPrefixed { 
            prefix: "highlight-".to_string() 
        },
        inject_css: true,
    });

    let markdown = r#"
```css
.container {
    display: flex;
    justify-content: center;
    align-items: center;
}
```
    "#;

    let html = push_html_with_highlighting(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Manual CSS Management

Handle CSS separately from the HTML:

```rust
use pulldown_html_ext::{
    HtmlConfig, SyntectWriter, create_html_renderer,
    SyntectConfigStyle,
};
use pulldown_cmark::Parser;
use pulldown_cmark_escape::FmtWriter;

fn generate_highlighted_page(
    markdown: &str,
    config: &HtmlConfig,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut html_output = String::new();
    let writer = SyntectWriter::new(FmtWriter(&mut html_output), config);
    let mut renderer = create_html_renderer(writer);
    
    // Get CSS separately
    let css = renderer.writer.get_theme_css()?;
    
    // Generate HTML
    renderer.run(Parser::new(markdown))?;
    
    Ok((css, html_output))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    config.syntect = Some(SyntectConfigStyle {
        theme: "base16-ocean.dark".to_string(),
        class_style: ClassStyle::Spaced,
        inject_css: false, // Don't inject CSS automatically
    });

    let markdown = r#"
```ruby
class Person
  def initialize(name)
    @name = name
  end
  
  def greet
    puts "Hello, #{@name}!"
  end
end
```
    "#;

    let (css, html) = generate_highlighted_page(markdown, &config)?;
    
    // Create complete HTML document
    let full_page = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <style>{}</style>
</head>
<body>
    {}
</body>
</html>"#,
        css, html
    );

    println!("{}", full_page);
    Ok(())
}
```

## Language Auto-Detection

Example showing automatic language detection:

```rust
use pulldown_html_ext::{HtmlConfig, push_html_with_highlighting};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    
    // Set default language for unspecified blocks
    config.elements.code_blocks.default_language = Some("text".to_string());

    let markdown = r#"
# Language Examples

Plain text (uses default):
```
This is plain text
```

Python (auto-detected):
```python
def factorial(n):
    return 1 if n <= 1 else n * factorial(n-1)
```

JSON (auto-detected):
```json
{
  "name": "John Doe",
  "age": 30,
  "cities": ["New York", "London"]
}
```

SQL (auto-detected):
```sql
SELECT users.name, COUNT(orders.id) as order_count
FROM users
LEFT JOIN orders ON users.id = orders.user_id
GROUP BY users.id;
```
    "#;

    let html = push_html_with_highlighting(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Error Handling

Handle syntax highlighting errors gracefully:

```rust
use pulldown_html_ext::{HtmlConfig, HtmlError, push_html_with_highlighting};

fn render_with_fallback(
    markdown: &str,
    config: &HtmlConfig
) -> Result<String, Box<dyn std::error::Error>> {
    match push_html_with_highlighting(markdown, config) {
        Ok(html) => Ok(html),
        Err(HtmlError::Theme(e)) => {
            eprintln!("Theme error: {}. Falling back to default theme.", e);
            let mut fallback_config = config.clone();
            fallback_config.syntect = None;
            Ok(push_html_with_highlighting(markdown, &fallback_config)?)
        }
        Err(e) => Err(e.into()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HtmlConfig::default();
    config.syntect = Some(SyntectConfigStyle {
        theme: "nonexistent-theme".to_string(),
        ..Default::default()
    });

    let markdown = r#"
```rust
fn main() {
    println!("Test");
}
```
    "#;

    let html = render_with_fallback(markdown, &config)?;
    println!("{}", html);
    Ok(())
}
```

## Next Steps

- Explore [Custom Writers Examples](custom-writers.md)
- Check the [Syntax Highlighting Guide](../guide/syntax-highlighting.md)
- Return to [Basic Usage Examples](basic-usage.md)
