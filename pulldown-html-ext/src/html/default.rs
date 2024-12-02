use pulldown_cmark_escape::StrWrite;

use crate::html::config::HtmlConfig;
use crate::html::state::HtmlState;
use crate::html::writer::HtmlWriter;
use crate::html_writer;

/// Base type for HTML writers that handles common functionality
pub struct HtmlWriterBase<W: StrWrite> {
    writer: W,
    config: HtmlConfig,
    state: HtmlState,
}

impl<W: StrWrite> HtmlWriterBase<W> {
    /// Create a new HtmlWriterBase with the given writer and configuration
    pub fn new(writer: W, config: HtmlConfig) -> Self {
        Self {
            writer,
            config,
            state: HtmlState::new(),
        }
    }

    /// Get a mutable reference to the underlying writer
    pub fn get_writer(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Get a reference to the configuration
    pub fn get_config(&self) -> &HtmlConfig {
        &self.config
    }

    /// Get a mutable reference to the state
    pub fn get_state(&mut self) -> &mut HtmlState {
        &mut self.state
    }
}

/// Default HTML writer implementation that can work with any StrWrite-compatible writer
/// This should be the approximate amount of code any custom implementation needs to
/// provide
#[html_writer]
pub struct DefaultHtmlWriter<W: StrWrite> {
    base: HtmlWriterBase<W>,
}

impl<W: StrWrite> DefaultHtmlWriter<W> {
    /// Create a new DefaultHtmlWriter with the given writer and configuration
    pub fn new(writer: W, config: HtmlConfig) -> Self {
        Self {
            base: HtmlWriterBase::new(writer, config.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark_escape::{escape_html, FmtWriter};
    use std::fmt;

    #[test]
    fn test_basic_writing() {
        let mut output = String::new();
        let config = HtmlConfig::default();
        let mut writer = DefaultHtmlWriter::new(FmtWriter(&mut output), config);

        writer.write_str("<p>").unwrap();
        let _ = escape_html(&mut writer.get_writer(), "Hello & World");
        writer.write_str("</p>").unwrap();

        assert_eq!(output, "<p>Hello &amp; World</p>");
    }

    #[test]
    fn test_attributes() {
        let mut output = String::new();
        let mut config = HtmlConfig::default();
        config.attributes.element_attributes.insert(
            "p".to_string(),
            [("class".to_string(), "test".to_string())]
                .into_iter()
                .collect(),
        );

        let mut writer = DefaultHtmlWriter::new(FmtWriter(&mut output), config);
        writer.start_paragraph().unwrap();
        writer.text("Test").unwrap();
        writer.end_paragraph().unwrap();

        assert_eq!(output, r#"<p class="test">Test</p>"#);
    }

    struct TestWriter(String);

    impl StrWrite for TestWriter {
        type Error = fmt::Error;

        fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
            self.0.push_str(s);
            Ok(())
        }

        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), Self::Error> {
            fmt::write(self, args)
        }
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.push_str(s);
            Ok(())
        }
    }

    #[test]
    fn test_custom_writer() {
        let config = HtmlConfig::default();
        let mut writer = DefaultHtmlWriter::new(TestWriter(String::new()), config);

        writer.write_str("Test").unwrap();
        assert_eq!(writer.get_writer().0, "Test");
    }

    #[test]
    fn test_state_tracking() {
        let mut output = String::new();
        let mut config = HtmlConfig::default();
        config.html.escape_html = true;
        let mut writer = DefaultHtmlWriter::new(FmtWriter(&mut output), config);

        assert!(!writer.get_state().currently_in_code_block);
        writer.get_state().currently_in_code_block = true;
        writer
            .start_code_block(pulldown_cmark::CodeBlockKind::Fenced("rust".into()))
            .unwrap();
        assert!(writer.get_state().currently_in_code_block);
        writer.end_code_block().unwrap();
        writer.get_state().currently_in_code_block = false;
        assert!(!writer.get_state().currently_in_code_block);
    }
}
