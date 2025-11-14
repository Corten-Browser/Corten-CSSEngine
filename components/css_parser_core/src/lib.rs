//! CSS Parser Core - CSS2.1 parsing implementation
//!
//! This module provides a basic CSS parser for CSS2.1 stylesheets,
//! supporting simple selectors (element, class, id) and basic properties.

pub use css_types::{Color, Length, Specificity};
use std::fmt;

mod declaration;
mod parser;
mod selector;

pub use parser::CssParser;

/// Stylesheet origin (author, user, user-agent)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// Author stylesheet (from web page)
    Author,
    /// User stylesheet (from browser settings)
    User,
    /// User agent stylesheet (browser defaults)
    UserAgent,
}

/// CSS parsing error with location information
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl ParseError {
    /// Create a new parse error
    pub fn new(line: usize, column: usize, message: impl Into<String>) -> Self {
        ParseError {
            line,
            column,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ParseError {}

/// Parsed CSS stylesheet
#[derive(Debug, Clone, PartialEq)]
pub struct Stylesheet {
    /// List of CSS rules
    pub rules: Vec<CssRule>,
    /// Stylesheet origin (author, user, user-agent)
    pub origin: Origin,
}

impl Stylesheet {
    /// Create a new empty stylesheet
    pub fn new(origin: Origin) -> Self {
        Stylesheet {
            rules: Vec::new(),
            origin,
        }
    }

    /// Create an author stylesheet
    pub fn author() -> Self {
        Stylesheet::new(Origin::Author)
    }
}

/// CSS rule types
#[derive(Debug, Clone, PartialEq)]
pub enum CssRule {
    /// Style rule (selectors + declarations)
    Style(StyleRule),
    /// Media query rule
    Media(MediaRule),
    /// Import rule
    Import(ImportRule),
}

/// Style rule with selectors and declarations
#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule {
    /// List of selectors
    pub selectors: Vec<Selector>,
    /// List of property declarations
    pub declarations: Vec<PropertyDeclaration>,
}

/// CSS selector (simple selectors for CSS2.1)
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// Element/type selector (e.g., div, span)
    Element(String),
    /// Class selector (e.g., .myclass)
    Class(String),
    /// ID selector (e.g., #myid)
    Id(String),
    /// Universal selector (*)
    Universal,
    /// Compound selector (element + classes/id)
    Compound {
        element: Option<String>,
        classes: Vec<String>,
        id: Option<String>,
    },
}

impl Selector {
    /// Calculate selector specificity
    pub fn specificity(&self) -> Specificity {
        match self {
            Selector::Element(_) => Specificity::new(0, 0, 1),
            Selector::Class(_) => Specificity::new(0, 1, 0),
            Selector::Id(_) => Specificity::new(1, 0, 0),
            Selector::Universal => Specificity::zero(),
            Selector::Compound {
                element,
                classes,
                id,
            } => {
                let id_count = if id.is_some() { 1 } else { 0 };
                let class_count = classes.len() as u32;
                let element_count = if element.is_some() { 1 } else { 0 };
                Specificity::new(id_count, class_count, element_count)
            }
        }
    }
}

/// CSS property declaration
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDeclaration {
    /// Property name (e.g., "color", "margin")
    pub name: String,
    /// Property value
    pub value: PropertyValue,
    /// Whether marked as !important
    pub important: bool,
}

/// CSS property value (simplified for CSS2.1)
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// Color value
    Color(Color),
    /// Length value
    Length(Length),
    /// Keyword value (e.g., "auto", "inherit")
    Keyword(String),
    /// String value
    String(String),
}

/// Media query rule
#[derive(Debug, Clone, PartialEq)]
pub struct MediaRule {
    /// Media query conditions
    pub media_queries: Vec<String>,
    /// Rules within the media query
    pub rules: Vec<CssRule>,
}

/// Import rule
#[derive(Debug, Clone, PartialEq)]
pub struct ImportRule {
    /// URL to import
    pub url: String,
    /// Media queries for the import
    pub media_queries: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_specificity() {
        assert_eq!(
            Selector::Element("div".to_string()).specificity(),
            Specificity::new(0, 0, 1)
        );
        assert_eq!(
            Selector::Class("myclass".to_string()).specificity(),
            Specificity::new(0, 1, 0)
        );
        assert_eq!(
            Selector::Id("myid".to_string()).specificity(),
            Specificity::new(1, 0, 0)
        );
        assert_eq!(Selector::Universal.specificity(), Specificity::zero());
    }

    #[test]
    fn test_compound_selector_specificity() {
        let selector = Selector::Compound {
            element: Some("div".to_string()),
            classes: vec!["class1".to_string(), "class2".to_string()],
            id: Some("myid".to_string()),
        };
        assert_eq!(selector.specificity(), Specificity::new(1, 2, 1));
    }
}
