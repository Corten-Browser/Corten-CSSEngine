use css_cascade::{CascadeResolver, ComputedValues, PropertyId, PropertyValue};

#[test]
fn test_inherit_color() {
    let mut parent = ComputedValues::default();
    parent.set(PropertyId::Color, PropertyValue::Keyword("red".to_string()));

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // Color is an inherited property
    assert_eq!(
        child.get(&PropertyId::Color),
        Some(&PropertyValue::Keyword("red".to_string()))
    );
}

#[test]
fn test_inherit_font_size() {
    let mut parent = ComputedValues::default();
    parent.set(
        PropertyId::FontSize,
        PropertyValue::Length(16.0, "px".to_string()),
    );

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // FontSize is an inherited property
    assert_eq!(
        child.get(&PropertyId::FontSize),
        Some(&PropertyValue::Length(16.0, "px".to_string()))
    );
}

#[test]
fn test_no_inherit_margin() {
    let mut parent = ComputedValues::default();
    parent.set(
        PropertyId::Margin,
        PropertyValue::Length(10.0, "px".to_string()),
    );

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // Margin is NOT an inherited property
    assert_eq!(child.get(&PropertyId::Margin), None);
}

#[test]
fn test_no_inherit_padding() {
    let mut parent = ComputedValues::default();
    parent.set(
        PropertyId::Padding,
        PropertyValue::Length(5.0, "px".to_string()),
    );

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // Padding is NOT an inherited property
    assert_eq!(child.get(&PropertyId::Padding), None);
}

#[test]
fn test_no_inherit_border() {
    let mut parent = ComputedValues::default();
    parent.set(
        PropertyId::Border,
        PropertyValue::Border {
            width: 1.0,
            style: "solid".to_string(),
            color: "black".to_string(),
        },
    );

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // Border is NOT an inherited property
    assert_eq!(child.get(&PropertyId::Border), None);
}

#[test]
fn test_explicit_inherit_overrides() {
    let mut parent = ComputedValues::default();
    parent.set(
        PropertyId::Margin,
        PropertyValue::Length(10.0, "px".to_string()),
    );

    let mut child = ComputedValues::default();
    // Explicitly set to inherit
    child.set(PropertyId::Margin, PropertyValue::Inherit);

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // Should inherit even though margin is normally not inherited
    assert_eq!(
        child.get(&PropertyId::Margin),
        Some(&PropertyValue::Length(10.0, "px".to_string()))
    );
}

#[test]
fn test_inherit_font_family() {
    let mut parent = ComputedValues::default();
    parent.set(
        PropertyId::FontFamily,
        PropertyValue::FontFamily(vec!["Arial".to_string(), "sans-serif".to_string()]),
    );

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // FontFamily is an inherited property
    assert_eq!(
        child.get(&PropertyId::FontFamily),
        Some(&PropertyValue::FontFamily(vec![
            "Arial".to_string(),
            "sans-serif".to_string()
        ]))
    );
}

#[test]
fn test_child_overrides_inherited() {
    let mut parent = ComputedValues::default();
    parent.set(PropertyId::Color, PropertyValue::Keyword("red".to_string()));

    let mut child = ComputedValues::default();
    // Child explicitly sets its own color
    child.set(
        PropertyId::Color,
        PropertyValue::Keyword("blue".to_string()),
    );

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // Child's explicit value should not be overridden
    assert_eq!(
        child.get(&PropertyId::Color),
        Some(&PropertyValue::Keyword("blue".to_string()))
    );
}

#[test]
fn test_inherit_line_height() {
    let mut parent = ComputedValues::default();
    parent.set(PropertyId::LineHeight, PropertyValue::Number(1.5));

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // LineHeight is an inherited property
    assert_eq!(
        child.get(&PropertyId::LineHeight),
        Some(&PropertyValue::Number(1.5))
    );
}

#[test]
fn test_inherit_text_align() {
    let mut parent = ComputedValues::default();
    parent.set(
        PropertyId::TextAlign,
        PropertyValue::Keyword("center".to_string()),
    );

    let mut child = ComputedValues::default();

    CascadeResolver::apply_inheritance(&parent, &mut child);

    // TextAlign is an inherited property
    assert_eq!(
        child.get(&PropertyId::TextAlign),
        Some(&PropertyValue::Keyword("center".to_string()))
    );
}
