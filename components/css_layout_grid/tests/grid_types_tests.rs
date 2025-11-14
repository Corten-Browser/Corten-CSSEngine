//! Unit tests for CSS Grid basic types

use css_layout_grid::{GridAutoFlow, GridLine, TrackSizing};
use css_types::{Length, LengthUnit};

// ============================================================================
// GridAutoFlow Tests
// ============================================================================

#[test]
fn test_grid_auto_flow_row() {
    let flow = GridAutoFlow::Row;
    assert_eq!(flow, GridAutoFlow::Row);
}

#[test]
fn test_grid_auto_flow_column() {
    let flow = GridAutoFlow::Column;
    assert_eq!(flow, GridAutoFlow::Column);
}

#[test]
fn test_grid_auto_flow_row_dense() {
    let flow = GridAutoFlow::RowDense;
    assert_eq!(flow, GridAutoFlow::RowDense);
}

#[test]
fn test_grid_auto_flow_column_dense() {
    let flow = GridAutoFlow::ColumnDense;
    assert_eq!(flow, GridAutoFlow::ColumnDense);
}

// ============================================================================
// TrackSizing Tests
// ============================================================================

#[test]
fn test_track_sizing_fixed() {
    let length = Length::new(100.0, LengthUnit::Px);
    let sizing = TrackSizing::Fixed(length);

    match sizing {
        TrackSizing::Fixed(l) => {
            assert_eq!(l.value(), 100.0);
            assert_eq!(l.unit(), LengthUnit::Px);
        }
        _ => panic!("Expected Fixed variant"),
    }
}

#[test]
fn test_track_sizing_flexible() {
    let sizing = TrackSizing::Flexible(2.0);

    match sizing {
        TrackSizing::Flexible(fr) => {
            assert_eq!(fr, 2.0);
        }
        _ => panic!("Expected Flexible variant"),
    }
}

#[test]
fn test_track_sizing_min_content() {
    let sizing = TrackSizing::MinContent;
    assert_eq!(sizing, TrackSizing::MinContent);
}

#[test]
fn test_track_sizing_max_content() {
    let sizing = TrackSizing::MaxContent;
    assert_eq!(sizing, TrackSizing::MaxContent);
}

#[test]
fn test_track_sizing_auto() {
    let sizing = TrackSizing::Auto;
    assert_eq!(sizing, TrackSizing::Auto);
}

// ============================================================================
// GridLine Tests
// ============================================================================

#[test]
fn test_grid_line_auto() {
    let line = GridLine::Auto;
    assert_eq!(line, GridLine::Auto);
}

#[test]
fn test_grid_line_line_number_positive() {
    let line = GridLine::LineNumber(3);
    match line {
        GridLine::LineNumber(n) => assert_eq!(n, 3),
        _ => panic!("Expected LineNumber variant"),
    }
}

#[test]
fn test_grid_line_line_number_negative() {
    let line = GridLine::LineNumber(-1);
    match line {
        GridLine::LineNumber(n) => assert_eq!(n, -1),
        _ => panic!("Expected LineNumber variant"),
    }
}

#[test]
fn test_grid_line_span() {
    let line = GridLine::Span(2);
    match line {
        GridLine::Span(n) => assert_eq!(n, 2),
        _ => panic!("Expected Span variant"),
    }
}
