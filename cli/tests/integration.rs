use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::Write;

use tempfile::NamedTempFile;

fn create_temp_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "{}", content).unwrap();
    file
}

#[test]
fn test_stdin_stdout() {
    let mut cmd = Command::cargo_bin("pulldown-html-ext-cli").unwrap();
    cmd.write_stdin("# Hello World")
        .assert()
        .success()
        .stdout(predicate::str::contains("<h1"))
        .stdout(predicate::str::contains("Hello World"));
}

#[test]
fn test_file_io() {
    let input = create_temp_file("# Test Heading");
    let output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("pulldown-html-ext-cli").unwrap();
    cmd.arg("-i")
        .arg(input.path())
        .arg("-o")
        .arg(output.path())
        .assert()
        .success();

    let content = fs::read_to_string(output.path()).unwrap();
    assert!(content.contains("<h1"));
    assert!(content.contains("Test Heading"));
}

#[test]
fn test_custom_config() {
    let config_content = r#"
[html]
escape_html = true
break_on_newline = true
xhtml_style = true
pretty_print = true

[elements.headings]
add_ids = true
id_prefix = "test-"
level_classes = { "1" = "title" }

[elements.links]
nofollow_external = false
open_external_blank = true

[elements.code_blocks]
default_language = "text"
line_numbers = false

[attributes]
element_attributes = {}"#;

    let config_file = create_temp_file(config_content);
    let input = create_temp_file("# Test Heading");
    let output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("pulldown-html-ext-cli").unwrap();
    cmd.arg("-i")
        .arg(input.path())
        .arg("-o")
        .arg(output.path())
        .arg("-c")
        .arg(config_file.path())
        .assert()
        .success();

    let content = fs::read_to_string(output.path()).unwrap();
    assert!(content.contains(r#"<h1 id="test-1" class="title""#));
}

#[test]
fn test_invalid_input_file() {
    let mut cmd = Command::cargo_bin("pulldown-html-ext-cli").unwrap();
    cmd.arg("-i").arg("nonexistent.md").assert().failure();
}

#[test]
fn test_invalid_config_file() {
    let invalid_config = create_temp_file("invalid toml content");
    let input = create_temp_file("# Test");

    let mut cmd = Command::cargo_bin("pulldown-html-ext-cli").unwrap();
    cmd.arg("-i")
        .arg(input.path())
        .arg("-c")
        .arg(invalid_config.path())
        .assert()
        .failure();
}

#[test]
#[ignore = "TODO: Fix/implement html_escape"]
fn test_html_escaping() {
    // Create a config that explicitly enables HTML escaping
    let config_content = r#"
[html]
escape_html = true
break_on_newline = true
xhtml_style = false
pretty_print = true

[elements.headings]
add_ids = true
id_prefix = "heading-"
level_classes = {}

[elements.links]
nofollow_external = false
open_external_blank = true

[elements.code_blocks]
default_language = "text"
line_numbers = false

[attributes]
element_attributes = {}"#;

    let config_file = create_temp_file(config_content);
    let mut cmd = Command::cargo_bin("pulldown-html-ext-cli").unwrap();
    cmd.write_stdin("<script>alert('test')</script>")
        .arg("-c")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("&lt;script&gt;"))
        .stdout(predicate::str::contains("&lt;/script&gt;"));
}

#[test]
fn test_complex_markdown() {
    let input = r#"
# Main Title

## Section 1

This is a *test* with some **bold** text and a [link](https://example.com).

```rust
fn main() {
    println!("Hello");
}
```

> Quote
"#;

    let config_content = r#"
[html]
escape_html = true
break_on_newline = true
xhtml_style = false
pretty_print = true

[elements.headings]
add_ids = true
id_prefix = "heading-"
level_classes = {}

[elements.links]
nofollow_external = true
open_external_blank = true

[elements.code_blocks]
default_language = "text"
line_numbers = false

[attributes]
element_attributes = {}"#;

    let config_file = create_temp_file(config_content);
    let input_file = create_temp_file(input);
    let output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("pulldown-html-ext-cli").unwrap();
    cmd.arg("-i")
        .arg(input_file.path())
        .arg("-o")
        .arg(output.path())
        .arg("-c")
        .arg(config_file.path())
        .assert()
        .success();

    let content = fs::read_to_string(output.path()).unwrap();
    assert!(content.contains("<h1"));
    assert!(content.contains("<h2"));
    assert!(content.contains("<em>test</em>"));
    assert!(content.contains("<strong>bold</strong>"));
    assert!(content.contains(r#"rel="nofollow""#));
    assert!(content.contains(r#"target="_blank""#));
    assert!(content.contains("<pre><code"));
    assert!(content.contains("<blockquote>"));
}
