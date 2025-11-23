//! Hot Reload support for CSS Engine
//!
//! This module provides live editing support for DevTools integration,
//! allowing stylesheets to be updated at runtime with minimal invalidation.
//!
//! # Features
//!
//! - Stylesheet version tracking for undo/redo support
//! - Diffing algorithm to detect added/removed/modified rules
//! - Minimal invalidation set calculation
//! - Atomic updates (all-or-nothing)
//! - Dependency tracking for @import changes
//!
//! # Example
//!
//! ```
//! use css_engine::hot_reload::{HotReloadManager, StylesheetDiff};
//! use css_engine::StyleSheetId;
//!
//! let mut manager = HotReloadManager::new();
//!
//! // Register initial stylesheet
//! let id = manager.register_stylesheet("body { color: red; }");
//!
//! // Update stylesheet and get diff
//! let diff = manager.update_stylesheet(&id, "body { color: blue; }").unwrap();
//!
//! // Check what changed
//! assert_eq!(diff.modified_rules.len(), 1);
//! ```

use crate::error::{CssError, ElementId, StyleSheetId};
use crate::state::ParsedRule;
use fxhash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use tracing::{debug, info, trace};

/// Maximum number of versions to keep in history for undo/redo
const MAX_VERSION_HISTORY: usize = 50;

/// Manager for hot-reloading stylesheets
#[derive(Debug)]
pub struct HotReloadManager {
    /// Registered stylesheets with their current content
    stylesheets: FxHashMap<StyleSheetId, StylesheetVersion>,
    /// Version history for undo/redo support (per stylesheet)
    version_history: FxHashMap<StyleSheetId, VersionHistory>,
    /// Dependency graph for @import tracking
    dependencies: FxHashMap<StyleSheetId, FxHashSet<StyleSheetId>>,
    /// Reverse dependency map (who depends on me)
    dependents: FxHashMap<StyleSheetId, FxHashSet<StyleSheetId>>,
    /// Next available stylesheet ID
    next_id: u32,
    /// Configuration options
    config: HotReloadConfig,
}

/// Configuration for hot reload behavior
#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    /// Maximum versions to keep in history
    pub max_history: usize,
    /// Whether to track dependencies automatically
    pub track_dependencies: bool,
    /// Whether to validate CSS before applying
    pub validate_before_apply: bool,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            max_history: MAX_VERSION_HISTORY,
            track_dependencies: true,
            validate_before_apply: true,
        }
    }
}

/// A versioned stylesheet entry
#[derive(Debug, Clone)]
pub struct StylesheetVersion {
    /// The stylesheet ID
    pub id: StyleSheetId,
    /// Current CSS source
    pub source: String,
    /// Source URL (if available)
    pub source_url: Option<String>,
    /// Parsed rules (simplified for diffing)
    pub rules: Vec<ParsedRuleSnapshot>,
    /// Current version number
    pub version: u64,
    /// Whether this stylesheet is enabled
    pub enabled: bool,
}

/// Snapshot of a parsed rule for diffing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParsedRuleSnapshot {
    /// Selector text (normalized)
    pub selector: String,
    /// Declarations as key-value pairs
    pub declarations: Vec<(String, String, bool)>, // (property, value, important)
    /// Original source position (line number)
    pub source_line: usize,
}

impl ParsedRuleSnapshot {
    /// Create a new rule snapshot
    pub fn new(selector: impl Into<String>) -> Self {
        Self {
            selector: selector.into(),
            declarations: Vec::new(),
            source_line: 0,
        }
    }

    /// Add a declaration
    pub fn with_declaration(
        mut self,
        property: impl Into<String>,
        value: impl Into<String>,
        important: bool,
    ) -> Self {
        self.declarations
            .push((property.into(), value.into(), important));
        self
    }

    /// Set source line
    pub fn with_source_line(mut self, line: usize) -> Self {
        self.source_line = line;
        self
    }

    /// Create from ParsedRule
    pub fn from_parsed_rule(rule: &ParsedRule, source_line: usize) -> Self {
        let declarations = rule
            .declarations
            .iter()
            .map(|d| (d.property.clone(), d.value.clone(), d.important))
            .collect();

        Self {
            selector: rule.selector.clone(),
            declarations,
            source_line,
        }
    }
}

/// Version history for a stylesheet
#[derive(Debug)]
struct VersionHistory {
    /// Past versions (for undo)
    past: VecDeque<StylesheetVersion>,
    /// Future versions (for redo)
    future: VecDeque<StylesheetVersion>,
    /// Maximum history size
    max_size: usize,
}

impl VersionHistory {
    fn new(max_size: usize) -> Self {
        Self {
            past: VecDeque::with_capacity(max_size),
            future: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, version: StylesheetVersion) {
        // Clear future when new change is made
        self.future.clear();

        // Add to past
        if self.past.len() >= self.max_size {
            self.past.pop_front();
        }
        self.past.push_back(version);
    }

    fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    fn undo(&mut self, current: StylesheetVersion) -> Option<StylesheetVersion> {
        if let Some(previous) = self.past.pop_back() {
            if self.future.len() >= self.max_size {
                self.future.pop_front();
            }
            self.future.push_back(current);
            Some(previous)
        } else {
            None
        }
    }

    fn redo(&mut self, current: StylesheetVersion) -> Option<StylesheetVersion> {
        if let Some(next) = self.future.pop_back() {
            if self.past.len() >= self.max_size {
                self.past.pop_front();
            }
            self.past.push_back(current);
            Some(next)
        } else {
            None
        }
    }
}

/// Represents the difference between two stylesheet versions
#[derive(Debug, Clone, Default)]
pub struct StylesheetDiff {
    /// Rules that were added
    pub added_rules: Vec<ParsedRuleSnapshot>,
    /// Rules that were removed
    pub removed_rules: Vec<ParsedRuleSnapshot>,
    /// Rules that were modified (selector unchanged, declarations changed)
    pub modified_rules: Vec<RuleModification>,
    /// Elements that need restyling after this update
    pub affected_elements: FxHashSet<ElementId>,
    /// Selectors that changed (for invalidation)
    pub changed_selectors: FxHashSet<String>,
    /// Whether any @import statements changed
    pub imports_changed: bool,
    /// Previous version number
    pub old_version: u64,
    /// New version number
    pub new_version: u64,
}

impl StylesheetDiff {
    /// Create a new empty diff
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        !self.added_rules.is_empty()
            || !self.removed_rules.is_empty()
            || !self.modified_rules.is_empty()
            || self.imports_changed
    }

    /// Get total number of affected rules
    pub fn affected_rule_count(&self) -> usize {
        self.added_rules.len() + self.removed_rules.len() + self.modified_rules.len()
    }

    /// Merge another diff into this one
    pub fn merge(&mut self, other: StylesheetDiff) {
        self.added_rules.extend(other.added_rules);
        self.removed_rules.extend(other.removed_rules);
        self.modified_rules.extend(other.modified_rules);
        self.affected_elements.extend(other.affected_elements);
        self.changed_selectors.extend(other.changed_selectors);
        self.imports_changed = self.imports_changed || other.imports_changed;
        self.new_version = other.new_version;
    }
}

/// Represents a modification to a single rule
#[derive(Debug, Clone)]
pub struct RuleModification {
    /// The selector (unchanged)
    pub selector: String,
    /// Declarations that were added
    pub added_declarations: Vec<(String, String, bool)>,
    /// Declarations that were removed
    pub removed_declarations: Vec<(String, String, bool)>,
    /// Declarations that were modified (same property, different value)
    pub modified_declarations: Vec<DeclarationModification>,
}

impl RuleModification {
    /// Create a new rule modification
    pub fn new(selector: impl Into<String>) -> Self {
        Self {
            selector: selector.into(),
            added_declarations: Vec::new(),
            removed_declarations: Vec::new(),
            modified_declarations: Vec::new(),
        }
    }

    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        !self.added_declarations.is_empty()
            || !self.removed_declarations.is_empty()
            || !self.modified_declarations.is_empty()
    }
}

/// Represents a modification to a single declaration
#[derive(Debug, Clone)]
pub struct DeclarationModification {
    /// Property name
    pub property: String,
    /// Old value
    pub old_value: String,
    /// New value
    pub new_value: String,
    /// Old important flag
    pub old_important: bool,
    /// New important flag
    pub new_important: bool,
}

impl HotReloadManager {
    /// Create a new hot reload manager with default configuration
    pub fn new() -> Self {
        Self::with_config(HotReloadConfig::default())
    }

    /// Create a new hot reload manager with custom configuration
    pub fn with_config(config: HotReloadConfig) -> Self {
        Self {
            stylesheets: FxHashMap::default(),
            version_history: FxHashMap::default(),
            dependencies: FxHashMap::default(),
            dependents: FxHashMap::default(),
            next_id: 0,
            config,
        }
    }

    /// Register a new stylesheet and return its ID
    pub fn register_stylesheet(&mut self, css: &str) -> StyleSheetId {
        self.register_stylesheet_with_url(css, None)
    }

    /// Register a new stylesheet with a source URL
    pub fn register_stylesheet_with_url(
        &mut self,
        css: &str,
        source_url: Option<&str>,
    ) -> StyleSheetId {
        let id = StyleSheetId::new(self.next_id);
        self.next_id += 1;

        let rules = self.parse_rules(css);
        let version = StylesheetVersion {
            id,
            source: css.to_string(),
            source_url: source_url.map(String::from),
            rules,
            version: 1,
            enabled: true,
        };

        self.stylesheets.insert(id, version);
        self.version_history
            .insert(id, VersionHistory::new(self.config.max_history));
        self.dependencies.insert(id, FxHashSet::default());
        self.dependents.insert(id, FxHashSet::default());

        // Extract and track dependencies
        if self.config.track_dependencies {
            self.update_dependencies(id, css);
        }

        info!(target: "hot_reload", ?id, "Registered new stylesheet");
        id
    }

    /// Update a stylesheet with new CSS content
    ///
    /// Returns a diff showing what changed, or an error if the update fails.
    /// This operation is atomic - either all changes are applied or none.
    pub fn update_stylesheet(
        &mut self,
        id: &StyleSheetId,
        new_css: &str,
    ) -> Result<StylesheetDiff, CssError> {
        // Get current version
        let current = self
            .stylesheets
            .get(id)
            .ok_or(CssError::StylesheetNotFound { stylesheet_id: *id })?
            .clone();

        // Validate new CSS if configured
        if self.config.validate_before_apply {
            self.validate_css(new_css)?;
        }

        debug!(
            target: "hot_reload",
            ?id,
            old_version = current.version,
            "Updating stylesheet"
        );

        // Parse new rules
        let new_rules = self.parse_rules(new_css);

        // Compute diff
        let mut diff = self.compute_diff(&current.rules, &new_rules);
        diff.old_version = current.version;
        diff.new_version = current.version + 1;

        // Check for @import changes
        diff.imports_changed = self.detect_import_changes(&current.source, new_css);

        // Calculate affected elements based on changed selectors
        self.calculate_affected_elements(&mut diff);

        // Store old version in history
        if let Some(history) = self.version_history.get_mut(id) {
            history.push(current.clone());
        }

        // Apply update atomically
        let new_version = StylesheetVersion {
            id: *id,
            source: new_css.to_string(),
            source_url: current.source_url.clone(),
            rules: new_rules,
            version: current.version + 1,
            enabled: current.enabled,
        };

        self.stylesheets.insert(*id, new_version);

        // Update dependencies if configured
        if self.config.track_dependencies {
            self.update_dependencies(*id, new_css);
        }

        info!(
            target: "hot_reload",
            ?id,
            new_version = diff.new_version,
            added = diff.added_rules.len(),
            removed = diff.removed_rules.len(),
            modified = diff.modified_rules.len(),
            "Stylesheet updated"
        );

        Ok(diff)
    }

    /// Get the current version of a stylesheet
    pub fn get_stylesheet(&self, id: &StyleSheetId) -> Option<&StylesheetVersion> {
        self.stylesheets.get(id)
    }

    /// Get the current CSS source for a stylesheet
    pub fn get_source(&self, id: &StyleSheetId) -> Option<&str> {
        self.stylesheets.get(id).map(|v| v.source.as_str())
    }

    /// Check if a stylesheet is registered
    pub fn contains(&self, id: &StyleSheetId) -> bool {
        self.stylesheets.contains_key(id)
    }

    /// Get the number of registered stylesheets
    pub fn len(&self) -> usize {
        self.stylesheets.len()
    }

    /// Check if there are no registered stylesheets
    pub fn is_empty(&self) -> bool {
        self.stylesheets.is_empty()
    }

    /// Remove a stylesheet
    pub fn remove_stylesheet(&mut self, id: &StyleSheetId) -> bool {
        let removed = self.stylesheets.remove(id).is_some();
        if removed {
            self.version_history.remove(id);

            // Clean up dependency tracking
            if let Some(deps) = self.dependencies.remove(id) {
                for dep in deps {
                    if let Some(dependents) = self.dependents.get_mut(&dep) {
                        dependents.remove(id);
                    }
                }
            }
            if let Some(deps) = self.dependents.remove(id) {
                for dep in deps {
                    if let Some(dependencies) = self.dependencies.get_mut(&dep) {
                        dependencies.remove(id);
                    }
                }
            }

            info!(target: "hot_reload", ?id, "Removed stylesheet");
        }
        removed
    }

    /// Undo the last change to a stylesheet
    pub fn undo(&mut self, id: &StyleSheetId) -> Result<StylesheetDiff, CssError> {
        let current = self
            .stylesheets
            .get(id)
            .ok_or(CssError::StylesheetNotFound { stylesheet_id: *id })?
            .clone();

        let history = self
            .version_history
            .get_mut(id)
            .ok_or(CssError::StylesheetNotFound { stylesheet_id: *id })?;

        if let Some(previous) = history.undo(current.clone()) {
            // Compute diff from current to previous
            let diff = self.compute_diff(&current.rules, &previous.rules);

            self.stylesheets.insert(*id, previous);

            debug!(target: "hot_reload", ?id, "Undo applied");
            Ok(diff)
        } else {
            Err(CssError::ComputationError {
                reason: "Nothing to undo".to_string(),
            })
        }
    }

    /// Redo a previously undone change
    pub fn redo(&mut self, id: &StyleSheetId) -> Result<StylesheetDiff, CssError> {
        let current = self
            .stylesheets
            .get(id)
            .ok_or(CssError::StylesheetNotFound { stylesheet_id: *id })?
            .clone();

        let history = self
            .version_history
            .get_mut(id)
            .ok_or(CssError::StylesheetNotFound { stylesheet_id: *id })?;

        if let Some(next) = history.redo(current.clone()) {
            // Compute diff from current to next
            let diff = self.compute_diff(&current.rules, &next.rules);

            self.stylesheets.insert(*id, next);

            debug!(target: "hot_reload", ?id, "Redo applied");
            Ok(diff)
        } else {
            Err(CssError::ComputationError {
                reason: "Nothing to redo".to_string(),
            })
        }
    }

    /// Check if undo is available for a stylesheet
    pub fn can_undo(&self, id: &StyleSheetId) -> bool {
        self.version_history.get(id).is_some_and(|h| h.can_undo())
    }

    /// Check if redo is available for a stylesheet
    pub fn can_redo(&self, id: &StyleSheetId) -> bool {
        self.version_history.get(id).is_some_and(|h| h.can_redo())
    }

    /// Get stylesheets that depend on the given stylesheet
    pub fn get_dependents(&self, id: &StyleSheetId) -> Vec<StyleSheetId> {
        self.dependents
            .get(id)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Get stylesheets that the given stylesheet depends on
    pub fn get_dependencies(&self, id: &StyleSheetId) -> Vec<StyleSheetId> {
        self.dependencies
            .get(id)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Enable or disable a stylesheet
    pub fn set_enabled(&mut self, id: &StyleSheetId, enabled: bool) -> Result<(), CssError> {
        let stylesheet = self
            .stylesheets
            .get_mut(id)
            .ok_or(CssError::StylesheetNotFound { stylesheet_id: *id })?;

        stylesheet.enabled = enabled;
        debug!(target: "hot_reload", ?id, enabled, "Stylesheet enabled state changed");
        Ok(())
    }

    /// Check if a stylesheet is enabled
    pub fn is_enabled(&self, id: &StyleSheetId) -> bool {
        self.stylesheets.get(id).is_some_and(|s| s.enabled)
    }

    // ========================================================================
    // Private helper methods
    // ========================================================================

    /// Parse CSS into rule snapshots (simplified parser)
    fn parse_rules(&self, css: &str) -> Vec<ParsedRuleSnapshot> {
        let mut rules = Vec::new();
        let mut current_selector = String::new();
        let mut in_rule = false;
        let mut brace_depth = 0;
        let mut line_number = 1;
        let mut rule_start_line = 1;

        let mut buffer = String::new();

        for ch in css.chars() {
            if ch == '\n' {
                line_number += 1;
            }

            match ch {
                '{' => {
                    brace_depth += 1;
                    if brace_depth == 1 && !in_rule {
                        current_selector = buffer.trim().to_string();
                        buffer.clear();
                        in_rule = true;
                        rule_start_line = line_number;
                    } else {
                        buffer.push(ch);
                    }
                }
                '}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 && in_rule {
                        // Parse declarations from buffer
                        let declarations = self.parse_declarations(&buffer);

                        // Skip @-rules for now (media queries, etc.)
                        if !current_selector.starts_with('@') {
                            let rule = ParsedRuleSnapshot {
                                selector: self.normalize_selector(&current_selector),
                                declarations,
                                source_line: rule_start_line,
                            };
                            rules.push(rule);
                        }

                        buffer.clear();
                        current_selector.clear();
                        in_rule = false;
                    } else if brace_depth >= 0 {
                        buffer.push(ch);
                    }
                }
                _ => {
                    buffer.push(ch);
                }
            }
        }

        trace!(target: "hot_reload", rule_count = rules.len(), "Parsed CSS rules");
        rules
    }

    /// Parse declarations from a string
    fn parse_declarations(&self, css: &str) -> Vec<(String, String, bool)> {
        let mut declarations = Vec::new();

        for decl in css.split(';') {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }

            if let Some(colon_pos) = decl.find(':') {
                let property = decl[..colon_pos].trim().to_lowercase();
                let mut value = decl[colon_pos + 1..].trim().to_string();
                let mut important = false;

                if value.ends_with("!important") {
                    value = value[..value.len() - 10].trim().to_string();
                    important = true;
                }

                declarations.push((property, value, important));
            }
        }

        declarations
    }

    /// Normalize a selector for comparison
    fn normalize_selector(&self, selector: &str) -> String {
        // Remove extra whitespace and normalize
        selector.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Validate CSS syntax
    fn validate_css(&self, css: &str) -> Result<(), CssError> {
        // Basic validation: check brace matching
        let mut brace_count = 0;
        let mut line = 1;
        let mut column = 0;

        for ch in css.chars() {
            column += 1;
            match ch {
                '\n' => {
                    line += 1;
                    column = 0;
                }
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count < 0 {
                        return Err(CssError::ParseError {
                            line,
                            column,
                            message: "Unexpected closing brace".to_string(),
                        });
                    }
                }
                _ => {}
            }
        }

        if brace_count != 0 {
            return Err(CssError::ParseError {
                line,
                column,
                message: format!("Unclosed braces: {} open", brace_count),
            });
        }

        Ok(())
    }

    /// Compute the diff between two sets of rules
    fn compute_diff(
        &self,
        old_rules: &[ParsedRuleSnapshot],
        new_rules: &[ParsedRuleSnapshot],
    ) -> StylesheetDiff {
        let mut diff = StylesheetDiff::new();

        // Build maps for efficient lookup
        let old_by_selector: FxHashMap<&str, &ParsedRuleSnapshot> =
            old_rules.iter().map(|r| (r.selector.as_str(), r)).collect();

        let new_by_selector: FxHashMap<&str, &ParsedRuleSnapshot> =
            new_rules.iter().map(|r| (r.selector.as_str(), r)).collect();

        // Find removed and modified rules
        for old_rule in old_rules {
            if let Some(new_rule) = new_by_selector.get(old_rule.selector.as_str()) {
                // Rule exists in both - check for modifications
                if old_rule.declarations != new_rule.declarations {
                    let modification = self.compute_rule_modification(old_rule, new_rule);
                    if modification.has_changes() {
                        diff.modified_rules.push(modification);
                        diff.changed_selectors.insert(old_rule.selector.clone());
                    }
                }
            } else {
                // Rule was removed
                diff.removed_rules.push(old_rule.clone());
                diff.changed_selectors.insert(old_rule.selector.clone());
            }
        }

        // Find added rules
        for new_rule in new_rules {
            if !old_by_selector.contains_key(new_rule.selector.as_str()) {
                diff.added_rules.push(new_rule.clone());
                diff.changed_selectors.insert(new_rule.selector.clone());
            }
        }

        diff
    }

    /// Compute modifications for a single rule
    fn compute_rule_modification(
        &self,
        old_rule: &ParsedRuleSnapshot,
        new_rule: &ParsedRuleSnapshot,
    ) -> RuleModification {
        let mut modification = RuleModification::new(&old_rule.selector);

        // Build maps of declarations
        let old_decls: FxHashMap<&str, (&str, bool)> = old_rule
            .declarations
            .iter()
            .map(|(p, v, i)| (p.as_str(), (v.as_str(), *i)))
            .collect();

        let new_decls: FxHashMap<&str, (&str, bool)> = new_rule
            .declarations
            .iter()
            .map(|(p, v, i)| (p.as_str(), (v.as_str(), *i)))
            .collect();

        // Find removed and modified declarations
        for (prop, (old_val, old_imp)) in &old_decls {
            if let Some((new_val, new_imp)) = new_decls.get(prop) {
                if old_val != new_val || old_imp != new_imp {
                    modification
                        .modified_declarations
                        .push(DeclarationModification {
                            property: (*prop).to_string(),
                            old_value: (*old_val).to_string(),
                            new_value: (*new_val).to_string(),
                            old_important: *old_imp,
                            new_important: *new_imp,
                        });
                }
            } else {
                modification.removed_declarations.push((
                    (*prop).to_string(),
                    (*old_val).to_string(),
                    *old_imp,
                ));
            }
        }

        // Find added declarations
        for (prop, (val, imp)) in &new_decls {
            if !old_decls.contains_key(prop) {
                modification.added_declarations.push((
                    (*prop).to_string(),
                    (*val).to_string(),
                    *imp,
                ));
            }
        }

        modification
    }

    /// Detect changes in @import statements
    fn detect_import_changes(&self, old_css: &str, new_css: &str) -> bool {
        let old_imports = self.extract_imports(old_css);
        let new_imports = self.extract_imports(new_css);
        old_imports != new_imports
    }

    /// Extract @import URLs from CSS
    fn extract_imports(&self, css: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for line in css.lines() {
            let line = line.trim();
            if line.starts_with("@import") {
                // Extract URL from @import statement
                if let Some(start) = line.find(['"', '\'']) {
                    if let Some(end) = line[start + 1..].find(['"', '\'']) {
                        imports.push(line[start + 1..start + 1 + end].to_string());
                    }
                } else if let Some(start) = line.find("url(") {
                    if let Some(end) = line[start + 4..].find(')') {
                        let url = line[start + 4..start + 4 + end].trim();
                        let url = url.trim_matches(['"', '\'']);
                        imports.push(url.to_string());
                    }
                }
            }
        }

        imports
    }

    /// Update dependency tracking for a stylesheet
    fn update_dependencies(&mut self, id: StyleSheetId, css: &str) {
        // Clear existing dependencies
        if let Some(old_deps) = self.dependencies.get(&id).cloned() {
            for dep in old_deps {
                if let Some(dependents) = self.dependents.get_mut(&dep) {
                    dependents.remove(&id);
                }
            }
        }

        // Extract new imports and update dependencies
        let imports = self.extract_imports(css);
        let new_deps = FxHashSet::default();

        // Note: In a real implementation, we would resolve import URLs to stylesheet IDs
        // For now, we just track that imports exist
        trace!(
            target: "hot_reload",
            ?id,
            import_count = imports.len(),
            "Updated dependencies"
        );

        self.dependencies.insert(id, new_deps);
    }

    /// Calculate which elements need restyling based on changed selectors
    fn calculate_affected_elements(&self, diff: &mut StylesheetDiff) {
        // In a real implementation, this would query the DOM to find matching elements
        // For now, we just mark that elements matching the selectors need updating
        // The actual element matching would be done by the CSS engine

        trace!(
            target: "hot_reload",
            selector_count = diff.changed_selectors.len(),
            "Calculated affected selectors"
        );
    }
}

impl Default for HotReloadManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::PropertyDeclaration;

    // ========================================================================
    // HotReloadManager basic tests
    // ========================================================================

    #[test]
    fn test_hot_reload_manager_new() {
        let manager = HotReloadManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_register_stylesheet() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        assert!(!manager.is_empty());
        assert_eq!(manager.len(), 1);
        assert!(manager.contains(&id));
    }

    #[test]
    fn test_register_stylesheet_with_url() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet_with_url("body { color: red; }", Some("styles.css"));

        let stylesheet = manager.get_stylesheet(&id).unwrap();
        assert_eq!(stylesheet.source_url.as_deref(), Some("styles.css"));
    }

    #[test]
    fn test_get_source() {
        let mut manager = HotReloadManager::new();
        let css = "body { color: red; }";
        let id = manager.register_stylesheet(css);

        assert_eq!(manager.get_source(&id), Some(css));
    }

    #[test]
    fn test_remove_stylesheet() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        assert!(manager.remove_stylesheet(&id));
        assert!(!manager.contains(&id));
        assert!(manager.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_stylesheet() {
        let mut manager = HotReloadManager::new();
        let id = StyleSheetId::new(999);

        assert!(!manager.remove_stylesheet(&id));
    }

    // ========================================================================
    // Stylesheet update tests
    // ========================================================================

    #[test]
    fn test_update_stylesheet_basic() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: blue; }")
            .unwrap();

        assert!(diff.has_changes());
        assert_eq!(diff.modified_rules.len(), 1);
        assert_eq!(diff.old_version, 1);
        assert_eq!(diff.new_version, 2);
    }

    #[test]
    fn test_update_stylesheet_add_rule() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: red; }\ndiv { margin: 10px; }")
            .unwrap();

        assert_eq!(diff.added_rules.len(), 1);
        assert_eq!(diff.added_rules[0].selector, "div");
    }

    #[test]
    fn test_update_stylesheet_remove_rule() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }\ndiv { margin: 10px; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: red; }")
            .unwrap();

        assert_eq!(diff.removed_rules.len(), 1);
        assert_eq!(diff.removed_rules[0].selector, "div");
    }

    #[test]
    fn test_update_stylesheet_modify_declaration() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; font-size: 16px; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: blue; font-size: 16px; }")
            .unwrap();

        assert_eq!(diff.modified_rules.len(), 1);
        let modification = &diff.modified_rules[0];
        assert_eq!(modification.modified_declarations.len(), 1);
        assert_eq!(modification.modified_declarations[0].property, "color");
        assert_eq!(modification.modified_declarations[0].old_value, "red");
        assert_eq!(modification.modified_declarations[0].new_value, "blue");
    }

    #[test]
    fn test_update_stylesheet_add_declaration() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: red; margin: 10px; }")
            .unwrap();

        assert_eq!(diff.modified_rules.len(), 1);
        assert_eq!(diff.modified_rules[0].added_declarations.len(), 1);
        assert_eq!(diff.modified_rules[0].added_declarations[0].0, "margin");
    }

    #[test]
    fn test_update_stylesheet_remove_declaration() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; margin: 10px; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: red; }")
            .unwrap();

        assert_eq!(diff.modified_rules.len(), 1);
        assert_eq!(diff.modified_rules[0].removed_declarations.len(), 1);
        assert_eq!(diff.modified_rules[0].removed_declarations[0].0, "margin");
    }

    #[test]
    fn test_update_nonexistent_stylesheet() {
        let mut manager = HotReloadManager::new();
        let id = StyleSheetId::new(999);

        let result = manager.update_stylesheet(&id, "body { color: red; }");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_stylesheet_no_changes() {
        let mut manager = HotReloadManager::new();
        let css = "body { color: red; }";
        let id = manager.register_stylesheet(css);

        let diff = manager.update_stylesheet(&id, css).unwrap();

        assert!(!diff.has_changes());
    }

    // ========================================================================
    // Invalidation tests
    // ========================================================================

    #[test]
    fn test_diff_changed_selectors() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: blue; }")
            .unwrap();

        assert!(diff.changed_selectors.contains("body"));
    }

    #[test]
    fn test_diff_affected_rule_count() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }\ndiv { margin: 0; }");

        let diff = manager
            .update_stylesheet(
                &id,
                "body { color: blue; }\nspan { padding: 5px; }", // modify body, remove div, add span
            )
            .unwrap();

        assert_eq!(diff.affected_rule_count(), 3); // 1 modified + 1 removed + 1 added
    }

    // ========================================================================
    // Malformed CSS tests
    // ========================================================================

    #[test]
    fn test_update_with_malformed_css_unmatched_braces() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let result = manager.update_stylesheet(&id, "body { color: blue;");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_with_malformed_css_extra_closing_brace() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let result = manager.update_stylesheet(&id, "body { color: blue; } }");
        assert!(result.is_err());
    }

    // ========================================================================
    // Undo/Redo tests
    // ========================================================================

    #[test]
    fn test_undo_basic() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        manager
            .update_stylesheet(&id, "body { color: blue; }")
            .unwrap();

        assert!(manager.can_undo(&id));
        let diff = manager.undo(&id).unwrap();

        // After undo, should be back to red
        let source = manager.get_source(&id).unwrap();
        assert!(source.contains("red"));
        assert!(diff.has_changes());
    }

    #[test]
    fn test_redo_basic() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        manager
            .update_stylesheet(&id, "body { color: blue; }")
            .unwrap();
        manager.undo(&id).unwrap();

        assert!(manager.can_redo(&id));
        let diff = manager.redo(&id).unwrap();

        // After redo, should be back to blue
        let source = manager.get_source(&id).unwrap();
        assert!(source.contains("blue"));
        assert!(diff.has_changes());
    }

    #[test]
    fn test_undo_when_nothing_to_undo() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        assert!(!manager.can_undo(&id));
        let result = manager.undo(&id);
        assert!(result.is_err());
    }

    #[test]
    fn test_redo_when_nothing_to_redo() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        assert!(!manager.can_redo(&id));
        let result = manager.redo(&id);
        assert!(result.is_err());
    }

    #[test]
    fn test_undo_redo_clears_future() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        manager
            .update_stylesheet(&id, "body { color: blue; }")
            .unwrap();
        manager.undo(&id).unwrap();

        // Make a new change - should clear redo history
        manager
            .update_stylesheet(&id, "body { color: green; }")
            .unwrap();

        assert!(!manager.can_redo(&id));
    }

    #[test]
    fn test_multiple_undos() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        manager
            .update_stylesheet(&id, "body { color: blue; }")
            .unwrap();
        manager
            .update_stylesheet(&id, "body { color: green; }")
            .unwrap();

        manager.undo(&id).unwrap();
        let source = manager.get_source(&id).unwrap();
        assert!(source.contains("blue"));

        manager.undo(&id).unwrap();
        let source = manager.get_source(&id).unwrap();
        assert!(source.contains("red"));
    }

    // ========================================================================
    // Enable/Disable tests
    // ========================================================================

    #[test]
    fn test_enable_disable_stylesheet() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        assert!(manager.is_enabled(&id));

        manager.set_enabled(&id, false).unwrap();
        assert!(!manager.is_enabled(&id));

        manager.set_enabled(&id, true).unwrap();
        assert!(manager.is_enabled(&id));
    }

    #[test]
    fn test_enable_nonexistent_stylesheet() {
        let mut manager = HotReloadManager::new();
        let id = StyleSheetId::new(999);

        let result = manager.set_enabled(&id, false);
        assert!(result.is_err());
    }

    // ========================================================================
    // Import detection tests
    // ========================================================================

    #[test]
    fn test_detect_import_changes() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("@import 'base.css';\nbody { color: red; }");

        let diff = manager
            .update_stylesheet(&id, "@import 'new.css';\nbody { color: red; }")
            .unwrap();

        assert!(diff.imports_changed);
    }

    #[test]
    fn test_no_import_changes() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("@import 'base.css';\nbody { color: red; }");

        let diff = manager
            .update_stylesheet(&id, "@import 'base.css';\nbody { color: blue; }")
            .unwrap();

        assert!(!diff.imports_changed);
    }

    // ========================================================================
    // ParsedRuleSnapshot tests
    // ========================================================================

    #[test]
    fn test_parsed_rule_snapshot_new() {
        let rule = ParsedRuleSnapshot::new("body");
        assert_eq!(rule.selector, "body");
        assert!(rule.declarations.is_empty());
    }

    #[test]
    fn test_parsed_rule_snapshot_builder() {
        let rule = ParsedRuleSnapshot::new("body")
            .with_declaration("color", "red", false)
            .with_declaration("font-size", "16px", true)
            .with_source_line(10);

        assert_eq!(rule.selector, "body");
        assert_eq!(rule.declarations.len(), 2);
        assert_eq!(rule.source_line, 10);
    }

    #[test]
    fn test_parsed_rule_snapshot_from_parsed_rule() {
        let mut rule = ParsedRule::new("div".to_string());
        rule.add_declaration(PropertyDeclaration::new("margin", "0"));
        rule.add_declaration(PropertyDeclaration::new("padding", "10px").with_important());

        let snapshot = ParsedRuleSnapshot::from_parsed_rule(&rule, 5);

        assert_eq!(snapshot.selector, "div");
        assert_eq!(snapshot.declarations.len(), 2);
        assert_eq!(snapshot.source_line, 5);
    }

    // ========================================================================
    // StylesheetDiff tests
    // ========================================================================

    #[test]
    fn test_stylesheet_diff_new() {
        let diff = StylesheetDiff::new();
        assert!(!diff.has_changes());
        assert_eq!(diff.affected_rule_count(), 0);
    }

    #[test]
    fn test_stylesheet_diff_merge() {
        let mut diff1 = StylesheetDiff::new();
        diff1.added_rules.push(ParsedRuleSnapshot::new("body"));

        let mut diff2 = StylesheetDiff::new();
        diff2.removed_rules.push(ParsedRuleSnapshot::new("div"));
        diff2.new_version = 5;

        diff1.merge(diff2);

        assert_eq!(diff1.added_rules.len(), 1);
        assert_eq!(diff1.removed_rules.len(), 1);
        assert_eq!(diff1.new_version, 5);
    }

    // ========================================================================
    // RuleModification tests
    // ========================================================================

    #[test]
    fn test_rule_modification_new() {
        let modification = RuleModification::new("body");
        assert_eq!(modification.selector, "body");
        assert!(!modification.has_changes());
    }

    #[test]
    fn test_rule_modification_has_changes() {
        let mut modification = RuleModification::new("body");
        assert!(!modification.has_changes());

        modification
            .added_declarations
            .push(("color".to_string(), "red".to_string(), false));
        assert!(modification.has_changes());
    }

    // ========================================================================
    // Configuration tests
    // ========================================================================

    #[test]
    fn test_hot_reload_config_default() {
        let config = HotReloadConfig::default();
        assert_eq!(config.max_history, MAX_VERSION_HISTORY);
        assert!(config.track_dependencies);
        assert!(config.validate_before_apply);
    }

    #[test]
    fn test_custom_config() {
        let config = HotReloadConfig {
            max_history: 10,
            track_dependencies: false,
            validate_before_apply: false,
        };

        let mut manager = HotReloadManager::with_config(config);
        let id = manager.register_stylesheet("body { color: red; }");

        // With validation disabled, malformed CSS should be accepted
        let result = manager.update_stylesheet(&id, "body { color: blue;");
        assert!(result.is_ok()); // Validation is off
    }

    // ========================================================================
    // Complex selector tests
    // ========================================================================

    #[test]
    fn test_complex_selectors() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet(
            ".container > .item:hover { color: red; }\n\
             #main .sidebar::before { content: ''; }",
        );

        let diff = manager
            .update_stylesheet(
                &id,
                ".container > .item:hover { color: blue; }\n\
                 #main .sidebar::before { content: 'x'; }",
            )
            .unwrap();

        assert_eq!(diff.modified_rules.len(), 2);
    }

    #[test]
    fn test_important_flag_changes() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let diff = manager
            .update_stylesheet(&id, "body { color: red !important; }")
            .unwrap();

        assert_eq!(diff.modified_rules.len(), 1);
        let modification = &diff.modified_rules[0].modified_declarations[0];
        assert!(!modification.old_important);
        assert!(modification.new_important);
    }

    // ========================================================================
    // Version tracking tests
    // ========================================================================

    #[test]
    fn test_version_increments() {
        let mut manager = HotReloadManager::new();
        let id = manager.register_stylesheet("body { color: red; }");

        let stylesheet = manager.get_stylesheet(&id).unwrap();
        assert_eq!(stylesheet.version, 1);

        manager
            .update_stylesheet(&id, "body { color: blue; }")
            .unwrap();
        let stylesheet = manager.get_stylesheet(&id).unwrap();
        assert_eq!(stylesheet.version, 2);

        manager
            .update_stylesheet(&id, "body { color: green; }")
            .unwrap();
        let stylesheet = manager.get_stylesheet(&id).unwrap();
        assert_eq!(stylesheet.version, 3);
    }
}
