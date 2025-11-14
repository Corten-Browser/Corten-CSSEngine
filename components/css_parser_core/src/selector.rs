//! Selector parsing for CSS2.1
//!
//! Supports simple selectors: element, class, id, and universal selector

use crate::{ParseError, Selector};

/// Parse a list of selectors separated by commas
pub fn parse_selector_list(input: &str) -> Result<Vec<Selector>, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::new(1, 1, "Empty selector"));
    }

    // Split by comma for multiple selectors
    let parts: Vec<&str> = input.split(',').collect();
    let mut selectors = Vec::new();

    for part in parts {
        let selector = parse_single_selector(part.trim())?;
        selectors.push(selector);
    }

    Ok(selectors)
}

/// Parse a single selector
fn parse_single_selector(input: &str) -> Result<Selector, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::new(1, 1, "Empty selector"));
    }

    // Universal selector
    if input == "*" {
        return Ok(Selector::Universal);
    }

    // Check for compound selector (element, class, id combination)
    // Compound if: multiple classes/ids OR element+class/id
    let dot_count = input.matches('.').count();
    let hash_count = input.matches('#').count();
    let is_compound = dot_count + hash_count > 1
        || (dot_count + hash_count > 0 && !input.starts_with('.') && !input.starts_with('#'));

    if is_compound {
        return parse_compound_selector(input);
    }

    // Simple selectors
    if let Some(id) = input.strip_prefix('#') {
        // ID selector
        if id.is_empty() {
            return Err(ParseError::new(1, 1, "Empty ID selector"));
        }
        Ok(Selector::Id(id.to_string()))
    } else if let Some(class) = input.strip_prefix('.') {
        // Class selector
        if class.is_empty() {
            return Err(ParseError::new(1, 1, "Empty class selector"));
        }
        Ok(Selector::Class(class.to_string()))
    } else {
        // Element selector
        if !is_valid_identifier(input) {
            return Err(ParseError::new(
                1,
                1,
                format!("Invalid selector: {}", input),
            ));
        }
        Ok(Selector::Element(input.to_string()))
    }
}

/// Parse a compound selector (e.g., div.class#id, .class1.class2)
fn parse_compound_selector(input: &str) -> Result<Selector, ParseError> {
    let mut element: Option<String> = None;
    let mut classes: Vec<String> = Vec::new();
    let mut id: Option<String> = None;

    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '.' => {
                // Save previous element if any
                if !current.is_empty() && element.is_none() {
                    element = Some(current.clone());
                    current.clear();
                }
                // Read class name
                current.clear();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '.' || next_ch == '#' {
                        break;
                    }
                    current.push(chars.next().unwrap());
                }
                if current.is_empty() {
                    return Err(ParseError::new(1, 1, "Empty class name"));
                }
                classes.push(current.clone());
                current.clear();
            }
            '#' => {
                // Save previous element if any
                if !current.is_empty() && element.is_none() {
                    element = Some(current.clone());
                    current.clear();
                }
                // Read ID
                current.clear();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '.' || next_ch == '#' {
                        break;
                    }
                    current.push(chars.next().unwrap());
                }
                if current.is_empty() {
                    return Err(ParseError::new(1, 1, "Empty ID"));
                }
                if id.is_some() {
                    return Err(ParseError::new(1, 1, "Multiple IDs in selector"));
                }
                id = Some(current.clone());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    // Handle remaining element name
    if !current.is_empty() && element.is_none() {
        element = Some(current);
    }

    Ok(Selector::Compound {
        element,
        classes,
        id,
    })
}

/// Check if a string is a valid CSS identifier
fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_element_selector() {
        let selector = parse_single_selector("div").unwrap();
        assert_eq!(selector, Selector::Element("div".to_string()));
    }

    #[test]
    fn test_parse_class_selector() {
        let selector = parse_single_selector(".myclass").unwrap();
        assert_eq!(selector, Selector::Class("myclass".to_string()));
    }

    #[test]
    fn test_parse_id_selector() {
        let selector = parse_single_selector("#myid").unwrap();
        assert_eq!(selector, Selector::Id("myid".to_string()));
    }

    #[test]
    fn test_parse_universal_selector() {
        let selector = parse_single_selector("*").unwrap();
        assert_eq!(selector, Selector::Universal);
    }

    #[test]
    fn test_parse_compound_selector_div_class() {
        let selector = parse_single_selector("div.myclass").unwrap();
        if let Selector::Compound {
            element,
            classes,
            id,
        } = selector
        {
            assert_eq!(element, Some("div".to_string()));
            assert_eq!(classes, vec!["myclass".to_string()]);
            assert_eq!(id, None);
        } else {
            panic!("Expected compound selector");
        }
    }

    #[test]
    fn test_parse_selector_list() {
        let selectors = parse_selector_list("div, .class, #id").unwrap();
        assert_eq!(selectors.len(), 3);
        assert_eq!(selectors[0], Selector::Element("div".to_string()));
        assert_eq!(selectors[1], Selector::Class("class".to_string()));
        assert_eq!(selectors[2], Selector::Id("id".to_string()));
    }

    #[test]
    fn test_empty_selector_error() {
        let result = parse_single_selector("");
        assert!(result.is_err());
    }
}
