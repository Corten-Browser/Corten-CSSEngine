//! Unit tests for CSS multi-column parsing functions

use css_layout_multicolumn::*;
use css_types::LengthUnit;

// ============================================================================
// Column Count Parsing Tests
// ============================================================================

#[test]
fn test_parse_column_count_auto() {
    let result = parse_column_count("auto");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ColumnCount::Auto);
}

#[test]
fn test_parse_column_count_integer() {
    let result = parse_column_count("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ColumnCount::Count(3));
}

#[test]
fn test_parse_column_count_large_number() {
    let result = parse_column_count("10");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ColumnCount::Count(10));
}

#[test]
fn test_parse_column_count_whitespace() {
    let result = parse_column_count("  5  ");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ColumnCount::Count(5));
}

#[test]
fn test_parse_column_count_zero_invalid() {
    let result = parse_column_count("0");
    assert!(result.is_err());
}

#[test]
fn test_parse_column_count_negative_invalid() {
    let result = parse_column_count("-1");
    assert!(result.is_err());
}

#[test]
fn test_parse_column_count_invalid_string() {
    let result = parse_column_count("invalid");
    assert!(result.is_err());
}

#[test]
fn test_parse_column_count_float_invalid() {
    let result = parse_column_count("3.5");
    assert!(result.is_err());
}

// ============================================================================
// Column Width Parsing Tests
// ============================================================================

#[test]
fn test_parse_column_width_auto() {
    let result = parse_column_width("auto");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ColumnWidth::Auto);
}

#[test]
fn test_parse_column_width_px() {
    let result = parse_column_width("200px");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnWidth::Length(length) => {
            assert_eq!(length.value(), 200.0);
            assert_eq!(length.unit(), LengthUnit::Px);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_width_em() {
    let result = parse_column_width("15em");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnWidth::Length(length) => {
            assert_eq!(length.value(), 15.0);
            assert_eq!(length.unit(), LengthUnit::Em);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_width_rem() {
    let result = parse_column_width("12rem");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnWidth::Length(length) => {
            assert_eq!(length.value(), 12.0);
            assert_eq!(length.unit(), LengthUnit::Rem);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_width_percent() {
    let result = parse_column_width("50%");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnWidth::Length(length) => {
            assert_eq!(length.value(), 50.0);
            assert_eq!(length.unit(), LengthUnit::Percent);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_width_whitespace() {
    let result = parse_column_width("  100px  ");
    assert!(result.is_ok());
}

#[test]
fn test_parse_column_width_invalid() {
    let result = parse_column_width("invalid");
    assert!(result.is_err());
}

#[test]
fn test_parse_column_width_no_unit() {
    let result = parse_column_width("200");
    assert!(result.is_err());
}

// ============================================================================
// Column Gap Parsing Tests
// ============================================================================

#[test]
fn test_parse_column_gap_normal() {
    let result = parse_column_gap("normal");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ColumnGap::Normal);
}

#[test]
fn test_parse_column_gap_px() {
    let result = parse_column_gap("20px");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnGap::Length(length) => {
            assert_eq!(length.value(), 20.0);
            assert_eq!(length.unit(), LengthUnit::Px);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_gap_em() {
    let result = parse_column_gap("1em");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnGap::Length(length) => {
            assert_eq!(length.value(), 1.0);
            assert_eq!(length.unit(), LengthUnit::Em);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_gap_rem() {
    let result = parse_column_gap("2rem");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnGap::Length(length) => {
            assert_eq!(length.value(), 2.0);
            assert_eq!(length.unit(), LengthUnit::Rem);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_gap_zero() {
    let result = parse_column_gap("0px");
    assert!(result.is_ok());
    match result.unwrap() {
        ColumnGap::Length(length) => {
            assert_eq!(length.value(), 0.0);
        }
        _ => panic!("Expected Length variant"),
    }
}

#[test]
fn test_parse_column_gap_whitespace() {
    let result = parse_column_gap("  normal  ");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ColumnGap::Normal);
}

#[test]
fn test_parse_column_gap_invalid() {
    let result = parse_column_gap("invalid");
    assert!(result.is_err());
}

// ============================================================================
// Column Rule Parsing Tests
// ============================================================================

#[test]
fn test_parse_column_rule_basic() {
    let result = parse_column_rule("1px solid #000000");
    assert!(result.is_ok());
    let rule = result.unwrap();
    assert_eq!(rule.width.value(), 1.0);
    assert_eq!(rule.width.unit(), LengthUnit::Px);
    assert_eq!(rule.style, BorderStyle::Solid);
    assert_eq!(rule.color.r(), 0);
    assert_eq!(rule.color.g(), 0);
    assert_eq!(rule.color.b(), 0);
}

#[test]
fn test_parse_column_rule_dashed() {
    let result = parse_column_rule("2px dashed #FF0000");
    assert!(result.is_ok());
    let rule = result.unwrap();
    assert_eq!(rule.width.value(), 2.0);
    assert_eq!(rule.style, BorderStyle::Dashed);
    assert_eq!(rule.color.r(), 255);
    assert_eq!(rule.color.g(), 0);
    assert_eq!(rule.color.b(), 0);
}

#[test]
fn test_parse_column_rule_dotted() {
    let result = parse_column_rule("1px dotted #00FF00");
    assert!(result.is_ok());
    let rule = result.unwrap();
    assert_eq!(rule.style, BorderStyle::Dotted);
    assert_eq!(rule.color.r(), 0);
    assert_eq!(rule.color.g(), 255);
    assert_eq!(rule.color.b(), 0);
}

#[test]
fn test_parse_column_rule_double() {
    let result = parse_column_rule("3px double #0000FF");
    assert!(result.is_ok());
    let rule = result.unwrap();
    assert_eq!(rule.style, BorderStyle::Double);
}

#[test]
fn test_parse_column_rule_rgb_color() {
    let result = parse_column_rule("1px solid rgb(128, 128, 128)");
    assert!(result.is_ok());
    let rule = result.unwrap();
    assert_eq!(rule.color.r(), 128);
    assert_eq!(rule.color.g(), 128);
    assert_eq!(rule.color.b(), 128);
}

#[test]
fn test_parse_column_rule_em_width() {
    let result = parse_column_rule("0.5em solid #000000");
    assert!(result.is_ok());
    let rule = result.unwrap();
    assert_eq!(rule.width.value(), 0.5);
    assert_eq!(rule.width.unit(), LengthUnit::Em);
}

#[test]
fn test_parse_column_rule_missing_parts() {
    let result = parse_column_rule("1px solid");
    assert!(result.is_err());
}

#[test]
fn test_parse_column_rule_invalid_width() {
    let result = parse_column_rule("invalid solid #000000");
    assert!(result.is_err());
}

#[test]
fn test_parse_column_rule_invalid_style() {
    let result = parse_column_rule("1px invalid #000000");
    assert!(result.is_err());
}

#[test]
fn test_parse_column_rule_invalid_color() {
    let result = parse_column_rule("1px solid invalid");
    assert!(result.is_err());
}

// ============================================================================
// Border Style Parsing Tests
// ============================================================================

#[test]
fn test_border_style_none() {
    let result = BorderStyle::parse("none");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::None);
}

#[test]
fn test_border_style_solid() {
    let result = BorderStyle::parse("solid");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Solid);
}

#[test]
fn test_border_style_dashed() {
    let result = BorderStyle::parse("dashed");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Dashed);
}

#[test]
fn test_border_style_dotted() {
    let result = BorderStyle::parse("dotted");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Dotted);
}

#[test]
fn test_border_style_double() {
    let result = BorderStyle::parse("double");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Double);
}

#[test]
fn test_border_style_groove() {
    let result = BorderStyle::parse("groove");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Groove);
}

#[test]
fn test_border_style_ridge() {
    let result = BorderStyle::parse("ridge");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Ridge);
}

#[test]
fn test_border_style_inset() {
    let result = BorderStyle::parse("inset");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Inset);
}

#[test]
fn test_border_style_outset() {
    let result = BorderStyle::parse("outset");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Outset);
}

#[test]
fn test_border_style_case_insensitive() {
    let result = BorderStyle::parse("SOLID");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Solid);
}

#[test]
fn test_border_style_whitespace() {
    let result = BorderStyle::parse("  solid  ");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BorderStyle::Solid);
}

#[test]
fn test_border_style_invalid() {
    let result = BorderStyle::parse("invalid");
    assert!(result.is_err());
}
