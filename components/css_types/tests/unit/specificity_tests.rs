use css_types::Specificity;

#[cfg(test)]
mod specificity_tests {
    use super::*;

    #[test]
    fn test_create_specificity() {
        let spec = Specificity::new(1, 2, 3);
        assert_eq!(spec.id_selectors(), 1);
        assert_eq!(spec.class_selectors(), 2);
        assert_eq!(spec.type_selectors(), 3);
    }

    #[test]
    fn test_zero_specificity() {
        let spec = Specificity::zero();
        assert_eq!(spec.id_selectors(), 0);
        assert_eq!(spec.class_selectors(), 0);
        assert_eq!(spec.type_selectors(), 0);
    }

    #[test]
    fn test_specificity_equality() {
        let spec1 = Specificity::new(1, 2, 3);
        let spec2 = Specificity::new(1, 2, 3);
        assert_eq!(spec1, spec2);
    }

    #[test]
    fn test_specificity_inequality() {
        let spec1 = Specificity::new(1, 2, 3);
        let spec2 = Specificity::new(1, 2, 4);
        assert_ne!(spec1, spec2);
    }

    #[test]
    fn test_specificity_comparison_id_wins() {
        let spec1 = Specificity::new(1, 0, 0);
        let spec2 = Specificity::new(0, 100, 100);
        assert!(spec1 > spec2);
    }

    #[test]
    fn test_specificity_comparison_class_wins() {
        let spec1 = Specificity::new(0, 1, 0);
        let spec2 = Specificity::new(0, 0, 100);
        assert!(spec1 > spec2);
    }

    #[test]
    fn test_specificity_comparison_type_matters() {
        let spec1 = Specificity::new(0, 0, 2);
        let spec2 = Specificity::new(0, 0, 1);
        assert!(spec1 > spec2);
    }

    #[test]
    fn test_specificity_comparison_equal() {
        let spec1 = Specificity::new(1, 2, 3);
        let spec2 = Specificity::new(1, 2, 3);
        assert!(!(spec1 > spec2));
        assert!(!(spec1 < spec2));
        assert!(spec1 >= spec2);
        assert!(spec1 <= spec2);
    }

    #[test]
    fn test_specificity_comparison_complex() {
        // (1, 2, 3) vs (1, 3, 0)
        let spec1 = Specificity::new(1, 2, 3);
        let spec2 = Specificity::new(1, 3, 0);
        assert!(spec2 > spec1);
    }

    #[test]
    fn test_specificity_ordering() {
        let mut specs = vec![
            Specificity::new(0, 0, 1),
            Specificity::new(1, 0, 0),
            Specificity::new(0, 1, 0),
            Specificity::new(0, 0, 2),
        ];
        specs.sort();
        assert_eq!(specs[0], Specificity::new(0, 0, 1));
        assert_eq!(specs[1], Specificity::new(0, 0, 2));
        assert_eq!(specs[2], Specificity::new(0, 1, 0));
        assert_eq!(specs[3], Specificity::new(1, 0, 0));
    }

    #[test]
    fn test_specificity_max() {
        let spec1 = Specificity::new(1, 2, 3);
        let spec2 = Specificity::new(0, 5, 1);
        assert_eq!(spec1.max(spec2), spec1);
    }

    #[test]
    fn test_specificity_min() {
        let spec1 = Specificity::new(1, 2, 3);
        let spec2 = Specificity::new(0, 5, 1);
        assert_eq!(spec1.min(spec2), spec2);
    }
}
