use css_types::{Color, CssError, CssValue};

#[cfg(test)]
mod color_parsing_tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_6_digits() {
        let result = Color::parse("#FF5733");
        assert!(result.is_ok());
        let color = result.unwrap();
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 87);
        assert_eq!(color.b(), 51);
        assert_eq!(color.a(), 1.0);
    }

    #[test]
    fn test_parse_hex_color_3_digits() {
        let result = Color::parse("#F53");
        assert!(result.is_ok());
        let color = result.unwrap();
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 85);
        assert_eq!(color.b(), 51);
        assert_eq!(color.a(), 1.0);
    }

    #[test]
    fn test_parse_hex_color_lowercase() {
        let result = Color::parse("#ff5733");
        assert!(result.is_ok());
        let color = result.unwrap();
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 87);
        assert_eq!(color.b(), 51);
    }

    #[test]
    fn test_parse_rgb_color() {
        let result = Color::parse("rgb(255, 87, 51)");
        assert!(result.is_ok());
        let color = result.unwrap();
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 87);
        assert_eq!(color.b(), 51);
        assert_eq!(color.a(), 1.0);
    }

    #[test]
    fn test_parse_rgba_color() {
        let result = Color::parse("rgba(255, 87, 51, 0.5)");
        assert!(result.is_ok());
        let color = result.unwrap();
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 87);
        assert_eq!(color.b(), 51);
        assert_eq!(color.a(), 0.5);
    }

    #[test]
    fn test_parse_rgb_with_whitespace() {
        let result = Color::parse("rgb( 255 , 87 , 51 )");
        assert!(result.is_ok());
        let color = result.unwrap();
        assert_eq!(color.r(), 255);
    }

    #[test]
    fn test_parse_invalid_hex_color() {
        let result = Color::parse("#GGGGGG");
        assert!(result.is_err());
        match result {
            Err(CssError::ParseError(_)) => (),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_parse_invalid_hex_length() {
        let result = Color::parse("#FF");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rgb_out_of_range() {
        let result = Color::parse("rgb(256, 0, 0)");
        assert!(result.is_err());
        match result {
            Err(CssError::OutOfRange(_)) => (),
            _ => panic!("Expected OutOfRange error"),
        }
    }

    #[test]
    fn test_parse_rgba_out_of_range_alpha() {
        let result = Color::parse("rgba(255, 0, 0, 1.5)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_hash() {
        let result = Color::parse("FF5733");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let result = Color::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = Color::parse("not-a-color");
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod color_serialization_tests {
    use super::*;

    #[test]
    fn test_serialize_rgb_color() {
        let color = Color::rgb(255, 87, 51);
        assert_eq!(color.serialize(), "rgb(255, 87, 51)");
    }

    #[test]
    fn test_serialize_rgba_color() {
        let color = Color::rgba(255, 87, 51, 0.5);
        assert_eq!(color.serialize(), "rgba(255, 87, 51, 0.5)");
    }

    #[test]
    fn test_serialize_rgba_full_opacity() {
        let color = Color::rgba(255, 87, 51, 1.0);
        // Should serialize as rgb when alpha is 1.0
        assert_eq!(color.serialize(), "rgb(255, 87, 51)");
    }

    #[test]
    fn test_roundtrip_parsing() {
        let original = "rgb(255, 87, 51)";
        let color = Color::parse(original).unwrap();
        let serialized = color.serialize();
        let reparsed = Color::parse(&serialized).unwrap();
        assert_eq!(color.r(), reparsed.r());
        assert_eq!(color.g(), reparsed.g());
        assert_eq!(color.b(), reparsed.b());
        assert_eq!(color.a(), reparsed.a());
    }
}

#[cfg(test)]
mod color_construction_tests {
    use super::*;

    #[test]
    fn test_create_rgb_color() {
        let color = Color::rgb(255, 87, 51);
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 87);
        assert_eq!(color.b(), 51);
        assert_eq!(color.a(), 1.0);
    }

    #[test]
    fn test_create_rgba_color() {
        let color = Color::rgba(255, 87, 51, 0.5);
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 87);
        assert_eq!(color.b(), 51);
        assert_eq!(color.a(), 0.5);
    }

    #[test]
    fn test_color_equality() {
        let color1 = Color::rgb(255, 87, 51);
        let color2 = Color::rgb(255, 87, 51);
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_color_inequality() {
        let color1 = Color::rgb(255, 87, 51);
        let color2 = Color::rgb(255, 87, 52);
        assert_ne!(color1, color2);
    }
}
