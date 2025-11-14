//! Unit tests for GridContainer and GridItem

use css_layout_grid::{GridAutoFlow, GridContainer, GridItem, GridLine, TrackSizing};
use css_types::{Length, LengthUnit};

// ============================================================================
// GridContainer Tests
// ============================================================================

#[test]
fn test_grid_container_default() {
    let container = GridContainer::new();

    assert_eq!(container.template_rows().len(), 0);
    assert_eq!(container.template_columns().len(), 0);
    assert_eq!(container.auto_flow(), GridAutoFlow::Row);
    assert_eq!(container.gap(), None);
}

#[test]
fn test_grid_container_with_template_rows() {
    let mut container = GridContainer::new();
    let rows = vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Flexible(1.0),
    ];

    container.set_template_rows(rows.clone());
    assert_eq!(container.template_rows().len(), 2);
}

#[test]
fn test_grid_container_with_template_columns() {
    let mut container = GridContainer::new();
    let cols = vec![
        TrackSizing::Flexible(1.0),
        TrackSizing::Flexible(2.0),
        TrackSizing::Flexible(1.0),
    ];

    container.set_template_columns(cols.clone());
    assert_eq!(container.template_columns().len(), 3);
}

#[test]
fn test_grid_container_with_auto_flow() {
    let mut container = GridContainer::new();
    container.set_auto_flow(GridAutoFlow::ColumnDense);

    assert_eq!(container.auto_flow(), GridAutoFlow::ColumnDense);
}

#[test]
fn test_grid_container_with_gap() {
    let mut container = GridContainer::new();
    container.set_gap(Some(10.0));

    assert_eq!(container.gap(), Some(10.0));
}

#[test]
fn test_grid_container_with_row_and_column_gap() {
    let mut container = GridContainer::new();
    container.set_row_gap(Some(15.0));
    container.set_column_gap(Some(20.0));

    assert_eq!(container.row_gap(), Some(15.0));
    assert_eq!(container.column_gap(), Some(20.0));
}

// ============================================================================
// GridItem Tests
// ============================================================================

#[test]
fn test_grid_item_default() {
    let item = GridItem::new();

    assert_eq!(item.row_start(), GridLine::Auto);
    assert_eq!(item.row_end(), GridLine::Auto);
    assert_eq!(item.column_start(), GridLine::Auto);
    assert_eq!(item.column_end(), GridLine::Auto);
}

#[test]
fn test_grid_item_with_explicit_placement() {
    let mut item = GridItem::new();
    item.set_row_start(GridLine::LineNumber(1));
    item.set_row_end(GridLine::LineNumber(3));
    item.set_column_start(GridLine::LineNumber(2));
    item.set_column_end(GridLine::Span(2));

    assert_eq!(item.row_start(), GridLine::LineNumber(1));
    assert_eq!(item.row_end(), GridLine::LineNumber(3));
    assert_eq!(item.column_start(), GridLine::LineNumber(2));
}

#[test]
fn test_grid_item_with_span() {
    let mut item = GridItem::new();
    item.set_row_end(GridLine::Span(3));
    item.set_column_end(GridLine::Span(2));

    match item.row_end() {
        GridLine::Span(n) => assert_eq!(n, 3),
        _ => panic!("Expected Span variant"),
    }

    match item.column_end() {
        GridLine::Span(n) => assert_eq!(n, 2),
        _ => panic!("Expected Span variant"),
    }
}
