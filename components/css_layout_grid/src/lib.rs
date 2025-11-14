//! CSS Grid Layout Computation
//!
//! This module provides CSS Grid layout computation including:
//! - Grid container properties (template rows/columns, auto-flow, gaps)
//! - Grid item placement (explicit and auto-placement)
//! - Track sizing with fr units
//! - Grid layout engine trait and implementation

use css_types::{Length, LengthUnit};

// ============================================================================
// Grid Auto Flow
// ============================================================================

/// Grid auto-placement algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridAutoFlow {
    /// Place items by filling rows
    #[default]
    Row,
    /// Place items by filling columns
    Column,
    /// Place items by filling rows, using dense packing
    RowDense,
    /// Place items by filling columns, using dense packing
    ColumnDense,
}

// ============================================================================
// Track Sizing
// ============================================================================

/// Grid track size specification
#[derive(Debug, Clone, PartialEq)]
pub enum TrackSizing {
    /// Fixed size (px, em, etc.)
    Fixed(Length),
    /// Flexible size (fr units)
    Flexible(f32),
    /// Size to minimum content
    MinContent,
    /// Size to maximum content
    MaxContent,
    /// Auto sizing
    Auto,
}

// ============================================================================
// Grid Line
// ============================================================================

/// Grid line specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridLine {
    /// Auto-placement
    #[default]
    Auto,
    /// Explicit line number (1-indexed, negative counts from end)
    LineNumber(i32),
    /// Span N tracks
    Span(i32),
}

// ============================================================================
// Grid Container
// ============================================================================

/// CSS Grid container properties
#[derive(Debug, Clone, PartialEq)]
pub struct GridContainer {
    template_rows: Vec<TrackSizing>,
    template_columns: Vec<TrackSizing>,
    auto_rows: Vec<TrackSizing>,
    auto_columns: Vec<TrackSizing>,
    auto_flow: GridAutoFlow,
    gap: Option<f32>,
    row_gap: Option<f32>,
    column_gap: Option<f32>,
}

impl GridContainer {
    /// Create a new grid container with default properties
    pub fn new() -> Self {
        Self {
            template_rows: Vec::new(),
            template_columns: Vec::new(),
            auto_rows: Vec::new(),
            auto_columns: Vec::new(),
            auto_flow: GridAutoFlow::default(),
            gap: None,
            row_gap: None,
            column_gap: None,
        }
    }

    /// Get template rows
    pub fn template_rows(&self) -> &[TrackSizing] {
        &self.template_rows
    }

    /// Set template rows
    pub fn set_template_rows(&mut self, rows: Vec<TrackSizing>) {
        self.template_rows = rows;
    }

    /// Get template columns
    pub fn template_columns(&self) -> &[TrackSizing] {
        &self.template_columns
    }

    /// Set template columns
    pub fn set_template_columns(&mut self, columns: Vec<TrackSizing>) {
        self.template_columns = columns;
    }

    /// Get auto rows
    pub fn auto_rows(&self) -> &[TrackSizing] {
        &self.auto_rows
    }

    /// Set auto rows
    pub fn set_auto_rows(&mut self, rows: Vec<TrackSizing>) {
        self.auto_rows = rows;
    }

    /// Get auto columns
    pub fn auto_columns(&self) -> &[TrackSizing] {
        &self.auto_columns
    }

    /// Set auto columns
    pub fn set_auto_columns(&mut self, columns: Vec<TrackSizing>) {
        self.auto_columns = columns;
    }

    /// Get auto flow
    pub fn auto_flow(&self) -> GridAutoFlow {
        self.auto_flow
    }

    /// Set auto flow
    pub fn set_auto_flow(&mut self, flow: GridAutoFlow) {
        self.auto_flow = flow;
    }

    /// Get gap (applies to both rows and columns if row_gap/column_gap not set)
    pub fn gap(&self) -> Option<f32> {
        self.gap
    }

    /// Set gap
    pub fn set_gap(&mut self, gap: Option<f32>) {
        self.gap = gap;
    }

    /// Get row gap
    pub fn row_gap(&self) -> Option<f32> {
        self.row_gap
    }

    /// Set row gap
    pub fn set_row_gap(&mut self, gap: Option<f32>) {
        self.row_gap = gap;
    }

    /// Get column gap
    pub fn column_gap(&self) -> Option<f32> {
        self.column_gap
    }

    /// Set column gap
    pub fn set_column_gap(&mut self, gap: Option<f32>) {
        self.column_gap = gap;
    }

    /// Get effective row gap (row_gap if set, otherwise gap)
    pub fn effective_row_gap(&self) -> f32 {
        self.row_gap.or(self.gap).unwrap_or(0.0)
    }

    /// Get effective column gap (column_gap if set, otherwise gap)
    pub fn effective_column_gap(&self) -> f32 {
        self.column_gap.or(self.gap).unwrap_or(0.0)
    }
}

impl Default for GridContainer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Grid Item
// ============================================================================

/// Grid item placement
#[derive(Debug, Clone, PartialEq)]
pub struct GridItem {
    row_start: GridLine,
    row_end: GridLine,
    column_start: GridLine,
    column_end: GridLine,
}

impl GridItem {
    /// Create a new grid item with auto placement
    pub fn new() -> Self {
        Self {
            row_start: GridLine::Auto,
            row_end: GridLine::Auto,
            column_start: GridLine::Auto,
            column_end: GridLine::Auto,
        }
    }

    /// Get row start
    pub fn row_start(&self) -> GridLine {
        self.row_start
    }

    /// Set row start
    pub fn set_row_start(&mut self, line: GridLine) {
        self.row_start = line;
    }

    /// Get row end
    pub fn row_end(&self) -> GridLine {
        self.row_end
    }

    /// Set row end
    pub fn set_row_end(&mut self, line: GridLine) {
        self.row_end = line;
    }

    /// Get column start
    pub fn column_start(&self) -> GridLine {
        self.column_start
    }

    /// Set column start
    pub fn set_column_start(&mut self, line: GridLine) {
        self.column_start = line;
    }

    /// Get column end
    pub fn column_end(&self) -> GridLine {
        self.column_end
    }

    /// Set column end
    pub fn set_column_end(&mut self, line: GridLine) {
        self.column_end = line;
    }
}

impl Default for GridItem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Grid Layout Results
// ============================================================================

/// Layout position and size for a grid item
#[derive(Debug, Clone, PartialEq)]
pub struct GridItemLayout {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    row: usize,
    column: usize,
    row_span: usize,
    column_span: usize,
}

impl GridItemLayout {
    /// Create a new grid item layout
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        row: usize,
        column: usize,
        row_span: usize,
        column_span: usize,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            row,
            column,
            row_span,
            column_span,
        }
    }

    /// Get x coordinate
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Get y coordinate
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Get width
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Get row index (0-based)
    pub fn row(&self) -> usize {
        self.row
    }

    /// Get column index (0-based)
    pub fn column(&self) -> usize {
        self.column
    }

    /// Get row span
    pub fn row_span(&self) -> usize {
        self.row_span
    }

    /// Get column span
    pub fn column_span(&self) -> usize {
        self.column_span
    }
}

/// Computed grid layout
#[derive(Debug, Clone, PartialEq)]
pub struct GridLayout {
    items: Vec<GridItemLayout>,
    container_size: (f32, f32),
}

impl GridLayout {
    /// Create a new grid layout
    pub fn new(items: Vec<GridItemLayout>, container_size: (f32, f32)) -> Self {
        Self {
            items,
            container_size,
        }
    }

    /// Get item layouts
    pub fn items(&self) -> &[GridItemLayout] {
        &self.items
    }

    /// Get container size (width, height)
    pub fn container_size(&self) -> (f32, f32) {
        self.container_size
    }
}

// ============================================================================
// Grid Layout Engine
// ============================================================================

/// Grid layout computation engine
pub trait GridLayoutEngine {
    /// Compute layout for grid container and items
    ///
    /// # Arguments
    /// * `container` - Grid container properties
    /// * `items` - Grid items to place
    /// * `available_space` - Available space (width, height)
    ///
    /// # Returns
    /// Computed grid layout with item positions
    fn compute_grid_layout(
        &self,
        container: &GridContainer,
        items: &[GridItem],
        available_space: (f32, f32),
    ) -> GridLayout;

    /// Resolve track sizes to pixel values
    ///
    /// # Arguments
    /// * `tracks` - Track sizing specifications
    /// * `available_size` - Available space for tracks
    /// * `gap` - Gap between tracks
    ///
    /// # Returns
    /// Vector of resolved track sizes in pixels
    fn resolve_track_sizes(
        &self,
        tracks: &[TrackSizing],
        available_size: f32,
        gap: f32,
    ) -> Vec<f32>;
}

// ============================================================================
// Basic Grid Layout Engine Implementation
// ============================================================================

/// Basic implementation of grid layout engine
pub struct BasicGridLayoutEngine;

impl BasicGridLayoutEngine {
    /// Create a new basic grid layout engine
    pub fn new() -> Self {
        Self
    }

    /// Resolve a single track size to pixels
    fn resolve_single_track(&self, track: &TrackSizing, _available_size: f32) -> Option<f32> {
        match track {
            TrackSizing::Fixed(length) => {
                // For now, only handle px units
                if length.unit() == LengthUnit::Px {
                    Some(length.value())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Calculate total fr units in tracks
    fn total_fr_units(&self, tracks: &[TrackSizing]) -> f32 {
        tracks
            .iter()
            .map(|t| match t {
                TrackSizing::Flexible(fr) => *fr,
                _ => 0.0,
            })
            .sum()
    }

    /// Calculate fixed space used by non-flexible tracks
    fn calculate_fixed_space(&self, tracks: &[TrackSizing]) -> f32 {
        tracks
            .iter()
            .filter_map(|t| self.resolve_single_track(t, 0.0))
            .sum()
    }

    /// Place items using auto-placement algorithm
    fn auto_place_items(
        &self,
        items: &[GridItem],
        row_sizes: &[f32],
        column_sizes: &[f32],
        container: &GridContainer,
    ) -> Vec<GridItemLayout> {
        let mut layouts = Vec::new();
        let mut cursor_row = 0;
        let mut cursor_col = 0;

        let row_count = row_sizes.len();
        let col_count = column_sizes.len();

        if row_count == 0 || col_count == 0 {
            return layouts;
        }

        let row_gap = container.effective_row_gap();
        let col_gap = container.effective_column_gap();

        for item in items {
            // Determine placement
            let (row, col) = match (item.row_start, item.column_start) {
                (GridLine::LineNumber(r), GridLine::LineNumber(c)) => {
                    // Explicit placement (convert 1-based to 0-based)
                    let row_idx = if r > 0 { (r - 1) as usize } else { 0 };
                    let col_idx = if c > 0 { (c - 1) as usize } else { 0 };
                    (row_idx, col_idx)
                }
                _ => {
                    // Auto placement
                    let placement = (cursor_row, cursor_col);

                    // Advance cursor based on auto-flow
                    match container.auto_flow {
                        GridAutoFlow::Row | GridAutoFlow::RowDense => {
                            cursor_col += 1;
                            if cursor_col >= col_count {
                                cursor_col = 0;
                                cursor_row += 1;
                            }
                        }
                        GridAutoFlow::Column | GridAutoFlow::ColumnDense => {
                            cursor_row += 1;
                            if cursor_row >= row_count {
                                cursor_row = 0;
                                cursor_col += 1;
                            }
                        }
                    }

                    placement
                }
            };

            // Ensure within bounds
            if row >= row_count || col >= col_count {
                continue;
            }

            // Calculate position
            let x = column_sizes[..col].iter().sum::<f32>() + (col as f32) * col_gap;
            let y = row_sizes[..row].iter().sum::<f32>() + (row as f32) * row_gap;

            // Calculate size (for now, single cell)
            let width = column_sizes[col];
            let height = row_sizes[row];

            layouts.push(GridItemLayout::new(x, y, width, height, row, col, 1, 1));
        }

        layouts
    }
}

impl Default for BasicGridLayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GridLayoutEngine for BasicGridLayoutEngine {
    fn resolve_track_sizes(
        &self,
        tracks: &[TrackSizing],
        available_size: f32,
        gap: f32,
    ) -> Vec<f32> {
        if tracks.is_empty() {
            return Vec::new();
        }

        // Calculate gap space
        let gap_count = tracks.len().saturating_sub(1);
        let total_gap = (gap_count as f32) * gap;

        // Calculate fixed space
        let fixed_space = self.calculate_fixed_space(tracks);

        // Calculate total fr units
        let total_fr = self.total_fr_units(tracks);

        // Remaining space for flexible tracks
        let remaining_space = (available_size - total_gap - fixed_space).max(0.0);

        // Calculate fr unit value
        let fr_value = if total_fr > 0.0 {
            remaining_space / total_fr
        } else {
            0.0
        };

        // Resolve each track
        tracks
            .iter()
            .map(|track| match track {
                TrackSizing::Fixed(length) => {
                    if length.unit() == LengthUnit::Px {
                        length.value()
                    } else {
                        0.0 // Unsupported unit for now
                    }
                }
                TrackSizing::Flexible(fr) => fr * fr_value,
                TrackSizing::Auto => 0.0, // TODO: Implement auto sizing
                TrackSizing::MinContent => 0.0, // TODO: Implement min-content
                TrackSizing::MaxContent => 0.0, // TODO: Implement max-content
            })
            .collect()
    }

    fn compute_grid_layout(
        &self,
        container: &GridContainer,
        items: &[GridItem],
        available_space: (f32, f32),
    ) -> GridLayout {
        let (width, height) = available_space;

        // Resolve track sizes
        let column_sizes = self.resolve_track_sizes(
            container.template_columns(),
            width,
            container.effective_column_gap(),
        );

        let row_sizes = self.resolve_track_sizes(
            container.template_rows(),
            height,
            container.effective_row_gap(),
        );

        // Place items
        let item_layouts = self.auto_place_items(items, &row_sizes, &column_sizes, container);

        GridLayout::new(item_layouts, available_space)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_auto_flow_default() {
        assert_eq!(GridAutoFlow::default(), GridAutoFlow::Row);
    }

    #[test]
    fn test_grid_line_default() {
        assert_eq!(GridLine::default(), GridLine::Auto);
    }

    #[test]
    fn test_container_effective_gaps() {
        let mut container = GridContainer::new();

        // No gaps set
        assert_eq!(container.effective_row_gap(), 0.0);
        assert_eq!(container.effective_column_gap(), 0.0);

        // Set gap
        container.set_gap(Some(10.0));
        assert_eq!(container.effective_row_gap(), 10.0);
        assert_eq!(container.effective_column_gap(), 10.0);

        // Override with specific gaps
        container.set_row_gap(Some(15.0));
        container.set_column_gap(Some(20.0));
        assert_eq!(container.effective_row_gap(), 15.0);
        assert_eq!(container.effective_column_gap(), 20.0);
    }
}
