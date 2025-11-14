//! Unit tests for box model computation functions

use css_layout_box_model::{compute_border, compute_content_box, compute_margin, compute_padding, BoxModelCalculator, DefaultBoxModelCalculator, EdgeSizes, Rect};
use css_stylist_core::ComputedValues;
use css_types::{Length, LengthUnit};

#[test]
fn test_compute_padding_all_pixels() {
    let mut style = ComputedValues::default();
    style.padding_top = Length::new(10.0, LengthUnit::Px);
    style.padding_right = Length::new(20.0, LengthUnit::Px);
    style.padding_bottom = Length::new(15.0, LengthUnit::Px);
    style.padding_left = Length::new(25.0, LengthUnit::Px);

    let containing_block_width = 800.0;
    let padding = compute_padding(&style, containing_block_width);

    assert_eq!(padding.top(), 10.0);
    assert_eq!(padding.right(), 20.0);
    assert_eq!(padding.bottom(), 15.0);
    assert_eq!(padding.left(), 25.0);
}

#[test]
fn test_compute_padding_with_percentages() {
    let mut style = ComputedValues::default();
    style.padding_top = Length::new(10.0, LengthUnit::Percent);
    style.padding_right = Length::new(5.0, LengthUnit::Percent);
    style.padding_bottom = Length::new(10.0, LengthUnit::Percent);
    style.padding_left = Length::new(5.0, LengthUnit::Percent);

    let containing_block_width = 800.0;
    let padding = compute_padding(&style, containing_block_width);

    // All percentages are relative to containing block width
    assert_eq!(padding.top(), 80.0); // 10% of 800
    assert_eq!(padding.right(), 40.0); // 5% of 800
    assert_eq!(padding.bottom(), 80.0);
    assert_eq!(padding.left(), 40.0);
}

#[test]
fn test_compute_border_all_pixels() {
    let style = ComputedValues::default();
    // Note: In a real implementation, border widths would be properties on ComputedValues
    // For now, we'll test with default values
    let border = compute_border(&style);

    // Default should be 0
    assert_eq!(border.top(), 0.0);
    assert_eq!(border.right(), 0.0);
    assert_eq!(border.bottom(), 0.0);
    assert_eq!(border.left(), 0.0);
}

#[test]
fn test_compute_margin_all_pixels() {
    let mut style = ComputedValues::default();
    style.margin_top = Length::new(10.0, LengthUnit::Px);
    style.margin_right = Length::new(20.0, LengthUnit::Px);
    style.margin_bottom = Length::new(15.0, LengthUnit::Px);
    style.margin_left = Length::new(25.0, LengthUnit::Px);

    let containing_block_width = 800.0;
    let margin = compute_margin(&style, containing_block_width);

    assert_eq!(margin.top(), 10.0);
    assert_eq!(margin.right(), 20.0);
    assert_eq!(margin.bottom(), 15.0);
    assert_eq!(margin.left(), 25.0);
}

#[test]
fn test_compute_margin_with_percentages() {
    let mut style = ComputedValues::default();
    style.margin_top = Length::new(10.0, LengthUnit::Percent);
    style.margin_right = Length::new(5.0, LengthUnit::Percent);
    style.margin_bottom = Length::new(10.0, LengthUnit::Percent);
    style.margin_left = Length::new(5.0, LengthUnit::Percent);

    let containing_block_width = 800.0;
    let margin = compute_margin(&style, containing_block_width);

    assert_eq!(margin.top(), 80.0);
    assert_eq!(margin.right(), 40.0);
    assert_eq!(margin.bottom(), 80.0);
    assert_eq!(margin.left(), 40.0);
}

#[test]
fn test_compute_content_box_with_explicit_dimensions() {
    let mut style = ComputedValues::default();
    style.width = Length::new(200.0, LengthUnit::Px);
    style.height = Length::new(100.0, LengthUnit::Px);

    let containing_block = Rect::new(0.0, 0.0, 800.0, 600.0);
    let content = compute_content_box(&style, &containing_block);

    assert_eq!(content.width(), 200.0);
    assert_eq!(content.height(), 100.0);
}

#[test]
fn test_compute_content_box_with_percentage_width() {
    let mut style = ComputedValues::default();
    style.width = Length::new(50.0, LengthUnit::Percent);
    style.height = Length::new(100.0, LengthUnit::Px);

    let containing_block = Rect::new(0.0, 0.0, 800.0, 600.0);
    let content = compute_content_box(&style, &containing_block);

    assert_eq!(content.width(), 400.0); // 50% of 800
    assert_eq!(content.height(), 100.0);
}

#[test]
fn test_box_model_calculator_resolve_width() {
    let calculator = DefaultBoxModelCalculator;
    let width = Length::new(200.0, LengthUnit::Px);
    let containing_block_width = 800.0;

    let resolved = calculator.resolve_width(&width, containing_block_width);
    assert_eq!(resolved, 200.0);
}

#[test]
fn test_box_model_calculator_resolve_width_percentage() {
    let calculator = DefaultBoxModelCalculator;
    let width = Length::new(25.0, LengthUnit::Percent);
    let containing_block_width = 800.0;

    let resolved = calculator.resolve_width(&width, containing_block_width);
    assert_eq!(resolved, 200.0); // 25% of 800
}

#[test]
fn test_box_model_calculator_resolve_height() {
    let calculator = DefaultBoxModelCalculator;
    let height = Length::new(100.0, LengthUnit::Px);
    let containing_block_height = 600.0;

    let resolved = calculator.resolve_height(&height, containing_block_height);
    assert_eq!(resolved, 100.0);
}

#[test]
fn test_box_model_calculator_resolve_height_percentage() {
    let calculator = DefaultBoxModelCalculator;
    let height = Length::new(50.0, LengthUnit::Percent);
    let containing_block_height = 600.0;

    let resolved = calculator.resolve_height(&height, containing_block_height);
    assert_eq!(resolved, 300.0); // 50% of 600
}

#[test]
fn test_box_model_calculator_compute_full_box_model() {
    let calculator = DefaultBoxModelCalculator;
    let mut style = ComputedValues::default();

    style.width = Length::new(200.0, LengthUnit::Px);
    style.height = Length::new(100.0, LengthUnit::Px);
    style.padding_top = Length::new(10.0, LengthUnit::Px);
    style.padding_right = Length::new(10.0, LengthUnit::Px);
    style.padding_bottom = Length::new(10.0, LengthUnit::Px);
    style.padding_left = Length::new(10.0, LengthUnit::Px);
    style.margin_top = Length::new(5.0, LengthUnit::Px);
    style.margin_right = Length::new(5.0, LengthUnit::Px);
    style.margin_bottom = Length::new(5.0, LengthUnit::Px);
    style.margin_left = Length::new(5.0, LengthUnit::Px);

    let containing_block = Rect::new(0.0, 0.0, 800.0, 600.0);
    let box_model = calculator.compute_box_model(&style, &containing_block);

    assert_eq!(box_model.content().width(), 200.0);
    assert_eq!(box_model.content().height(), 100.0);
    assert_eq!(box_model.padding().top(), 10.0);
    assert_eq!(box_model.margin().top(), 5.0);
}
