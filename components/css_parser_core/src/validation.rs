//! Input Validation Security for CSS Parser
//!
//! This module provides security-focused validation for CSS input to prevent
//! denial-of-service attacks and ensure safe parsing.
//!
//! # Security Features
//!
//! - **Size Limits**: Reject stylesheets larger than configurable maximum (default 10MB)
//! - **Nesting Depth**: Limit nesting depth to prevent stack overflow
//! - **URL Validation**: Validate URLs in @import and url() functions
//! - **Selector Complexity**: Limit selector complexity
//!
//! # Example
//!
//! ```
//! use css_parser_core::{InputValidator, ValidationConfig};
//!
//! let validator = InputValidator::default();
//! assert!(validator.validate_stylesheet_size("body { color: red; }"));
//!
//! // Validate URL
//! assert!(validator.validate_url("https://example.com/styles.css").is_ok());
//! assert!(validator.validate_url("javascript:alert(1)").is_err());
//! ```

use crate::ParseError;

/// Default maximum stylesheet size (10 MB)
pub const DEFAULT_MAX_SIZE: usize = 10 * 1024 * 1024;

/// Default maximum nesting depth
pub const DEFAULT_MAX_NESTING: usize = 100;

/// Default maximum URL length
pub const DEFAULT_MAX_URL_LENGTH: usize = 2048;

/// Default maximum selector length
pub const DEFAULT_MAX_SELECTOR_LENGTH: usize = 4096;

/// Allowed URL schemes for @import and url()
pub const ALLOWED_URL_SCHEMES: &[&str] = &["http", "https", "data"];

/// Forbidden URL schemes (security risk)
pub const FORBIDDEN_URL_SCHEMES: &[&str] = &["javascript", "vbscript", "file"];

/// Validation configuration for CSS input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConfig {
    /// Maximum stylesheet size in bytes
    pub max_stylesheet_size: usize,
    /// Maximum nesting depth for @-rules
    pub max_nesting_depth: usize,
    /// Maximum URL length in characters
    pub max_url_length: usize,
    /// Maximum selector length in characters
    pub max_selector_length: usize,
    /// Whether to validate URLs
    pub validate_urls: bool,
    /// Whether to allow data: URLs
    pub allow_data_urls: bool,
}

impl ValidationConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a strict configuration for untrusted content
    pub fn strict() -> Self {
        ValidationConfig {
            max_stylesheet_size: 1024 * 1024, // 1 MB
            max_nesting_depth: 20,
            max_url_length: 512,
            max_selector_length: 1024,
            validate_urls: true,
            allow_data_urls: false,
        }
    }

    /// Create a permissive configuration for trusted content
    pub fn permissive() -> Self {
        ValidationConfig {
            max_stylesheet_size: 100 * 1024 * 1024, // 100 MB
            max_nesting_depth: 500,
            max_url_length: 8192,
            max_selector_length: 16384,
            validate_urls: false,
            allow_data_urls: true,
        }
    }

    /// Set maximum stylesheet size
    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_stylesheet_size = size;
        self
    }

    /// Set maximum nesting depth
    pub fn with_max_nesting(mut self, depth: usize) -> Self {
        self.max_nesting_depth = depth;
        self
    }

    /// Set maximum URL length
    pub fn with_max_url_length(mut self, length: usize) -> Self {
        self.max_url_length = length;
        self
    }

    /// Set maximum selector length
    pub fn with_max_selector_length(mut self, length: usize) -> Self {
        self.max_selector_length = length;
        self
    }

    /// Enable or disable URL validation
    pub fn with_url_validation(mut self, enabled: bool) -> Self {
        self.validate_urls = enabled;
        self
    }

    /// Enable or disable data: URLs
    pub fn with_data_urls(mut self, allowed: bool) -> Self {
        self.allow_data_urls = allowed;
        self
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        ValidationConfig {
            max_stylesheet_size: DEFAULT_MAX_SIZE,
            max_nesting_depth: DEFAULT_MAX_NESTING,
            max_url_length: DEFAULT_MAX_URL_LENGTH,
            max_selector_length: DEFAULT_MAX_SELECTOR_LENGTH,
            validate_urls: true,
            allow_data_urls: true,
        }
    }
}

/// Input validation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Stylesheet exceeds size limit
    StylesheetTooLarge {
        size: usize,
        max: usize,
    },
    /// Nesting depth exceeds limit
    NestingTooDeep {
        depth: usize,
        max: usize,
    },
    /// URL exceeds length limit
    UrlTooLong {
        length: usize,
        max: usize,
    },
    /// URL uses forbidden scheme
    ForbiddenUrlScheme {
        scheme: String,
    },
    /// URL uses unknown scheme
    UnknownUrlScheme {
        scheme: String,
    },
    /// Invalid URL format
    InvalidUrlFormat {
        url: String,
        reason: String,
    },
    /// Selector exceeds length limit
    SelectorTooLong {
        length: usize,
        max: usize,
    },
    /// Data URLs not allowed
    DataUrlNotAllowed,
    /// Empty input
    EmptyInput,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::StylesheetTooLarge { size, max } => {
                write!(
                    f,
                    "Stylesheet size ({} bytes) exceeds maximum ({} bytes)",
                    size, max
                )
            }
            ValidationError::NestingTooDeep { depth, max } => {
                write!(
                    f,
                    "Nesting depth ({}) exceeds maximum ({})",
                    depth, max
                )
            }
            ValidationError::UrlTooLong { length, max } => {
                write!(
                    f,
                    "URL length ({} chars) exceeds maximum ({} chars)",
                    length, max
                )
            }
            ValidationError::ForbiddenUrlScheme { scheme } => {
                write!(f, "Forbidden URL scheme: {}", scheme)
            }
            ValidationError::UnknownUrlScheme { scheme } => {
                write!(f, "Unknown URL scheme: {}", scheme)
            }
            ValidationError::InvalidUrlFormat { url, reason } => {
                write!(f, "Invalid URL '{}': {}", url, reason)
            }
            ValidationError::SelectorTooLong { length, max } => {
                write!(
                    f,
                    "Selector length ({} chars) exceeds maximum ({} chars)",
                    length, max
                )
            }
            ValidationError::DataUrlNotAllowed => {
                write!(f, "Data URLs are not allowed in this context")
            }
            ValidationError::EmptyInput => {
                write!(f, "Empty input is not allowed")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

impl From<ValidationError> for ParseError {
    fn from(err: ValidationError) -> Self {
        ParseError::new(0, 0, err.to_string())
    }
}

/// Input validator for CSS content
#[derive(Debug, Clone)]
pub struct InputValidator {
    config: ValidationConfig,
}

impl InputValidator {
    /// Create a new validator with the given configuration
    pub fn new(config: ValidationConfig) -> Self {
        InputValidator { config }
    }

    /// Create a validator with default configuration
    pub fn with_defaults() -> Self {
        Self::new(ValidationConfig::default())
    }

    /// Create a strict validator for untrusted content
    pub fn strict() -> Self {
        Self::new(ValidationConfig::strict())
    }

    /// Get the current configuration
    pub fn config(&self) -> &ValidationConfig {
        &self.config
    }

    /// Validate stylesheet size
    ///
    /// # Returns
    ///
    /// `true` if the stylesheet size is within limits
    pub fn validate_stylesheet_size(&self, css: &str) -> bool {
        css.len() <= self.config.max_stylesheet_size
    }

    /// Validate stylesheet size with detailed error
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::StylesheetTooLarge` if size exceeds limit
    pub fn check_stylesheet_size(&self, css: &str) -> Result<(), ValidationError> {
        let size = css.len();
        if size > self.config.max_stylesheet_size {
            Err(ValidationError::StylesheetTooLarge {
                size,
                max: self.config.max_stylesheet_size,
            })
        } else {
            Ok(())
        }
    }

    /// Validate nesting depth
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NestingTooDeep` if depth exceeds limit
    pub fn check_nesting_depth(&self, depth: usize) -> Result<(), ValidationError> {
        if depth > self.config.max_nesting_depth {
            Err(ValidationError::NestingTooDeep {
                depth,
                max: self.config.max_nesting_depth,
            })
        } else {
            Ok(())
        }
    }

    /// Calculate the current nesting depth in CSS text
    ///
    /// This counts the maximum nesting level of curly braces.
    pub fn calculate_nesting_depth(&self, css: &str) -> usize {
        let mut max_depth: usize = 0;
        let mut current_depth: usize = 0;

        for ch in css.chars() {
            match ch {
                '{' => {
                    current_depth += 1;
                    if current_depth > max_depth {
                        max_depth = current_depth;
                    }
                }
                '}' => {
                    current_depth = current_depth.saturating_sub(1);
                }
                _ => {}
            }
        }

        max_depth
    }

    /// Validate nesting depth in CSS text
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NestingTooDeep` if depth exceeds limit
    pub fn validate_nesting(&self, css: &str) -> Result<(), ValidationError> {
        let depth = self.calculate_nesting_depth(css);
        self.check_nesting_depth(depth)
    }

    /// Validate a URL
    ///
    /// Checks:
    /// - URL length
    /// - URL scheme (must be allowed, not forbidden)
    /// - Basic URL format
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if URL is invalid
    pub fn validate_url(&self, url: &str) -> Result<(), ValidationError> {
        let url = url.trim();

        // Check for empty URL
        if url.is_empty() {
            return Err(ValidationError::EmptyInput);
        }

        // Check length
        if url.len() > self.config.max_url_length {
            return Err(ValidationError::UrlTooLong {
                length: url.len(),
                max: self.config.max_url_length,
            });
        }

        // Skip validation if disabled
        if !self.config.validate_urls {
            return Ok(());
        }

        // Extract scheme (if present)
        if let Some(scheme_end) = url.find(':') {
            let scheme = &url[..scheme_end].to_lowercase();

            // Check for forbidden schemes
            if FORBIDDEN_URL_SCHEMES.contains(&scheme.as_str()) {
                return Err(ValidationError::ForbiddenUrlScheme {
                    scheme: scheme.clone(),
                });
            }

            // Check for data: URLs
            if scheme == "data" {
                if !self.config.allow_data_urls {
                    return Err(ValidationError::DataUrlNotAllowed);
                }
                return Ok(());
            }

            // Check for allowed schemes
            if !ALLOWED_URL_SCHEMES.contains(&scheme.as_str()) {
                // Allow relative URLs (no scheme) and URLs starting with //
                if !url.starts_with('/') && !url.starts_with("//") {
                    return Err(ValidationError::UnknownUrlScheme {
                        scheme: scheme.clone(),
                    });
                }
            }
        }

        // Basic format validation
        self.validate_url_format(url)?;

        Ok(())
    }

    /// Validate URL format (basic checks)
    fn validate_url_format(&self, url: &str) -> Result<(), ValidationError> {
        // Check for null bytes (security risk)
        if url.contains('\0') {
            return Err(ValidationError::InvalidUrlFormat {
                url: url.to_string(),
                reason: "URL contains null bytes".to_string(),
            });
        }

        // Check for control characters
        for ch in url.chars() {
            if ch.is_ascii_control() && ch != '\t' {
                return Err(ValidationError::InvalidUrlFormat {
                    url: url.to_string(),
                    reason: format!("URL contains control character: {:?}", ch),
                });
            }
        }

        // Check for newlines (potential header injection)
        if url.contains('\n') || url.contains('\r') {
            return Err(ValidationError::InvalidUrlFormat {
                url: url.to_string(),
                reason: "URL contains newline characters".to_string(),
            });
        }

        Ok(())
    }

    /// Extract and validate URLs from CSS text
    ///
    /// Finds all url() and @import declarations and validates them.
    pub fn validate_urls_in_css(&self, css: &str) -> Result<Vec<String>, ValidationError> {
        let mut urls = Vec::new();

        // Find url() functions
        for url in Self::extract_url_functions(css) {
            self.validate_url(&url)?;
            urls.push(url);
        }

        // Find @import URLs
        for url in Self::extract_import_urls(css) {
            self.validate_url(&url)?;
            urls.push(url);
        }

        Ok(urls)
    }

    /// Extract URLs from url() functions
    fn extract_url_functions(css: &str) -> Vec<String> {
        let mut urls = Vec::new();
        let mut remaining = css;

        while let Some(start) = remaining.find("url(") {
            let after_url = &remaining[start + 4..];

            // Find the closing parenthesis
            if let Some(end) = Self::find_url_end(after_url) {
                let url_content = &after_url[..end].trim();

                // Remove quotes if present
                let url = url_content
                    .trim_start_matches('"')
                    .trim_start_matches('\'')
                    .trim_end_matches('"')
                    .trim_end_matches('\'')
                    .to_string();

                if !url.is_empty() {
                    urls.push(url);
                }

                remaining = &after_url[end..];
            } else {
                break;
            }
        }

        urls
    }

    /// Find the end of a url() function
    fn find_url_end(s: &str) -> Option<usize> {
        let mut in_quotes = false;
        let mut quote_char = ' ';
        let mut escaped = false;

        for (i, ch) in s.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }

            match ch {
                '\\' => {
                    escaped = true;
                }
                '"' | '\'' if !in_quotes => {
                    in_quotes = true;
                    quote_char = ch;
                }
                c if in_quotes && c == quote_char => {
                    in_quotes = false;
                }
                ')' if !in_quotes => {
                    return Some(i);
                }
                _ => {}
            }
        }

        None
    }

    /// Extract URLs from @import rules
    fn extract_import_urls(css: &str) -> Vec<String> {
        let mut urls = Vec::new();
        let mut remaining = css;

        while let Some(start) = remaining.find("@import") {
            let after_import = remaining[start + 7..].trim_start();

            // Check for url() or quoted string
            if let Some(url_content) = after_import.strip_prefix("url(") {
                // URL function
                if let Some(end) = Self::find_url_end(url_content) {
                    let url_str = url_content[..end].trim();
                    let url = url_str
                        .trim_start_matches('"')
                        .trim_start_matches('\'')
                        .trim_end_matches('"')
                        .trim_end_matches('\'')
                        .to_string();

                    if !url.is_empty() {
                        urls.push(url);
                    }

                    remaining = &url_content[end..];
                } else {
                    remaining = url_content;
                }
            } else if after_import.starts_with('"') || after_import.starts_with('\'') {
                // Quoted string
                let quote = after_import.chars().next().unwrap();
                if let Some(end) = after_import[1..].find(quote) {
                    let url = after_import[1..1 + end].to_string();
                    if !url.is_empty() {
                        urls.push(url);
                    }
                    remaining = &after_import[2 + end..];
                } else {
                    remaining = &after_import[1..];
                }
            } else {
                remaining = after_import;
            }
        }

        urls
    }

    /// Validate a selector
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::SelectorTooLong` if selector exceeds limit
    pub fn validate_selector(&self, selector: &str) -> Result<(), ValidationError> {
        if selector.len() > self.config.max_selector_length {
            Err(ValidationError::SelectorTooLong {
                length: selector.len(),
                max: self.config.max_selector_length,
            })
        } else {
            Ok(())
        }
    }

    /// Perform full validation on CSS input
    ///
    /// This runs all validation checks:
    /// - Size check
    /// - Nesting depth check
    /// - URL validation
    ///
    /// # Errors
    ///
    /// Returns the first validation error encountered
    pub fn validate(&self, css: &str) -> Result<(), ValidationError> {
        // Check size
        self.check_stylesheet_size(css)?;

        // Check nesting
        self.validate_nesting(css)?;

        // Validate URLs
        self.validate_urls_in_css(css)?;

        Ok(())
    }
}

impl Default for InputValidator {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ValidationConfig::default();
        assert_eq!(config.max_stylesheet_size, 10 * 1024 * 1024);
        assert_eq!(config.max_nesting_depth, 100);
        assert_eq!(config.max_url_length, 2048);
        assert!(config.validate_urls);
        assert!(config.allow_data_urls);
    }

    #[test]
    fn test_strict_config() {
        let config = ValidationConfig::strict();
        assert_eq!(config.max_stylesheet_size, 1024 * 1024);
        assert_eq!(config.max_nesting_depth, 20);
        assert!(!config.allow_data_urls);
    }

    #[test]
    fn test_permissive_config() {
        let config = ValidationConfig::permissive();
        assert_eq!(config.max_stylesheet_size, 100 * 1024 * 1024);
        assert!(!config.validate_urls);
    }

    #[test]
    fn test_config_builder() {
        let config = ValidationConfig::new()
            .with_max_size(5000)
            .with_max_nesting(10)
            .with_max_url_length(500)
            .with_url_validation(false)
            .with_data_urls(false);

        assert_eq!(config.max_stylesheet_size, 5000);
        assert_eq!(config.max_nesting_depth, 10);
        assert_eq!(config.max_url_length, 500);
        assert!(!config.validate_urls);
        assert!(!config.allow_data_urls);
    }

    #[test]
    fn test_stylesheet_size_validation() {
        let validator = InputValidator::new(ValidationConfig::new().with_max_size(100));

        assert!(validator.validate_stylesheet_size("body { color: red; }"));
        assert!(!validator.validate_stylesheet_size(&"x".repeat(101)));
    }

    #[test]
    fn test_check_stylesheet_size() {
        let validator = InputValidator::new(ValidationConfig::new().with_max_size(50));

        assert!(validator.check_stylesheet_size("body { }").is_ok());
        assert!(validator.check_stylesheet_size(&"x".repeat(100)).is_err());
    }

    #[test]
    fn test_nesting_depth_calculation() {
        let validator = InputValidator::default();

        // No nesting
        assert_eq!(validator.calculate_nesting_depth("body { }"), 1);

        // Single level
        assert_eq!(
            validator.calculate_nesting_depth("@media { body { } }"),
            2
        );

        // Multiple levels
        assert_eq!(
            validator.calculate_nesting_depth("@media { @supports { body { } } }"),
            3
        );

        // Empty
        assert_eq!(validator.calculate_nesting_depth(""), 0);
    }

    #[test]
    fn test_nesting_depth_validation() {
        let validator = InputValidator::new(ValidationConfig::new().with_max_nesting(2));

        let shallow = "body { color: red; }";
        assert!(validator.validate_nesting(shallow).is_ok());

        let deep = "@media { @supports { @layer { body { } } } }";
        assert!(validator.validate_nesting(deep).is_err());
    }

    #[test]
    fn test_url_validation_https() {
        let validator = InputValidator::default();

        assert!(validator.validate_url("https://example.com/style.css").is_ok());
        assert!(validator.validate_url("http://example.com/style.css").is_ok());
    }

    #[test]
    fn test_url_validation_data() {
        let validator = InputValidator::default();
        assert!(validator.validate_url("data:image/png;base64,ABC").is_ok());

        let strict_validator = InputValidator::strict();
        assert!(strict_validator.validate_url("data:image/png;base64,ABC").is_err());
    }

    #[test]
    fn test_url_validation_forbidden_schemes() {
        let validator = InputValidator::default();

        assert!(validator.validate_url("javascript:alert(1)").is_err());
        assert!(validator.validate_url("vbscript:msgbox(1)").is_err());
        assert!(validator.validate_url("file:///etc/passwd").is_err());
    }

    #[test]
    fn test_url_validation_relative() {
        let validator = InputValidator::default();

        assert!(validator.validate_url("/styles/main.css").is_ok());
        assert!(validator.validate_url("//cdn.example.com/style.css").is_ok());
        assert!(validator.validate_url("../styles.css").is_ok());
    }

    #[test]
    fn test_url_validation_length() {
        let validator = InputValidator::new(ValidationConfig::new().with_max_url_length(50));

        assert!(validator.validate_url("https://example.com/a").is_ok());
        assert!(validator.validate_url(&format!("https://example.com/{}", "a".repeat(100))).is_err());
    }

    #[test]
    fn test_url_validation_format() {
        let validator = InputValidator::default();

        // Null bytes
        assert!(validator.validate_url("https://example.com\0/bad").is_err());

        // Newlines
        assert!(validator.validate_url("https://example.com\n/bad").is_err());
        assert!(validator.validate_url("https://example.com\r/bad").is_err());

        // Empty
        assert!(validator.validate_url("").is_err());
    }

    #[test]
    fn test_extract_url_functions() {
        let css = r#"
            body {
                background: url("image.png");
                background: url('image2.png');
                background: url(image3.png);
            }
        "#;

        let urls = InputValidator::extract_url_functions(css);
        assert_eq!(urls.len(), 3);
        assert!(urls.contains(&"image.png".to_string()));
        assert!(urls.contains(&"image2.png".to_string()));
        assert!(urls.contains(&"image3.png".to_string()));
    }

    #[test]
    fn test_extract_import_urls() {
        let css = r#"
            @import "styles.css";
            @import 'other.css';
            @import url("third.css");
        "#;

        let urls = InputValidator::extract_import_urls(css);
        assert_eq!(urls.len(), 3);
        assert!(urls.contains(&"styles.css".to_string()));
        assert!(urls.contains(&"other.css".to_string()));
        assert!(urls.contains(&"third.css".to_string()));
    }

    #[test]
    fn test_validate_urls_in_css() {
        let validator = InputValidator::default();

        let valid_css = r#"
            @import "https://example.com/style.css";
            body { background: url("image.png"); }
        "#;
        assert!(validator.validate_urls_in_css(valid_css).is_ok());

        let invalid_css = r#"
            @import "javascript:alert(1)";
        "#;
        assert!(validator.validate_urls_in_css(invalid_css).is_err());
    }

    #[test]
    fn test_selector_validation() {
        let validator = InputValidator::new(
            ValidationConfig::new().with_max_selector_length(50)
        );

        assert!(validator.validate_selector("div.class#id").is_ok());
        assert!(validator.validate_selector(&"a".repeat(100)).is_err());
    }

    #[test]
    fn test_full_validation() {
        let validator = InputValidator::default();

        let valid_css = r#"
            body {
                color: red;
            }
        "#;
        assert!(validator.validate(valid_css).is_ok());
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::StylesheetTooLarge {
            size: 20_000_000,
            max: 10_000_000,
        };
        assert!(err.to_string().contains("20000000"));

        let err = ValidationError::ForbiddenUrlScheme {
            scheme: "javascript".to_string(),
        };
        assert!(err.to_string().contains("javascript"));

        let err = ValidationError::NestingTooDeep { depth: 150, max: 100 };
        assert!(err.to_string().contains("150"));
    }

    #[test]
    fn test_validation_error_to_parse_error() {
        let validation_err = ValidationError::StylesheetTooLarge {
            size: 100,
            max: 50,
        };
        let parse_err: ParseError = validation_err.into();
        assert!(parse_err.message.contains("100"));
    }
}
