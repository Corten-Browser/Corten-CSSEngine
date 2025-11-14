//! Error types for the CSS Engine

use thiserror::Error;

/// Main error type for CSS Engine operations
#[derive(Debug, Error)]
pub enum CssError {
    /// Parse error occurred while parsing CSS
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Invalid selector syntax
    #[error("Invalid selector: {selector}")]
    InvalidSelector { selector: String },

    /// Unsupported CSS property
    #[error("Unsupported property: {property}")]
    UnsupportedProperty { property: String },

    /// Invalid value for a CSS property
    #[error("Invalid value for property {property}: {value}")]
    InvalidValue { property: String, value: String },

    /// Circular custom property reference detected
    #[error("Circular custom property reference: {name}")]
    CircularReference { name: String },

    /// Style computation failed
    #[error("Style computation failed: {reason}")]
    ComputationError { reason: String },

    /// Element not found in style tree
    #[error("Element not found: {element_id:?}")]
    ElementNotFound { element_id: ElementId },

    /// Stylesheet not found
    #[error("Stylesheet not found: {stylesheet_id:?}")]
    StylesheetNotFound { stylesheet_id: StyleSheetId },

    /// Out of memory
    #[error("Out of memory")]
    OutOfMemory,
}

/// Unique identifier for a parsed stylesheet
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StyleSheetId(pub u32);

/// Unique identifier for a DOM element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(pub u64);

impl StyleSheetId {
    pub fn new(id: u32) -> Self {
        StyleSheetId(id)
    }
}

impl ElementId {
    pub fn new(id: u64) -> Self {
        ElementId(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stylesheet_id_creation() {
        let id = StyleSheetId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_element_id_creation() {
        let id = ElementId::new(123);
        assert_eq!(id.0, 123);
    }

    #[test]
    fn test_error_display() {
        let error = CssError::ParseError {
            line: 10,
            column: 5,
            message: "unexpected token".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("line 10"));
        assert!(msg.contains("column 5"));
    }
}
