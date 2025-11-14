use std::collections::HashMap;

// Re-export Specificity from css_types
pub use css_types::Specificity;

/// Rule origin for cascade ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    UserAgent,
    User,
    Author,
}

/// Simple selector representation for testing
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Universal,
    Type(String),
    Class(String),
    Id(String),
    Attribute { name: String, value: Option<String> },
    PseudoClass(String),
    PseudoElement(String),
    Compound(Vec<Selector>),
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    AdjacentSibling(Box<Selector>, Box<Selector>),
}

/// Property identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyId {
    Color,
    FontSize,
    FontFamily,
    LineHeight,
    TextAlign,
    Margin,
    Padding,
    Border,
    Width,
    Height,
    Display,
}

/// Property value
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Keyword(String),
    Length(f64, String),
    Number(f64),
    FontFamily(Vec<String>),
    Border {
        width: f64,
        style: String,
        color: String,
    },
    Important(Box<PropertyValue>),
    Inherit,
}

/// Style rule with declarations
#[derive(Debug, Clone)]
pub struct StyleRule {
    pub declarations: Vec<(PropertyId, PropertyValue)>,
}

/// Rule applicable to an element with its specificity and origin
#[derive(Debug, Clone)]
pub struct ApplicableRule {
    pub rule: StyleRule,
    pub specificity: Specificity,
    pub origin: Origin,
    pub source_order: usize,
}

/// Result of cascade resolution
#[derive(Debug, Clone)]
pub struct CascadeResult {
    pub properties: HashMap<PropertyId, PropertyValue>,
}

impl CascadeResult {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
}

impl Default for CascadeResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Computed values for an element
#[derive(Debug, Clone, Default)]
pub struct ComputedValues {
    properties: HashMap<PropertyId, PropertyValue>,
}

impl ComputedValues {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn set(&mut self, property: PropertyId, value: PropertyValue) {
        self.properties.insert(property, value);
    }

    pub fn get(&self, property: &PropertyId) -> Option<&PropertyValue> {
        self.properties.get(property)
    }

    pub fn get_mut(&mut self, property: &PropertyId) -> Option<&mut PropertyValue> {
        self.properties.get_mut(property)
    }

    pub fn contains_key(&self, property: &PropertyId) -> bool {
        self.properties.contains_key(property)
    }
}
