//! CSS Flexbox Layout Engine
//!
//! This crate provides CSS Flexbox layout computation including:
//! - Flex container properties (direction, wrap, justify-content, align-items, align-content)
//! - Flex item properties (flex-grow, flex-shrink, flex-basis, align-self, order)
//! - Flexbox layout algorithm implementing CSS Flexbox specification
//! - Gap properties support (gap, row-gap, column-gap)

use css_types::Length;

// ============================================================================
// Core Enums
// ============================================================================

/// Flex container direction
///
/// Defines the main axis direction for flex items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDirection {
    /// Main axis is horizontal, left to right
    #[default]
    Row,
    /// Main axis is horizontal, right to left
    RowReverse,
    /// Main axis is vertical, top to bottom
    Column,
    /// Main axis is vertical, bottom to top
    ColumnReverse,
}

/// Flex wrap behavior
///
/// Determines whether flex items wrap to multiple lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexWrap {
    /// Single line, items may overflow
    #[default]
    NoWrap,
    /// Multi-line, items wrap top to bottom
    Wrap,
    /// Multi-line, items wrap bottom to top
    WrapReverse,
}

/// Main axis alignment
///
/// Defines how flex items are aligned along the main axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JustifyContent {
    /// Items packed at start of line
    #[default]
    FlexStart,
    /// Items packed at end of line
    FlexEnd,
    /// Items centered along the line
    Center,
    /// Items evenly distributed, first/last at edges
    SpaceBetween,
    /// Items evenly distributed with equal space around
    SpaceAround,
    /// Items evenly distributed with equal space between
    SpaceEvenly,
}

/// Cross axis alignment
///
/// Defines how flex items are aligned along the cross axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignItems {
    /// Items aligned at cross-start
    FlexStart,
    /// Items aligned at cross-end
    FlexEnd,
    /// Items centered along cross axis
    Center,
    /// Items aligned along baseline
    Baseline,
    /// Items stretched to fill container
    #[default]
    Stretch,
}

/// Multi-line cross axis alignment
///
/// Defines how multiple lines are aligned along the cross axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignContent {
    /// Lines packed at cross-start
    FlexStart,
    /// Lines packed at cross-end
    FlexEnd,
    /// Lines centered along cross axis
    Center,
    /// Lines evenly distributed, first/last at edges
    SpaceBetween,
    /// Lines evenly distributed with equal space around
    SpaceAround,
    /// Lines stretched to fill container
    #[default]
    Stretch,
}

// ============================================================================
// Flex Container
// ============================================================================

/// Flexbox container properties
///
/// Represents all properties that control flexbox layout behavior.
///
/// # Examples
/// ```
/// use css_layout_flexbox::{FlexContainer, FlexDirection, JustifyContent};
///
/// let container = FlexContainer::new()
///     .with_direction(FlexDirection::Column)
///     .with_justify_content(JustifyContent::Center)
///     .with_gap(10.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FlexContainer {
    direction: FlexDirection,
    wrap: FlexWrap,
    justify_content: JustifyContent,
    align_items: AlignItems,
    align_content: AlignContent,
    gap: Option<f32>,
    row_gap: Option<f32>,
    column_gap: Option<f32>,
}

impl FlexContainer {
    /// Create a new flex container with default properties
    ///
    /// # Examples
    /// ```
    /// use css_layout_flexbox::FlexContainer;
    ///
    /// let container = FlexContainer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            direction: FlexDirection::default(),
            wrap: FlexWrap::default(),
            justify_content: JustifyContent::default(),
            align_items: AlignItems::default(),
            align_content: AlignContent::default(),
            gap: None,
            row_gap: None,
            column_gap: None,
        }
    }

    /// Set the flex direction
    pub fn with_direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the flex wrap behavior
    pub fn with_wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Set the justify-content alignment
    pub fn with_justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    /// Set the align-items alignment
    pub fn with_align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }

    /// Set the align-content alignment
    pub fn with_align_content(mut self, align_content: AlignContent) -> Self {
        self.align_content = align_content;
        self
    }

    /// Set the gap (shorthand for row-gap and column-gap)
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Set the row gap
    pub fn with_row_gap(mut self, row_gap: f32) -> Self {
        self.row_gap = Some(row_gap);
        self
    }

    /// Set the column gap
    pub fn with_column_gap(mut self, column_gap: f32) -> Self {
        self.column_gap = Some(column_gap);
        self
    }

    /// Get the flex direction
    pub fn direction(&self) -> FlexDirection {
        self.direction
    }

    /// Get the flex wrap behavior
    pub fn wrap(&self) -> FlexWrap {
        self.wrap
    }

    /// Get the justify-content alignment
    pub fn justify_content(&self) -> JustifyContent {
        self.justify_content
    }

    /// Get the align-items alignment
    pub fn align_items(&self) -> AlignItems {
        self.align_items
    }

    /// Get the align-content alignment
    pub fn align_content(&self) -> AlignContent {
        self.align_content
    }

    /// Get the gap value
    pub fn gap(&self) -> Option<f32> {
        self.gap
    }

    /// Get the row gap value
    pub fn row_gap(&self) -> Option<f32> {
        self.row_gap
    }

    /// Get the column gap value
    pub fn column_gap(&self) -> Option<f32> {
        self.column_gap
    }

    /// Get the effective row gap (row_gap or gap or 0)
    pub fn effective_row_gap(&self) -> f32 {
        self.row_gap.or(self.gap).unwrap_or(0.0)
    }

    /// Get the effective column gap (column_gap or gap or 0)
    pub fn effective_column_gap(&self) -> f32 {
        self.column_gap.or(self.gap).unwrap_or(0.0)
    }
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Flex Item
// ============================================================================

/// Flexbox item properties
///
/// Represents properties that control individual flex item behavior.
///
/// # Examples
/// ```
/// use css_layout_flexbox::{FlexItem, AlignItems};
/// use css_types::{Length, LengthUnit};
///
/// let item = FlexItem::new(100.0, 50.0)
///     .with_flex_grow(1.0)
///     .with_flex_shrink(0.0)
///     .with_align_self(AlignItems::Center);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FlexItem {
    width: f32,
    height: f32,
    flex_grow: f32,
    flex_shrink: f32,
    flex_basis: Option<Length>,
    align_self: Option<AlignItems>,
    order: i32,
}

impl FlexItem {
    /// Create a new flex item with given dimensions
    ///
    /// # Arguments
    /// * `width` - Initial width in pixels
    /// * `height` - Initial height in pixels
    ///
    /// # Examples
    /// ```
    /// use css_layout_flexbox::FlexItem;
    ///
    /// let item = FlexItem::new(100.0, 50.0);
    /// ```
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            align_self: None,
            order: 0,
        }
    }

    /// Set the flex-grow factor
    pub fn with_flex_grow(mut self, flex_grow: f32) -> Self {
        self.flex_grow = flex_grow;
        self
    }

    /// Set the flex-shrink factor
    pub fn with_flex_shrink(mut self, flex_shrink: f32) -> Self {
        self.flex_shrink = flex_shrink;
        self
    }

    /// Set the flex-basis
    pub fn with_flex_basis(mut self, flex_basis: Length) -> Self {
        self.flex_basis = Some(flex_basis);
        self
    }

    /// Set the align-self property
    pub fn with_align_self(mut self, align_self: AlignItems) -> Self {
        self.align_self = Some(align_self);
        self
    }

    /// Set the order property
    pub fn with_order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Get the item width
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get the item height
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Get the flex-grow factor
    pub fn flex_grow(&self) -> f32 {
        self.flex_grow
    }

    /// Get the flex-shrink factor
    pub fn flex_shrink(&self) -> f32 {
        self.flex_shrink
    }

    /// Get the flex-basis
    pub fn flex_basis(&self) -> Option<Length> {
        self.flex_basis
    }

    /// Get the align-self property
    pub fn align_self(&self) -> Option<AlignItems> {
        self.align_self
    }

    /// Get the order property
    pub fn order(&self) -> i32 {
        self.order
    }
}

// ============================================================================
// Layout Results
// ============================================================================

/// Layout position and size for a flex item
///
/// Represents the computed position and dimensions of a single flex item
/// after layout calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexItemLayout {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl FlexItemLayout {
    /// Create a new flex item layout
    ///
    /// # Arguments
    /// * `x` - X position
    /// * `y` - Y position
    /// * `width` - Width
    /// * `height` - Height
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the x position
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Get the y position
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Get the width
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get the height
    pub fn height(&self) -> f32 {
        self.height
    }
}

/// Computed flexbox layout
///
/// Represents the complete layout result including all item positions and sizes.
#[derive(Debug, Clone, PartialEq)]
pub struct FlexLayout {
    items: Vec<FlexItemLayout>,
    container_size: (f32, f32),
}

impl FlexLayout {
    /// Create a new flex layout
    ///
    /// # Arguments
    /// * `items` - Vector of flex item layouts
    /// * `container_size` - Container size as (width, height)
    pub fn new(items: Vec<FlexItemLayout>, container_size: (f32, f32)) -> Self {
        Self {
            items,
            container_size,
        }
    }

    /// Get the item layouts
    pub fn items(&self) -> &[FlexItemLayout] {
        &self.items
    }

    /// Get the container size
    pub fn container_size(&self) -> (f32, f32) {
        self.container_size
    }
}

// ============================================================================
// Flex Layout Engine Trait
// ============================================================================

/// Flexbox layout computation engine
///
/// Trait for implementing flexbox layout algorithms.
pub trait FlexLayoutEngine {
    /// Compute layout for flex container and items
    ///
    /// # Arguments
    /// * `container` - Flex container properties
    /// * `items` - Slice of flex items to layout
    /// * `available_space` - Available space as (width, height)
    ///
    /// # Returns
    /// Complete flex layout with positioned items
    fn compute_flex_layout(
        &self,
        container: &FlexContainer,
        items: &[FlexItem],
        available_space: (f32, f32),
    ) -> FlexLayout;
}

// ============================================================================
// Default Flex Layout Engine Implementation
// ============================================================================

/// Default flexbox layout engine
///
/// Implements CSS Flexbox specification for layout computation.
///
/// # Examples
/// ```
/// use css_layout_flexbox::{DefaultFlexLayoutEngine, FlexContainer, FlexItem, FlexLayoutEngine};
///
/// let engine = DefaultFlexLayoutEngine;
/// let container = FlexContainer::new();
/// let items = vec![FlexItem::new(100.0, 50.0), FlexItem::new(100.0, 50.0)];
/// let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));
/// ```
pub struct DefaultFlexLayoutEngine;

impl FlexLayoutEngine for DefaultFlexLayoutEngine {
    fn compute_flex_layout(
        &self,
        container: &FlexContainer,
        items: &[FlexItem],
        available_space: (f32, f32),
    ) -> FlexLayout {
        if items.is_empty() {
            return FlexLayout::new(vec![], available_space);
        }

        // Sort items by order property
        let mut indexed_items: Vec<(usize, &FlexItem)> = items.iter().enumerate().collect();
        indexed_items.sort_by_key(|(_, item)| item.order());

        let is_row = matches!(
            container.direction(),
            FlexDirection::Row | FlexDirection::RowReverse
        );
        let is_reverse = matches!(
            container.direction(),
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        );

        // Calculate main and cross axis sizes
        let main_size = if is_row {
            available_space.0
        } else {
            available_space.1
        };
        let cross_size = if is_row {
            available_space.1
        } else {
            available_space.0
        };

        // Get gap values
        let gap = if is_row {
            container.effective_column_gap()
        } else {
            container.effective_row_gap()
        };

        // Calculate flex item sizes and positions
        let mut flex_items = compute_flex_sizes(
            &indexed_items,
            main_size,
            gap,
            is_row,
            container.align_items(),
            cross_size,
        );

        // Apply main axis alignment (justify-content)
        apply_justify_content(&mut flex_items, container.justify_content(), main_size, gap);

        // Apply cross axis alignment (align-items)
        apply_align_items(&mut flex_items, container.align_items(), cross_size);

        // Convert to absolute positions based on direction
        let mut item_layouts = vec![FlexItemLayout::new(0.0, 0.0, 0.0, 0.0); items.len()];

        for (original_idx, computed) in flex_items {
            let (x, y) = if is_row {
                let x_pos = if is_reverse {
                    main_size - computed.main_end
                } else {
                    computed.main_start
                };
                (x_pos, computed.cross_start)
            } else {
                let y_pos = if is_reverse {
                    main_size - computed.main_end
                } else {
                    computed.main_start
                };
                (computed.cross_start, y_pos)
            };

            let (width, height) = if is_row {
                (computed.main_size, computed.cross_size)
            } else {
                (computed.cross_size, computed.main_size)
            };

            item_layouts[original_idx] = FlexItemLayout::new(x, y, width, height);
        }

        FlexLayout::new(item_layouts, available_space)
    }
}

// ============================================================================
// Helper Structures and Functions
// ============================================================================

#[derive(Debug, Clone)]
struct ComputedFlexItem {
    main_start: f32,
    main_end: f32,
    main_size: f32,
    cross_start: f32,
    cross_size: f32,
}

fn compute_flex_sizes(
    indexed_items: &[(usize, &FlexItem)],
    main_size: f32,
    gap: f32,
    is_row: bool,
    align_items: AlignItems,
    cross_size: f32,
) -> Vec<(usize, ComputedFlexItem)> {
    let total_gaps = if indexed_items.len() > 1 {
        gap * (indexed_items.len() - 1) as f32
    } else {
        0.0
    };

    // Calculate initial sizes
    let mut total_main_size = 0.0;
    let mut item_main_sizes: Vec<f32> = Vec::new();
    let mut total_grow = 0.0;
    let mut total_shrink_weight = 0.0;

    for (_, item) in indexed_items.iter() {
        let item_main_size = if is_row { item.width() } else { item.height() };
        item_main_sizes.push(item_main_size);
        total_main_size += item_main_size;
        total_grow += item.flex_grow();
        total_shrink_weight += item.flex_shrink();
    }

    let available_main = main_size - total_gaps;
    let free_space = available_main - total_main_size;

    // Apply flex grow or shrink
    if free_space > 0.0 && total_grow > 0.0 {
        // Grow items
        for (i, (_, item)) in indexed_items.iter().enumerate() {
            if item.flex_grow() > 0.0 {
                let grow_amount = free_space * (item.flex_grow() / total_grow);
                item_main_sizes[i] += grow_amount;
            }
        }
    } else if free_space < 0.0 && total_shrink_weight > 0.0 {
        // Shrink items
        let shrink_space = -free_space;
        for (i, (_, item)) in indexed_items.iter().enumerate() {
            if item.flex_shrink() > 0.0 {
                let shrink_amount = shrink_space * (item.flex_shrink() / total_shrink_weight);
                item_main_sizes[i] = (item_main_sizes[i] - shrink_amount).max(0.0);
            }
        }
    }

    // Create computed items
    let mut computed_items = Vec::new();
    let mut main_pos = 0.0;

    for (i, (original_idx, item)) in indexed_items.iter().enumerate() {
        let main_item_size = item_main_sizes[i];
        let cross_item_size = if align_items == AlignItems::Stretch {
            cross_size
        } else if is_row {
            item.height()
        } else {
            item.width()
        };

        computed_items.push((
            *original_idx,
            ComputedFlexItem {
                main_start: main_pos,
                main_end: main_pos + main_item_size,
                main_size: main_item_size,
                cross_start: 0.0,
                cross_size: cross_item_size,
            },
        ));

        main_pos += main_item_size + gap;
    }

    computed_items
}

fn apply_justify_content(
    items: &mut [(usize, ComputedFlexItem)],
    justify: JustifyContent,
    main_size: f32,
    gap: f32,
) {
    if items.is_empty() {
        return;
    }

    let total_item_size: f32 = items.iter().map(|(_, item)| item.main_size).sum();
    let total_gaps = if items.len() > 1 {
        gap * (items.len() - 1) as f32
    } else {
        0.0
    };
    let free_space = main_size - total_item_size - total_gaps;

    match justify {
        JustifyContent::FlexStart => {
            // Items are already positioned from start
        }
        JustifyContent::FlexEnd => {
            // Shift all items to the end
            for (_, item) in items.iter_mut() {
                item.main_start += free_space;
                item.main_end += free_space;
            }
        }
        JustifyContent::Center => {
            // Center items
            let offset = free_space / 2.0;
            for (_, item) in items.iter_mut() {
                item.main_start += offset;
                item.main_end += offset;
            }
        }
        JustifyContent::SpaceBetween => {
            if items.len() > 1 {
                let space = free_space / (items.len() - 1) as f32;
                for (i, (_, item)) in items.iter_mut().enumerate() {
                    let offset = space * i as f32;
                    item.main_start += offset;
                    item.main_end += offset;
                }
            }
        }
        JustifyContent::SpaceAround => {
            let space = free_space / items.len() as f32;
            for (i, (_, item)) in items.iter_mut().enumerate() {
                let offset = space * (i as f32 + 0.5);
                item.main_start += offset;
                item.main_end += offset;
            }
        }
        JustifyContent::SpaceEvenly => {
            let space = free_space / (items.len() + 1) as f32;
            for (i, (_, item)) in items.iter_mut().enumerate() {
                let offset = space * (i + 1) as f32;
                item.main_start += offset;
                item.main_end += offset;
            }
        }
    }
}

fn apply_align_items(items: &mut [(usize, ComputedFlexItem)], align: AlignItems, cross_size: f32) {
    for (_, item) in items.iter_mut() {
        match align {
            AlignItems::FlexStart => {
                item.cross_start = 0.0;
            }
            AlignItems::FlexEnd => {
                item.cross_start = cross_size - item.cross_size;
            }
            AlignItems::Center => {
                item.cross_start = (cross_size - item.cross_size) / 2.0;
            }
            AlignItems::Baseline => {
                // For now, treat as flex-start
                // Full implementation would align by text baseline
                item.cross_start = 0.0;
            }
            AlignItems::Stretch => {
                // Already handled in compute_flex_sizes
                item.cross_start = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_direction_default() {
        assert_eq!(FlexDirection::default(), FlexDirection::Row);
    }

    #[test]
    fn test_flex_container_builder() {
        let container = FlexContainer::new()
            .with_direction(FlexDirection::Column)
            .with_gap(10.0);

        assert_eq!(container.direction(), FlexDirection::Column);
        assert_eq!(container.effective_row_gap(), 10.0);
    }
}
