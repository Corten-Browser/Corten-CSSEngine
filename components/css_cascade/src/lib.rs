//! CSS Cascade Algorithm Implementation
//!
//! This component implements the CSS cascade algorithm, including:
//! - Specificity calculation for selectors
//! - Cascade resolution (origin, specificity, source order)
//! - Property inheritance
//! - !important declaration handling
//!
//! # Examples
//!
//! ```
//! use css_cascade::{CascadeResolver, Selector};
//! use css_types::Specificity;
//!
//! // Calculate specificity
//! let selector = Selector::Id("header".to_string());
//! let spec = CascadeResolver::compute_specificity(&selector);
//! assert_eq!(spec, Specificity::new(1, 0, 0));
//! ```

mod resolver;
mod types;

// Re-export Specificity from css_types for convenience
pub use css_types::Specificity;

// Re-export public types and functions from our modules
pub use resolver::CascadeResolver;
pub use types::{
    ApplicableRule, CascadeResult, ComputedValues, Origin, PropertyId, PropertyValue, Selector,
    StyleRule,
};
