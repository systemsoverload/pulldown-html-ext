use serde::Deserialize;
use std::collections::HashMap;

/// Main configuration struct for the HTML renderer
#[derive(Debug, Clone, Deserialize)]
pub struct HtmlConfig {
    /// HTML-specific rendering options
    pub html: HtmlOptions,
    /// Options for different Markdown elements
    pub elements: ElementOptions,
    /// Custom attribute mappings
    pub attributes: AttributeMappings,
    /// Syntect syntax highlighting configuration (style only)
    pub syntect: Option<crate::html::syntect::SyntectConfigStyle>,
}
/// Configuration options for HTML output
#[derive(Debug, Clone, Deserialize)]
pub struct HtmlOptions {
    /// Whether to escape HTML in the input
    pub escape_html: bool,
    /// Whether to convert newlines to <br> tags
    pub break_on_newline: bool,
    /// Whether to use XHTML-style self-closing tags
    pub xhtml_style: bool,
    /// Whether to add newlines after block elements for prettier output
    pub pretty_print: bool,
}

/// Configuration options for different Markdown elements
#[derive(Debug, Clone, Deserialize)]
pub struct ElementOptions {
    /// Options for heading elements
    pub headings: HeadingOptions,
    /// Options for link elements
    pub links: LinkOptions,
    /// Options for code blocks
    pub code_blocks: CodeBlockOptions,
}

/// Configuration options for headings
#[derive(Debug, Clone, Deserialize)]
pub struct HeadingOptions {
    /// Whether to add IDs to headings
    pub add_ids: bool,
    /// Prefix to use for heading IDs
    pub id_prefix: String,
    /// CSS classes to add to different heading levels
    #[serde(deserialize_with = "deserialize_heading_map")]
    pub level_classes: HashMap<u8, String>,
}

/// Configuration options for links
#[derive(Debug, Clone, Deserialize)]
pub struct LinkOptions {
    /// Whether to add rel="nofollow" to external links
    pub nofollow_external: bool,
    /// Whether to add target="_blank" to external links
    pub open_external_blank: bool,
}

/// Configuration options for code blocks
#[derive(Debug, Clone, Deserialize)]
pub struct CodeBlockOptions {
    /// Default language for code blocks that don't specify one
    pub default_language: Option<String>,
    /// Whether to add line numbers to code blocks
    pub line_numbers: bool,
}

/// Custom attribute mappings for HTML elements
#[derive(Debug, Clone, Deserialize)]
pub struct AttributeMappings {
    /// Mapping of element names to their attributes
    #[serde(deserialize_with = "deserialize_nested_string_map")]
    pub element_attributes: HashMap<String, HashMap<String, String>>,
}

impl Default for HtmlConfig {
    fn default() -> Self {
        HtmlConfig {
            html: HtmlOptions {
                escape_html: false,
                break_on_newline: true,
                xhtml_style: false,
                pretty_print: true,
            },
            elements: ElementOptions {
                headings: HeadingOptions {
                    add_ids: true,
                    id_prefix: "heading-".to_string(),
                    level_classes: HashMap::new(),
                },
                links: LinkOptions {
                    nofollow_external: true,
                    open_external_blank: true,
                },
                code_blocks: CodeBlockOptions {
                    default_language: None,
                    line_numbers: false,
                },
            },
            attributes: AttributeMappings {
                element_attributes: HashMap::new(),
            },
            #[cfg(feature = "syntect")]
            syntect: None,
        }
    }
}

fn deserialize_heading_map<'de, D>(deserializer: D) -> Result<HashMap<u8, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Object(map) => {
            let mut result = HashMap::new();
            for (k, v) in map {
                let level = k.parse::<u8>().map_err(D::Error::custom)?;
                if !(1..=6).contains(&level) {
                    return Err(D::Error::custom(format!(
                        "heading level must be between 1 and 6, got {}",
                        level
                    )));
                }
                let class = v
                    .as_str()
                    .ok_or_else(|| D::Error::custom("value must be a string"))?
                    .to_string();
                result.insert(level, class);
            }
            Ok(result)
        }
        _ => Err(D::Error::custom("expected a map")),
    }
}

fn deserialize_nested_string_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, HashMap<String, String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Object(outer_map) => {
            let mut result = HashMap::new();
            for (outer_key, inner_value) in outer_map {
                match inner_value {
                    Value::Object(inner_map) => {
                        let mut inner_result = HashMap::new();
                        for (inner_key, value) in inner_map {
                            let str_value = value
                                .as_str()
                                .ok_or_else(|| D::Error::custom("value must be a string"))?
                                .to_string();
                            inner_result.insert(inner_key, str_value);
                        }
                        result.insert(outer_key, inner_result);
                    }
                    _ => return Err(D::Error::custom("expected a nested map")),
                }
            }
            Ok(result)
        }
        _ => Err(D::Error::custom("expected a map")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default_config() {
        let config = HtmlConfig::default();
        assert!(!config.html.escape_html);
        assert!(config.html.break_on_newline);
        assert!(!config.html.xhtml_style);
        assert!(config.html.pretty_print);
    }

    #[test]
    fn test_heading_map_deserialization() {
        let json = json!({
            "1": "title",
            "2": "subtitle",
            "6": "small-title"
        });

        let map: HashMap<u8, String> = deserialize_heading_map(json).unwrap();
        assert_eq!(map.get(&1).unwrap(), "title");
        assert_eq!(map.get(&2).unwrap(), "subtitle");
        assert_eq!(map.get(&6).unwrap(), "small-title");
    }

    #[test]
    fn test_invalid_heading_level() {
        let json = json!({
            "7": "invalid"
        });

        let result: Result<HashMap<u8, String>, _> = deserialize_heading_map(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_attribute_map_deserialization() {
        let json = json!({
            "h1": {
                "class": "title",
                "data-level": "1"
            },
            "pre": {
                "class": "code-block"
            }
        });

        let map: HashMap<String, HashMap<String, String>> =
            deserialize_nested_string_map(json).unwrap();
        assert_eq!(map.get("h1").unwrap().get("class").unwrap(), "title");
        assert_eq!(map.get("h1").unwrap().get("data-level").unwrap(), "1");
        assert_eq!(map.get("pre").unwrap().get("class").unwrap(), "code-block");
    }
}
