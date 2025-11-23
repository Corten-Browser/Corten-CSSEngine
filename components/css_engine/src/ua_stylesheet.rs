//! User Agent Stylesheet
//!
//! This module provides the default browser stylesheet that defines the initial
//! styling for HTML elements. These styles are applied with the lowest priority
//! (user-agent origin) and can be overridden by author or user styles.
//!
//! # HTML5 Specification Compliance
//!
//! These default styles are based on the HTML5 specification's rendering
//! requirements and common browser implementations.
//!
//! # Example
//!
//! ```
//! use css_engine::ua_stylesheet::UserAgentStylesheet;
//!
//! let ua = UserAgentStylesheet::new();
//!
//! // Get the stylesheet CSS source
//! let css = ua.css();
//!
//! // Check if a tag has default display: block
//! assert!(ua.is_block_element("div"));
//! assert!(!ua.is_block_element("span"));
//! ```

use std::collections::HashSet;

/// Default CSS for block elements
const BLOCK_ELEMENTS_CSS: &str = r#"
/* Block-level elements */
html, body, div, section, article, aside, nav, header, footer, main,
h1, h2, h3, h4, h5, h6, p, blockquote, pre, address,
ul, ol, li, dl, dt, dd,
figure, figcaption, hgroup,
form, fieldset, legend,
table, caption, thead, tbody, tfoot, tr,
hr, noscript {
    display: block;
}
"#;

/// Default CSS for heading elements
const HEADINGS_CSS: &str = r#"
/* Heading styles */
h1 {
    font-size: 2em;
    font-weight: bold;
    margin-top: 0.67em;
    margin-bottom: 0.67em;
}

h2 {
    font-size: 1.5em;
    font-weight: bold;
    margin-top: 0.83em;
    margin-bottom: 0.83em;
}

h3 {
    font-size: 1.17em;
    font-weight: bold;
    margin-top: 1em;
    margin-bottom: 1em;
}

h4 {
    font-size: 1em;
    font-weight: bold;
    margin-top: 1.33em;
    margin-bottom: 1.33em;
}

h5 {
    font-size: 0.83em;
    font-weight: bold;
    margin-top: 1.67em;
    margin-bottom: 1.67em;
}

h6 {
    font-size: 0.67em;
    font-weight: bold;
    margin-top: 2.33em;
    margin-bottom: 2.33em;
}
"#;

/// Default CSS for lists
const LIST_CSS: &str = r#"
/* List styles */
ul, ol {
    margin-top: 1em;
    margin-bottom: 1em;
    padding-left: 40px;
}

ul {
    list-style-type: disc;
}

ol {
    list-style-type: decimal;
}

li {
    display: list-item;
}

dl {
    margin-top: 1em;
    margin-bottom: 1em;
}

dd {
    margin-left: 40px;
}

/* Nested lists */
ul ul, ol ul {
    list-style-type: circle;
    margin-top: 0;
    margin-bottom: 0;
}

ul ul ul, ol ul ul, ul ol ul, ol ol ul {
    list-style-type: square;
}
"#;

/// Default CSS for table elements
const TABLE_CSS: &str = r#"
/* Table styles */
table {
    display: table;
    border-collapse: separate;
    border-spacing: 2px;
    border-color: gray;
}

thead {
    display: table-header-group;
    vertical-align: middle;
}

tbody {
    display: table-row-group;
    vertical-align: middle;
}

tfoot {
    display: table-footer-group;
    vertical-align: middle;
}

tr {
    display: table-row;
    vertical-align: inherit;
}

th, td {
    display: table-cell;
    vertical-align: inherit;
    padding: 1px;
}

th {
    font-weight: bold;
    text-align: center;
}

caption {
    display: table-caption;
    text-align: center;
}

colgroup {
    display: table-column-group;
}

col {
    display: table-column;
}
"#;

/// Default CSS for form elements
const FORM_CSS: &str = r#"
/* Form element styles */
form {
    display: block;
    margin-top: 0;
}

fieldset {
    display: block;
    margin-left: 2px;
    margin-right: 2px;
    padding-top: 0.35em;
    padding-bottom: 0.625em;
    padding-left: 0.75em;
    padding-right: 0.75em;
    border: 2px groove;
}

legend {
    display: block;
    padding-left: 2px;
    padding-right: 2px;
}

label {
    cursor: default;
}

input {
    display: inline-block;
}

input[type="text"],
input[type="password"],
input[type="email"],
input[type="search"],
input[type="url"],
input[type="tel"],
input[type="number"] {
    padding: 1px;
}

input[type="button"],
input[type="submit"],
input[type="reset"] {
    display: inline-block;
    text-align: center;
    cursor: default;
}

input[type="checkbox"],
input[type="radio"] {
    margin: 3px 0.5ex;
    padding: 0;
}

input[type="hidden"] {
    display: none;
}

button {
    display: inline-block;
    text-align: center;
    cursor: default;
}

select {
    display: inline-block;
}

textarea {
    display: inline-block;
    font-family: monospace;
    white-space: pre-wrap;
}

optgroup {
    font-weight: bold;
}

option {
    font-weight: normal;
}
"#;

/// Default CSS for text content elements
const TEXT_CSS: &str = r#"
/* Text content styles */
p {
    margin-top: 1em;
    margin-bottom: 1em;
}

blockquote {
    margin-top: 1em;
    margin-bottom: 1em;
    margin-left: 40px;
    margin-right: 40px;
}

pre {
    display: block;
    font-family: monospace;
    white-space: pre;
    margin-top: 1em;
    margin-bottom: 1em;
}

code, kbd, samp {
    font-family: monospace;
}

address {
    font-style: italic;
}

hr {
    display: block;
    margin-top: 0.5em;
    margin-bottom: 0.5em;
    margin-left: auto;
    margin-right: auto;
    border-style: inset;
    border-width: 1px;
}
"#;

/// Default CSS for inline text semantics
const INLINE_CSS: &str = r#"
/* Inline element styles */
a {
    color: blue;
    text-decoration: underline;
    cursor: pointer;
}

a:visited {
    color: purple;
}

a:active {
    color: red;
}

em, i, cite, var, dfn {
    font-style: italic;
}

strong, b {
    font-weight: bold;
}

u, ins {
    text-decoration: underline;
}

s, strike, del {
    text-decoration: line-through;
}

small {
    font-size: smaller;
}

big {
    font-size: larger;
}

sub {
    vertical-align: sub;
    font-size: smaller;
}

sup {
    vertical-align: super;
    font-size: smaller;
}

mark {
    background-color: yellow;
    color: black;
}

abbr[title], acronym[title] {
    text-decoration: dotted underline;
    cursor: help;
}

q::before {
    content: open-quote;
}

q::after {
    content: close-quote;
}
"#;

/// Default CSS for body and html
const ROOT_CSS: &str = r#"
/* Root element styles */
html {
    display: block;
}

body {
    display: block;
    margin: 8px;
}

/* Default font settings */
body {
    font-family: serif;
    font-size: medium;
    line-height: normal;
    color: black;
    background-color: white;
}
"#;

/// Default CSS for images and media
const MEDIA_CSS: &str = r#"
/* Image and media styles */
img {
    display: inline-block;
}

img[src] {
    display: inline;
}

svg {
    display: inline;
}

video, audio {
    display: inline;
}

canvas {
    display: inline;
}

iframe {
    display: inline;
    border: 2px inset;
}

object, embed {
    display: inline;
}
"#;

/// Default CSS for scripting and other elements
const MISC_CSS: &str = r#"
/* Hidden elements */
head, title, meta, link, style, script, noscript[hidden],
template, [hidden] {
    display: none;
}

/* Ruby annotation */
ruby {
    display: ruby;
}

rt {
    display: ruby-text;
    font-size: 50%;
}

rp {
    display: none;
}

/* Details/Summary */
details {
    display: block;
}

summary {
    display: block;
    cursor: pointer;
}

/* Dialog */
dialog {
    display: block;
    position: absolute;
    left: 0;
    right: 0;
    width: fit-content;
    height: fit-content;
    margin: auto;
    border: solid;
    padding: 1em;
    background: white;
    color: black;
}

dialog:not([open]) {
    display: none;
}

/* Progress and Meter */
progress, meter {
    display: inline-block;
    vertical-align: -0.2em;
}
"#;

/// User agent stylesheet containing all default browser styles
#[derive(Debug, Clone)]
pub struct UserAgentStylesheet {
    /// Complete CSS content
    css: String,
    /// Set of block-level element names
    block_elements: HashSet<String>,
    /// Set of inline element names
    inline_elements: HashSet<String>,
    /// Set of hidden element names
    hidden_elements: HashSet<String>,
}

impl UserAgentStylesheet {
    /// Create a new user agent stylesheet with all default styles
    pub fn new() -> Self {
        let css = Self::build_css();
        let block_elements = Self::build_block_elements();
        let inline_elements = Self::build_inline_elements();
        let hidden_elements = Self::build_hidden_elements();

        UserAgentStylesheet {
            css,
            block_elements,
            inline_elements,
            hidden_elements,
        }
    }

    /// Build the complete CSS string
    fn build_css() -> String {
        let mut css = String::with_capacity(8192);
        css.push_str("/* User Agent Stylesheet */\n");
        css.push_str("/* Based on HTML5 rendering specification */\n\n");
        css.push_str(ROOT_CSS);
        css.push_str(BLOCK_ELEMENTS_CSS);
        css.push_str(HEADINGS_CSS);
        css.push_str(TEXT_CSS);
        css.push_str(INLINE_CSS);
        css.push_str(LIST_CSS);
        css.push_str(TABLE_CSS);
        css.push_str(FORM_CSS);
        css.push_str(MEDIA_CSS);
        css.push_str(MISC_CSS);
        css
    }

    /// Build the set of block-level elements
    fn build_block_elements() -> HashSet<String> {
        [
            "html", "body", "div", "section", "article", "aside", "nav",
            "header", "footer", "main", "h1", "h2", "h3", "h4", "h5", "h6",
            "p", "blockquote", "pre", "address", "ul", "ol", "li", "dl",
            "dt", "dd", "figure", "figcaption", "hgroup", "form", "fieldset",
            "legend", "table", "caption", "thead", "tbody", "tfoot", "tr",
            "hr", "noscript", "details", "summary", "dialog",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Build the set of inline elements
    fn build_inline_elements() -> HashSet<String> {
        [
            "span", "a", "em", "strong", "i", "b", "u", "s", "small", "big",
            "sub", "sup", "mark", "abbr", "acronym", "q", "cite", "code",
            "kbd", "samp", "var", "dfn", "del", "ins", "img", "br", "wbr",
            "svg", "video", "audio", "canvas", "iframe", "object", "embed",
            "input", "button", "select", "textarea", "label", "output",
            "progress", "meter", "time", "data", "ruby", "rt", "rp",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Build the set of hidden elements
    fn build_hidden_elements() -> HashSet<String> {
        [
            "head", "title", "meta", "link", "style", "script", "template",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Get the complete CSS source
    pub fn css(&self) -> &str {
        &self.css
    }

    /// Get the CSS length in bytes
    pub fn css_len(&self) -> usize {
        self.css.len()
    }

    /// Check if an element is a block-level element by default
    pub fn is_block_element(&self, tag_name: &str) -> bool {
        self.block_elements.contains(&tag_name.to_lowercase())
    }

    /// Check if an element is an inline element by default
    pub fn is_inline_element(&self, tag_name: &str) -> bool {
        self.inline_elements.contains(&tag_name.to_lowercase())
    }

    /// Check if an element is hidden by default
    pub fn is_hidden_element(&self, tag_name: &str) -> bool {
        self.hidden_elements.contains(&tag_name.to_lowercase())
    }

    /// Get the default display value for an element
    pub fn default_display(&self, tag_name: &str) -> &'static str {
        let tag = tag_name.to_lowercase();

        if self.hidden_elements.contains(&tag) {
            return "none";
        }

        // Check for special display types first (before block check)
        match tag.as_str() {
            "table" => "table",
            "tr" => "table-row",
            "th" | "td" => "table-cell",
            "thead" => "table-header-group",
            "tbody" => "table-row-group",
            "tfoot" => "table-footer-group",
            "caption" => "table-caption",
            "colgroup" => "table-column-group",
            "col" => "table-column",
            "li" => "list-item",
            "ruby" => "ruby",
            "rt" => "ruby-text",
            _ => {
                if self.block_elements.contains(&tag) {
                    "block"
                } else {
                    "inline"
                }
            }
        }
    }

    /// Get the default font-size for headings (in em units)
    pub fn heading_font_size(&self, level: u8) -> f32 {
        match level {
            1 => 2.0,
            2 => 1.5,
            3 => 1.17,
            4 => 1.0,
            5 => 0.83,
            6 => 0.67,
            _ => 1.0,
        }
    }

    /// Get all block-level element names
    pub fn block_element_names(&self) -> impl Iterator<Item = &str> {
        self.block_elements.iter().map(|s| s.as_str())
    }

    /// Get all inline element names
    pub fn inline_element_names(&self) -> impl Iterator<Item = &str> {
        self.inline_elements.iter().map(|s| s.as_str())
    }

    /// Get all hidden element names
    pub fn hidden_element_names(&self) -> impl Iterator<Item = &str> {
        self.hidden_elements.iter().map(|s| s.as_str())
    }
}

impl Default for UserAgentStylesheet {
    fn default() -> Self {
        Self::new()
    }
}

/// Minimal user agent stylesheet for testing
pub struct MinimalUserAgentStylesheet;

impl MinimalUserAgentStylesheet {
    /// Get a minimal CSS reset for testing
    pub fn css() -> &'static str {
        r#"
/* Minimal UA stylesheet for testing */
* { margin: 0; padding: 0; }
html, body { display: block; }
h1, h2, h3, h4, h5, h6, p, div { display: block; }
span, a, strong, em { display: inline; }
head, script, style { display: none; }
"#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ua_stylesheet_creation() {
        let ua = UserAgentStylesheet::new();
        assert!(!ua.css().is_empty());
        assert!(ua.css_len() > 1000); // Should have substantial content
    }

    #[test]
    fn test_ua_stylesheet_default() {
        let ua = UserAgentStylesheet::default();
        assert_eq!(ua.css(), UserAgentStylesheet::new().css());
    }

    #[test]
    fn test_block_elements() {
        let ua = UserAgentStylesheet::new();

        // Common block elements
        assert!(ua.is_block_element("div"));
        assert!(ua.is_block_element("p"));
        assert!(ua.is_block_element("section"));
        assert!(ua.is_block_element("article"));
        assert!(ua.is_block_element("header"));
        assert!(ua.is_block_element("footer"));
        assert!(ua.is_block_element("main"));

        // Headings
        assert!(ua.is_block_element("h1"));
        assert!(ua.is_block_element("h2"));
        assert!(ua.is_block_element("h3"));
        assert!(ua.is_block_element("h4"));
        assert!(ua.is_block_element("h5"));
        assert!(ua.is_block_element("h6"));

        // Lists
        assert!(ua.is_block_element("ul"));
        assert!(ua.is_block_element("ol"));
        assert!(ua.is_block_element("li"));

        // Forms
        assert!(ua.is_block_element("form"));
        assert!(ua.is_block_element("fieldset"));
    }

    #[test]
    fn test_inline_elements() {
        let ua = UserAgentStylesheet::new();

        assert!(ua.is_inline_element("span"));
        assert!(ua.is_inline_element("a"));
        assert!(ua.is_inline_element("strong"));
        assert!(ua.is_inline_element("em"));
        assert!(ua.is_inline_element("code"));
        assert!(ua.is_inline_element("img"));
        assert!(ua.is_inline_element("br"));
    }

    #[test]
    fn test_hidden_elements() {
        let ua = UserAgentStylesheet::new();

        assert!(ua.is_hidden_element("head"));
        assert!(ua.is_hidden_element("title"));
        assert!(ua.is_hidden_element("meta"));
        assert!(ua.is_hidden_element("script"));
        assert!(ua.is_hidden_element("style"));
        assert!(ua.is_hidden_element("link"));
        assert!(ua.is_hidden_element("template"));
    }

    #[test]
    fn test_case_insensitivity() {
        let ua = UserAgentStylesheet::new();

        assert!(ua.is_block_element("DIV"));
        assert!(ua.is_block_element("Div"));
        assert!(ua.is_inline_element("SPAN"));
        assert!(ua.is_inline_element("Span"));
        assert!(ua.is_hidden_element("HEAD"));
    }

    #[test]
    fn test_default_display() {
        let ua = UserAgentStylesheet::new();

        // Block elements
        assert_eq!(ua.default_display("div"), "block");
        assert_eq!(ua.default_display("p"), "block");
        assert_eq!(ua.default_display("h1"), "block");

        // Table elements
        assert_eq!(ua.default_display("table"), "table");
        assert_eq!(ua.default_display("tr"), "table-row");
        assert_eq!(ua.default_display("td"), "table-cell");
        assert_eq!(ua.default_display("th"), "table-cell");
        assert_eq!(ua.default_display("thead"), "table-header-group");
        assert_eq!(ua.default_display("tbody"), "table-row-group");
        assert_eq!(ua.default_display("tfoot"), "table-footer-group");

        // List items
        assert_eq!(ua.default_display("li"), "list-item");

        // Hidden elements
        assert_eq!(ua.default_display("head"), "none");
        assert_eq!(ua.default_display("script"), "none");

        // Inline (default)
        assert_eq!(ua.default_display("span"), "inline");
        assert_eq!(ua.default_display("unknown"), "inline");
    }

    #[test]
    fn test_heading_font_sizes() {
        let ua = UserAgentStylesheet::new();

        assert_eq!(ua.heading_font_size(1), 2.0);
        assert_eq!(ua.heading_font_size(2), 1.5);
        assert_eq!(ua.heading_font_size(3), 1.17);
        assert_eq!(ua.heading_font_size(4), 1.0);
        assert_eq!(ua.heading_font_size(5), 0.83);
        assert_eq!(ua.heading_font_size(6), 0.67);

        // Invalid levels return 1.0
        assert_eq!(ua.heading_font_size(0), 1.0);
        assert_eq!(ua.heading_font_size(7), 1.0);
    }

    #[test]
    fn test_css_contains_essential_rules() {
        let ua = UserAgentStylesheet::new();
        let css = ua.css();

        // Check for essential elements
        assert!(css.contains("body"));
        assert!(css.contains("html"));
        assert!(css.contains("div"));
        assert!(css.contains("h1"));
        assert!(css.contains("p"));
        assert!(css.contains("a"));
        assert!(css.contains("ul"));
        assert!(css.contains("ol"));
        assert!(css.contains("li"));
        assert!(css.contains("table"));
        assert!(css.contains("form"));

        // Check for essential properties
        assert!(css.contains("display: block"));
        assert!(css.contains("font-size"));
        assert!(css.contains("font-weight"));
        assert!(css.contains("margin"));
        assert!(css.contains("text-decoration"));
    }

    #[test]
    fn test_element_iterators() {
        let ua = UserAgentStylesheet::new();

        // Block elements should contain div
        let block_names: Vec<_> = ua.block_element_names().collect();
        assert!(block_names.contains(&"div"));
        assert!(block_names.contains(&"p"));

        // Inline elements should contain span
        let inline_names: Vec<_> = ua.inline_element_names().collect();
        assert!(inline_names.contains(&"span"));
        assert!(inline_names.contains(&"a"));

        // Hidden elements should contain head
        let hidden_names: Vec<_> = ua.hidden_element_names().collect();
        assert!(hidden_names.contains(&"head"));
        assert!(hidden_names.contains(&"script"));
    }

    #[test]
    fn test_minimal_stylesheet() {
        let css = MinimalUserAgentStylesheet::css();
        assert!(css.contains("html"));
        assert!(css.contains("body"));
        assert!(css.contains("display: block"));
        assert!(css.contains("display: none"));
    }

    #[test]
    fn test_clone() {
        let ua1 = UserAgentStylesheet::new();
        let ua2 = ua1.clone();
        assert_eq!(ua1.css(), ua2.css());
    }
}
