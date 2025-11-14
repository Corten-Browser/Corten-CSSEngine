//! Unit tests for GridLayout computation

use css_layout_grid::{
    BasicGridLayoutEngine, GridContainer, GridItem, GridItemLayout, GridLayout, GridLayoutEngine,
    GridLine, TrackSizing,
};
use css_types::{Length, LengthUnit};

// ============================================================================
// GridItemLayout Tests
// ============================================================================

#[test]
fn test_grid_item_layout_creation() {
    let layout = GridItemLayout::new(10.0, 20.0, 100.0, 50.0, 0, 0, 1, 1);

    assert_eq!(layout.x(), 10.0);
    assert_eq!(layout.y(), 20.0);
    assert_eq!(layout.width(), 100.0);
    assert_eq!(layout.height(), 50.0);
    assert_eq!(layout.row(), 0);
    assert_eq!(layout.column(), 0);
    assert_eq!(layout.row_span(), 1);
    assert_eq!(layout.column_span(), 1);
}

// ============================================================================
// GridLayout Tests
// ============================================================================

#[test]
fn test_grid_layout_creation() {
    let items = vec![GridItemLayout::new(0.0, 0.0, 100.0, 100.0, 0, 0, 1, 1)];
    let layout = GridLayout::new(items, (400.0, 300.0));

    assert_eq!(layout.items().len(), 1);
    assert_eq!(layout.container_size(), (400.0, 300.0));
}

// ============================================================================
// Track Sizing Resolution Tests
// ============================================================================

#[test]
fn test_resolve_fixed_tracks() {
    let engine = BasicGridLayoutEngine::new();
    let tracks = vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(200.0, LengthUnit::Px)),
    ];

    let sizes = engine.resolve_track_sizes(&tracks, 400.0, 0.0);

    assert_eq!(sizes.len(), 2);
    assert_eq!(sizes[0], 100.0);
    assert_eq!(sizes[1], 200.0);
}

#[test]
fn test_resolve_flexible_tracks() {
    let engine = BasicGridLayoutEngine::new();
    let tracks = vec![TrackSizing::Flexible(1.0), TrackSizing::Flexible(2.0)];

    let sizes = engine.resolve_track_sizes(&tracks, 300.0, 0.0);

    assert_eq!(sizes.len(), 2);
    // 1fr gets 100px, 2fr gets 200px
    assert_eq!(sizes[0], 100.0);
    assert_eq!(sizes[1], 200.0);
}

#[test]
fn test_resolve_mixed_tracks() {
    let engine = BasicGridLayoutEngine::new();
    let tracks = vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Flexible(1.0),
        TrackSizing::Flexible(1.0),
    ];

    let sizes = engine.resolve_track_sizes(&tracks, 400.0, 0.0);

    assert_eq!(sizes.len(), 3);
    assert_eq!(sizes[0], 100.0);
    // Remaining 300px split between 2 fr units (150px each)
    assert_eq!(sizes[1], 150.0);
    assert_eq!(sizes[2], 150.0);
}

#[test]
fn test_resolve_tracks_with_gap() {
    let engine = BasicGridLayoutEngine::new();
    let tracks = vec![TrackSizing::Flexible(1.0), TrackSizing::Flexible(1.0)];

    let sizes = engine.resolve_track_sizes(&tracks, 300.0, 20.0);

    assert_eq!(sizes.len(), 2);
    // 300px - 20px gap = 280px, split into 2 tracks (140px each)
    assert_eq!(sizes[0], 140.0);
    assert_eq!(sizes[1], 140.0);
}

// ============================================================================
// Basic Grid Layout Tests
// ============================================================================

#[test]
fn test_simple_grid_layout() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px))]);
    container.set_template_columns(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);

    let items = vec![GridItem::new(), GridItem::new()];

    let layout = engine.compute_grid_layout(&container, &items, (400.0, 300.0));

    assert_eq!(layout.items().len(), 2);
    assert_eq!(layout.container_size(), (400.0, 300.0));

    // First item should be at (0, 0)
    assert_eq!(layout.items()[0].x(), 0.0);
    assert_eq!(layout.items()[0].y(), 0.0);
    assert_eq!(layout.items()[0].width(), 100.0);
    assert_eq!(layout.items()[0].height(), 100.0);

    // Second item should be at (100, 0) with auto-flow row
    assert_eq!(layout.items()[1].x(), 100.0);
    assert_eq!(layout.items()[1].y(), 0.0);
}

#[test]
fn test_grid_layout_with_gap() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px))]);
    container.set_template_columns(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);
    container.set_gap(Some(10.0));

    let items = vec![GridItem::new(), GridItem::new()];

    let layout = engine.compute_grid_layout(&container, &items, (400.0, 300.0));

    // Second item should be at (110, 0) because of 10px gap
    assert_eq!(layout.items()[1].x(), 110.0);
}

#[test]
fn test_grid_layout_explicit_placement() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);
    container.set_template_columns(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);

    let mut item = GridItem::new();
    item.set_row_start(GridLine::LineNumber(2));
    item.set_column_start(GridLine::LineNumber(2));

    let items = vec![item];

    let layout = engine.compute_grid_layout(&container, &items, (400.0, 300.0));

    // Item should be at (100, 100) - second row, second column
    assert_eq!(layout.items()[0].x(), 100.0);
    assert_eq!(layout.items()[0].y(), 100.0);
    assert_eq!(layout.items()[0].row(), 1);
    assert_eq!(layout.items()[0].column(), 1);
}
