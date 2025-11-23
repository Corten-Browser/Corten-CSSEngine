//! Debug Logging Infrastructure
//!
//! This module provides structured logging for the CSS engine using the `tracing`
//! crate. It enables detailed debugging output for:
//!
//! - Cascade resolution
//! - Selector matching
//! - Style computation
//! - Cache operations
//!
//! # Log Levels
//!
//! - `ERROR`: Critical failures that prevent operation
//! - `WARN`: Recoverable issues or unexpected conditions
//! - `INFO`: High-level operation summaries
//! - `DEBUG`: Detailed operation information
//! - `TRACE`: Very detailed debugging information (performance impact)
//!
//! # Example
//!
//! ```
//! use css_engine::logging::CssLogging;
//!
//! // Get logging targets for filtering
//! let engine_target = CssLogging::target();
//! let parser_target = CssLogging::parser_target();
//! let cache_target = CssLogging::cache_target();
//!
//! assert_eq!(engine_target, "css_engine");
//! ```
//!
//! # Span Conventions
//!
//! Spans are created for major operations:
//!
//! - `parse_stylesheet`: Stylesheet parsing
//! - `compute_styles`: Style tree computation
//! - `match_selectors`: Selector matching
//! - `cascade`: Cascade resolution
//! - `cache_lookup`: Cache operations

/// CSS Engine logging configuration and utilities
pub struct CssLogging;

impl CssLogging {
    /// Get the target name for CSS engine logs
    pub const fn target() -> &'static str {
        "css_engine"
    }

    /// Get the target name for parser logs
    pub const fn parser_target() -> &'static str {
        "css_parser"
    }

    /// Get the target name for cascade logs
    pub const fn cascade_target() -> &'static str {
        "css_cascade"
    }

    /// Get the target name for selector matching logs
    pub const fn selector_target() -> &'static str {
        "css_selector"
    }

    /// Get the target name for cache logs
    pub const fn cache_target() -> &'static str {
        "css_cache"
    }
}

/// Logging macros for consistent structured logging
///
/// These macros wrap tracing macros with CSS engine specific targets
/// and field conventions.
#[macro_export]
macro_rules! css_trace {
    ($($arg:tt)*) => {
        tracing::trace!(target: "css_engine", $($arg)*)
    };
}

#[macro_export]
macro_rules! css_debug {
    ($($arg:tt)*) => {
        tracing::debug!(target: "css_engine", $($arg)*)
    };
}

#[macro_export]
macro_rules! css_info {
    ($($arg:tt)*) => {
        tracing::info!(target: "css_engine", $($arg)*)
    };
}

#[macro_export]
macro_rules! css_warn {
    ($($arg:tt)*) => {
        tracing::warn!(target: "css_engine", $($arg)*)
    };
}

#[macro_export]
macro_rules! css_error {
    ($($arg:tt)*) => {
        tracing::error!(target: "css_engine", $($arg)*)
    };
}

/// Span macros for operation tracing
#[macro_export]
macro_rules! css_span {
    ($name:expr) => {
        tracing::span!(target: "css_engine", tracing::Level::DEBUG, $name)
    };
    ($name:expr, $($field:tt)*) => {
        tracing::span!(target: "css_engine", tracing::Level::DEBUG, $name, $($field)*)
    };
}

/// Logging events for structured log output
#[derive(Debug, Clone)]
pub enum LogEvent {
    /// Stylesheet parsing started
    ParseStart {
        source_url: Option<String>,
        size: usize,
    },
    /// Stylesheet parsing completed
    ParseComplete {
        rules_count: usize,
        duration_ms: f64,
    },
    /// Style computation started
    ComputeStart { element_count: usize },
    /// Style computation completed
    ComputeComplete {
        computed_count: usize,
        duration_ms: f64,
    },
    /// Cache hit
    CacheHit { element_id: u32 },
    /// Cache miss
    CacheMiss { element_id: u32 },
    /// Cache invalidation
    CacheInvalidate { element_id: u32, reason: String },
    /// Selector matching started
    SelectorMatchStart { selector: String, element: String },
    /// Selector matching result
    SelectorMatchResult { selector: String, matched: bool },
    /// Cascade resolution
    CascadeResolve { property: String, source: String },
}

impl LogEvent {
    /// Log this event at the appropriate level
    pub fn log(&self) {
        match self {
            LogEvent::ParseStart { source_url, size } => {
                tracing::info!(
                    target: "css_parser",
                    source_url = ?source_url,
                    size = size,
                    "Starting stylesheet parse"
                );
            }
            LogEvent::ParseComplete {
                rules_count,
                duration_ms,
            } => {
                tracing::info!(
                    target: "css_parser",
                    rules_count = rules_count,
                    duration_ms = duration_ms,
                    "Stylesheet parse complete"
                );
            }
            LogEvent::ComputeStart { element_count } => {
                tracing::debug!(
                    target: "css_engine",
                    element_count = element_count,
                    "Starting style computation"
                );
            }
            LogEvent::ComputeComplete {
                computed_count,
                duration_ms,
            } => {
                tracing::debug!(
                    target: "css_engine",
                    computed_count = computed_count,
                    duration_ms = duration_ms,
                    "Style computation complete"
                );
            }
            LogEvent::CacheHit { element_id } => {
                tracing::trace!(
                    target: "css_cache",
                    element_id = element_id,
                    "Cache hit"
                );
            }
            LogEvent::CacheMiss { element_id } => {
                tracing::trace!(
                    target: "css_cache",
                    element_id = element_id,
                    "Cache miss"
                );
            }
            LogEvent::CacheInvalidate { element_id, reason } => {
                tracing::debug!(
                    target: "css_cache",
                    element_id = element_id,
                    reason = %reason,
                    "Cache invalidation"
                );
            }
            LogEvent::SelectorMatchStart { selector, element } => {
                tracing::trace!(
                    target: "css_selector",
                    selector = %selector,
                    element = %element,
                    "Starting selector match"
                );
            }
            LogEvent::SelectorMatchResult { selector, matched } => {
                tracing::trace!(
                    target: "css_selector",
                    selector = %selector,
                    matched = matched,
                    "Selector match result"
                );
            }
            LogEvent::CascadeResolve { property, source } => {
                tracing::trace!(
                    target: "css_cascade",
                    property = %property,
                    source = %source,
                    "Cascade resolution"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_event_creation() {
        let event = LogEvent::ParseStart {
            source_url: Some("test.css".to_string()),
            size: 1000,
        };

        // Just verify we can create events - actual logging is tested separately
        match event {
            LogEvent::ParseStart { source_url, size } => {
                assert_eq!(source_url, Some("test.css".to_string()));
                assert_eq!(size, 1000);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_log_event_cache_hit() {
        let event = LogEvent::CacheHit { element_id: 42 };

        match event {
            LogEvent::CacheHit { element_id } => {
                assert_eq!(element_id, 42);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_log_event_selector_match() {
        let event = LogEvent::SelectorMatchResult {
            selector: "div.class".to_string(),
            matched: true,
        };

        match event {
            LogEvent::SelectorMatchResult { selector, matched } => {
                assert_eq!(selector, "div.class");
                assert!(matched);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_css_logging_targets() {
        assert_eq!(CssLogging::target(), "css_engine");
        assert_eq!(CssLogging::parser_target(), "css_parser");
        assert_eq!(CssLogging::cascade_target(), "css_cascade");
        assert_eq!(CssLogging::selector_target(), "css_selector");
        assert_eq!(CssLogging::cache_target(), "css_cache");
    }

    #[test]
    fn test_log_event_clone() {
        let event = LogEvent::ParseComplete {
            rules_count: 50,
            duration_ms: 1.5,
        };
        let cloned = event.clone();

        match cloned {
            LogEvent::ParseComplete {
                rules_count,
                duration_ms,
            } => {
                assert_eq!(rules_count, 50);
                assert!((duration_ms - 1.5).abs() < f64::EPSILON);
            }
            _ => panic!("Wrong event type"),
        }
    }
}
