use std::error::Error;
use std::fmt;
use std::io;

/// Custom error type for HTML rendering operations
#[derive(Debug)]
pub enum HtmlError {
    /// IO-related errors
    Io(io::Error),
    /// Writing-related errors
    Write(fmt::Error),
    /// Theme-related errors (for syntax highlighting)
    Theme(String),
    /// Configuration validation errors
    Config(String),
    /// General rendering errors
    Render(String),
}

impl fmt::Display for HtmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlError::Io(err) => write!(f, "IO error: {}", err),
            HtmlError::Write(err) => write!(f, "Write error: {}", err),
            HtmlError::Theme(err) => write!(f, "Theme error: {}", err),
            HtmlError::Config(err) => write!(f, "Configuration error: {}", err),
            HtmlError::Render(err) => write!(f, "Rendering error: {}", err),
        }
    }
}

impl Error for HtmlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HtmlError::Io(err) => Some(err),
            HtmlError::Write(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for HtmlError {
    fn from(err: io::Error) -> Self {
        HtmlError::Io(err)
    }
}

impl From<fmt::Error> for HtmlError {
    fn from(err: fmt::Error) -> Self {
        HtmlError::Write(err)
    }
}
