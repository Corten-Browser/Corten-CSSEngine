//! Unit tests for CSS multi-column layout computation

use css_layout_multicolumn::*;
use css_types::{Length, LengthUnit};

// ============================================================================
// Column Layout Computation Tests
// ============================================================================

#[test]
fn test_compute_column_layout_both_auto() {
    let config = MultiColumnLayout::new();
    let computed = compute_column_layout(&config, 600.0);

    assert_eq!(computed.column_count, 1);
    assert_eq!(computed.column_width, 600.0);
    assert_eq!(computed.total_width, 600.0);
}

#[test]
fn test_compute_column_layout_count_specified() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(3);

    let computed = compute_column_layout(&config, 600.0);

    assert_eq!(computed.column_count, 3);
    // (600 - 2*16) / 3 = 568 / 3 ≈ 189.33
    assert!((computed.column_width - 189.33).abs() < 0.1);
}

#[test]
fn test_compute_column_layout_count_specified_exact() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(2);
    config.column_gap = ColumnGap::Length(Length::new(20.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 600.0);

    assert_eq!(computed.column_count, 2);
    // (600 - 20) / 2 = 290
    assert_eq!(computed.column_width, 290.0);
    assert_eq!(computed.gap_width, 20.0);
}

#[test]
fn test_compute_column_layout_width_specified() {
    let mut config = MultiColumnLayout::new();
    config.column_width = ColumnWidth::Length(Length::new(200.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 650.0);

    // 200px + 16px gap + 200px + 16px gap + 200px = 632px (fits 3 columns)
    assert_eq!(computed.column_count, 3);
    assert_eq!(computed.column_width, 200.0);
}

#[test]
fn test_compute_column_layout_width_specified_tight_fit() {
    let mut config = MultiColumnLayout::new();
    config.column_width = ColumnWidth::Length(Length::new(200.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 400.0);

    // Only 1 column fits (200px + 16px + 200px = 416px > 400px)
    assert_eq!(computed.column_count, 1);
    assert_eq!(computed.column_width, 200.0);
}

#[test]
fn test_compute_column_layout_width_specified_exact_fit() {
    let mut config = MultiColumnLayout::new();
    config.column_width = ColumnWidth::Length(Length::new(200.0, LengthUnit::Px));
    config.column_gap = ColumnGap::Length(Length::new(20.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 420.0);

    // 200px + 20px + 200px = 420px (fits exactly 2 columns)
    assert_eq!(computed.column_count, 2);
    assert_eq!(computed.column_width, 200.0);
    assert_eq!(computed.gap_width, 20.0);
}

#[test]
fn test_compute_column_layout_both_specified() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(4);
    config.column_width = ColumnWidth::Length(Length::new(150.0, LengthUnit::Px));
    config.column_gap = ColumnGap::Length(Length::new(10.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 1000.0);

    assert_eq!(computed.column_count, 4);
    assert_eq!(computed.column_width, 150.0);
    assert_eq!(computed.gap_width, 10.0);
    // 4*150 + 3*10 = 630
    assert_eq!(computed.total_width, 630.0);
}

#[test]
fn test_compute_column_layout_single_column() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(1);

    let computed = compute_column_layout(&config, 600.0);

    assert_eq!(computed.column_count, 1);
    assert_eq!(computed.column_width, 600.0);
}

#[test]
fn test_compute_column_layout_many_columns() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(10);

    let computed = compute_column_layout(&config, 1000.0);

    assert_eq!(computed.column_count, 10);
    // (1000 - 9*16) / 10 = 856 / 10 = 85.6
    assert!((computed.column_width - 85.6).abs() < 0.1);
}

#[test]
fn test_compute_column_layout_custom_gap() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(3);
    config.column_gap = ColumnGap::Length(Length::new(30.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 600.0);

    assert_eq!(computed.column_count, 3);
    assert_eq!(computed.gap_width, 30.0);
    // (600 - 2*30) / 3 = 540 / 3 = 180
    assert_eq!(computed.column_width, 180.0);
}

#[test]
fn test_compute_column_layout_zero_gap() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(2);
    config.column_gap = ColumnGap::Length(Length::new(0.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 400.0);

    assert_eq!(computed.column_count, 2);
    assert_eq!(computed.gap_width, 0.0);
    assert_eq!(computed.column_width, 200.0);
}

#[test]
fn test_compute_column_layout_large_gap() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(2);
    config.column_gap = ColumnGap::Length(Length::new(100.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 600.0);

    assert_eq!(computed.column_count, 2);
    assert_eq!(computed.gap_width, 100.0);
    // (600 - 100) / 2 = 250
    assert_eq!(computed.column_width, 250.0);
}

// ============================================================================
// Content Balancing Tests
// ============================================================================

#[test]
fn test_balance_content_simple() {
    let height = balance_content(1000.0, 4);
    assert_eq!(height, 250.0);
}

#[test]
fn test_balance_content_two_columns() {
    let height = balance_content(500.0, 2);
    assert_eq!(height, 250.0);
}

#[test]
fn test_balance_content_single_column() {
    let height = balance_content(1000.0, 1);
    assert_eq!(height, 1000.0);
}

#[test]
fn test_balance_content_many_columns() {
    let height = balance_content(1000.0, 10);
    assert_eq!(height, 100.0);
}

#[test]
fn test_balance_content_uneven_division() {
    let height = balance_content(1000.0, 3);
    assert!((height - 333.33).abs() < 0.1);
}

#[test]
fn test_balance_content_zero_columns() {
    let height = balance_content(1000.0, 0);
    assert_eq!(height, 1000.0);
}

#[test]
fn test_balance_content_small_height() {
    let height = balance_content(10.0, 5);
    assert_eq!(height, 2.0);
}

#[test]
fn test_balance_content_fractional() {
    let height = balance_content(100.5, 2);
    assert_eq!(height, 50.25);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_multicolumn_layout_complete_workflow() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(3);
    config.column_gap = ColumnGap::Length(Length::new(20.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 660.0);
    let height_per_column = balance_content(900.0, computed.column_count);

    assert_eq!(computed.column_count, 3);
    // (660 - 2*20) / 3 = 620 / 3 = 206.67
    assert!((computed.column_width - 206.67).abs() < 0.1);
    assert_eq!(height_per_column, 300.0);
}

#[test]
fn test_multicolumn_layout_auto_width_calculation() {
    let mut config = MultiColumnLayout::new();
    config.column_width = ColumnWidth::Length(Length::new(250.0, LengthUnit::Px));
    config.column_gap = ColumnGap::Length(Length::new(25.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 800.0);

    // 250 + 25 + 250 + 25 + 250 = 800 (fits 3 columns)
    assert_eq!(computed.column_count, 3);
    assert_eq!(computed.column_width, 250.0);
}

#[test]
fn test_multicolumn_layout_default_gap() {
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(2);
    config.column_gap = ColumnGap::Normal;

    let computed = compute_column_layout(&config, 400.0);

    assert_eq!(computed.gap_width, 16.0); // Default normal gap
}

// ============================================================================
// Trait Implementation Tests
// ============================================================================

#[test]
fn test_multicolumn_computer_trait() {
    let computer = DefaultMultiColumnComputer;
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(2);

    let computed = computer.compute_layout(&config, 600.0, 1000.0);

    assert_eq!(computed.column_count, 2);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_very_narrow_container() {
    let mut config = MultiColumnLayout::new();
    config.column_width = ColumnWidth::Length(Length::new(200.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 100.0);

    // Container too narrow for specified width, but still creates 1 column
    assert_eq!(computed.column_count, 1);
    assert_eq!(computed.column_width, 200.0);
}

#[test]
fn test_very_wide_container() {
    let mut config = MultiColumnLayout::new();
    config.column_width = ColumnWidth::Length(Length::new(100.0, LengthUnit::Px));

    let computed = compute_column_layout(&config, 5000.0);

    // Should fit many columns
    assert!(computed.column_count > 10);
}
