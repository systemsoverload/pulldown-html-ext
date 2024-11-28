use html_compare_rs::assert_html_eq;
use pulldown_cmark::Parser;
use pulldown_html_ext::*;
use std::collections::HashMap;

fn render_with_config(input: &str, config: &HtmlConfig) -> String {
    let mut output = String::new();
    let handler = DefaultHtmlWriter::new(&mut output, config);
    let mut renderer = HtmlRenderer::new(handler);
    renderer.run(Parser::new(input));
    output
}

// Individual HTML options tests
#[test]
#[ignore = "TODO: Fix/define escape_html handling in renderer"]
fn test_escape_html_option() {
    let mut config = HtmlConfig::default();

    // With HTML escaping (default)
    config.html.escape_html = true;
    assert_html_eq!(
        render_with_config("<div>test</div>", &config),
        "<p>&lt;div&gt;test&lt;/div&gt;</p>"
    );

    // Without HTML escaping
    config.html.escape_html = false;
    assert_html_eq!(
        render_with_config("<div>test</div>", &config),
        "<p><div>test</div></p>"
    );
}

#[test]
fn test_break_on_newline_option() {
    let mut config = HtmlConfig::default();

    // With break on newline (default)
    config.html.break_on_newline = true;
    assert_html_eq!(
        render_with_config("Line 1\nLine 2", &config),
        "<p>Line 1<br>Line 2</p>"
    );

    // Without break on newline
    config.html.break_on_newline = false;
    assert_html_eq!(
        render_with_config("Line 1\nLine 2", &config),
        "<p>Line 1\nLine 2</p>"
    );
}

#[test]
fn test_xhtml_style_option() {
    let mut config = HtmlConfig::default();

    // Without XHTML style (default)
    config.html.xhtml_style = false;
    assert_html_eq!(
        render_with_config("![Alt](image.jpg)", &config),
        "<p><img src=\"image.jpg\" alt=\"Alt\"></p>"
    );

    // With XHTML style
    config.html.xhtml_style = true;
    assert_html_eq!(
        render_with_config("![Alt](image.jpg)", &config),
        "<p><img src=\"image.jpg\" alt=\"Alt\" /></p>"
    );
}

// Individual element options tests
#[test]
fn test_heading_id_option() {
    let mut config = HtmlConfig::default();

    // With heading IDs (default)
    config.elements.headings.add_ids = true;
    assert_html_eq!(
        render_with_config("# Test Heading", &config),
        "<h1 id=\"heading-1\">Test Heading</h1>"
    );

    // Without heading IDs
    config.elements.headings.add_ids = false;
    assert_html_eq!(
        render_with_config("# Test Heading", &config),
        "<h1>Test Heading</h1>"
    );
}

#[test]
fn test_heading_id_prefix_option() {
    let mut config = HtmlConfig::default();
    config.elements.headings.id_prefix = "section-".to_string();

    assert_html_eq!(
        render_with_config("# Test Heading", &config),
        "<h1 id=\"section-1\">Test Heading</h1>"
    );
}

#[test]
fn test_heading_level_classes() {
    let mut config = HtmlConfig::default();
    let mut level_classes = HashMap::new();
    level_classes.insert(1, "title".to_string());
    level_classes.insert(2, "subtitle".to_string());
    config.elements.headings.level_classes = level_classes;

    assert_html_eq!(
        render_with_config("# Heading 1\n## Heading 2", &config),
        "<h1 id=\"heading-1\" class=\"title\">Heading 1</h1>\
             <h2 id=\"heading-2\" class=\"subtitle\">Heading 2</h2>"
    );
}

#[test]
fn test_link_options() {
    let mut config = HtmlConfig::default();
    config.elements.links.nofollow_external = true;
    config.elements.links.open_external_blank = true;

    assert_html_eq!(
        render_with_config(
            "[Internal](/test) and [External](https://example.com)",
            &config
        ),
        "<p><a href=\"/test\">Internal</a> and \
             <a href=\"https://example.com\" rel=\"nofollow\" target=\"_blank\">External</a></p>"
    );
}

#[test]
fn test_code_block_options() {
    let mut config = HtmlConfig::default();
    config.elements.code_blocks.default_language = Some("text".to_string());

    // Explicit language should override default
    assert_html_eq!(
        render_with_config("```python\nprint('hello')\n```", &config),
        "<pre><code class=\"language-python\">print('hello')</code></pre>"
    );

    // No language specified should use default
    assert_html_eq!(
        render_with_config("```\nhello\n```", &config),
        "<pre><code class=\"language-text\">hello</code></pre>"
    );
}

#[test]
fn test_custom_attributes() {
    let mut config = HtmlConfig::default();
    let mut element_attributes = HashMap::new();

    // Add attributes for paragraphs
    let mut p_attrs = HashMap::new();
    p_attrs.insert("class".to_string(), "content".to_string());
    element_attributes.insert("p".to_string(), p_attrs);

    // Add attributes for code blocks
    let mut pre_attrs = HashMap::new();
    pre_attrs.insert("data-type".to_string(), "code".to_string());
    element_attributes.insert("pre".to_string(), pre_attrs);

    config.attributes.element_attributes = element_attributes;

    assert_html_eq!(
        render_with_config("Regular paragraph\n\n```\nCode block\n```", &config),
        "<p class=\"content\">Regular paragraph</p>\
             <pre data-type=\"code\"><code>Code block</code></pre>"
    );
}

// Mixed configuration tests
#[test]
fn test_mixed_config_blog_style() {
    let mut config = HtmlConfig::default();

    // Configure for blog-style output
    config.html.break_on_newline = false;
    config.elements.headings.add_ids = true;
    config.elements.links.open_external_blank = true;
    config.elements.links.nofollow_external = false;

    let mut heading_classes = HashMap::new();
    heading_classes.insert(1, "post-title".to_string());
    config.elements.headings.level_classes = heading_classes;

    let input = "# Blog Post Title\n\
                     Some text with an [external link](https://example.com).\n\n\
                     Multiple paragraphs look better\n\
                     without forced line breaks.";

    assert_html_eq!(
            render_with_config(input, &config),
            "<h1 id=\"heading-1\" class=\"post-title\">Blog Post Title</h1>\
             <p>Some text with an <a href=\"https://example.com\" target=\"_blank\">external link</a>.</p>\
             <p>Multiple paragraphs look better\nwithout forced line breaks.</p>"
        );
}

#[test]
fn test_mixed_config_documentation_style() {
    let mut config = HtmlConfig::default();

    // Configure for documentation-style output
    config.elements.headings.id_prefix = "doc-".to_string();
    config.elements.code_blocks.default_language = Some("text".to_string());
    config.elements.links.nofollow_external = true;
    config.elements.links.open_external_blank = false;

    let mut element_attrs = HashMap::new();
    let mut pre_attrs = HashMap::new();
    pre_attrs.insert("class".to_string(), "documentation-code".to_string());
    element_attrs.insert("pre".to_string(), pre_attrs);
    config.attributes.element_attributes = element_attrs;

    let input = "# Documentation\n\
                     Check the [reference](https://example.com).\n\n\
                     ```python\ndef example():\n    pass\n```\n\n\
                     ```\nPlain text example\n```";

    assert_html_eq!(
            render_with_config(input, &config),
            "<h1 id=\"doc-1\">Documentation</h1>\
             <p>Check the <a href=\"https://example.com\" rel=\"nofollow\">reference</a>.</p>\
             <pre class=\"documentation-code\"><code class=\"language-python\">def example():\n    pass</code></pre>\
             <pre class=\"documentation-code\"><code class=\"language-text\">Plain text example</code></pre>"
        );
}

#[test]
fn test_mixed_config_presentation_style() {
    let mut config = HtmlConfig::default();

    // Configure for presentation-style output
    config.html.xhtml_style = true;
    config.html.break_on_newline = true;

    let mut heading_classes = HashMap::new();
    heading_classes.insert(1, "slide-title".to_string());
    heading_classes.insert(2, "slide-subtitle".to_string());
    config.elements.headings.level_classes = heading_classes;

    let mut element_attrs = HashMap::new();
    let mut p_attrs = HashMap::new();
    p_attrs.insert("class".to_string(), "slide-content".to_string());
    element_attrs.insert("p".to_string(), p_attrs);
    config.attributes.element_attributes = element_attrs;

    let input = "# Slide Title\n\
                     ## Subtitle\n\n\
                     Content line 1\n\
                     Content line 2\n\n\
                     ![Diagram](image.jpg)";

    assert_html_eq!(
        render_with_config(input, &config),
        "<h1 id=\"heading-1\" class=\"slide-title\">Slide Title</h1>\
             <h2 id=\"heading-2\" class=\"slide-subtitle\">Subtitle</h2>\
             <p class=\"slide-content\">Content line 1<br />Content line 2</p>\
             <p class=\"slide-content\"><img src=\"image.jpg\" alt=\"Diagram\" /></p>"
    );
}
