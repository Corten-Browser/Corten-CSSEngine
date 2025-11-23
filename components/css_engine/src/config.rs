//! Resource Limits Configuration for CSS Engine
//!
//! This module provides configurable resource limits to prevent denial-of-service
//! attacks and ensure predictable memory usage when parsing stylesheets.
//!
//! # Default Limits
//!
//! - `MAX_STYLESHEET_SIZE`: 10 MB
//! - `MAX_RULES_PER_SHEET`: 50,000 rules
//! - `MAX_SELECTORS_PER_RULE`: 1,000 selectors
//! - `MAX_NESTING_DEPTH`: 100 levels
//!
//! # Example
//!
//! ```
//! use css_engine::ResourceLimits;
//!
//! let limits = ResourceLimits::default();
//! assert_eq!(limits.max_stylesheet_size, 10 * 1024 * 1024);
//!
//! // Create custom limits for a more restrictive environment
//! let custom_limits = ResourceLimits::new()
//!     .with_max_stylesheet_size(1024 * 1024)  // 1 MB
//!     .with_max_rules_per_sheet(10000);
//! ```

/// Default maximum stylesheet size (10 MB)
pub const DEFAULT_MAX_STYLESHEET_SIZE: usize = 10 * 1024 * 1024;

/// Default maximum number of rules per stylesheet (50,000)
pub const DEFAULT_MAX_RULES_PER_SHEET: usize = 50_000;

/// Default maximum number of selectors per rule (1,000)
pub const DEFAULT_MAX_SELECTORS_PER_RULE: usize = 1_000;

/// Default maximum nesting depth for rules (100)
pub const DEFAULT_MAX_NESTING_DEPTH: usize = 100;

/// Default maximum declaration count per rule (500)
pub const DEFAULT_MAX_DECLARATIONS_PER_RULE: usize = 500;

/// Default maximum selector length in characters (4096)
pub const DEFAULT_MAX_SELECTOR_LENGTH: usize = 4096;

/// Default maximum URL length in characters (2048)
pub const DEFAULT_MAX_URL_LENGTH: usize = 2048;

/// Resource limits configuration for the CSS engine
///
/// This struct contains all configurable limits that control resource usage
/// during stylesheet parsing and processing. These limits help prevent
/// denial-of-service attacks and ensure predictable memory consumption.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLimits {
    /// Maximum stylesheet size in bytes
    pub max_stylesheet_size: usize,
    /// Maximum number of rules per stylesheet
    pub max_rules_per_sheet: usize,
    /// Maximum number of selectors per rule
    pub max_selectors_per_rule: usize,
    /// Maximum nesting depth for @-rules (media queries, supports, etc.)
    pub max_nesting_depth: usize,
    /// Maximum number of declarations per rule
    pub max_declarations_per_rule: usize,
    /// Maximum selector string length in characters
    pub max_selector_length: usize,
    /// Maximum URL length in characters
    pub max_url_length: usize,
}

impl ResourceLimits {
    /// Create a new ResourceLimits with default values
    ///
    /// # Example
    ///
    /// ```
    /// use css_engine::ResourceLimits;
    ///
    /// let limits = ResourceLimits::new();
    /// assert_eq!(limits.max_stylesheet_size, 10 * 1024 * 1024);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create ResourceLimits with no limits (for testing or trusted content)
    ///
    /// # Warning
    ///
    /// This should only be used for trusted content or testing purposes.
    /// Using unlimited limits with untrusted content can lead to denial-of-service.
    ///
    /// # Example
    ///
    /// ```
    /// use css_engine::ResourceLimits;
    ///
    /// let limits = ResourceLimits::unlimited();
    /// assert_eq!(limits.max_stylesheet_size, usize::MAX);
    /// ```
    pub fn unlimited() -> Self {
        ResourceLimits {
            max_stylesheet_size: usize::MAX,
            max_rules_per_sheet: usize::MAX,
            max_selectors_per_rule: usize::MAX,
            max_nesting_depth: usize::MAX,
            max_declarations_per_rule: usize::MAX,
            max_selector_length: usize::MAX,
            max_url_length: usize::MAX,
        }
    }

    /// Create restrictive limits suitable for processing untrusted content
    ///
    /// These limits are more conservative than the defaults, suitable for
    /// environments where security is paramount.
    ///
    /// # Example
    ///
    /// ```
    /// use css_engine::ResourceLimits;
    ///
    /// let limits = ResourceLimits::restrictive();
    /// assert!(limits.max_stylesheet_size < ResourceLimits::default().max_stylesheet_size);
    /// ```
    pub fn restrictive() -> Self {
        ResourceLimits {
            max_stylesheet_size: 1024 * 1024,     // 1 MB
            max_rules_per_sheet: 10_000,
            max_selectors_per_rule: 100,
            max_nesting_depth: 20,
            max_declarations_per_rule: 100,
            max_selector_length: 1024,
            max_url_length: 512,
        }
    }

    /// Set the maximum stylesheet size in bytes
    ///
    /// # Example
    ///
    /// ```
    /// use css_engine::ResourceLimits;
    ///
    /// let limits = ResourceLimits::new()
    ///     .with_max_stylesheet_size(5 * 1024 * 1024);  // 5 MB
    /// assert_eq!(limits.max_stylesheet_size, 5 * 1024 * 1024);
    /// ```
    pub fn with_max_stylesheet_size(mut self, size: usize) -> Self {
        self.max_stylesheet_size = size;
        self
    }

    /// Set the maximum number of rules per stylesheet
    ///
    /// # Example
    ///
    /// ```
    /// use css_engine::ResourceLimits;
    ///
    /// let limits = ResourceLimits::new()
    ///     .with_max_rules_per_sheet(25000);
    /// assert_eq!(limits.max_rules_per_sheet, 25000);
    /// ```
    pub fn with_max_rules_per_sheet(mut self, count: usize) -> Self {
        self.max_rules_per_sheet = count;
        self
    }

    /// Set the maximum number of selectors per rule
    ///
    /// # Example
    ///
    /// ```
    /// use css_engine::ResourceLimits;
    ///
    /// let limits = ResourceLimits::new()
    ///     .with_max_selectors_per_rule(500);
    /// assert_eq!(limits.max_selectors_per_rule, 500);
    /// ```
    pub fn with_max_selectors_per_rule(mut self, count: usize) -> Self {
        self.max_selectors_per_rule = count;
        self
    }

    /// Set the maximum nesting depth
    ///
    /// # Example
    ///
    /// ```
    /// use css_engine::ResourceLimits;
    ///
    /// let limits = ResourceLimits::new()
    ///     .with_max_nesting_depth(50);
    /// assert_eq!(limits.max_nesting_depth, 50);
    /// ```
    pub fn with_max_nesting_depth(mut self, depth: usize) -> Self {
        self.max_nesting_depth = depth;
        self
    }

    /// Set the maximum declarations per rule
    pub fn with_max_declarations_per_rule(mut self, count: usize) -> Self {
        self.max_declarations_per_rule = count;
        self
    }

    /// Set the maximum selector length
    pub fn with_max_selector_length(mut self, length: usize) -> Self {
        self.max_selector_length = length;
        self
    }

    /// Set the maximum URL length
    pub fn with_max_url_length(mut self, length: usize) -> Self {
        self.max_url_length = length;
        self
    }

    /// Check if a stylesheet size is within limits
    ///
    /// # Returns
    ///
    /// `true` if the size is within limits, `false` otherwise
    pub fn check_stylesheet_size(&self, size: usize) -> bool {
        size <= self.max_stylesheet_size
    }

    /// Check if a rule count is within limits
    pub fn check_rules_count(&self, count: usize) -> bool {
        count <= self.max_rules_per_sheet
    }

    /// Check if a selector count is within limits
    pub fn check_selectors_count(&self, count: usize) -> bool {
        count <= self.max_selectors_per_rule
    }

    /// Check if a nesting depth is within limits
    pub fn check_nesting_depth(&self, depth: usize) -> bool {
        depth <= self.max_nesting_depth
    }

    /// Check if a declaration count is within limits
    pub fn check_declarations_count(&self, count: usize) -> bool {
        count <= self.max_declarations_per_rule
    }

    /// Check if a selector length is within limits
    pub fn check_selector_length(&self, length: usize) -> bool {
        length <= self.max_selector_length
    }

    /// Check if a URL length is within limits
    pub fn check_url_length(&self, length: usize) -> bool {
        length <= self.max_url_length
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        ResourceLimits {
            max_stylesheet_size: DEFAULT_MAX_STYLESHEET_SIZE,
            max_rules_per_sheet: DEFAULT_MAX_RULES_PER_SHEET,
            max_selectors_per_rule: DEFAULT_MAX_SELECTORS_PER_RULE,
            max_nesting_depth: DEFAULT_MAX_NESTING_DEPTH,
            max_declarations_per_rule: DEFAULT_MAX_DECLARATIONS_PER_RULE,
            max_selector_length: DEFAULT_MAX_SELECTOR_LENGTH,
            max_url_length: DEFAULT_MAX_URL_LENGTH,
        }
    }
}

/// Validation result for resource limit checks
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitViolation {
    /// Stylesheet size exceeds maximum
    StylesheetTooLarge {
        size: usize,
        max: usize,
    },
    /// Too many rules in stylesheet
    TooManyRules {
        count: usize,
        max: usize,
    },
    /// Too many selectors in a rule
    TooManySelectors {
        count: usize,
        max: usize,
    },
    /// Nesting depth exceeds maximum
    NestingTooDeep {
        depth: usize,
        max: usize,
    },
    /// Too many declarations in a rule
    TooManyDeclarations {
        count: usize,
        max: usize,
    },
    /// Selector string is too long
    SelectorTooLong {
        length: usize,
        max: usize,
    },
    /// URL is too long
    UrlTooLong {
        length: usize,
        max: usize,
    },
}

impl std::fmt::Display for LimitViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LimitViolation::StylesheetTooLarge { size, max } => {
                write!(
                    f,
                    "Stylesheet size ({} bytes) exceeds maximum ({} bytes)",
                    size, max
                )
            }
            LimitViolation::TooManyRules { count, max } => {
                write!(
                    f,
                    "Rule count ({}) exceeds maximum ({})",
                    count, max
                )
            }
            LimitViolation::TooManySelectors { count, max } => {
                write!(
                    f,
                    "Selector count ({}) exceeds maximum ({})",
                    count, max
                )
            }
            LimitViolation::NestingTooDeep { depth, max } => {
                write!(
                    f,
                    "Nesting depth ({}) exceeds maximum ({})",
                    depth, max
                )
            }
            LimitViolation::TooManyDeclarations { count, max } => {
                write!(
                    f,
                    "Declaration count ({}) exceeds maximum ({})",
                    count, max
                )
            }
            LimitViolation::SelectorTooLong { length, max } => {
                write!(
                    f,
                    "Selector length ({} chars) exceeds maximum ({} chars)",
                    length, max
                )
            }
            LimitViolation::UrlTooLong { length, max } => {
                write!(
                    f,
                    "URL length ({} chars) exceeds maximum ({} chars)",
                    length, max
                )
            }
        }
    }
}

impl std::error::Error for LimitViolation {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_stylesheet_size, 10 * 1024 * 1024);
        assert_eq!(limits.max_rules_per_sheet, 50_000);
        assert_eq!(limits.max_selectors_per_rule, 1_000);
        assert_eq!(limits.max_nesting_depth, 100);
        assert_eq!(limits.max_declarations_per_rule, 500);
        assert_eq!(limits.max_selector_length, 4096);
        assert_eq!(limits.max_url_length, 2048);
    }

    #[test]
    fn test_new_equals_default() {
        assert_eq!(ResourceLimits::new(), ResourceLimits::default());
    }

    #[test]
    fn test_unlimited_limits() {
        let limits = ResourceLimits::unlimited();
        assert_eq!(limits.max_stylesheet_size, usize::MAX);
        assert_eq!(limits.max_rules_per_sheet, usize::MAX);
        assert_eq!(limits.max_selectors_per_rule, usize::MAX);
        assert_eq!(limits.max_nesting_depth, usize::MAX);
    }

    #[test]
    fn test_restrictive_limits() {
        let limits = ResourceLimits::restrictive();
        let default = ResourceLimits::default();

        assert!(limits.max_stylesheet_size < default.max_stylesheet_size);
        assert!(limits.max_rules_per_sheet < default.max_rules_per_sheet);
        assert!(limits.max_selectors_per_rule < default.max_selectors_per_rule);
        assert!(limits.max_nesting_depth < default.max_nesting_depth);
    }

    #[test]
    fn test_builder_pattern() {
        let limits = ResourceLimits::new()
            .with_max_stylesheet_size(5 * 1024 * 1024)
            .with_max_rules_per_sheet(25000)
            .with_max_selectors_per_rule(500)
            .with_max_nesting_depth(50);

        assert_eq!(limits.max_stylesheet_size, 5 * 1024 * 1024);
        assert_eq!(limits.max_rules_per_sheet, 25000);
        assert_eq!(limits.max_selectors_per_rule, 500);
        assert_eq!(limits.max_nesting_depth, 50);
    }

    #[test]
    fn test_check_stylesheet_size() {
        let limits = ResourceLimits::new().with_max_stylesheet_size(1000);

        assert!(limits.check_stylesheet_size(500));
        assert!(limits.check_stylesheet_size(1000));
        assert!(!limits.check_stylesheet_size(1001));
    }

    #[test]
    fn test_check_rules_count() {
        let limits = ResourceLimits::new().with_max_rules_per_sheet(100);

        assert!(limits.check_rules_count(50));
        assert!(limits.check_rules_count(100));
        assert!(!limits.check_rules_count(101));
    }

    #[test]
    fn test_check_selectors_count() {
        let limits = ResourceLimits::new().with_max_selectors_per_rule(10);

        assert!(limits.check_selectors_count(5));
        assert!(limits.check_selectors_count(10));
        assert!(!limits.check_selectors_count(11));
    }

    #[test]
    fn test_check_nesting_depth() {
        let limits = ResourceLimits::new().with_max_nesting_depth(5);

        assert!(limits.check_nesting_depth(3));
        assert!(limits.check_nesting_depth(5));
        assert!(!limits.check_nesting_depth(6));
    }

    #[test]
    fn test_check_declarations_count() {
        let limits = ResourceLimits::new().with_max_declarations_per_rule(100);

        assert!(limits.check_declarations_count(50));
        assert!(limits.check_declarations_count(100));
        assert!(!limits.check_declarations_count(101));
    }

    #[test]
    fn test_check_selector_length() {
        let limits = ResourceLimits::new().with_max_selector_length(100);

        assert!(limits.check_selector_length(50));
        assert!(limits.check_selector_length(100));
        assert!(!limits.check_selector_length(101));
    }

    #[test]
    fn test_check_url_length() {
        let limits = ResourceLimits::new().with_max_url_length(100);

        assert!(limits.check_url_length(50));
        assert!(limits.check_url_length(100));
        assert!(!limits.check_url_length(101));
    }

    #[test]
    fn test_limit_violation_display() {
        let violation = LimitViolation::StylesheetTooLarge {
            size: 20_000_000,
            max: 10_000_000,
        };
        assert!(violation.to_string().contains("20000000"));
        assert!(violation.to_string().contains("10000000"));

        let violation = LimitViolation::TooManyRules {
            count: 60000,
            max: 50000,
        };
        assert!(violation.to_string().contains("60000"));
        assert!(violation.to_string().contains("50000"));

        let violation = LimitViolation::NestingTooDeep {
            depth: 150,
            max: 100,
        };
        assert!(violation.to_string().contains("150"));
        assert!(violation.to_string().contains("100"));
    }

    #[test]
    fn test_clone_and_eq() {
        let limits1 = ResourceLimits::new();
        let limits2 = limits1.clone();
        assert_eq!(limits1, limits2);

        let limits3 = ResourceLimits::new().with_max_stylesheet_size(1000);
        assert_ne!(limits1, limits3);
    }
}
