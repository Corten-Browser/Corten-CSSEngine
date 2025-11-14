//! Unit tests for box model types

use css_layout_box_model::{BoxModel, BoxSizing, Display, EdgeSizes, Rect};

#[test]
fn test_rect_new() {
    let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
    assert_eq!(rect.x(), 10.0);
    assert_eq!(rect.y(), 20.0);
    assert_eq!(rect.width(), 100.0);
    assert_eq!(rect.height(), 50.0);
}

#[test]
fn test_rect_default() {
    let rect = Rect::default();
    assert_eq!(rect.x(), 0.0);
    assert_eq!(rect.y(), 0.0);
    assert_eq!(rect.width(), 0.0);
    assert_eq!(rect.height(), 0.0);
}

#[test]
fn test_rect_area() {
    let rect = Rect::new(0.0, 0.0, 100.0, 50.0);
    assert_eq!(rect.area(), 5000.0);
}

#[test]
fn test_rect_contains_point() {
    let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
    assert!(rect.contains(50.0, 40.0));
    assert!(!rect.contains(5.0, 40.0));
    assert!(!rect.contains(50.0, 10.0));
}

#[test]
fn test_edge_sizes_new() {
    let edges = EdgeSizes::new(10.0, 20.0, 30.0, 40.0);
    assert_eq!(edges.top(), 10.0);
    assert_eq!(edges.right(), 20.0);
    assert_eq!(edges.bottom(), 30.0);
    assert_eq!(edges.left(), 40.0);
}

#[test]
fn test_edge_sizes_uniform() {
    let edges = EdgeSizes::uniform(10.0);
    assert_eq!(edges.top(), 10.0);
    assert_eq!(edges.right(), 10.0);
    assert_eq!(edges.bottom(), 10.0);
    assert_eq!(edges.left(), 10.0);
}

#[test]
fn test_edge_sizes_default() {
    let edges = EdgeSizes::default();
    assert_eq!(edges.top(), 0.0);
    assert_eq!(edges.right(), 0.0);
    assert_eq!(edges.bottom(), 0.0);
    assert_eq!(edges.left(), 0.0);
}

#[test]
fn test_edge_sizes_horizontal() {
    let edges = EdgeSizes::new(10.0, 20.0, 30.0, 40.0);
    assert_eq!(edges.horizontal(), 60.0); // left + right
}

#[test]
fn test_edge_sizes_vertical() {
    let edges = EdgeSizes::new(10.0, 20.0, 30.0, 40.0);
    assert_eq!(edges.vertical(), 40.0); // top + bottom
}

#[test]
fn test_box_sizing_content_box() {
    let sizing = BoxSizing::ContentBox;
    assert_eq!(sizing, BoxSizing::ContentBox);
}

#[test]
fn test_box_sizing_border_box() {
    let sizing = BoxSizing::BorderBox;
    assert_eq!(sizing, BoxSizing::BorderBox);
}

#[test]
fn test_display_variants() {
    assert_eq!(Display::Block, Display::Block);
    assert_eq!(Display::Inline, Display::Inline);
    assert_eq!(Display::InlineBlock, Display::InlineBlock);
    assert_eq!(Display::None, Display::None);
    assert_eq!(Display::Flex, Display::Flex);
    assert_eq!(Display::Grid, Display::Grid);
    assert_eq!(Display::Table, Display::Table);
}

#[test]
fn test_box_model_new() {
    let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    let padding = EdgeSizes::uniform(10.0);
    let border = EdgeSizes::uniform(2.0);
    let margin = EdgeSizes::uniform(5.0);

    let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);

    assert_eq!(box_model.content(), &content);
    assert_eq!(box_model.padding(), &padding);
    assert_eq!(box_model.border(), &border);
    assert_eq!(box_model.margin(), &margin);
    assert_eq!(box_model.box_sizing(), BoxSizing::ContentBox);
}

#[test]
fn test_box_model_padding_box() {
    let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    let padding = EdgeSizes::new(10.0, 15.0, 10.0, 15.0);
    let border = EdgeSizes::uniform(2.0);
    let margin = EdgeSizes::uniform(5.0);

    let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);
    let padding_box = box_model.padding_box();

    // Width: 200 + 15 (left) + 15 (right) = 230
    // Height: 100 + 10 (top) + 10 (bottom) = 120
    assert_eq!(padding_box.width(), 230.0);
    assert_eq!(padding_box.height(), 120.0);
}

#[test]
fn test_box_model_border_box() {
    let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    let padding = EdgeSizes::uniform(10.0);
    let border = EdgeSizes::uniform(2.0);
    let margin = EdgeSizes::uniform(5.0);

    let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);
    let border_box = box_model.border_box();

    // Width: 200 + 10*2 (padding) + 2*2 (border) = 224
    // Height: 100 + 10*2 (padding) + 2*2 (border) = 124
    assert_eq!(border_box.width(), 224.0);
    assert_eq!(border_box.height(), 124.0);
}

#[test]
fn test_box_model_margin_box() {
    let content = Rect::new(0.0, 0.0, 200.0, 100.0);
    let padding = EdgeSizes::uniform(10.0);
    let border = EdgeSizes::uniform(2.0);
    let margin = EdgeSizes::uniform(5.0);

    let box_model = BoxModel::new(content, padding, border, margin, BoxSizing::ContentBox);
    let margin_box = box_model.margin_box();

    // Width: 200 + 10*2 + 2*2 + 5*2 = 234
    // Height: 100 + 10*2 + 2*2 + 5*2 = 134
    assert_eq!(margin_box.width(), 234.0);
    assert_eq!(margin_box.height(), 134.0);
}
