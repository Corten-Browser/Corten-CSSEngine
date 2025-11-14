//! Unit tests for flexbox layout engine

use css_layout_flexbox::*;

// ============================================================================
// Basic Layout Tests
// ============================================================================

#[test]
fn test_simple_row_layout() {
    let engine = DefaultFlexLayoutEngine;
    // Use FlexStart to prevent stretching
    let container = FlexContainer::new().with_align_items(AlignItems::FlexStart);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    // Verify container size
    assert_eq!(layout.container_size(), (400.0, 200.0));

    // Verify item count
    assert_eq!(layout.items().len(), 3);

    // Verify items are laid out horizontally
    let item_layouts = layout.items();
    assert_eq!(item_layouts[0].x(), 0.0);
    assert_eq!(item_layouts[1].x(), 100.0);
    assert_eq!(item_layouts[2].x(), 200.0);

    // All items should have same y position (aligned to start)
    assert_eq!(item_layouts[0].y(), 0.0);
    assert_eq!(item_layouts[1].y(), 0.0);
    assert_eq!(item_layouts[2].y(), 0.0);

    // Verify dimensions are preserved (no stretching)
    assert_eq!(item_layouts[0].width(), 100.0);
    assert_eq!(item_layouts[0].height(), 50.0);
}

#[test]
fn test_simple_column_layout() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_direction(FlexDirection::Column);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    // Verify items are laid out vertically
    let item_layouts = layout.items();
    assert_eq!(item_layouts[0].y(), 0.0);
    assert_eq!(item_layouts[1].y(), 50.0);
    assert_eq!(item_layouts[2].y(), 100.0);

    // All items should have same x position
    assert_eq!(item_layouts[0].x(), 0.0);
    assert_eq!(item_layouts[1].x(), 0.0);
    assert_eq!(item_layouts[2].x(), 0.0);
}

// ============================================================================
// Gap Tests
// ============================================================================

#[test]
fn test_row_layout_with_gap() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_gap(10.0);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // First item at 0
    assert_eq!(item_layouts[0].x(), 0.0);
    // Second item at 100 + 10 (gap)
    assert_eq!(item_layouts[1].x(), 110.0);
    // Third item at 210 + 10 (gap)
    assert_eq!(item_layouts[2].x(), 220.0);
}

#[test]
fn test_column_layout_with_gap() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new()
        .with_direction(FlexDirection::Column)
        .with_gap(10.0);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    assert_eq!(item_layouts[0].y(), 0.0);
    assert_eq!(item_layouts[1].y(), 60.0); // 50 + 10 gap
    assert_eq!(item_layouts[2].y(), 120.0); // 110 + 10 gap
}

// ============================================================================
// Justify Content Tests
// ============================================================================

#[test]
fn test_justify_content_flex_end() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_justify_content(JustifyContent::FlexEnd);
    let items = vec![FlexItem::new(100.0, 50.0), FlexItem::new(100.0, 50.0)];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Items should be at the end: total width = 200, container = 400
    // So they should start at 200
    assert_eq!(item_layouts[0].x(), 200.0);
    assert_eq!(item_layouts[1].x(), 300.0);
}

#[test]
fn test_justify_content_center() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_justify_content(JustifyContent::Center);
    let items = vec![FlexItem::new(100.0, 50.0), FlexItem::new(100.0, 50.0)];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Items should be centered: total width = 200, container = 400
    // Center offset = (400 - 200) / 2 = 100
    assert_eq!(item_layouts[0].x(), 100.0);
    assert_eq!(item_layouts[1].x(), 200.0);
}

#[test]
fn test_justify_content_space_between() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_justify_content(JustifyContent::SpaceBetween);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Space between: first at 0, last at end (300), middle evenly spaced
    // Space = (400 - 300) / 2 = 50
    assert_eq!(item_layouts[0].x(), 0.0);
    assert_eq!(item_layouts[1].x(), 150.0); // 100 + 50
    assert_eq!(item_layouts[2].x(), 300.0); // 250 + 50
}

#[test]
fn test_justify_content_space_around() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_justify_content(JustifyContent::SpaceAround);
    let items = vec![FlexItem::new(100.0, 50.0), FlexItem::new(100.0, 50.0)];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Space around: (400 - 200) / 4 = 50 on each side of each item
    assert_eq!(item_layouts[0].x(), 50.0);
    assert_eq!(item_layouts[1].x(), 250.0); // 150 + 100
}

#[test]
fn test_justify_content_space_evenly() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_justify_content(JustifyContent::SpaceEvenly);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (500.0, 200.0));

    let item_layouts = layout.items();
    // Space evenly: (500 - 300) / 4 spaces = 50 per space
    assert_eq!(item_layouts[0].x(), 50.0);
    assert_eq!(item_layouts[1].x(), 200.0); // 50 + 100 + 50
    assert_eq!(item_layouts[2].x(), 350.0); // 200 + 100 + 50
}

// ============================================================================
// Align Items Tests
// ============================================================================

#[test]
fn test_align_items_flex_start() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_align_items(AlignItems::FlexStart);
    let items = vec![
        FlexItem::new(100.0, 30.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 40.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // All items should be aligned to top (y=0)
    assert_eq!(item_layouts[0].y(), 0.0);
    assert_eq!(item_layouts[1].y(), 0.0);
    assert_eq!(item_layouts[2].y(), 0.0);
}

#[test]
fn test_align_items_flex_end() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_align_items(AlignItems::FlexEnd);
    let items = vec![
        FlexItem::new(100.0, 30.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 40.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Items should be aligned to bottom (200 - height)
    assert_eq!(item_layouts[0].y(), 170.0); // 200 - 30
    assert_eq!(item_layouts[1].y(), 150.0); // 200 - 50
    assert_eq!(item_layouts[2].y(), 160.0); // 200 - 40
}

#[test]
fn test_align_items_center() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_align_items(AlignItems::Center);
    let items = vec![
        FlexItem::new(100.0, 30.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 40.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Items should be centered: (200 - height) / 2
    assert_eq!(item_layouts[0].y(), 85.0); // (200 - 30) / 2
    assert_eq!(item_layouts[1].y(), 75.0); // (200 - 50) / 2
    assert_eq!(item_layouts[2].y(), 80.0); // (200 - 40) / 2
}

#[test]
fn test_align_items_stretch() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_align_items(AlignItems::Stretch);
    let items = vec![FlexItem::new(100.0, 30.0), FlexItem::new(100.0, 50.0)];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Items should be stretched to container height
    assert_eq!(item_layouts[0].height(), 200.0);
    assert_eq!(item_layouts[1].height(), 200.0);
    assert_eq!(item_layouts[0].y(), 0.0);
    assert_eq!(item_layouts[1].y(), 0.0);
}

// ============================================================================
// Flex Grow/Shrink Tests
// ============================================================================

#[test]
fn test_flex_grow() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new();
    let items = vec![
        FlexItem::new(100.0, 50.0).with_flex_grow(1.0),
        FlexItem::new(100.0, 50.0).with_flex_grow(2.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (600.0, 200.0));

    let item_layouts = layout.items();
    // Available space = 600 - 200 = 400
    // Total grow = 3.0
    // Item 0 gets 100 + (400 * 1/3) = 233.33...
    // Item 1 gets 100 + (400 * 2/3) = 366.66...
    assert!((item_layouts[0].width() - 233.33).abs() < 0.1);
    assert!((item_layouts[1].width() - 366.67).abs() < 0.1);
}

#[test]
fn test_flex_shrink() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new();
    let items = vec![
        FlexItem::new(200.0, 50.0).with_flex_shrink(1.0),
        FlexItem::new(200.0, 50.0).with_flex_shrink(2.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (300.0, 200.0));

    let item_layouts = layout.items();
    // Overflow = 400 - 300 = 100
    // Total shrink weight = 1 + 2 = 3
    // Item 0 shrinks by 100 * 1/3 = 33.33... -> 166.67
    // Item 1 shrinks by 100 * 2/3 = 66.67... -> 133.33
    assert!((item_layouts[0].width() - 166.67).abs() < 0.1);
    assert!((item_layouts[1].width() - 133.33).abs() < 0.1);
}

// ============================================================================
// Order Tests
// ============================================================================

#[test]
fn test_item_order() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_align_items(AlignItems::FlexStart);
    let items = vec![
        FlexItem::new(100.0, 50.0).with_order(2),
        FlexItem::new(100.0, 50.0).with_order(1),
        FlexItem::new(100.0, 50.0).with_order(0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    // Note: The layout engine should preserve input order but respect visual ordering
    // Items are laid out in visual order (sorted by order property)
    let item_layouts = layout.items();

    // Items are returned in original order but positioned by their order property
    // Item 0 (order 2) should be at x=200
    // Item 1 (order 1) should be at x=100
    // Item 2 (order 0) should be at x=0
    assert_eq!(item_layouts[0].x(), 200.0);
    assert_eq!(item_layouts[1].x(), 100.0);
    assert_eq!(item_layouts[2].x(), 0.0);
}

// ============================================================================
// Reverse Direction Tests
// ============================================================================

#[test]
fn test_row_reverse() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_direction(FlexDirection::RowReverse);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Items should be laid out right to left
    assert_eq!(item_layouts[0].x(), 300.0); // Last position
    assert_eq!(item_layouts[1].x(), 200.0); // Middle
    assert_eq!(item_layouts[2].x(), 100.0); // First position (from right)
}

#[test]
fn test_column_reverse() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_direction(FlexDirection::ColumnReverse);
    let items = vec![
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
        FlexItem::new(100.0, 50.0),
    ];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    // Items should be laid out bottom to top
    assert_eq!(item_layouts[0].y(), 150.0); // Last position (from bottom)
    assert_eq!(item_layouts[1].y(), 100.0); // Middle
    assert_eq!(item_layouts[2].y(), 50.0); // First position (from bottom)
}

// ============================================================================
// Empty Container Tests
// ============================================================================

#[test]
fn test_empty_container() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new();
    let items = vec![];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    assert_eq!(layout.items().len(), 0);
    assert_eq!(layout.container_size(), (400.0, 200.0));
}

#[test]
fn test_single_item() {
    let engine = DefaultFlexLayoutEngine;
    let container = FlexContainer::new().with_align_items(AlignItems::FlexStart);
    let items = vec![FlexItem::new(100.0, 50.0)];

    let layout = engine.compute_flex_layout(&container, &items, (400.0, 200.0));

    let item_layouts = layout.items();
    assert_eq!(item_layouts.len(), 1);
    assert_eq!(item_layouts[0].x(), 0.0);
    assert_eq!(item_layouts[0].y(), 0.0);
    assert_eq!(item_layouts[0].width(), 100.0);
    assert_eq!(item_layouts[0].height(), 50.0);
}
