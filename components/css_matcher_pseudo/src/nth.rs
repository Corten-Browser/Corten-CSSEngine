//! Nth selector parsing (an+b notation)
//!
//! Supports parsing of CSS nth-child/nth-of-type expressions like:
//! - "2n+1" (odd)
//! - "2n" (even)
//! - "3" (specific index)
//! - "-n+5" (first 5)

use css_types::CssError;

/// Represents an nth-selector in the form an+b
///
/// Examples:
/// - 2n+1 → NthSelector { a: 2, b: 1 } (odd elements)
/// - 2n → NthSelector { a: 2, b: 0 } (even elements)
/// - 3 → NthSelector { a: 0, b: 3 } (third element)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NthSelector {
    /// The 'a' coefficient (multiplier for n)
    pub a: i32,
    /// The 'b' offset
    pub b: i32,
}

impl NthSelector {
    /// Create a new nth-selector
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    /// Check if a given index matches this nth-selector
    ///
    /// Index is 1-based (first element is index 1)
    pub fn matches(&self, index: usize) -> bool {
        let index = index as i32;

        if self.a == 0 {
            // Simple index match: b
            return index == self.b;
        }

        // Check if there exists an integer n >= 0 such that an+b = index
        if self.a > 0 {
            // n = (index - b) / a must be non-negative integer
            let numerator = index - self.b;
            if numerator < 0 {
                return false;
            }
            numerator % self.a == 0
        } else {
            // a < 0: n = (index - b) / a must be non-positive integer (n <= 0)
            let numerator = index - self.b;
            if numerator > 0 {
                return false;
            }
            numerator % self.a == 0
        }
    }
}

/// Parse an nth-selector string (an+b notation)
///
/// Supports various formats:
/// - "odd" → 2n+1
/// - "even" → 2n
/// - "2n+1" → 2n+1
/// - "3n" → 3n+0
/// - "5" → 0n+5
/// - "-n+5" → -1n+5
///
/// # Examples
///
/// ```
/// use css_matcher_pseudo::parse_nth_selector;
///
/// let selector = parse_nth_selector("2n+1").unwrap();
/// assert_eq!(selector.a, 2);
/// assert_eq!(selector.b, 1);
///
/// let selector = parse_nth_selector("odd").unwrap();
/// assert_eq!(selector.a, 2);
/// assert_eq!(selector.b, 1);
/// ```
pub fn parse_nth_selector(input: &str) -> Result<NthSelector, CssError> {
    let input = input.trim();

    // Handle keywords
    match input {
        "odd" => return Ok(NthSelector::new(2, 1)),
        "even" => return Ok(NthSelector::new(2, 0)),
        _ => {}
    }

    // Parse the general an+b form
    parse_nth_an_plus_b(input)
}

/// Parse the general an+b form
fn parse_nth_an_plus_b(input: &str) -> Result<NthSelector, CssError> {
    let input = input.replace(" ", ""); // Remove spaces

    // Try to find 'n' in the input
    if let Some(n_pos) = input.find('n') {
        // Parse the 'a' part (coefficient of n)
        let a_str = &input[..n_pos];
        let a = if a_str.is_empty() || a_str == "+" {
            1
        } else if a_str == "-" {
            -1
        } else {
            a_str
                .parse::<i32>()
                .map_err(|_| CssError::ParseError(format!("Invalid coefficient: {}", a_str)))?
        };

        // Parse the 'b' part (offset)
        let b_str = &input[n_pos + 1..];
        let b = if b_str.is_empty() {
            0
        } else {
            b_str
                .parse::<i32>()
                .map_err(|_| CssError::ParseError(format!("Invalid offset: {}", b_str)))?
        };

        Ok(NthSelector::new(a, b))
    } else {
        // No 'n', just a number (b)
        let b = input
            .parse::<i32>()
            .map_err(|_| CssError::ParseError(format!("Invalid number: {}", input)))?;
        Ok(NthSelector::new(0, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // NthSelector::matches tests
    // ========================================================================

    #[test]
    fn test_nth_selector_odd_matches() {
        let selector = NthSelector::new(2, 1); // 2n+1 (odd)
        assert!(selector.matches(1)); // 1st element
        assert!(!selector.matches(2)); // 2nd element
        assert!(selector.matches(3)); // 3rd element
        assert!(!selector.matches(4)); // 4th element
        assert!(selector.matches(5)); // 5th element
    }

    #[test]
    fn test_nth_selector_even_matches() {
        let selector = NthSelector::new(2, 0); // 2n (even)
        assert!(!selector.matches(1)); // 1st element
        assert!(selector.matches(2)); // 2nd element
        assert!(!selector.matches(3)); // 3rd element
        assert!(selector.matches(4)); // 4th element
    }

    #[test]
    fn test_nth_selector_specific_index() {
        let selector = NthSelector::new(0, 3); // 3rd element only
        assert!(!selector.matches(1));
        assert!(!selector.matches(2));
        assert!(selector.matches(3));
        assert!(!selector.matches(4));
    }

    #[test]
    fn test_nth_selector_every_third() {
        let selector = NthSelector::new(3, 0); // 3n (every third)
        assert!(!selector.matches(1));
        assert!(!selector.matches(2));
        assert!(selector.matches(3));
        assert!(!selector.matches(4));
        assert!(!selector.matches(5));
        assert!(selector.matches(6));
    }

    #[test]
    fn test_nth_selector_negative_a() {
        let selector = NthSelector::new(-1, 5); // -n+5 (first 5 elements)
        assert!(selector.matches(1));
        assert!(selector.matches(2));
        assert!(selector.matches(3));
        assert!(selector.matches(4));
        assert!(selector.matches(5));
        assert!(!selector.matches(6));
        assert!(!selector.matches(7));
    }

    // ========================================================================
    // parse_nth_selector tests
    // ========================================================================

    #[test]
    fn test_parse_odd_keyword() {
        let result = parse_nth_selector("odd");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 2);
        assert_eq!(selector.b, 1);
    }

    #[test]
    fn test_parse_even_keyword() {
        let result = parse_nth_selector("even");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 2);
        assert_eq!(selector.b, 0);
    }

    #[test]
    fn test_parse_simple_number() {
        let result = parse_nth_selector("3");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 0);
        assert_eq!(selector.b, 3);
    }

    #[test]
    fn test_parse_2n_plus_1() {
        let result = parse_nth_selector("2n+1");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 2);
        assert_eq!(selector.b, 1);
    }

    #[test]
    fn test_parse_2n() {
        let result = parse_nth_selector("2n");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 2);
        assert_eq!(selector.b, 0);
    }

    #[test]
    fn test_parse_3n_plus_2() {
        let result = parse_nth_selector("3n+2");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 3);
        assert_eq!(selector.b, 2);
    }

    #[test]
    fn test_parse_n() {
        let result = parse_nth_selector("n");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 1);
        assert_eq!(selector.b, 0);
    }

    #[test]
    fn test_parse_plus_n() {
        let result = parse_nth_selector("+n");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 1);
        assert_eq!(selector.b, 0);
    }

    #[test]
    fn test_parse_minus_n_plus_5() {
        let result = parse_nth_selector("-n+5");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, -1);
        assert_eq!(selector.b, 5);
    }

    #[test]
    fn test_parse_with_spaces() {
        let result = parse_nth_selector(" 2n + 1 ");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 2);
        assert_eq!(selector.b, 1);
    }

    #[test]
    fn test_parse_negative_offset() {
        let result = parse_nth_selector("2n-1");
        assert!(result.is_ok());
        let selector = result.unwrap();
        assert_eq!(selector.a, 2);
        assert_eq!(selector.b, -1);
    }

    #[test]
    fn test_parse_invalid_input() {
        let result = parse_nth_selector("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_coefficient() {
        let result = parse_nth_selector("xn+1");
        assert!(result.is_err());
    }
}
