//! String Interning for CSS Property Names and Values
//!
//! String interning is an optimization technique where identical strings share
//! the same memory allocation. Instead of storing multiple copies of "color",
//! we store it once and refer to it by a small integer ID.
//!
//! # Benefits
//!
//! - **Memory reduction**: Common strings like "color", "px", "inherit" are stored once
//! - **Fast comparison**: Compare integers instead of string contents
//! - **Cache-friendly**: Small IDs fit in registers/cache lines
//! - **Hash-free lookup**: Once interned, no hashing needed for comparison
//!
//! # Use Cases
//!
//! - CSS property names ("color", "margin", "display")
//! - Common property values ("inherit", "initial", "auto", "none")
//! - Units ("px", "em", "rem", "%")
//! - Tag names ("div", "span", "p")
//! - Class names (often repeated across elements)
//!
//! # Thread Safety
//!
//! This implementation is NOT thread-safe. For concurrent access, wrap in
//! appropriate synchronization primitives or use per-thread interners.

use std::collections::HashMap;

/// An interned string identifier
///
/// This is a small, copyable type that represents an interned string.
/// Two InternedStrings are equal if and only if they refer to the same
/// original string.
///
/// # Examples
///
/// ```
/// use css_stylist_core::interning::StringInterner;
///
/// let mut interner = StringInterner::new();
/// let id1 = interner.intern("hello");
/// let id2 = interner.intern("hello");
/// let id3 = interner.intern("world");
///
/// assert_eq!(id1, id2);  // Same string = same ID
/// assert_ne!(id1, id3);  // Different string = different ID
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InternedString(u32);

impl InternedString {
    /// Create an interned string from a raw ID (for advanced use)
    ///
    /// # Safety
    ///
    /// The ID must be valid for the interner it will be used with.
    pub fn from_raw(id: u32) -> Self {
        Self(id)
    }

    /// Get the raw ID value
    pub fn raw(self) -> u32 {
        self.0
    }

    /// Check if this is the empty string (ID 0)
    pub fn is_empty_string(self) -> bool {
        self.0 == 0
    }
}

impl Default for InternedString {
    /// Default to the empty string (ID 0)
    fn default() -> Self {
        Self(0)
    }
}

/// String interner that maps strings to unique IDs
///
/// The interner owns all interned strings and provides bidirectional
/// lookup: string -> ID and ID -> string.
///
/// # Examples
///
/// ```
/// use css_stylist_core::interning::StringInterner;
///
/// let mut interner = StringInterner::new();
///
/// // Intern some strings
/// let color_id = interner.intern("color");
/// let margin_id = interner.intern("margin");
///
/// // Look up strings by ID
/// assert_eq!(interner.get(color_id), "color");
/// assert_eq!(interner.get(margin_id), "margin");
///
/// // Check if a string is already interned
/// assert!(interner.contains("color"));
/// assert!(!interner.contains("padding"));
/// ```
#[derive(Debug, Clone)]
pub struct StringInterner {
    /// Storage for interned strings
    strings: Vec<String>,
    /// Lookup table: string -> ID
    lookup: HashMap<String, InternedString>,
}

impl StringInterner {
    /// Create a new empty string interner
    ///
    /// The interner is pre-populated with the empty string at ID 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::interning::StringInterner;
    ///
    /// let interner = StringInterner::new();
    /// assert_eq!(interner.len(), 1); // Contains empty string
    /// ```
    pub fn new() -> Self {
        let mut interner = Self {
            strings: Vec::new(),
            lookup: HashMap::new(),
        };
        // Pre-intern the empty string at index 0
        interner.intern("");
        interner
    }

    /// Create an interner with pre-interned common CSS strings
    ///
    /// This is useful for CSS engines where certain strings are extremely common.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::interning::StringInterner;
    ///
    /// let interner = StringInterner::with_css_keywords();
    /// assert!(interner.contains("inherit"));
    /// assert!(interner.contains("auto"));
    /// ```
    pub fn with_css_keywords() -> Self {
        let mut interner = Self::new();

        // Common property names
        let properties = [
            "color",
            "background",
            "background-color",
            "margin",
            "margin-top",
            "margin-right",
            "margin-bottom",
            "margin-left",
            "padding",
            "padding-top",
            "padding-right",
            "padding-bottom",
            "padding-left",
            "border",
            "border-width",
            "border-style",
            "border-color",
            "width",
            "height",
            "min-width",
            "min-height",
            "max-width",
            "max-height",
            "display",
            "position",
            "top",
            "right",
            "bottom",
            "left",
            "font",
            "font-family",
            "font-size",
            "font-weight",
            "font-style",
            "line-height",
            "text-align",
            "text-decoration",
            "vertical-align",
            "overflow",
            "visibility",
            "z-index",
            "flex",
            "flex-direction",
            "flex-wrap",
            "justify-content",
            "align-items",
            "align-content",
            "grid",
            "grid-template",
            "gap",
        ];

        // Common values
        let values = [
            "inherit",
            "initial",
            "unset",
            "revert",
            "auto",
            "none",
            "hidden",
            "visible",
            "block",
            "inline",
            "inline-block",
            "flex",
            "grid",
            "static",
            "relative",
            "absolute",
            "fixed",
            "sticky",
            "normal",
            "bold",
            "italic",
            "underline",
            "center",
            "left",
            "right",
            "top",
            "bottom",
            "solid",
            "dashed",
            "dotted",
            "transparent",
            "currentColor",
            "0",
            "1",
            "100%",
        ];

        // Common units
        let units = [
            "px", "em", "rem", "%", "vw", "vh", "vmin", "vmax", "pt", "cm", "mm", "in",
        ];

        // Common tag names
        let tags = [
            "div", "span", "p", "a", "ul", "ol", "li", "h1", "h2", "h3", "h4", "h5", "h6",
            "header", "footer", "nav", "main", "section", "article", "aside", "form", "input",
            "button", "table", "tr", "td", "th", "img", "video", "audio", "canvas", "svg",
        ];

        for s in properties
            .iter()
            .chain(values.iter())
            .chain(units.iter())
            .chain(tags.iter())
        {
            interner.intern(s);
        }

        interner
    }

    /// Intern a string and return its ID
    ///
    /// If the string is already interned, returns the existing ID.
    /// Otherwise, stores the string and returns a new ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// let id1 = interner.intern("hello");
    /// let id2 = interner.intern("hello");
    /// assert_eq!(id1, id2);
    /// ```
    pub fn intern(&mut self, s: &str) -> InternedString {
        if let Some(&id) = self.lookup.get(s) {
            return id;
        }

        let id = InternedString(self.strings.len() as u32);
        self.strings.push(s.to_string());
        self.lookup.insert(s.to_string(), id);
        id
    }

    /// Intern a string, taking ownership
    ///
    /// This can be more efficient when you already have an owned String.
    pub fn intern_owned(&mut self, s: String) -> InternedString {
        if let Some(&id) = self.lookup.get(&s) {
            return id;
        }

        let id = InternedString(self.strings.len() as u32);
        self.lookup.insert(s.clone(), id);
        self.strings.push(s);
        id
    }

    /// Get the string for an interned ID
    ///
    /// # Panics
    ///
    /// Panics if the ID is not valid for this interner.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// let id = interner.intern("hello");
    /// assert_eq!(interner.get(id), "hello");
    /// ```
    pub fn get(&self, id: InternedString) -> &str {
        &self.strings[id.0 as usize]
    }

    /// Try to get the string for an interned ID
    ///
    /// Returns None if the ID is not valid.
    pub fn try_get(&self, id: InternedString) -> Option<&str> {
        self.strings.get(id.0 as usize).map(|s| s.as_str())
    }

    /// Check if a string is already interned
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// interner.intern("hello");
    ///
    /// assert!(interner.contains("hello"));
    /// assert!(!interner.contains("world"));
    /// ```
    pub fn contains(&self, s: &str) -> bool {
        self.lookup.contains_key(s)
    }

    /// Get the ID for a string if it's already interned
    ///
    /// Returns None if the string is not interned.
    pub fn get_id(&self, s: &str) -> Option<InternedString> {
        self.lookup.get(s).copied()
    }

    /// Get the number of interned strings
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Check if the interner is empty (only has empty string)
    pub fn is_empty(&self) -> bool {
        self.strings.len() <= 1
    }

    /// Iterate over all interned strings
    pub fn iter(&self) -> impl Iterator<Item = (InternedString, &str)> {
        self.strings
            .iter()
            .enumerate()
            .map(|(i, s)| (InternedString(i as u32), s.as_str()))
    }

    /// Get memory usage statistics
    pub fn memory_stats(&self) -> InternerStats {
        let string_bytes: usize = self
            .strings
            .iter()
            .map(|s| s.len() + std::mem::size_of::<String>())
            .sum();
        let lookup_bytes = self.lookup.capacity()
            * (std::mem::size_of::<String>() + std::mem::size_of::<InternedString>());

        InternerStats {
            string_count: self.strings.len(),
            total_string_bytes: string_bytes,
            lookup_overhead_bytes: lookup_bytes,
            total_bytes: string_bytes + lookup_bytes,
        }
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about interner memory usage
#[derive(Debug, Clone)]
pub struct InternerStats {
    /// Number of unique strings
    pub string_count: usize,
    /// Bytes used for string storage
    pub total_string_bytes: usize,
    /// Bytes used for lookup table
    pub lookup_overhead_bytes: usize,
    /// Total memory usage
    pub total_bytes: usize,
}

/// A specialized interner for CSS property names
///
/// This is a thin wrapper around StringInterner with type-safe IDs
/// specifically for property names.
#[derive(Debug, Clone)]
pub struct PropertyNameInterner {
    inner: StringInterner,
}

/// Type-safe ID for an interned property name
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternedPropertyName(InternedString);

impl PropertyNameInterner {
    /// Create a new property name interner
    pub fn new() -> Self {
        Self {
            inner: StringInterner::with_css_keywords(),
        }
    }

    /// Intern a property name
    pub fn intern(&mut self, name: &str) -> InternedPropertyName {
        InternedPropertyName(self.inner.intern(name))
    }

    /// Get the property name string
    pub fn get(&self, id: InternedPropertyName) -> &str {
        self.inner.get(id.0)
    }

    /// Check if a property name is interned
    pub fn contains(&self, name: &str) -> bool {
        self.inner.contains(name)
    }
}

impl Default for PropertyNameInterner {
    fn default() -> Self {
        Self::new()
    }
}

/// A specialized interner for CSS class names
///
/// Class names are often repeated across many elements, making
/// interning particularly beneficial.
#[derive(Debug, Clone)]
pub struct ClassNameInterner {
    inner: StringInterner,
}

/// Type-safe ID for an interned class name
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternedClassName(InternedString);

impl ClassNameInterner {
    /// Create a new class name interner
    pub fn new() -> Self {
        Self {
            inner: StringInterner::new(),
        }
    }

    /// Intern a class name
    pub fn intern(&mut self, name: &str) -> InternedClassName {
        InternedClassName(self.inner.intern(name))
    }

    /// Get the class name string
    pub fn get(&self, id: InternedClassName) -> &str {
        self.inner.get(id.0)
    }

    /// Check if a class name is interned
    pub fn contains(&self, name: &str) -> bool {
        self.inner.contains(name)
    }

    /// Get the number of interned class names
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Default for ClassNameInterner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interned_string_equality() {
        let id1 = InternedString(1);
        let id2 = InternedString(1);
        let id3 = InternedString(2);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_interned_string_default() {
        let id = InternedString::default();
        assert!(id.is_empty_string());
        assert_eq!(id.raw(), 0);
    }

    #[test]
    fn test_interned_string_from_raw() {
        let id = InternedString::from_raw(42);
        assert_eq!(id.raw(), 42);
    }

    #[test]
    fn test_interner_new() {
        let interner = StringInterner::new();
        assert_eq!(interner.len(), 1); // Contains empty string
        assert!(interner.contains(""));
    }

    #[test]
    fn test_intern_and_get() {
        let mut interner = StringInterner::new();
        let id = interner.intern("hello");

        assert_eq!(interner.get(id), "hello");
    }

    #[test]
    fn test_intern_returns_same_id() {
        let mut interner = StringInterner::new();
        let id1 = interner.intern("hello");
        let id2 = interner.intern("hello");

        assert_eq!(id1, id2);
        assert_eq!(interner.len(), 2); // Empty + "hello"
    }

    #[test]
    fn test_intern_different_strings() {
        let mut interner = StringInterner::new();
        let id1 = interner.intern("hello");
        let id2 = interner.intern("world");

        assert_ne!(id1, id2);
        assert_eq!(interner.get(id1), "hello");
        assert_eq!(interner.get(id2), "world");
    }

    #[test]
    fn test_intern_owned() {
        let mut interner = StringInterner::new();
        let id1 = interner.intern_owned("hello".to_string());
        let id2 = interner.intern("hello");

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_contains() {
        let mut interner = StringInterner::new();
        interner.intern("hello");

        assert!(interner.contains("hello"));
        assert!(!interner.contains("world"));
    }

    #[test]
    fn test_get_id() {
        let mut interner = StringInterner::new();
        let id = interner.intern("hello");

        assert_eq!(interner.get_id("hello"), Some(id));
        assert_eq!(interner.get_id("world"), None);
    }

    #[test]
    fn test_try_get() {
        let mut interner = StringInterner::new();
        let id = interner.intern("hello");

        assert_eq!(interner.try_get(id), Some("hello"));
        assert_eq!(interner.try_get(InternedString(999)), None);
    }

    #[test]
    fn test_iter() {
        let mut interner = StringInterner::new();
        interner.intern("hello");
        interner.intern("world");

        let pairs: Vec<_> = interner.iter().collect();
        assert_eq!(pairs.len(), 3); // Empty + hello + world
        assert_eq!(pairs[0].1, "");
        assert_eq!(pairs[1].1, "hello");
        assert_eq!(pairs[2].1, "world");
    }

    #[test]
    fn test_with_css_keywords() {
        let interner = StringInterner::with_css_keywords();

        assert!(interner.contains("color"));
        assert!(interner.contains("inherit"));
        assert!(interner.contains("px"));
        assert!(interner.contains("div"));
    }

    #[test]
    fn test_memory_stats() {
        let mut interner = StringInterner::new();
        interner.intern("hello");
        interner.intern("world");

        let stats = interner.memory_stats();
        assert_eq!(stats.string_count, 3);
        assert!(stats.total_bytes > 0);
    }

    #[test]
    fn test_property_name_interner() {
        let mut interner = PropertyNameInterner::new();

        // Pre-populated with CSS keywords
        assert!(interner.contains("color"));

        // Can add new ones
        let id = interner.intern("custom-property");
        assert_eq!(interner.get(id), "custom-property");
    }

    #[test]
    fn test_class_name_interner() {
        let mut interner = ClassNameInterner::new();

        let id1 = interner.intern("container");
        let id2 = interner.intern("primary");
        let id3 = interner.intern("container");

        assert_ne!(id1, id2);
        assert_eq!(id1.0, id3.0);

        assert_eq!(interner.get(id1), "container");
        assert_eq!(interner.get(id2), "primary");
    }

    #[test]
    fn test_class_name_interner_len() {
        let mut interner = ClassNameInterner::new();
        assert!(interner.is_empty()); // Only empty string

        interner.intern("foo");
        assert!(!interner.is_empty());
        assert_eq!(interner.len(), 2); // Empty + foo
    }

    #[test]
    fn test_default_implementations() {
        let _ = StringInterner::default();
        let _ = PropertyNameInterner::default();
        let _ = ClassNameInterner::default();
    }

    #[test]
    fn test_interned_string_ordering() {
        let id1 = InternedString(1);
        let id2 = InternedString(2);
        let id3 = InternedString(1);

        assert!(id1 < id2);
        assert!(id2 > id1);
        assert!(id1 <= id3);
        assert!(id1 >= id3);
    }

    #[test]
    fn test_interned_string_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(InternedString(1));
        set.insert(InternedString(2));
        set.insert(InternedString(1)); // Duplicate

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_large_number_of_strings() {
        let mut interner = StringInterner::new();

        for i in 0..1000 {
            let s = format!("string_{}", i);
            let id = interner.intern(&s);
            assert_eq!(interner.get(id), s);
        }

        assert_eq!(interner.len(), 1001); // Empty + 1000 strings
    }

    #[test]
    fn test_empty_string() {
        let mut interner = StringInterner::new();

        // Empty string is pre-interned at ID 0
        let id = interner.intern("");
        assert_eq!(id.raw(), 0);
        assert!(id.is_empty_string());
        assert_eq!(interner.get(id), "");
    }

    #[test]
    fn test_unicode_strings() {
        let mut interner = StringInterner::new();

        // Test with actual unicode content
        let id_ja = interner.intern("\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}"); // Japanese: konnichiwa
        let id_emoji = interner.intern("\u{1F44B}"); // Wave emoji
        let id_hello = interner.intern("hello");

        // Verify all are distinct
        assert_ne!(id_ja, id_emoji);
        assert_ne!(id_ja, id_hello);
        assert_ne!(id_emoji, id_hello);

        // Verify retrieval
        assert_eq!(
            interner.get(id_ja),
            "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}"
        );
        assert_eq!(interner.get(id_emoji), "\u{1F44B}");
        assert_eq!(interner.get(id_hello), "hello");
    }
}
