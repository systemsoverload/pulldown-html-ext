# Custom Writers

This guide explains how to create custom HTML writers to control exactly how your Markdown is rendered to HTML.

## Understanding the HtmlWriter Trait

The `HtmlWriter` trait is the core of customization in `pulldown-html-ext`. It defines how each Markdown element is converted to HTML.

### Basic Structure

```rust
use pulldown_html_ext::{HtmlWriter, HtmlConfig, HtmlState};
use pulldown_cmark_escape::StrWrite;

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
}
```

## Implementing Custom Behavior

### Headers Example

Here's how to customize header rendering:

```rust
impl<W: StrWrite> HtmlWriter<W> for CustomWriter<W> {
    // ... other required methods ...

    fn start_heading(
        &mut self,
        level: HeadingLevel,
        id: Option<&str>,
        classes: Vec<&str>
    ) -> Result<(), HtmlError> {
        let level_num = self.heading_level_to_u8(level);
        
        // Write opening tag with custom attributes
        self.write_str(&format!("<h{} class=\"custom-heading\"", level_num))?;
        
        // Add ID if provided
        if let Some(id) = id {
            self.write_str(&format!(" id=\"{}\"", id))?;
        }
        
        // Add custom data attribute
        self.write_str(&format!(" data-level=\"{}\"", level_num))?;
        
        self.write_str(">")?;
        
        // Add prefix emoji based on level
        let emoji = match level_num {
            1 => "🎯",
            2 => "💫",
            _ => "✨",
        };
        self.write_str(emoji)?;
        self.write_str(" ");
        
        Ok(())
    }

    fn end_heading(&mut self, level: HeadingLevel) -> Result<(), HtmlError> {
        let level_num = self.heading_level_to_u8(level);
        self.write_str(&format!("</h{}>", level_num))
    }
}
```

### Lists Example

Customize list rendering:

```rust
impl<W: StrWrite> HtmlWriter<W> for CustomWriter<W> {
    fn start_list(&mut self, first_number: Option<u64>) -> Result<(), HtmlError> {
        match first_number {
            Some(n) => {
                // Ordered list with custom class
                self.write_str("<ol class=\"custom-ordered-list\"")?;
                if n != 1 {
                    self.write_str(&format!(" start=\"{}\"", n))?;
                }
                self.write_str(">")?;
                self.get_state().numbers.push(n.try_into().unwrap());
            }
            None => {
                // Unordered list with custom class
                self.write_str("<ul class=\"custom-unordered-list\">")?;
            }
        }
        Ok(())
    }

    fn start_list_item(&mut self) -> Result<(), HtmlError> {
        let depth = self.get_state().list_stack.len();
        self.write_str(&format!(
            "<li class=\"depth-{}\" data-depth=\"{}\">",
            depth, depth
        ))
    }
}
```

### Code Blocks Example

Add custom code block rendering:

```rust
impl<W: StrWrite> HtmlWriter<W> for CustomWriter<W> {
    fn start_code_block(&mut self, kind: CodeBlockKind) -> Result<(), HtmlError> {
        self.get_state().currently_in_code_block = true;
        
        // Start pre tag with custom class
        self.write_str("<pre class=\"code-block\">")?;
        
        // Add code tag with language class if available
        match kind {
            CodeBlockKind::Fenced(info) => {
                let lang = if info.is_empty() {
                    "text"
                } else {
                    &*info
                };
                self.write_str(&format!(
                    "<code class=\"language-{}\" data-language=\"{}\">",
                    lang, lang
                ))?;
            }
            CodeBlockKind::Indented => {
                self.write_str("<code class=\"language-text\">")?;
            }
        }
        
        Ok(())
    }

    fn text(&mut self, text: &str) -> Result<(), HtmlError> {
        if self.get_state().currently_in_code_block {
            // Add line numbers
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                self.write_str(&format!(
                    "<span class=\"line-number\">{}</span>{}\n",
                    i + 1,
                    line
                ))?;
            }
            Ok(())
        } else {
            // Normal text handling
            self.write_str(text)
        }
    }
}
```

## Using Custom Writers

### Basic Usage

```rust
let mut output = String::new();
let writer = CustomWriter::new(FmtWriter(&mut output), &config);
let mut renderer = create_html_renderer(writer);

let parser = Parser::new(markdown);
renderer.run(parser)?;
```

### With State Management

```rust
impl<W: StrWrite> CustomWriter<W> {
    fn new(writer: W, config: HtmlConfig) -> Self {
        Self {
            writer,
            config,
            state: HtmlState::new(),
        }
    }

    fn reset(&mut self) {
        self.state = HtmlState::new();
    }
}

// Use in a loop
for document in documents {
    writer.reset();
    renderer.run(Parser::new(document))?;
}
```

## Best Practices

1. **State Management**
   - Always maintain proper state
   - Reset state between documents
   - Handle nested structures correctly

2. **Error Handling**
   - Use `Result<(), HtmlError>` consistently
   - Propagate errors appropriately
   - Provide meaningful error context

3. **Performance**
   - Minimize string allocations
   - Reuse writers when possible
   - Consider buffering for large outputs

4. **Accessibility**
   - Add appropriate ARIA attributes
   - Maintain semantic HTML structure
   - Include descriptive classes

## Examples

Check out the [examples directory](../examples/) for complete working examples of custom writers:
- Basic custom writer
- Writer with syntax highlighting
- Writer with custom attribute handling
- Writer with state management
