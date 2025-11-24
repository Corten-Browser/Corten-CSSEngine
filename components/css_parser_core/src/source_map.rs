//! Source Map Support for CSS Debugging
//!
//! This module provides source map parsing and generation for CSS debugging,
//! implementing the Source Map v3 specification.
//!
//! # Example
//!
//! ```
//! use css_parser_core::source_map::{SourceMap, parse_source_map, generate_source_map};
//!
//! // Parse a source map from JSON
//! let json = r#"{"version":3,"file":"out.css","sources":["input.css"],"names":[],"mappings":"AAAA"}"#;
//! let source_map = parse_source_map(json).unwrap();
//!
//! assert_eq!(source_map.version(), 3);
//! assert_eq!(source_map.sources().len(), 1);
//! ```

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::Stylesheet;

/// Errors that can occur during source map operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum SourceMapError {
    /// Invalid JSON format
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    /// Unsupported source map version
    #[error("Unsupported source map version: {0}, expected 3")]
    UnsupportedVersion(u32),

    /// Invalid VLQ encoding
    #[error("Invalid VLQ encoding: {0}")]
    InvalidVlq(String),

    /// Invalid base64 encoding
    #[error("Invalid base64 encoding: {0}")]
    InvalidBase64(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid source map URL
    #[error("Invalid source map URL: {0}")]
    InvalidUrl(String),
}

/// A location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceLocation {
    /// Source file index (into SourceMap.sources)
    pub source_index: usize,
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based)
    pub column: usize,
    /// Name index (into SourceMap.names), if any
    pub name_index: Option<usize>,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(source_index: usize, line: usize, column: usize) -> Self {
        SourceLocation {
            source_index,
            line,
            column,
            name_index: None,
        }
    }

    /// Create a source location with a name reference
    pub fn with_name(source_index: usize, line: usize, column: usize, name_index: usize) -> Self {
        SourceLocation {
            source_index,
            line,
            column,
            name_index: Some(name_index),
        }
    }
}

/// A mapping from generated position to original position
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    /// Generated line (0-based)
    pub generated_line: usize,
    /// Generated column (0-based)
    pub generated_column: usize,
    /// Original source location (if available)
    pub original: Option<SourceLocation>,
}

/// Source Map v3 representation
///
/// Represents a CSS source map following the Source Map v3 specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceMap {
    /// Source map version (always 3)
    version: u32,
    /// The generated file this source map is for
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    /// Root URL for relative source paths
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceRoot")]
    source_root: Option<String>,
    /// List of original source files
    sources: Vec<String>,
    /// Content of original source files (if embedded)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourcesContent")]
    sources_content: Option<Vec<Option<String>>>,
    /// List of symbol names referenced in mappings
    names: Vec<String>,
    /// VLQ-encoded mappings string
    mappings: String,

    /// Parsed mappings (not serialized)
    #[serde(skip)]
    parsed_mappings: Vec<Mapping>,
}

impl SourceMap {
    /// Create a new empty source map
    pub fn new() -> Self {
        SourceMap {
            version: 3,
            file: None,
            source_root: None,
            sources: Vec::new(),
            sources_content: None,
            names: Vec::new(),
            mappings: String::new(),
            parsed_mappings: Vec::new(),
        }
    }

    /// Create a source map with a file name
    pub fn with_file(file: impl Into<String>) -> Self {
        let mut sm = SourceMap::new();
        sm.file = Some(file.into());
        sm
    }

    /// Get the source map version
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Get the generated file name
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Set the generated file name
    pub fn set_file(&mut self, file: impl Into<String>) {
        self.file = Some(file.into());
    }

    /// Get the source root
    pub fn source_root(&self) -> Option<&str> {
        self.source_root.as_deref()
    }

    /// Set the source root
    pub fn set_source_root(&mut self, root: impl Into<String>) {
        self.source_root = Some(root.into());
    }

    /// Get the list of source files
    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    /// Add a source file
    pub fn add_source(&mut self, source: impl Into<String>) -> usize {
        let idx = self.sources.len();
        self.sources.push(source.into());
        idx
    }

    /// Get source content for a given source index
    pub fn source_content(&self, index: usize) -> Option<&str> {
        self.sources_content
            .as_ref()
            .and_then(|contents| contents.get(index))
            .and_then(|opt| opt.as_deref())
    }

    /// Set source content for a given source index
    pub fn set_source_content(&mut self, index: usize, content: impl Into<String>) {
        if self.sources_content.is_none() {
            self.sources_content = Some(vec![None; self.sources.len()]);
        }

        if let Some(ref mut contents) = self.sources_content {
            while contents.len() <= index {
                contents.push(None);
            }
            contents[index] = Some(content.into());
        }
    }

    /// Get the list of names
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// Add a name
    pub fn add_name(&mut self, name: impl Into<String>) -> usize {
        let idx = self.names.len();
        self.names.push(name.into());
        idx
    }

    /// Get the raw mappings string
    pub fn mappings(&self) -> &str {
        &self.mappings
    }

    /// Get parsed mappings
    pub fn parsed_mappings(&self) -> &[Mapping] {
        &self.parsed_mappings
    }

    /// Add a mapping
    pub fn add_mapping(&mut self, mapping: Mapping) {
        self.parsed_mappings.push(mapping);
    }

    /// Look up the original location for a generated position
    pub fn lookup(&self, generated_line: usize, generated_column: usize) -> Option<&Mapping> {
        // Find the best matching mapping (last one that starts at or before the position)
        // Use rev().find() for efficiency on double-ended iterators
        self.parsed_mappings.iter().rev().find(|m| {
            m.generated_line < generated_line
                || (m.generated_line == generated_line && m.generated_column <= generated_column)
        })
    }

    /// Encode all mappings to VLQ format
    pub fn encode_mappings(&mut self) {
        self.mappings = encode_mappings(&self.parsed_mappings);
    }

    /// Decode VLQ mappings string
    pub fn decode_mappings(&mut self) -> Result<(), SourceMapError> {
        self.parsed_mappings = decode_mappings(&self.mappings)?;
        Ok(())
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, SourceMapError> {
        serde_json::to_string(self).map_err(|e| SourceMapError::InvalidJson(e.to_string()))
    }

    /// Convert to pretty-printed JSON string
    pub fn to_json_pretty(&self) -> Result<String, SourceMapError> {
        serde_json::to_string_pretty(self).map_err(|e| SourceMapError::InvalidJson(e.to_string()))
    }

    /// Generate an inline source map comment
    pub fn to_inline_comment(&self) -> Result<String, SourceMapError> {
        let json = self.to_json()?;
        let encoded = BASE64.encode(json.as_bytes());
        Ok(format!(
            "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,{} */",
            encoded
        ))
    }

    /// Generate an external source map reference comment
    pub fn to_external_comment(url: &str) -> String {
        format!("/*# sourceMappingURL={} */", url)
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a source map from JSON
pub fn parse_source_map(json: &str) -> Result<SourceMap, SourceMapError> {
    let mut source_map: SourceMap =
        serde_json::from_str(json).map_err(|e| SourceMapError::InvalidJson(e.to_string()))?;

    // Validate version
    if source_map.version != 3 {
        return Err(SourceMapError::UnsupportedVersion(source_map.version));
    }

    // Decode mappings
    source_map.decode_mappings()?;

    Ok(source_map)
}

/// Parse an inline source map from a CSS comment
///
/// Handles comments like: `/*# sourceMappingURL=data:application/json;charset=utf-8;base64,... */`
pub fn parse_inline_source_map(css: &str) -> Result<Option<SourceMap>, SourceMapError> {
    let prefix = "/*# sourceMappingURL=data:";

    if let Some(start) = css.find(prefix) {
        let start = start + prefix.len();

        // Find the end of the comment
        let end = css[start..]
            .find("*/")
            .ok_or_else(|| SourceMapError::InvalidUrl("Unterminated source map comment".into()))?;

        let data_url = &css[start..start + end].trim();

        // Parse data URL
        // Format: application/json;charset=utf-8;base64,<data>
        // or: application/json;base64,<data>
        let base64_marker = "base64,";
        if let Some(base64_start) = data_url.find(base64_marker) {
            let encoded = &data_url[base64_start + base64_marker.len()..].trim();
            let decoded = BASE64
                .decode(encoded)
                .map_err(|e| SourceMapError::InvalidBase64(e.to_string()))?;
            let json = String::from_utf8(decoded)
                .map_err(|e| SourceMapError::InvalidJson(e.to_string()))?;
            return parse_source_map(&json).map(Some);
        }
    }

    Ok(None)
}

/// Extract source map URL from CSS comment
///
/// Returns the URL if found, or None if no source map reference exists.
pub fn extract_source_map_url(css: &str) -> Option<String> {
    let prefix = "/*# sourceMappingURL=";

    if let Some(start) = css.find(prefix) {
        let start = start + prefix.len();
        let end = css[start..].find("*/").unwrap_or(css.len() - start);
        let url = css[start..start + end].trim();

        // Skip inline data URLs
        if !url.starts_with("data:") {
            return Some(url.to_string());
        }
    }

    None
}

/// Generate a source map for a stylesheet
///
/// This creates source map mappings for each declaration in the stylesheet,
/// tracking the original source positions.
pub fn generate_source_map(stylesheet: &Stylesheet, source_file: &str) -> SourceMap {
    let mut source_map = SourceMap::new();
    let source_index = source_map.add_source(source_file);

    // For now, we create simple line-by-line mappings
    // In a real implementation, we'd track positions during parsing
    let mut generated_line = 0;

    for rule in &stylesheet.rules {
        if let crate::CssRule::Style(style_rule) = rule {
            // Map the selector line
            source_map.add_mapping(Mapping {
                generated_line,
                generated_column: 0,
                original: Some(SourceLocation::new(source_index, generated_line, 0)),
            });

            generated_line += 1;

            // Map each declaration
            for (i, _decl) in style_rule.declarations.iter().enumerate() {
                source_map.add_mapping(Mapping {
                    generated_line,
                    generated_column: 2, // Indentation
                    original: Some(SourceLocation::new(source_index, generated_line, 2)),
                });

                // Add name if useful
                if i == 0 {
                    // Example: track first declaration name
                }

                generated_line += 1;
            }

            // Closing brace line
            generated_line += 1;
        }
    }

    // Encode the mappings
    source_map.encode_mappings();

    source_map
}

// ============================================================================
// VLQ (Variable Length Quantity) Encoding/Decoding
// ============================================================================

/// Base64 VLQ character set
const VLQ_BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Decode a single VLQ value from the input
fn decode_vlq_value(chars: &mut std::str::Chars) -> Result<i32, SourceMapError> {
    let mut result = 0i32;
    let mut shift = 0u32;

    loop {
        let ch = chars
            .next()
            .ok_or_else(|| SourceMapError::InvalidVlq("Unexpected end of VLQ".into()))?;

        let digit = VLQ_BASE64_CHARS
            .iter()
            .position(|&c| c as char == ch)
            .ok_or_else(|| SourceMapError::InvalidVlq(format!("Invalid VLQ character: {}", ch)))?
            as i32;

        let continuation = (digit & 0b100000) != 0;
        let value_bits = digit & 0b011111;

        result |= value_bits << shift;
        shift += 5;

        if !continuation {
            break;
        }

        if shift > 30 {
            return Err(SourceMapError::InvalidVlq("VLQ value too large".into()));
        }
    }

    // Convert from VLQ signed format
    // The least significant bit is the sign
    let is_negative = (result & 1) != 0;
    result >>= 1;

    if is_negative {
        result = -result;
    }

    Ok(result)
}

/// Encode a single value to VLQ format
fn encode_vlq_value(mut value: i32) -> String {
    let mut result = String::new();

    // Convert to VLQ signed format (LSB is sign)
    let negative = value < 0;
    if negative {
        value = -value;
    }
    value = (value << 1) | (if negative { 1 } else { 0 });

    loop {
        let mut digit = value & 0b011111;
        value >>= 5;

        if value > 0 {
            digit |= 0b100000; // Set continuation bit
        }

        result.push(VLQ_BASE64_CHARS[digit as usize] as char);

        if value == 0 {
            break;
        }
    }

    result
}

/// Decode VLQ mappings string into Mapping structs
fn decode_mappings(mappings: &str) -> Result<Vec<Mapping>, SourceMapError> {
    let mut result = Vec::new();

    if mappings.is_empty() {
        return Ok(result);
    }

    // State for relative values
    let mut prev_source = 0i32;
    let mut prev_source_line = 0i32;
    let mut prev_source_column = 0i32;
    let mut prev_name = 0i32;

    // Split by semicolons (each represents a generated line)
    for (generated_line, line_mappings) in mappings.split(';').enumerate() {
        if line_mappings.is_empty() {
            continue;
        }

        let mut generated_column = 0i32;

        // Split by commas (each represents a segment)
        for segment in line_mappings.split(',') {
            if segment.is_empty() {
                continue;
            }

            let mut chars = segment.chars();
            let mut values = Vec::new();

            // Decode all VLQ values in this segment
            while chars.clone().next().is_some() {
                values.push(decode_vlq_value(&mut chars)?);
            }

            if values.is_empty() {
                continue;
            }

            // First value is always the generated column (relative to previous in this line)
            generated_column += values[0];

            let original = if values.len() >= 4 {
                // Has source mapping
                prev_source += values[1];
                prev_source_line += values[2];
                prev_source_column += values[3];

                let name_index = if values.len() >= 5 {
                    prev_name += values[4];
                    Some(prev_name as usize)
                } else {
                    None
                };

                Some(SourceLocation {
                    source_index: prev_source as usize,
                    line: prev_source_line as usize,
                    column: prev_source_column as usize,
                    name_index,
                })
            } else {
                None
            };

            result.push(Mapping {
                generated_line,
                generated_column: generated_column as usize,
                original,
            });
        }
    }

    Ok(result)
}

/// Encode Mapping structs to VLQ mappings string
fn encode_mappings(mappings: &[Mapping]) -> String {
    if mappings.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    // Group mappings by generated line
    let mut by_line: HashMap<usize, Vec<&Mapping>> = HashMap::new();
    let mut max_line = 0;

    for mapping in mappings {
        by_line
            .entry(mapping.generated_line)
            .or_default()
            .push(mapping);
        max_line = max_line.max(mapping.generated_line);
    }

    // State for relative encoding
    let mut prev_source = 0i32;
    let mut prev_source_line = 0i32;
    let mut prev_source_column = 0i32;
    let mut prev_name = 0i32;

    for line in 0..=max_line {
        if line > 0 {
            result.push(';');
        }

        if let Some(line_mappings) = by_line.get(&line) {
            let mut prev_generated_column = 0i32;
            let mut first_in_line = true;

            // Sort by generated column
            let mut sorted: Vec<_> = line_mappings.iter().collect();
            sorted.sort_by_key(|m| m.generated_column);

            for mapping in sorted {
                if !first_in_line {
                    result.push(',');
                }
                first_in_line = false;

                // Generated column (relative within line)
                let gen_col = mapping.generated_column as i32;
                result.push_str(&encode_vlq_value(gen_col - prev_generated_column));
                prev_generated_column = gen_col;

                if let Some(ref orig) = mapping.original {
                    // Source index (relative)
                    let source = orig.source_index as i32;
                    result.push_str(&encode_vlq_value(source - prev_source));
                    prev_source = source;

                    // Source line (relative)
                    let source_line = orig.line as i32;
                    result.push_str(&encode_vlq_value(source_line - prev_source_line));
                    prev_source_line = source_line;

                    // Source column (relative)
                    let source_col = orig.column as i32;
                    result.push_str(&encode_vlq_value(source_col - prev_source_column));
                    prev_source_column = source_col;

                    // Name index (relative, optional)
                    if let Some(name_idx) = orig.name_index {
                        let name = name_idx as i32;
                        result.push_str(&encode_vlq_value(name - prev_name));
                        prev_name = name;
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // VLQ Encoding/Decoding Tests
    // ========================================================================

    #[test]
    fn test_vlq_encode_zero() {
        assert_eq!(encode_vlq_value(0), "A");
    }

    #[test]
    fn test_vlq_encode_positive() {
        assert_eq!(encode_vlq_value(1), "C");
        assert_eq!(encode_vlq_value(2), "E");
        assert_eq!(encode_vlq_value(15), "e");
    }

    #[test]
    fn test_vlq_encode_negative() {
        assert_eq!(encode_vlq_value(-1), "D");
        assert_eq!(encode_vlq_value(-2), "F");
    }

    #[test]
    fn test_vlq_encode_large() {
        // Large positive number
        let encoded = encode_vlq_value(100);
        let mut chars = encoded.chars();
        let decoded = decode_vlq_value(&mut chars).unwrap();
        assert_eq!(decoded, 100);
    }

    #[test]
    fn test_vlq_decode_zero() {
        let mut chars = "A".chars();
        assert_eq!(decode_vlq_value(&mut chars).unwrap(), 0);
    }

    #[test]
    fn test_vlq_decode_positive() {
        let mut chars = "C".chars();
        assert_eq!(decode_vlq_value(&mut chars).unwrap(), 1);
    }

    #[test]
    fn test_vlq_decode_negative() {
        let mut chars = "D".chars();
        assert_eq!(decode_vlq_value(&mut chars).unwrap(), -1);
    }

    #[test]
    fn test_vlq_roundtrip() {
        for value in [-1000, -100, -10, -1, 0, 1, 10, 100, 1000] {
            let encoded = encode_vlq_value(value);
            let mut chars = encoded.chars();
            let decoded = decode_vlq_value(&mut chars).unwrap();
            assert_eq!(decoded, value, "Roundtrip failed for {}", value);
        }
    }

    // ========================================================================
    // Source Map Parsing Tests
    // ========================================================================

    #[test]
    fn test_parse_minimal_source_map() {
        let json = r#"{"version":3,"sources":[],"names":[],"mappings":""}"#;
        let sm = parse_source_map(json).unwrap();

        assert_eq!(sm.version(), 3);
        assert!(sm.sources().is_empty());
        assert!(sm.names().is_empty());
        assert!(sm.mappings().is_empty());
    }

    #[test]
    fn test_parse_source_map_with_file() {
        let json = r#"{"version":3,"file":"output.css","sources":["input.css"],"names":[],"mappings":"AAAA"}"#;
        let sm = parse_source_map(json).unwrap();

        assert_eq!(sm.file(), Some("output.css"));
        assert_eq!(sm.sources(), &["input.css"]);
    }

    #[test]
    fn test_parse_source_map_with_source_root() {
        let json = r#"{"version":3,"sourceRoot":"/src/","sources":["style.css"],"names":[],"mappings":""}"#;
        let sm = parse_source_map(json).unwrap();

        assert_eq!(sm.source_root(), Some("/src/"));
    }

    #[test]
    fn test_parse_source_map_with_sources_content() {
        let json = r#"{"version":3,"sources":["a.css"],"sourcesContent":["body { color: red; }"],"names":[],"mappings":""}"#;
        let sm = parse_source_map(json).unwrap();

        assert_eq!(sm.source_content(0), Some("body { color: red; }"));
    }

    #[test]
    fn test_parse_source_map_invalid_version() {
        let json = r#"{"version":2,"sources":[],"names":[],"mappings":""}"#;
        let result = parse_source_map(json);

        assert!(matches!(result, Err(SourceMapError::UnsupportedVersion(2))));
    }

    #[test]
    fn test_parse_source_map_invalid_json() {
        let json = "not valid json";
        let result = parse_source_map(json);

        assert!(matches!(result, Err(SourceMapError::InvalidJson(_))));
    }

    // ========================================================================
    // Mappings Tests
    // ========================================================================

    #[test]
    fn test_decode_simple_mapping() {
        // AAAA = column 0, source 0, line 0, column 0
        let mappings = decode_mappings("AAAA").unwrap();

        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].generated_line, 0);
        assert_eq!(mappings[0].generated_column, 0);

        let orig = mappings[0].original.as_ref().unwrap();
        assert_eq!(orig.source_index, 0);
        assert_eq!(orig.line, 0);
        assert_eq!(orig.column, 0);
    }

    #[test]
    fn test_decode_multiple_lines() {
        let mappings = decode_mappings("AAAA;AACA").unwrap();

        assert_eq!(mappings.len(), 2);
        assert_eq!(mappings[0].generated_line, 0);
        assert_eq!(mappings[1].generated_line, 1);
    }

    #[test]
    fn test_decode_multiple_segments() {
        let mappings = decode_mappings("AAAA,EAAE").unwrap();

        assert_eq!(mappings.len(), 2);
        assert_eq!(mappings[0].generated_column, 0);
        assert_eq!(mappings[1].generated_column, 2);
    }

    #[test]
    fn test_encode_simple_mapping() {
        let mappings = vec![Mapping {
            generated_line: 0,
            generated_column: 0,
            original: Some(SourceLocation::new(0, 0, 0)),
        }];

        let encoded = encode_mappings(&mappings);
        assert_eq!(encoded, "AAAA");
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original_mappings = vec![
            Mapping {
                generated_line: 0,
                generated_column: 0,
                original: Some(SourceLocation::new(0, 0, 0)),
            },
            Mapping {
                generated_line: 0,
                generated_column: 4,
                original: Some(SourceLocation::new(0, 0, 4)),
            },
            Mapping {
                generated_line: 1,
                generated_column: 2,
                original: Some(SourceLocation::new(0, 1, 2)),
            },
        ];

        let encoded = encode_mappings(&original_mappings);
        let decoded = decode_mappings(&encoded).unwrap();

        assert_eq!(decoded.len(), original_mappings.len());
        for (orig, dec) in original_mappings.iter().zip(decoded.iter()) {
            assert_eq!(orig.generated_line, dec.generated_line);
            assert_eq!(orig.generated_column, dec.generated_column);
            assert_eq!(
                orig.original.as_ref().map(|o| o.source_index),
                dec.original.as_ref().map(|o| o.source_index)
            );
        }
    }

    // ========================================================================
    // Inline Source Map Tests
    // ========================================================================

    #[test]
    fn test_parse_inline_source_map() {
        let json = r#"{"version":3,"sources":["test.css"],"names":[],"mappings":"AAAA"}"#;
        let encoded = BASE64.encode(json.as_bytes());
        let css = format!(
            "body {{ color: red; }}\n/*# sourceMappingURL=data:application/json;base64,{} */",
            encoded
        );

        let result = parse_inline_source_map(&css).unwrap();
        assert!(result.is_some());

        let sm = result.unwrap();
        assert_eq!(sm.sources(), &["test.css"]);
    }

    #[test]
    fn test_parse_inline_source_map_with_charset() {
        let json = r#"{"version":3,"sources":[],"names":[],"mappings":""}"#;
        let encoded = BASE64.encode(json.as_bytes());
        let css = format!(
            "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,{} */",
            encoded
        );

        let result = parse_inline_source_map(&css).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_inline_source_map_none() {
        let css = "body { color: red; }";
        let result = parse_inline_source_map(css).unwrap();
        assert!(result.is_none());
    }

    // ========================================================================
    // External Source Map URL Tests
    // ========================================================================

    #[test]
    fn test_extract_external_url() {
        let css = "body { color: red; }\n/*# sourceMappingURL=style.css.map */";
        let url = extract_source_map_url(css);

        assert_eq!(url, Some("style.css.map".to_string()));
    }

    #[test]
    fn test_extract_external_url_with_path() {
        let css = "/*# sourceMappingURL=/maps/app.css.map */";
        let url = extract_source_map_url(css);

        assert_eq!(url, Some("/maps/app.css.map".to_string()));
    }

    #[test]
    fn test_extract_url_ignores_inline() {
        let css = "/*# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozfQ== */";
        let url = extract_source_map_url(css);

        assert!(url.is_none());
    }

    #[test]
    fn test_extract_url_none() {
        let css = "body { color: red; }";
        let url = extract_source_map_url(css);

        assert!(url.is_none());
    }

    // ========================================================================
    // Source Map Generation Tests
    // ========================================================================

    #[test]
    fn test_generate_source_map_empty_stylesheet() {
        use crate::{Origin, Stylesheet};

        let stylesheet = Stylesheet::new(Origin::Author);
        let sm = generate_source_map(&stylesheet, "test.css");

        assert_eq!(sm.sources(), &["test.css"]);
        assert!(sm.parsed_mappings().is_empty());
    }

    #[test]
    fn test_generate_source_map_with_rules() {
        use crate::{
            CssRule, Origin, PropertyDeclaration, PropertyValue, Selector, StyleRule, Stylesheet,
        };

        let mut stylesheet = Stylesheet::new(Origin::Author);
        stylesheet.rules.push(CssRule::Style(StyleRule {
            selectors: vec![Selector::Element("body".to_string())],
            declarations: vec![PropertyDeclaration {
                name: "color".to_string(),
                value: PropertyValue::Keyword("red".to_string()),
                important: false,
            }],
        }));

        let sm = generate_source_map(&stylesheet, "input.css");

        assert_eq!(sm.sources(), &["input.css"]);
        assert!(!sm.parsed_mappings().is_empty());
        assert!(!sm.mappings().is_empty());
    }

    // ========================================================================
    // Source Map Builder Tests
    // ========================================================================

    #[test]
    fn test_source_map_builder() {
        let mut sm = SourceMap::new();
        sm.set_file("output.css");
        sm.set_source_root("/src/");

        let idx = sm.add_source("input.css");
        sm.set_source_content(idx, "body { color: red; }");

        let name_idx = sm.add_name("color");

        sm.add_mapping(Mapping {
            generated_line: 0,
            generated_column: 0,
            original: Some(SourceLocation::with_name(idx, 0, 0, name_idx)),
        });

        sm.encode_mappings();

        assert_eq!(sm.file(), Some("output.css"));
        assert_eq!(sm.source_root(), Some("/src/"));
        assert_eq!(sm.sources(), &["input.css"]);
        assert_eq!(sm.source_content(0), Some("body { color: red; }"));
        assert_eq!(sm.names(), &["color"]);
        assert!(!sm.mappings().is_empty());
    }

    #[test]
    fn test_source_map_to_json() {
        let mut sm = SourceMap::new();
        sm.set_file("out.css");
        sm.add_source("in.css");

        let json = sm.to_json().unwrap();
        assert!(json.contains("\"version\":3"));
        assert!(json.contains("\"file\":\"out.css\""));
        assert!(json.contains("\"sources\":[\"in.css\"]"));
    }

    #[test]
    fn test_source_map_to_inline_comment() {
        let mut sm = SourceMap::new();
        sm.add_source("test.css");

        let comment = sm.to_inline_comment().unwrap();
        assert!(
            comment.starts_with("/*# sourceMappingURL=data:application/json;charset=utf-8;base64,")
        );
        assert!(comment.ends_with(" */"));
    }

    #[test]
    fn test_source_map_to_external_comment() {
        let comment = SourceMap::to_external_comment("style.css.map");
        assert_eq!(comment, "/*# sourceMappingURL=style.css.map */");
    }

    // ========================================================================
    // Lookup Tests
    // ========================================================================

    #[test]
    fn test_lookup_exact_position() {
        let mut sm = SourceMap::new();
        sm.add_source("test.css");
        sm.add_mapping(Mapping {
            generated_line: 0,
            generated_column: 0,
            original: Some(SourceLocation::new(0, 0, 0)),
        });
        sm.add_mapping(Mapping {
            generated_line: 0,
            generated_column: 10,
            original: Some(SourceLocation::new(0, 0, 10)),
        });

        let result = sm.lookup(0, 10);
        assert!(result.is_some());
        assert_eq!(result.unwrap().generated_column, 10);
    }

    #[test]
    fn test_lookup_between_mappings() {
        let mut sm = SourceMap::new();
        sm.add_source("test.css");
        sm.add_mapping(Mapping {
            generated_line: 0,
            generated_column: 0,
            original: Some(SourceLocation::new(0, 0, 0)),
        });
        sm.add_mapping(Mapping {
            generated_line: 0,
            generated_column: 10,
            original: Some(SourceLocation::new(0, 0, 10)),
        });

        // Looking up position 5 should return the mapping at 0
        let result = sm.lookup(0, 5);
        assert!(result.is_some());
        assert_eq!(result.unwrap().generated_column, 0);
    }

    #[test]
    fn test_lookup_no_mapping() {
        let sm = SourceMap::new();
        let result = sm.lookup(0, 0);
        assert!(result.is_none());
    }

    // ========================================================================
    // Error Handling Tests
    // ========================================================================

    #[test]
    fn test_source_map_error_display() {
        let err = SourceMapError::InvalidJson("test".to_string());
        assert_eq!(format!("{}", err), "Invalid JSON: test");

        let err = SourceMapError::UnsupportedVersion(2);
        assert_eq!(
            format!("{}", err),
            "Unsupported source map version: 2, expected 3"
        );

        let err = SourceMapError::InvalidVlq("bad".to_string());
        assert_eq!(format!("{}", err), "Invalid VLQ encoding: bad");
    }

    #[test]
    fn test_invalid_vlq_character() {
        let result = decode_mappings("@@@");
        assert!(matches!(result, Err(SourceMapError::InvalidVlq(_))));
    }

    // ========================================================================
    // Source Location Tests
    // ========================================================================

    #[test]
    fn test_source_location_new() {
        let loc = SourceLocation::new(1, 10, 5);
        assert_eq!(loc.source_index, 1);
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
        assert!(loc.name_index.is_none());
    }

    #[test]
    fn test_source_location_with_name() {
        let loc = SourceLocation::with_name(0, 5, 10, 3);
        assert_eq!(loc.source_index, 0);
        assert_eq!(loc.line, 5);
        assert_eq!(loc.column, 10);
        assert_eq!(loc.name_index, Some(3));
    }

    #[test]
    fn test_source_location_default() {
        let loc = SourceLocation::default();
        assert_eq!(loc.source_index, 0);
        assert_eq!(loc.line, 0);
        assert_eq!(loc.column, 0);
        assert!(loc.name_index.is_none());
    }
}
