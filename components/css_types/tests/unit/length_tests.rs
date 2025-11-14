use css_types::{CssError, CssValue, Length, LengthUnit};

#[cfg(test)]
mod length_parsing_tests {
    use super::*;

    #[test]
    fn test_parse_px_length() {
        let result = Length::parse("10px");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 10.0);
        assert_eq!(length.unit(), LengthUnit::Px);
    }

    #[test]
    fn test_parse_em_length() {
        let result = Length::parse("2.5em");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 2.5);
        assert_eq!(length.unit(), LengthUnit::Em);
    }

    #[test]
    fn test_parse_rem_length() {
        let result = Length::parse("1.5rem");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 1.5);
        assert_eq!(length.unit(), LengthUnit::Rem);
    }

    #[test]
    fn test_parse_percent_length() {
        let result = Length::parse("50%");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 50.0);
        assert_eq!(length.unit(), LengthUnit::Percent);
    }

    #[test]
    fn test_parse_vw_length() {
        let result = Length::parse("100vw");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 100.0);
        assert_eq!(length.unit(), LengthUnit::Vw);
    }

    #[test]
    fn test_parse_vh_length() {
        let result = Length::parse("50vh");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 50.0);
        assert_eq!(length.unit(), LengthUnit::Vh);
    }

    #[test]
    fn test_parse_negative_length() {
        let result = Length::parse("-10px");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), -10.0);
    }

    #[test]
    fn test_parse_zero_length() {
        let result = Length::parse("0px");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 0.0);
    }

    #[test]
    fn test_parse_length_with_whitespace() {
        let result = Length::parse(" 10px ");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 10.0);
    }

    #[test]
    fn test_parse_decimal_length() {
        let result = Length::parse("10.5px");
        assert!(result.is_ok());
        let length = result.unwrap();
        assert_eq!(length.value(), 10.5);
    }

    #[test]
    fn test_parse_invalid_unit() {
        let result = Length::parse("10foo");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_unit() {
        let result = Length::parse("10");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = Length::parse("notanumberpx");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let result = Length::parse("");
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod length_serialization_tests {
    use super::*;

    #[test]
    fn test_serialize_px_length() {
        let length = Length::new(10.0, LengthUnit::Px);
        assert_eq!(length.serialize(), "10px");
    }

    #[test]
    fn test_serialize_em_length() {
        let length = Length::new(2.5, LengthUnit::Em);
        assert_eq!(length.serialize(), "2.5em");
    }

    #[test]
    fn test_serialize_percent_length() {
        let length = Length::new(50.0, LengthUnit::Percent);
        assert_eq!(length.serialize(), "50%");
    }

    #[test]
    fn test_serialize_negative_length() {
        let length = Length::new(-10.0, LengthUnit::Px);
        assert_eq!(length.serialize(), "-10px");
    }

    #[test]
    fn test_roundtrip_parsing() {
        let original = "10.5px";
        let length = Length::parse(original).unwrap();
        let serialized = length.serialize();
        let reparsed = Length::parse(&serialized).unwrap();
        assert_eq!(length.value(), reparsed.value());
        assert_eq!(length.unit(), reparsed.unit());
    }
}

#[cfg(test)]
mod length_unit_tests {
    use super::*;

    #[test]
    fn test_length_unit_equality() {
        assert_eq!(LengthUnit::Px, LengthUnit::Px);
        assert_ne!(LengthUnit::Px, LengthUnit::Em);
    }

    #[test]
    fn test_length_equality() {
        let l1 = Length::new(10.0, LengthUnit::Px);
        let l2 = Length::new(10.0, LengthUnit::Px);
        assert_eq!(l1, l2);
    }

    #[test]
    fn test_length_inequality_different_value() {
        let l1 = Length::new(10.0, LengthUnit::Px);
        let l2 = Length::new(20.0, LengthUnit::Px);
        assert_ne!(l1, l2);
    }

    #[test]
    fn test_length_inequality_different_unit() {
        let l1 = Length::new(10.0, LengthUnit::Px);
        let l2 = Length::new(10.0, LengthUnit::Em);
        assert_ne!(l1, l2);
    }
}
