//! Integration tests for CSS multi-column layout API

use css_layout_multicolumn::*;
use css_types::{Length, LengthUnit};

#[test]
fn test_complete_multicolumn_pipeline() {
    // Parse properties
    let count = parse_column_count("3").unwrap();
    let width = parse_column_width("auto").unwrap();
    let gap = parse_column_gap("20px").unwrap();

    // Create layout config
    let mut config = MultiColumnLayout::new();
    config.column_count = count;
    config.column_width = width;
    config.column_gap = gap;

    // Compute layout
    let computed = compute_column_layout(&config, 660.0);

    // Verify results
    assert_eq!(computed.column_count, 3);
    // (660 - 2*20) / 3 = 620 / 3 = 206.67
    assert!((computed.column_width - 206.67).abs() < 0.1);
    assert_eq!(computed.gap_width, 20.0);
}

#[test]
fn test_auto_column_count_calculation() {
    // Parse properties with auto count
    let count = parse_column_count("auto").unwrap();
    let width = parse_column_width("150px").unwrap();
    let gap = parse_column_gap("normal").unwrap();

    // Create layout config
    let mut config = MultiColumnLayout::new();
    config.column_count = count;
    config.column_width = width;
    config.column_gap = gap;

    // Compute layout (should fit multiple columns)
    let computed = compute_column_layout(&config, 650.0);

    // Should automatically calculate optimal column count
    assert!(computed.column_count >= 2);
    assert_eq!(computed.column_width, 150.0);
}

#[test]
fn test_column_rule_with_layout() {
    // Parse column rule
    let rule = parse_column_rule("2px solid #FF0000").unwrap();

    // Create layout with rule
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(2);
    config.column_rule = Some(rule);

    // Verify rule properties
    assert_eq!(config.column_rule.unwrap().width.value(), 2.0);
    assert_eq!(config.column_rule.unwrap().style, BorderStyle::Solid);
}

#[test]
fn test_content_balancing_integration() {
    // Setup layout
    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(4);

    // Compute layout
    let computed = compute_column_layout(&config, 800.0);

    // Balance content
    let height_per_column = balance_content(1000.0, computed.column_count);

    // Verify balanced height
    assert_eq!(height_per_column, 250.0);
}

#[test]
fn test_multicolumn_computer_trait_integration() {
    let computer = DefaultMultiColumnComputer;

    let mut config = MultiColumnLayout::new();
    config.column_count = ColumnCount::Count(3);
    config.column_gap = ColumnGap::Length(Length::new(15.0, LengthUnit::Px));

    let computed = computer.compute_layout(&config, 600.0, 900.0);

    assert_eq!(computed.column_count, 3);
    assert_eq!(computed.gap_width, 15.0);
}

#[test]
fn test_complex_multicolumn_scenario() {
    // Parse multiple properties
    let count = parse_column_count("4").unwrap();
    let width = parse_column_width("auto").unwrap();
    let gap = parse_column_gap("1.5em").unwrap();
    let rule = parse_column_rule("1px dashed #CCCCCC").unwrap();

    // Build complete configuration
    let mut config = MultiColumnLayout::new();
    config.column_count = count;
    config.column_width = width;
    config.column_gap = gap;
    config.column_rule = Some(rule);
    config.column_span = ColumnSpan::None;
    config.column_fill = ColumnFill::Balance;

    // Compute layout
    let computed = compute_column_layout(&config, 1000.0);

    // Verify all aspects
    assert_eq!(computed.column_count, 4);
    assert!(computed.column_width > 0.0);
    assert_eq!(config.column_rule.unwrap().style, BorderStyle::Dashed);
    assert_eq!(config.column_span, ColumnSpan::None);
    assert_eq!(config.column_fill, ColumnFill::Balance);
}

#[test]
fn test_responsive_column_layout() {
    // Same configuration, different container widths
    let mut config = MultiColumnLayout::new();
    config.column_width = ColumnWidth::Length(Length::new(200.0, LengthUnit::Px));
    config.column_gap = ColumnGap::Length(Length::new(20.0, LengthUnit::Px));

    // Mobile viewport (400px)
    let mobile = compute_column_layout(&config, 400.0);
    assert_eq!(mobile.column_count, 1);

    // Tablet viewport (800px)
    let tablet = compute_column_layout(&config, 800.0);
    assert!(tablet.column_count >= 2);

    // Desktop viewport (1200px)
    let desktop = compute_column_layout(&config, 1200.0);
    assert!(desktop.column_count >= 4);
}

#[test]
fn test_all_border_styles() {
    // Test each border style
    let styles = vec![
        ("none", BorderStyle::None),
        ("solid", BorderStyle::Solid),
        ("dashed", BorderStyle::Dashed),
        ("dotted", BorderStyle::Dotted),
        ("double", BorderStyle::Double),
        ("groove", BorderStyle::Groove),
        ("ridge", BorderStyle::Ridge),
        ("inset", BorderStyle::Inset),
        ("outset", BorderStyle::Outset),
    ];

    for (style_str, expected_style) in styles {
        let rule_str = format!("1px {} #000000", style_str);
        let rule = parse_column_rule(&rule_str).unwrap();
        assert_eq!(rule.style, expected_style);
    }
}

#[test]
fn test_various_length_units() {
    // Test different length units for column width
    let units = vec![
        ("200px", LengthUnit::Px),
        ("15em", LengthUnit::Em),
        ("12rem", LengthUnit::Rem),
        ("50%", LengthUnit::Percent),
    ];

    for (length_str, expected_unit) in units {
        let width = parse_column_width(length_str).unwrap();
        match width {
            ColumnWidth::Length(length) => {
                assert_eq!(length.unit(), expected_unit);
            }
            _ => panic!("Expected Length variant"),
        }
    }
}

#[test]
fn test_error_handling() {
    // Test that invalid inputs return errors
    assert!(parse_column_count("invalid").is_err());
    assert!(parse_column_count("0").is_err());
    assert!(parse_column_width("invalid").is_err());
    assert!(parse_column_gap("invalid").is_err());
    assert!(parse_column_rule("invalid").is_err());
    assert!(parse_column_rule("1px solid").is_err());
}

#[test]
fn test_default_multicolumn_layout() {
    let config = MultiColumnLayout::default();

    assert_eq!(config.column_count, ColumnCount::Auto);
    assert_eq!(config.column_width, ColumnWidth::Auto);
    assert_eq!(config.column_gap, ColumnGap::Normal);
    assert!(config.column_rule.is_none());
    assert_eq!(config.column_span, ColumnSpan::None);
    assert_eq!(config.column_fill, ColumnFill::Balance);
}
