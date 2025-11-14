//! Integration tests for box model calculations
//!
//! These tests verify the complete box model computation workflow,
//! including edge cases and complex scenarios.

use css_layout_box_model::{
    BoxModel, BoxModelCalculator, BoxSizing, DefaultBoxModelCalculator, Display, EdgeSizes, Rect,
};
use css_stylist_core::ComputedValues;
use css_types::{Length, LengthUnit};

#[test]
fn test_content_box_sizing_complete_workflow() {
    // Test a complete box model calculation with content-box sizing
    let calculator = DefaultBoxModelCalculator;
    let mut style = ComputedValues::default();

    // Set dimensions
    style.width = Length::new(300.0, LengthUnit::Px);
    style.height = Length::new(200.0, LengthUnit::Px);

    // Set padding
    style.padding_top = Length::new(20.0, LengthUnit::Px);
    style.padding_right = Length::new(30.0, LengthUnit::Px);
    style.padding_bottom = Length::new(20.0, LengthUnit::Px);
    style.padding_left = Length::new(30.0, LengthUnit::Px);

    // Set margins
    style.margin_top = Length::new(10.0, LengthUnit::Px);
    style.margin_right = Length::new(15.0, LengthUnit::Px);
    style.margin_bottom = Length::new(10.0, LengthUnit::Px);
    style.margin_left = Length::new(15.0, LengthUnit::Px);

    let containing_block = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let box_model = calculator.compute_box_model(&style, &containing_block);

    // Verify content box
    assert_eq!(box_model.content().width(), 300.0);
    assert_eq!(box_model.content().height(), 200.0);

    // Verify padding
    assert_eq!(box_model.padding().horizontal(), 60.0);
    assert_eq!(box_model.padding().vertical(), 40.0);

    // Verify margin
    assert_eq!(box_model.margin().horizontal(), 30.0);
    assert_eq!(box_model.margin().vertical(), 20.0);

    // Verify computed boxes
    let padding_box = box_model.padding_box();
    assert_eq!(padding_box.width(), 360.0); // 300 + 30 + 30
    assert_eq!(padding_box.height(), 240.0); // 200 + 20 + 20

    let border_box = box_model.border_box();
    assert_eq!(border_box.width(), 360.0); // No border in this test
    assert_eq!(border_box.height(), 240.0);

    let margin_box = box_model.margin_box();
    assert_eq!(margin_box.width(), 390.0); // 360 + 15 + 15
    assert_eq!(margin_box.height(), 260.0); // 240 + 10 + 10
}

#[test]
fn test_percentage_based_dimensions() {
    // Test box model with percentage-based dimensions
    let calculator = DefaultBoxModelCalculator;
    let mut style = ComputedValues::default();

    // 50% width of containing block
    style.width = Length::new(50.0, LengthUnit::Percent);
    style.height = Length::new(200.0, LengthUnit::Px);

    // Percentage padding (relative to containing block width)
    style.padding_top = Length::new(5.0, LengthUnit::Percent);
    style.padding_right = Length::new(2.5, LengthUnit::Percent);
    style.padding_bottom = Length::new(5.0, LengthUnit::Percent);
    style.padding_left = Length::new(2.5, LengthUnit::Percent);

    // Percentage margins
    style.margin_top = Length::new(2.0, LengthUnit::Percent);
    style.margin_right = Length::new(1.0, LengthUnit::Percent);
    style.margin_bottom = Length::new(2.0, LengthUnit::Percent);
    style.margin_left = Length::new(1.0, LengthUnit::Percent);

    let containing_block = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let box_model = calculator.compute_box_model(&style, &containing_block);

    // Verify content width: 50% of 1000 = 500
    assert_eq!(box_model.content().width(), 500.0);
    assert_eq!(box_model.content().height(), 200.0);

    // Verify padding (percentages relative to width)
    assert_eq!(box_model.padding().top(), 50.0); // 5% of 1000
    assert_eq!(box_model.padding().right(), 25.0); // 2.5% of 1000
    assert_eq!(box_model.padding().bottom(), 50.0);
    assert_eq!(box_model.padding().left(), 25.0);

    // Verify margins
    assert_eq!(box_model.margin().top(), 20.0); // 2% of 1000
    assert_eq!(box_model.margin().right(), 10.0); // 1% of 1000
}

#[test]
fn test_mixed_units() {
    // Test box model with mixed units (px and %)
    let calculator = DefaultBoxModelCalculator;
    let mut style = ComputedValues::default();

    style.width = Length::new(400.0, LengthUnit::Px);
    style.height = Length::new(25.0, LengthUnit::Percent); // 25% of 800 = 200

    style.padding_top = Length::new(10.0, LengthUnit::Px);
    style.padding_right = Length::new(5.0, LengthUnit::Percent); // 5% of 1000 = 50
    style.padding_bottom = Length::new(10.0, LengthUnit::Px);
    style.padding_left = Length::new(5.0, LengthUnit::Percent); // 5% of 1000 = 50

    let containing_block = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let box_model = calculator.compute_box_model(&style, &containing_block);

    assert_eq!(box_model.content().width(), 400.0);
    assert_eq!(box_model.content().height(), 200.0); // 25% of 800

    assert_eq!(box_model.padding().top(), 10.0);
    assert_eq!(box_model.padding().right(), 50.0);
    assert_eq!(box_model.padding().bottom(), 10.0);
    assert_eq!(box_model.padding().left(), 50.0);
}

#[test]
fn test_zero_dimensions() {
    // Test box model with zero or default dimensions
    let calculator = DefaultBoxModelCalculator;
    let style = ComputedValues::default(); // All zeros/defaults

    let containing_block = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let box_model = calculator.compute_box_model(&style, &containing_block);

    // Default dimensions should be 0
    assert_eq!(box_model.content().width(), 0.0);
    assert_eq!(box_model.content().height(), 0.0);

    // All padding/margin/border should be 0
    assert_eq!(box_model.padding().horizontal(), 0.0);
    assert_eq!(box_model.padding().vertical(), 0.0);
    assert_eq!(box_model.margin().horizontal(), 0.0);
    assert_eq!(box_model.margin().vertical(), 0.0);
    assert_eq!(box_model.border().horizontal(), 0.0);
    assert_eq!(box_model.border().vertical(), 0.0);
}

#[test]
fn test_asymmetric_padding_and_margins() {
    // Test box model with different values for each edge
    let calculator = DefaultBoxModelCalculator;
    let mut style = ComputedValues::default();

    style.width = Length::new(200.0, LengthUnit::Px);
    style.height = Length::new(150.0, LengthUnit::Px);

    // Asymmetric padding
    style.padding_top = Length::new(5.0, LengthUnit::Px);
    style.padding_right = Length::new(10.0, LengthUnit::Px);
    style.padding_bottom = Length::new(15.0, LengthUnit::Px);
    style.padding_left = Length::new(20.0, LengthUnit::Px);

    // Asymmetric margins
    style.margin_top = Length::new(2.0, LengthUnit::Px);
    style.margin_right = Length::new(4.0, LengthUnit::Px);
    style.margin_bottom = Length::new(6.0, LengthUnit::Px);
    style.margin_left = Length::new(8.0, LengthUnit::Px);

    let containing_block = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let box_model = calculator.compute_box_model(&style, &containing_block);

    // Verify all edges are computed correctly
    assert_eq!(box_model.padding().top(), 5.0);
    assert_eq!(box_model.padding().right(), 10.0);
    assert_eq!(box_model.padding().bottom(), 15.0);
    assert_eq!(box_model.padding().left(), 20.0);

    assert_eq!(box_model.margin().top(), 2.0);
    assert_eq!(box_model.margin().right(), 4.0);
    assert_eq!(box_model.margin().bottom(), 6.0);
    assert_eq!(box_model.margin().left(), 8.0);

    // Verify computed boxes account for asymmetry
    let padding_box = box_model.padding_box();
    assert_eq!(padding_box.width(), 230.0); // 200 + 10 + 20
    assert_eq!(padding_box.height(), 170.0); // 150 + 5 + 15
}

#[test]
fn test_large_dimensions() {
    // Test box model with very large dimensions
    let calculator = DefaultBoxModelCalculator;
    let mut style = ComputedValues::default();

    style.width = Length::new(10000.0, LengthUnit::Px);
    style.height = Length::new(5000.0, LengthUnit::Px);

    style.padding_top = Length::new(100.0, LengthUnit::Px);
    style.padding_right = Length::new(100.0, LengthUnit::Px);
    style.padding_bottom = Length::new(100.0, LengthUnit::Px);
    style.padding_left = Length::new(100.0, LengthUnit::Px);

    let containing_block = Rect::new(0.0, 0.0, 20000.0, 15000.0);
    let box_model = calculator.compute_box_model(&style, &containing_block);

    assert_eq!(box_model.content().width(), 10000.0);
    assert_eq!(box_model.content().height(), 5000.0);

    let padding_box = box_model.padding_box();
    assert_eq!(padding_box.width(), 10200.0);
    assert_eq!(padding_box.height(), 5200.0);
}

#[test]
fn test_rect_contains_with_box_model() {
    // Test that rect.contains() works correctly with computed box model
    let content = Rect::new(100.0, 100.0, 200.0, 150.0);
    let padding = EdgeSizes::uniform(10.0);
    let border = EdgeSizes::uniform(2.0);
    let margin = EdgeSizes::uniform(5.0);

    let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);

    // Point inside content box
    assert!(box_model.content().contains(150.0, 150.0));

    // Point outside content box but inside margin box
    let margin_box = box_model.margin_box();
    assert!(margin_box.contains(95.0, 95.0)); // Near edge of margin box

    // Point completely outside
    assert!(!margin_box.contains(50.0, 50.0));
}

#[test]
fn test_display_enum_usage() {
    // Verify Display enum can be used in practice
    let display_values = vec![
        Display::Block,
        Display::Inline,
        Display::InlineBlock,
        Display::None,
        Display::Flex,
        Display::Grid,
        Display::Table,
    ];

    // All variants should be distinct
    for (i, &val1) in display_values.iter().enumerate() {
        for (j, &val2) in display_values.iter().enumerate() {
            if i == j {
                assert_eq!(val1, val2);
            } else {
                assert_ne!(val1, val2);
            }
        }
    }
}

#[test]
fn test_box_sizing_modes() {
    // Test both box sizing modes
    let content_box = BoxSizing::ContentBox;
    let border_box = BoxSizing::BorderBox;

    assert_eq!(content_box, BoxSizing::ContentBox);
    assert_eq!(border_box, BoxSizing::BorderBox);
    assert_ne!(content_box, border_box);

    // Verify they can be used in BoxModel
    let rect = Rect::default();
    let edges = EdgeSizes::default();

    let box_model_content = BoxModel::new(rect, edges, edges, edges, content_box);
    assert_eq!(box_model_content.box_sizing(), BoxSizing::ContentBox);

    let box_model_border = BoxModel::new(rect, edges, edges, edges, border_box);
    assert_eq!(box_model_border.box_sizing(), BoxSizing::BorderBox);
}
