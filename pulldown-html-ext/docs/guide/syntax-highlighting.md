# Syntax Highlighting

`pulldown-html-ext` provides syntax highlighting through the Syntect library. This guide covers how to enable and customize syntax highlighting for code blocks.

## Enabling Syntax Highlighting

Add the `syntect` feature to your `Cargo.toml`:

```toml
[dependencies]
pulldown-html-ext = { version = "0.1.0", features = ["syntect"] }
```

## Basic Usage

```rust
use pulldown_html_ext::{HtmlConfig, push_html_with_highlighting};
use syntect::html::ClassStyle;

// Create config with syntax highlighting
let mut config = HtmlConfig::default();
config.syntect = Some(SyntectConfigStyle {
    theme: "base16-ocean.dark".to_string(),
    class_style: ClassStyle::Spaced,
    inject_css: true,
});

// Convert markdown with syntax highlighting
let markdown = r#"```rust
fn main() {
    println!("Hello, world!");
}
```"#;

let html = push_html_with_highlighting(markdown, &config)?;
```

## Configuration Options

### Themes

Available built-in themes:
- `base16-ocean.dark` (default)
- `base16-ocean.light`
- `InspiredGitHub`
- `Solarized-dark`
- `Solarized-light`

```rust
config.syntect = Some(SyntectConfigStyle {
    theme: "InspiredGitHub".to_string(),
    ..Default::default()
});
```

### Class Styles

Two options for CSS class generation:

```rust
// Simple space-separated classes
config.syntect = Some(SyntectConfigStyle {
    class_style: ClassStyle::Spaced,  // Results in "foo bar"
    ..Default::default()
});

// Prefixed classes
config.syntect = Some(SyntectConfigStyle {
    class_style: ClassStyle::SpacedPrefixed { prefix: "syntax-" },  // Results in "syntax-foo syntax-bar"
    ..Default::default()
});
```

## CSS Management

### Automatic CSS Injection

```rust
// CSS will be automatically included in the output
config.syntect = Some(SyntectConfigStyle {
    inject_css: true,
    ..Default::default()
});
```

### Manual CSS Handling

```rust
use pulldown_html_ext::SyntectWriter;
use pulldown_cmark_escape::FmtWriter;

let mut output = String::new();
let writer = SyntectWriter::new(FmtWriter(&mut output), &config);

// Get CSS separately
let css = writer.get_theme_css()?;
println!("<style>{}</style>", css);
```

## Language Support

### Automatic Language Detection

The library automatically detects languages from code fence info strings:

````markdown
```rust
fn main() {
    println!("Hello!");
}
```

```python
def main():
    print("Hello!")
```
````

### Default Language

Set a default language for unspecified code blocks:

```rust
config.elements.code_blocks.default_language = Some("text".to_string());
```

## Custom Implementation

### Using SyntectWriter Directly

```rust
use pulldown_html_ext::{create_html_renderer, SyntectWriter};
use pulldown_cmark::Parser;

let mut output = String::new();
let writer = SyntectWriter::new(FmtWriter(&mut output), &config);
let mut renderer = create_html_renderer(writer);

let parser = Parser::new(markdown);
renderer.run(parser)?;
```

### Custom Syntax Sets

```rust
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

// Load custom syntax and theme sets
let syntax_set = SyntaxSet::load_from_folder("path/to/syntaxes")?;
let theme_set = ThemeSet::load_from_folder("path/to/themes")?;

let writer = SyntectWriter::with_custom_sets(
    FmtWriter(&mut output),
    &config,
    Some(&syntax_set),
    Some(&theme_set)
);
```

## Error Handling

Handle potential syntax highlighting errors:

```rust
use pulldown_html_ext::HtmlError;

match push_html_with_highlighting(markdown, &config) {
    Ok(html) => println!("Success: {}", html),
    Err(HtmlError::Theme(e)) => eprintln!("Theme error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Best Practices

1. **Theme Selection**
   - Choose appropriate themes for your use case
   - Consider light/dark mode support
   - Test themes with various languages

2. **Performance**
   - Cache CSS when not using auto-injection
   - Reuse SyntectWriter instances when possible
   - Consider lazy loading for large code bases

3. **Accessibility**
   - Ensure sufficient color contrast in chosen themes
   - Provide alternative text/descriptions when needed
   - Test with screen readers

## Next Steps

- Explore [Custom Writers](custom-writers.md) for more customization
- See the [Examples](../examples/syntax-highlighting.md) section
- Check the [API Documentation](https://docs.rs/pulldown-html-ext) for detailed reference
