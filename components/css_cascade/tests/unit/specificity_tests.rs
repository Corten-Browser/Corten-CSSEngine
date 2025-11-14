use css_cascade::{CascadeResolver, Specificity};

#[test]
fn test_specificity_equality() {
    let spec1 = Specificity::new(1, 2, 3);
    let spec2 = Specificity::new(1, 2, 3);
    assert_eq!(spec1, spec2);
}

#[test]
fn test_specificity_ordering_by_id() {
    let spec_high_id = Specificity::new(2, 0, 0);
    let spec_low_id = Specificity::new(1, 5, 5);
    assert!(spec_high_id > spec_low_id);
}

#[test]
fn test_specificity_ordering_by_class() {
    let spec_high_class = Specificity::new(1, 3, 0);
    let spec_low_class = Specificity::new(1, 2, 5);
    assert!(spec_high_class > spec_low_class);
}

#[test]
fn test_specificity_ordering_by_element() {
    let spec_high_element = Specificity::new(1, 2, 4);
    let spec_low_element = Specificity::new(1, 2, 3);
    assert!(spec_high_element > spec_low_element);
}

#[test]
fn test_specificity_zero() {
    let spec = Specificity::new(0, 0, 0);
    assert_eq!(spec, Specificity::new(0, 0, 0));
}

#[cfg(test)]
mod compute_specificity_tests {
    use super::*;
    use css_cascade::Selector;

    #[test]
    fn test_type_selector_specificity() {
        // "div" should have specificity (0,0,1)
        let selector = Selector::Type("div".to_string());
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 0, 1));
    }

    #[test]
    fn test_class_selector_specificity() {
        // ".button" should have specificity (0,1,0)
        let selector = Selector::Class("button".to_string());
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 1, 0));
    }

    #[test]
    fn test_id_selector_specificity() {
        // "#header" should have specificity (1,0,0)
        let selector = Selector::Id("header".to_string());
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(1, 0, 0));
    }

    #[test]
    fn test_compound_selector_specificity() {
        // "div.button" should have specificity (0,1,1)
        let selector = Selector::Compound(vec![
            Selector::Type("div".to_string()),
            Selector::Class("button".to_string()),
        ]);
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 1, 1));
    }

    #[test]
    fn test_complex_selector_specificity() {
        // "div#main .button" should have specificity (1,1,1)
        let selector = Selector::Descendant(
            Box::new(Selector::Compound(vec![
                Selector::Type("div".to_string()),
                Selector::Id("main".to_string()),
            ])),
            Box::new(Selector::Class("button".to_string())),
        );
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(1, 1, 1));
    }

    #[test]
    fn test_pseudo_class_specificity() {
        // ":hover" should have specificity (0,1,0)
        let selector = Selector::PseudoClass("hover".to_string());
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 1, 0));
    }

    #[test]
    fn test_attribute_selector_specificity() {
        // "[type='text']" should have specificity (0,1,0)
        let selector = Selector::Attribute {
            name: "type".to_string(),
            value: Some("text".to_string()),
        };
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 1, 0));
    }

    #[test]
    fn test_universal_selector_specificity() {
        // "*" should have specificity (0,0,0)
        let selector = Selector::Universal;
        let spec = CascadeResolver::compute_specificity(&selector);
        assert_eq!(spec, Specificity::new(0, 0, 0));
    }
}
