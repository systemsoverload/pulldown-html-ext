#[cfg(test)]
mod tests {
    use pulldown_html_ext::{
        push_html_with_highlighting, HtmlConfig, SyntectConfig, SyntectConfigStyle,
    };
    use syntect::highlighting::ThemeSet;
    use syntect::html::ClassStyle;
    use syntect::parsing::SyntaxSet;

    #[test]
    fn test_basic_highlighting() {
        let config = HtmlConfig::with_syntect(SyntectConfig::default());
        let markdown = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(html.contains("<pre><code class=\"language-rust\">"));
        assert!(html.contains("println!"));
    }

    #[test]
    fn test_custom_theme() {
        let config = HtmlConfig::with_syntect(SyntectConfig {
            style: SyntectConfigStyle {
                theme: "base16-mocha.dark".to_string(),
                ..SyntectConfigStyle::default()
            },
            ..Default::default()
        });

        let markdown = "```rust\nlet x = 42;\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(html.contains("<style>"));
        assert!(html.contains("language-rust"));
    }

    #[test]
    fn test_custom_syntax_set() {
        let syntax_set = SyntaxSet::new();
        let _ = SyntaxSet::load_defaults_newlines();

        let config = HtmlConfig::with_syntect(SyntectConfig {
            syntax_set: Some(syntax_set),
            ..Default::default()
        });

        let markdown = "```python\nprint('hello')\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(html.contains("language-python"));
    }

    #[test]
    fn test_no_css_injection() {
        let config = HtmlConfig::with_syntect(SyntectConfig {
            style: SyntectConfigStyle {
                inject_css: false,
                ..SyntectConfigStyle::default()
            },
            ..Default::default()
        });

        let markdown = "```rust\nlet x = 42;\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(!html.contains("<style>"));
        assert!(html.contains("language-rust"));
    }

    #[test]
    fn test_custom_class_style() {
        let config = HtmlConfig::with_syntect(SyntectConfig {
            style: SyntectConfigStyle {
                class_style: ClassStyle::SpacedPrefixed { prefix: "" },
                ..SyntectConfigStyle::default()
            },
            ..Default::default()
        });

        let markdown = "```rust\nfn main() {}\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(html.contains("language-rust"));
    }

    #[test]
    fn test_custom_theme_and_syntax_set() {
        let syntax_set = SyntaxSet::new();
        let _ = SyntaxSet::load_defaults_newlines();

        let theme_set = ThemeSet::load_defaults();

        let config = HtmlConfig::with_syntect(SyntectConfig {
            style: SyntectConfigStyle {
                theme: "base16-ocean.dark".to_string(),
                ..SyntectConfigStyle::default()
            },
            syntax_set: Some(syntax_set),
            theme_set: Some(theme_set),
        });

        let markdown = "```rust\nfn test() -> u32 { 42 }\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(html.contains("<style>"));
        assert!(html.contains("language-rust"));
    }

    #[test]
    fn test_unknown_language() {
        let config = HtmlConfig::with_syntect(SyntectConfig::default());
        let markdown = "```unknown-lang\nSome code\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(html.contains("language-unknown-lang"));
    }

    #[test]
    fn test_no_language_specified() {
        let config = HtmlConfig::with_syntect(SyntectConfig::default());
        let markdown = "```\nPlain text code block\n```";
        let html = push_html_with_highlighting(markdown, &config).unwrap();

        assert!(html.contains("<pre><code>"));
        assert!(html.contains("Plain text code block"));
    }
}
