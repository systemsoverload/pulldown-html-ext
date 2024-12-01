# Basic Usage Examples

This guide provides practical examples of common use cases for `pulldown-html-ext`.

## Simple Document Conversion

Convert a basic Markdown document to HTML:

```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = HtmlConfig::default();
    
    let markdown = r#"
# Welcome to My Document

This is a paragraph with some **bold** and *italic* text.

## Lists

- Item 1
- Item 2
  - Nested item
  - Another nested item
- Item 3

1. First ordered item
2. Second ordered item
3. Third ordered item

## Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```
    "#;

    // Create parser
    let parser = Parser::new(markdown);
    let mut output = String::new();

    // Convert to HTML
    push_html(&mut output, parser, &config)?;
    println!("{}", output);
    Ok(())
}
```

## Working with Files

Read from and write to files:

```rust
use std::fs::File;
use std::io::Read;
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, write_html_io};

fn convert_file(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read markdown file
    let mut markdown = String::new();
    File::open(input)?.read_to_string(&mut markdown)?;

    // Set up config and parser
    let config = HtmlConfig::default();
    let parser = Parser::new(&markdown);

    // Write HTML to file
    let output_file = File::create(output)?;
    write_html_io(output_file, parser, &config)?;

    Ok(())
}

fn main() {
    if let Err(e) = convert_file("input.md", "output.html") {
        eprintln!("Error: {}", e);
    }
}
```

## HTML Template Integration

Integrate with an HTML template:

```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, push_html};

fn generate_page(title: &str, content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = HtmlConfig::default();
    let parser = Parser::new(content);
    let mut html_content = String::new();
    push_html(&mut html_content, parser, &config)?;

    Ok(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ 
            font-family: system-ui, sans-serif;
            line-height: 1.5;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
        title,
        html_content
    ))
}

fn main() {
    let markdown = r#"
# Hello World

This is a test document.

- Item 1
- Item 2
    "#;

    match generate_page("My Page", markdown) {
        Ok(html) => println!("{}", html),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Multiple Document Processing

Process multiple documents efficiently:

```rust
use pulldown_html_ext::{create_html_renderer, DefaultHtmlWriter};
use pulldown_cmark::Parser;
use pulldown_cmark_escape::FmtWriter;
use std::collections::HashMap;

fn process_documents(
    documents: HashMap<String, String>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let config = HtmlConfig::default();
    let mut results = HashMap::new();

    for (name, markdown) in documents {
        let mut output = String::new();
        let writer = DefaultHtmlWriter::new(FmtWriter(&mut output), &config);
        let mut renderer = create_html_renderer(writer);
        
        let parser = Parser::new(&markdown);
        renderer.run(parser)?;
        results.insert(name, output);
    }

    Ok(results)
}

fn main() {
    let mut docs = HashMap::new();
    docs.insert("doc1".to_string(), "# Document 1\nContent...".to_string());
    docs.insert("doc2".to_string(), "# Document 2\nContent...".to_string());

    match process_documents(docs) {
        Ok(results) => {
            for (name, html) in results {
                println!("Processed {}: {} bytes", name, html.len());
            }
        }
        Err(e) => eprintln!("Error processing documents: {}", e),
    }
}
```

## Error Handling

Proper error handling example:

```rust
use pulldown_cmark::Parser;
use pulldown_html_ext::{HtmlConfig, HtmlError, push_html};
use std::error::Error;

fn process_markdown(markdown: &str) -> Result<String, Box<dyn Error>> {
    let config = HtmlConfig::default();
    let parser = Parser::new(markdown);
    let mut output = String::new();
    
    match push_html(&mut output, parser, &config) {
        Ok(()) => Ok(output),
        Err(HtmlError::Config(e)) => Err(format!("Configuration error: {}", e).into()),
        Err(HtmlError::Render(e)) => Err(format!("Rendering error: {}", e).into()),
        Err(HtmlError::Theme(e)) => Err(format!("Theme error: {}", e).into()),
        Err(e) => Err(format!("Other error: {}", e).into()),
    }
}

fn main() {
    let invalid_markdown = "# Title\n\n```invalid-lang\nBad code block\n```";
    
    match process_markdown(invalid_markdown) {
        Ok(html) => println!("Success: {}", html),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Next Steps

- Check out [Custom Configuration Examples](custom-config.md)
- Learn about [Syntax Highlighting Examples](syntax-highlighting.md)
- Explore the [User Guide](../guide/getting-started.md)
