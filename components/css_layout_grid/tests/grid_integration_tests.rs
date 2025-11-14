//! Integration tests for complete grid layout scenarios

use css_layout_grid::{
    BasicGridLayoutEngine, GridAutoFlow, GridContainer, GridItem, GridLayoutEngine, GridLine,
    TrackSizing,
};
use css_types::{Length, LengthUnit};

// ============================================================================
// Complex Grid Layouts
// ============================================================================

#[test]
fn test_three_column_layout_with_flexible_tracks() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![TrackSizing::Fixed(Length::new(200.0, LengthUnit::Px))]);
    container.set_template_columns(vec![
        TrackSizing::Flexible(1.0),
        TrackSizing::Flexible(2.0),
        TrackSizing::Flexible(1.0),
    ]);

    let items = vec![GridItem::new(), GridItem::new(), GridItem::new()];

    let layout = engine.compute_grid_layout(&container, &items, (800.0, 600.0));

    assert_eq!(layout.items().len(), 3);

    // First column: 200px (1fr)
    assert_eq!(layout.items()[0].width(), 200.0);
    assert_eq!(layout.items()[0].x(), 0.0);

    // Second column: 400px (2fr)
    assert_eq!(layout.items()[1].width(), 400.0);
    assert_eq!(layout.items()[1].x(), 200.0);

    // Third column: 200px (1fr)
    assert_eq!(layout.items()[2].width(), 200.0);
    assert_eq!(layout.items()[2].x(), 600.0);

    // All in same row
    assert_eq!(layout.items()[0].y(), 0.0);
    assert_eq!(layout.items()[1].y(), 0.0);
    assert_eq!(layout.items()[2].y(), 0.0);
}

#[test]
fn test_grid_with_mixed_fixed_and_flexible() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Flexible(1.0),
    ]);
    container.set_template_columns(vec![
        TrackSizing::Fixed(Length::new(150.0, LengthUnit::Px)),
        TrackSizing::Flexible(1.0),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);

    let items = vec![
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
    ];

    let layout = engine.compute_grid_layout(&container, &items, (1000.0, 600.0));

    assert_eq!(layout.items().len(), 6);

    // Column widths: 150px, 750px (1000 - 150 - 100), 100px
    assert_eq!(layout.items()[0].width(), 150.0);
    assert_eq!(layout.items()[1].width(), 750.0);
    assert_eq!(layout.items()[2].width(), 100.0);

    // Row heights: 100px, 500px (600 - 100)
    assert_eq!(layout.items()[0].height(), 100.0);
    assert_eq!(layout.items()[3].height(), 500.0);

    // Check positions
    assert_eq!(layout.items()[0].row(), 0);
    assert_eq!(layout.items()[0].column(), 0);

    assert_eq!(layout.items()[3].row(), 1);
    assert_eq!(layout.items()[3].column(), 0);
}

#[test]
fn test_grid_layout_with_gaps() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);
    container.set_template_columns(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);
    container.set_row_gap(Some(20.0));
    container.set_column_gap(Some(15.0));

    let items = vec![
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
    ];

    let layout = engine.compute_grid_layout(&container, &items, (500.0, 400.0));

    // First row
    assert_eq!(layout.items()[0].x(), 0.0);
    assert_eq!(layout.items()[0].y(), 0.0);

    assert_eq!(layout.items()[1].x(), 115.0); // 100 + 15 gap
    assert_eq!(layout.items()[1].y(), 0.0);

    assert_eq!(layout.items()[2].x(), 230.0); // 100 + 15 + 100 + 15
    assert_eq!(layout.items()[2].y(), 0.0);

    // Second row
    assert_eq!(layout.items()[3].x(), 0.0);
    assert_eq!(layout.items()[3].y(), 120.0); // 100 + 20 gap
}

#[test]
fn test_grid_with_column_auto_flow() {
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
    container.set_auto_flow(GridAutoFlow::Column);

    let items = vec![
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
        GridItem::new(),
    ];

    let layout = engine.compute_grid_layout(&container, &items, (400.0, 400.0));

    // With column flow: fills column 0 first, then column 1
    assert_eq!(layout.items()[0].row(), 0);
    assert_eq!(layout.items()[0].column(), 0);

    assert_eq!(layout.items()[1].row(), 1); // Next row, same column
    assert_eq!(layout.items()[1].column(), 0);

    assert_eq!(layout.items()[2].row(), 0); // Back to first row, next column
    assert_eq!(layout.items()[2].column(), 1);

    assert_eq!(layout.items()[3].row(), 1);
    assert_eq!(layout.items()[3].column(), 1);
}

#[test]
fn test_explicit_grid_placement_multiple_items() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);
    container.set_template_columns(vec![
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ]);

    // Place items explicitly in different positions
    let mut item1 = GridItem::new();
    item1.set_row_start(GridLine::LineNumber(1));
    item1.set_column_start(GridLine::LineNumber(1));

    let mut item2 = GridItem::new();
    item2.set_row_start(GridLine::LineNumber(2));
    item2.set_column_start(GridLine::LineNumber(2));

    let mut item3 = GridItem::new();
    item3.set_row_start(GridLine::LineNumber(3));
    item3.set_column_start(GridLine::LineNumber(3));

    let items = vec![item1, item2, item3];

    let layout = engine.compute_grid_layout(&container, &items, (400.0, 400.0));

    // Item 1 at (0, 0)
    assert_eq!(layout.items()[0].x(), 0.0);
    assert_eq!(layout.items()[0].y(), 0.0);

    // Item 2 at (100, 100)
    assert_eq!(layout.items()[1].x(), 100.0);
    assert_eq!(layout.items()[1].y(), 100.0);

    // Item 3 at (200, 200)
    assert_eq!(layout.items()[2].x(), 200.0);
    assert_eq!(layout.items()[2].y(), 200.0);
}

#[test]
fn test_flexible_tracks_with_multiple_fr_values() {
    let engine = BasicGridLayoutEngine::new();

    let tracks = vec![
        TrackSizing::Flexible(1.0),
        TrackSizing::Flexible(3.0),
        TrackSizing::Flexible(2.0),
    ];

    let sizes = engine.resolve_track_sizes(&tracks, 600.0, 0.0);

    assert_eq!(sizes.len(), 3);
    // Total 6fr: 1fr=100px, 3fr=300px, 2fr=200px
    assert_eq!(sizes[0], 100.0);
    assert_eq!(sizes[1], 300.0);
    assert_eq!(sizes[2], 200.0);
}

#[test]
fn test_all_fixed_tracks() {
    let engine = BasicGridLayoutEngine::new();

    let tracks = vec![
        TrackSizing::Fixed(Length::new(200.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(150.0, LengthUnit::Px)),
        TrackSizing::Fixed(Length::new(100.0, LengthUnit::Px)),
    ];

    let sizes = engine.resolve_track_sizes(&tracks, 1000.0, 0.0);

    assert_eq!(sizes.len(), 3);
    assert_eq!(sizes[0], 200.0);
    assert_eq!(sizes[1], 150.0);
    assert_eq!(sizes[2], 100.0);
}

#[test]
fn test_empty_grid() {
    let engine = BasicGridLayoutEngine::new();
    let container = GridContainer::new();
    let items: Vec<GridItem> = vec![];

    let layout = engine.compute_grid_layout(&container, &items, (800.0, 600.0));

    assert_eq!(layout.items().len(), 0);
    assert_eq!(layout.container_size(), (800.0, 600.0));
}

#[test]
fn test_single_item_grid() {
    let engine = BasicGridLayoutEngine::new();

    let mut container = GridContainer::new();
    container.set_template_rows(vec![TrackSizing::Flexible(1.0)]);
    container.set_template_columns(vec![TrackSizing::Flexible(1.0)]);

    let items = vec![GridItem::new()];

    let layout = engine.compute_grid_layout(&container, &items, (400.0, 300.0));

    assert_eq!(layout.items().len(), 1);
    assert_eq!(layout.items()[0].x(), 0.0);
    assert_eq!(layout.items()[0].y(), 0.0);
    assert_eq!(layout.items()[0].width(), 400.0);
    assert_eq!(layout.items()[0].height(), 300.0);
}
