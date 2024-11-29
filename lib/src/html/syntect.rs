use crate::html::config;
use lazy_static::lazy_static;
use pulldown_cmark_escape::StrWrite;
use serde::{Deserialize, Deserializer};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::html::{DefaultHtmlWriter, HtmlConfig, HtmlState, HtmlWriter};

lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

fn deserialize_class_style<'de, D>(deserializer: D) -> Result<ClassStyle, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum ClassStyleHelper {
        Spaced,
        SpacedPrefix,
    }

    let style = ClassStyleHelper::deserialize(deserializer)?;
    Ok(match style {
        ClassStyleHelper::Spaced => ClassStyle::Spaced,
        ClassStyleHelper::SpacedPrefix => ClassStyle::SpacedPrefixed { prefix: "" },
    })
}

/// Configuration options for syntax highlighting that can be cloned
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SyntectConfigStyle {
    /// Name of the theme to use (e.g., "base16-ocean.dark")
    pub theme: String,
    /// Style of CSS classes to generate
    #[serde(
        deserialize_with = "deserialize_class_style",
        default = "default_class_style"
    )]
    pub class_style: ClassStyle,
    /// Whether to include CSS in the output
    #[serde(default = "default_inject_css")]
    pub inject_css: bool,
}

fn default_class_style() -> ClassStyle {
    ClassStyle::Spaced
}

fn default_inject_css() -> bool {
    true
}

/// Complete syntax highlighting configuration including non-clonable parts
#[derive(Debug, Default)]
pub struct SyntectConfig {
    /// Style configuration
    pub style: SyntectConfigStyle,
    /// Custom syntax set to use (optional) - primarily for testing
    #[doc(hidden)]
    pub syntax_set: Option<SyntaxSet>,
    /// Custom theme set to use (optional) - primarily for testing
    #[doc(hidden)]
    pub theme_set: Option<ThemeSet>,
}

impl Default for SyntectConfigStyle {
    fn default() -> Self {
        Self {
            theme: "base16-ocean.dark".to_string(),
            class_style: ClassStyle::Spaced,
            inject_css: true,
        }
    }
}

impl HtmlConfig {
    /// Create a new configuration with syntect syntax highlighting enabled
    pub fn with_syntect(syntect_config: SyntectConfig) -> Self {
        HtmlConfig {
            syntect: Some(syntect_config.style),
            ..Default::default()
        }
    }
}

/// Writer that adds syntax highlighting to code blocks
pub struct SyntectWriter<'a, W: StrWrite> {
    inner: DefaultHtmlWriter<'a, W>,
    style: SyntectConfigStyle,
    syntax_set: Option<&'a SyntaxSet>,
    theme_set: Option<&'a ThemeSet>,
    current_lang: Option<String>,
}

impl<'a, W: StrWrite> SyntectWriter<'a, W> {
    pub fn new(writer: W, config: &'a config::HtmlConfig) -> Self {
        let style = config.syntect.clone().unwrap_or_default();

        Self {
            inner: DefaultHtmlWriter::new(writer, config),
            style,
            syntax_set: None,
            theme_set: None,
            current_lang: None,
        }
    }

    pub fn with_custom_sets(
        writer: W,
        config: &'a config::HtmlConfig,
        syntax_set: Option<&'a SyntaxSet>,
        theme_set: Option<&'a ThemeSet>,
    ) -> Self {
        let style = config.syntect.clone().unwrap_or_default();

        Self {
            inner: DefaultHtmlWriter::new(writer, config),
            style,
            syntax_set,
            theme_set,
            current_lang: None,
        }
    }

    fn highlight_code(&self, code: &str, lang: Option<&str>) -> String {
        let syntax_set = self.syntax_set.unwrap_or(&SYNTAX_SET);

        let syntax = match lang {
            Some(lang) => syntax_set
                .find_syntax_by_token(lang)
                .or_else(|| syntax_set.find_syntax_by_extension(lang)),
            None => None,
        }
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        let mut html_generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, syntax_set, self.style.class_style);

        for line in LinesWithEndings::from(code) {
            let _ = html_generator.parse_html_for_line_which_includes_newline(line);
        }

        html_generator.finalize()
    }

    fn get_theme(&self) -> Result<&Theme, String> {
        let theme_set = self.theme_set.unwrap_or(&THEME_SET);
        theme_set
            .themes
            .get(&self.style.theme)
            .ok_or_else(|| format!("Theme '{}' not found", self.style.theme))
    }

    pub fn get_theme_css(&self) -> Result<String, String> {
        let theme = self.get_theme()?;
        syntect::html::css_for_theme_with_class_style(theme, self.style.class_style)
            .map_err(|e| e.to_string())
    }
}

impl<'a, W: StrWrite> HtmlWriter<W> for SyntectWriter<'a, W> {
    fn get_writer(&mut self) -> &mut W {
        self.inner.get_writer()
    }

    fn get_config(&self) -> &HtmlConfig {
        self.inner.get_config()
    }

    fn get_state(&mut self) -> &mut HtmlState {
        self.inner.get_state()
    }

    fn start_code_block(&mut self, kind: pulldown_cmark::CodeBlockKind) {
        self.current_lang = match kind {
            pulldown_cmark::CodeBlockKind::Fenced(ref info) => {
                if info.is_empty() {
                    None
                } else {
                    Some(info.to_string())
                }
            }
            _ => None,
        };

        self.write_str("<pre");
        self.write_attributes("pre");
        self.write_str("><code");

        if let Some(ref lang) = self.current_lang {
            self.write_str(&format!(" class=\"language-{}\"", lang));
        }

        self.write_attributes("code");
        self.write_str(">");

        self.get_state().currently_in_code_block = true;
    }

    fn text(&mut self, text: &str) {
        if self.get_state().currently_in_code_block {
            let highlighted = self.highlight_code(text, self.current_lang.as_deref());
            self.write_str(&highlighted);
        } else {
            self.inner.text(text);
        }
    }

    fn end_code_block(&mut self) {
        self.write_str("</code></pre>");
        self.current_lang = None;
        self.get_state().currently_in_code_block = false;
    }
}

/// Convenience function to render Markdown with syntax highlighting
pub fn push_html_with_highlighting(markdown: &str, config: &HtmlConfig) -> String {
    use pulldown_cmark::Parser;
    use pulldown_cmark_escape::FmtWriter;

    let mut output = String::new();
    let writer = SyntectWriter::new(FmtWriter(&mut output), config);
    let mut renderer = crate::html::create_html_renderer(writer);

    let parser = Parser::new(markdown);
    renderer.run(parser);

    // Add CSS if configured
    if let Some(ref style) = config.syntect {
        if style.inject_css {
            match renderer.writer.get_theme_css() {
                Ok(css) => return format!("<style>{}</style>\n{}", css, output),
                Err(e) => eprintln!("Failed to generate syntax highlighting CSS: {}", e),
            }
        }
    }

    output
}
