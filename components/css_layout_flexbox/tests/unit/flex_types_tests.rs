//! Unit tests for flexbox type definitions

use css_layout_flexbox::*;
use css_types::{Length, LengthUnit};

// ============================================================================
// FlexDirection Tests
// ============================================================================

#[test]
fn test_flex_direction_variants() {
    // Test that all FlexDirection variants exist
    let _row = FlexDirection::Row;
    let _row_reverse = FlexDirection::RowReverse;
    let _column = FlexDirection::Column;
    let _column_reverse = FlexDirection::ColumnReverse;
}

#[test]
fn test_flex_direction_equality() {
    assert_eq!(FlexDirection::Row, FlexDirection::Row);
    assert_ne!(FlexDirection::Row, FlexDirection::Column);
}

#[test]
fn test_flex_direction_default() {
    assert_eq!(FlexDirection::default(), FlexDirection::Row);
}

// ============================================================================
// FlexWrap Tests
// ============================================================================

#[test]
fn test_flex_wrap_variants() {
    let _nowrap = FlexWrap::NoWrap;
    let _wrap = FlexWrap::Wrap;
    let _wrap_reverse = FlexWrap::WrapReverse;
}

#[test]
fn test_flex_wrap_equality() {
    assert_eq!(FlexWrap::NoWrap, FlexWrap::NoWrap);
    assert_ne!(FlexWrap::Wrap, FlexWrap::WrapReverse);
}

#[test]
fn test_flex_wrap_default() {
    assert_eq!(FlexWrap::default(), FlexWrap::NoWrap);
}

// ============================================================================
// JustifyContent Tests
// ============================================================================

#[test]
fn test_justify_content_variants() {
    let _flex_start = JustifyContent::FlexStart;
    let _flex_end = JustifyContent::FlexEnd;
    let _center = JustifyContent::Center;
    let _space_between = JustifyContent::SpaceBetween;
    let _space_around = JustifyContent::SpaceAround;
    let _space_evenly = JustifyContent::SpaceEvenly;
}

#[test]
fn test_justify_content_default() {
    assert_eq!(JustifyContent::default(), JustifyContent::FlexStart);
}

// ============================================================================
// AlignItems Tests
// ============================================================================

#[test]
fn test_align_items_variants() {
    let _flex_start = AlignItems::FlexStart;
    let _flex_end = AlignItems::FlexEnd;
    let _center = AlignItems::Center;
    let _baseline = AlignItems::Baseline;
    let _stretch = AlignItems::Stretch;
}

#[test]
fn test_align_items_default() {
    assert_eq!(AlignItems::default(), AlignItems::Stretch);
}

// ============================================================================
// AlignContent Tests
// ============================================================================

#[test]
fn test_align_content_variants() {
    let _flex_start = AlignContent::FlexStart;
    let _flex_end = AlignContent::FlexEnd;
    let _center = AlignContent::Center;
    let _space_between = AlignContent::SpaceBetween;
    let _space_around = AlignContent::SpaceAround;
    let _stretch = AlignContent::Stretch;
}

#[test]
fn test_align_content_default() {
    assert_eq!(AlignContent::default(), AlignContent::Stretch);
}

// ============================================================================
// FlexContainer Tests
// ============================================================================

#[test]
fn test_flex_container_creation() {
    let container = FlexContainer::new();
    assert_eq!(container.direction(), FlexDirection::Row);
    assert_eq!(container.wrap(), FlexWrap::NoWrap);
    assert_eq!(container.justify_content(), JustifyContent::FlexStart);
    assert_eq!(container.align_items(), AlignItems::Stretch);
    assert_eq!(container.align_content(), AlignContent::Stretch);
}

#[test]
fn test_flex_container_with_direction() {
    let container = FlexContainer::new().with_direction(FlexDirection::Column);
    assert_eq!(container.direction(), FlexDirection::Column);
}

#[test]
fn test_flex_container_with_wrap() {
    let container = FlexContainer::new().with_wrap(FlexWrap::Wrap);
    assert_eq!(container.wrap(), FlexWrap::Wrap);
}

#[test]
fn test_flex_container_with_justify_content() {
    let container = FlexContainer::new().with_justify_content(JustifyContent::Center);
    assert_eq!(container.justify_content(), JustifyContent::Center);
}

#[test]
fn test_flex_container_with_align_items() {
    let container = FlexContainer::new().with_align_items(AlignItems::Center);
    assert_eq!(container.align_items(), AlignItems::Center);
}

#[test]
fn test_flex_container_with_align_content() {
    let container = FlexContainer::new().with_align_content(AlignContent::SpaceBetween);
    assert_eq!(container.align_content(), AlignContent::SpaceBetween);
}

#[test]
fn test_flex_container_with_gap() {
    let container = FlexContainer::new().with_gap(10.0);
    assert_eq!(container.gap(), Some(10.0));
}

#[test]
fn test_flex_container_with_row_gap() {
    let container = FlexContainer::new().with_row_gap(15.0);
    assert_eq!(container.row_gap(), Some(15.0));
}

#[test]
fn test_flex_container_with_column_gap() {
    let container = FlexContainer::new().with_column_gap(20.0);
    assert_eq!(container.column_gap(), Some(20.0));
}

#[test]
fn test_flex_container_effective_gaps() {
    // When gap is set, row_gap and column_gap default to gap
    let container = FlexContainer::new().with_gap(10.0);
    assert_eq!(container.effective_row_gap(), 10.0);
    assert_eq!(container.effective_column_gap(), 10.0);

    // When row_gap/column_gap are explicitly set, they override gap
    let container = FlexContainer::new()
        .with_gap(10.0)
        .with_row_gap(15.0)
        .with_column_gap(20.0);
    assert_eq!(container.effective_row_gap(), 15.0);
    assert_eq!(container.effective_column_gap(), 20.0);

    // When no gaps are set, defaults to 0
    let container = FlexContainer::new();
    assert_eq!(container.effective_row_gap(), 0.0);
    assert_eq!(container.effective_column_gap(), 0.0);
}

// ============================================================================
// FlexItem Tests
// ============================================================================

#[test]
fn test_flex_item_creation() {
    let item = FlexItem::new(100.0, 50.0);
    assert_eq!(item.width(), 100.0);
    assert_eq!(item.height(), 50.0);
    assert_eq!(item.flex_grow(), 0.0);
    assert_eq!(item.flex_shrink(), 1.0);
    assert_eq!(item.flex_basis(), None);
    assert_eq!(item.align_self(), None);
    assert_eq!(item.order(), 0);
}

#[test]
fn test_flex_item_with_flex_grow() {
    let item = FlexItem::new(100.0, 50.0).with_flex_grow(2.0);
    assert_eq!(item.flex_grow(), 2.0);
}

#[test]
fn test_flex_item_with_flex_shrink() {
    let item = FlexItem::new(100.0, 50.0).with_flex_shrink(0.5);
    assert_eq!(item.flex_shrink(), 0.5);
}

#[test]
fn test_flex_item_with_flex_basis() {
    let basis = Length::new(200.0, LengthUnit::Px);
    let item = FlexItem::new(100.0, 50.0).with_flex_basis(basis);
    assert_eq!(item.flex_basis(), Some(basis));
}

#[test]
fn test_flex_item_with_align_self() {
    let item = FlexItem::new(100.0, 50.0).with_align_self(AlignItems::Center);
    assert_eq!(item.align_self(), Some(AlignItems::Center));
}

#[test]
fn test_flex_item_with_order() {
    let item = FlexItem::new(100.0, 50.0).with_order(5);
    assert_eq!(item.order(), 5);
}

// ============================================================================
// FlexItemLayout Tests
// ============================================================================

#[test]
fn test_flex_item_layout_creation() {
    let layout = FlexItemLayout::new(10.0, 20.0, 100.0, 50.0);
    assert_eq!(layout.x(), 10.0);
    assert_eq!(layout.y(), 20.0);
    assert_eq!(layout.width(), 100.0);
    assert_eq!(layout.height(), 50.0);
}

// ============================================================================
// FlexLayout Tests
// ============================================================================

#[test]
fn test_flex_layout_creation() {
    let items = vec![
        FlexItemLayout::new(0.0, 0.0, 100.0, 50.0),
        FlexItemLayout::new(100.0, 0.0, 100.0, 50.0),
    ];
    let layout = FlexLayout::new(items.clone(), (200.0, 50.0));

    assert_eq!(layout.items().len(), 2);
    assert_eq!(layout.container_size(), (200.0, 50.0));
}
