//! Utility functions for HTML rendering and string manipulation

use pulldown_cmark::escape::StrWrite;
/// Escape special HTML characters in a string
///
/// # Arguments
///
/// * `output` - The string buffer to write to
/// * `text` - The text to escape
///
/// # Example
///
/// ```
/// let mut output = String::new();
/// pulldown_html_ext::utils::escape_html(&mut output, "<div>test</div>");
/// assert_eq!(output, "&lt;div&gt;test&lt;/div&gt;");
/// ```
pub fn escape_html(output: &mut String, text: &str) {
    // TODO - Opt for using the `pulldown-cmark-escape` crate here
    for c in text.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '&' => output.push_str("&amp;"),
            '\'' => output.push_str("&#x27;"),
            _ => output.push(c),
        }
    }
}

/// Escape special characters in URLs
///
/// # Arguments
///
/// * `output` - The string buffer to write to
/// * `href` - The URL to escape
///
/// # Example
///
/// ```
/// let mut output = String::new();
/// pulldown_html_ext::utils::escape_href(&mut output, "https://example.com/path with spaces");
/// assert!(output.contains("%20"));
/// ```
pub fn escape_href(output: &mut String, href: &str) {
    for c in href.chars() {
        match c {
            '<' | '>' | '"' | '\'' | ' ' | '\n' | '\r' | '\t' => {
                write!(output, "%{:02X}", c as u32).unwrap();
            }
            c => output.push(c),
        }
    }
}

/// Sanitize a string for use as an HTML ID
///
/// Converts a string to lowercase, replaces spaces with hyphens,
/// and removes any characters that aren't alphanumeric or hyphens.
///
/// # Arguments
///
/// * `text` - The text to sanitize
///
/// # Example
///
/// ```
/// let id = pulldown_html_ext::utils::sanitize_id("Hello World! 123");
/// assert_eq!(id, "hello-world-123");
/// ```
pub fn sanitize_id(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Count the length of a string in Unicode scalars
///
/// This is useful for generating heading IDs and other cases
/// where we need to know the true length of a string.
///
/// # Arguments
///
/// * `text` - The text to measure
///
/// # Example
///
/// ```
/// let len = pulldown_html_ext::utils::unicode_length("Hello 👋");
/// assert_eq!(len, 7); // 6 ASCII chars + 1 emoji
/// ```
pub fn unicode_length(text: &str) -> usize {
    text.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        let mut output = String::new();
        escape_html(&mut output, "<div class=\"test\">&");
        assert_eq!(output, "&lt;div class=&quot;test&quot;&gt;&amp;");
    }

    #[test]
    fn test_escape_href() {
        let mut output = String::new();
        escape_href(
            &mut output,
            "https://example.com/path with spaces?q=test&x=1",
        );
        assert!(output.contains("%20"));
        assert!(!output.contains(' '));
        assert!(output.contains('&')); // URL parameters shouldn't be escaped
    }

    #[test]
    fn test_sanitize_id() {
        assert_eq!(sanitize_id("Hello World!"), "hello-world");
        assert_eq!(sanitize_id("Test 123"), "test-123");
        assert_eq!(sanitize_id("Multiple   Spaces"), "multiple-spaces");
        assert_eq!(sanitize_id("special@#chars"), "special-chars");
        assert_eq!(sanitize_id("--multiple---dashes--"), "multiple-dashes");
    }

    #[test]
    fn test_unicode_length() {
        assert_eq!(unicode_length("Hello"), 5);
        assert_eq!(unicode_length("👋 Hello"), 7);
        assert_eq!(unicode_length("汉字"), 2);
        assert_eq!(unicode_length(""), 0);
    }

    #[test]
    fn test_complex_escaping() {
        let mut output = String::new();
        escape_html(&mut output, "<script>alert('xss')</script>");
        assert_eq!(
            output,
            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
        );
    }

    #[test]
    fn test_href_special_chars() {
        let mut output = String::new();
        escape_href(&mut output, "/path/with\"quotes'and<brackets>");
        assert!(output.contains("%22")); // escaped quote
        assert!(output.contains("%27")); // escaped single quote
        assert!(output.contains("%3C")); // escaped <
        assert!(output.contains("%3E")); // escaped >
    }
}
