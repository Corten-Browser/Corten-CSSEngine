//! CSS Selector Matcher Core
//!
//! This crate provides core selector matching functionality for CSS selectors.
//! It supports CSS2.1 selectors including:
//! - Simple selectors (tag, class, ID, universal)
//! - Compound selectors (combinations of simple selectors)
//! - Combinators (descendant, child, adjacent sibling)
//!
//! # Examples
//!
//! ```
//! use css_matcher_core::{Selector, Component, SelectorMatcher, ElementLike};
//!
//! # #[derive(Debug, Clone)]
//! # struct Element {
//! #     tag_name: String,
//! #     id: Option<String>,
//! #     classes: Vec<String>,
//! # }
//! # impl Element {
//! #     fn new(tag: &str) -> Self {
//! #         Self { tag_name: tag.to_string(), id: None, classes: Vec::new() }
//! #     }
//! # }
//! # impl ElementLike for Element {
//! #     fn tag_name(&self) -> &str { &self.tag_name }
//! #     fn id(&self) -> Option<&str> { self.id.as_deref() }
//! #     fn classes(&self) -> &[String] { &self.classes }
//! #     fn parent(&self) -> Option<&Self> { None }
//! #     fn previous_sibling(&self) -> Option<&Self> { None }
//! # }
//! let matcher = SelectorMatcher;
//! let selector = Selector {
//!     components: vec![Component::Tag("div".to_string())],
//! };
//! let element = Element::new("div");
//!
//! assert!(matcher.matches(&selector, &element));
//! ```

mod matcher;
mod types;

pub use matcher::{ElementLike, SelectorMatcher};
pub use types::{Combinator, ComplexSelector, Component, Selector};
