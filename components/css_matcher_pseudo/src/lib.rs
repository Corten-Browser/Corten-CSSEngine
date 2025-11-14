//! CSS Pseudo-Class and Pseudo-Element Matcher
//!
//! This crate provides pseudo-class and pseudo-element selector matching for CSS selectors.
//! It supports CSS3 pseudo-classes and pseudo-elements including:
//! - Structural pseudo-classes (:first-child, :last-child, :nth-child)
//! - User action pseudo-classes (:hover, :active, :focus)
//! - UI state pseudo-classes (:enabled, :disabled, :checked)
//! - Pseudo-elements (::before, ::after, ::first-line, ::first-letter)

mod nth;
mod pseudo_class;
mod pseudo_element;

pub use nth::{parse_nth_selector, NthSelector};
pub use pseudo_class::{
    evaluate_pseudo_class, ElementLikeExt, MatchContext, PseudoClass, PseudoClassKind,
};
pub use pseudo_element::{
    DefaultPseudoElementMatcher, PseudoElement, PseudoElementKind, PseudoElementMatcher,
    PseudoElementStyle,
};
