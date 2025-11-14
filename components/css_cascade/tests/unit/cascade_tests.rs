use css_cascade::{
    ApplicableRule, CascadeResolver, Origin, PropertyId, PropertyValue,
    Specificity, StyleRule,
};

#[test]
fn test_origin_ordering() {
    // User agent < User < Author
    assert!(Origin::UserAgent < Origin::User);
    assert!(Origin::User < Origin::Author);
    assert!(Origin::UserAgent < Origin::Author);
}

#[test]
fn test_applicable_rule_creation() {
    let rule = StyleRule {
        declarations: vec![(PropertyId::Color, PropertyValue::Keyword("red".to_string()))],
    };
    let applicable = ApplicableRule {
        rule,
        specificity: Specificity::new(0, 1, 0),
        origin: Origin::Author,
        source_order: 0,
    };
    assert_eq!(applicable.specificity, Specificity::new(0, 1, 0));
    assert_eq!(applicable.origin, Origin::Author);
}

#[test]
fn test_cascade_single_rule() {
    let resolver = CascadeResolver::new();

    let rule = StyleRule {
        declarations: vec![
            (PropertyId::Color, PropertyValue::Keyword("red".to_string())),
            (
                PropertyId::FontSize,
                PropertyValue::Length(16.0, "px".to_string()),
            ),
        ],
    };

    let applicable_rules = vec![ApplicableRule {
        rule,
        specificity: Specificity::new(0, 1, 0),
        origin: Origin::Author,
        source_order: 0,
    }];

    let result = resolver.resolve(&applicable_rules);

    assert_eq!(result.properties.len(), 2);
    assert!(result.properties.contains_key(&PropertyId::Color));
    assert!(result.properties.contains_key(&PropertyId::FontSize));
}

#[test]
fn test_cascade_specificity_override() {
    let resolver = CascadeResolver::new();

    // Lower specificity rule (comes first)
    let rule1 = StyleRule {
        declarations: vec![(PropertyId::Color, PropertyValue::Keyword("red".to_string()))],
    };

    // Higher specificity rule (should win)
    let rule2 = StyleRule {
        declarations: vec![(
            PropertyId::Color,
            PropertyValue::Keyword("blue".to_string()),
        )],
    };

    let applicable_rules = vec![
        ApplicableRule {
            rule: rule1,
            specificity: Specificity::new(0, 1, 0), // .class
            origin: Origin::Author,
            source_order: 0,
        },
        ApplicableRule {
            rule: rule2,
            specificity: Specificity::new(1, 0, 0), // #id
            origin: Origin::Author,
            source_order: 1,
        },
    ];

    let result = resolver.resolve(&applicable_rules);

    // Should pick blue from higher specificity rule
    assert_eq!(
        result.properties.get(&PropertyId::Color),
        Some(&PropertyValue::Keyword("blue".to_string()))
    );
}

#[test]
fn test_cascade_source_order() {
    let resolver = CascadeResolver::new();

    // Same specificity, different source order
    let rule1 = StyleRule {
        declarations: vec![(PropertyId::Color, PropertyValue::Keyword("red".to_string()))],
    };

    let rule2 = StyleRule {
        declarations: vec![(
            PropertyId::Color,
            PropertyValue::Keyword("blue".to_string()),
        )],
    };

    let applicable_rules = vec![
        ApplicableRule {
            rule: rule1,
            specificity: Specificity::new(0, 1, 0),
            origin: Origin::Author,
            source_order: 0,
        },
        ApplicableRule {
            rule: rule2,
            specificity: Specificity::new(0, 1, 0),
            origin: Origin::Author,
            source_order: 1, // Later in source order
        },
    ];

    let result = resolver.resolve(&applicable_rules);

    // Later rule should win with same specificity
    assert_eq!(
        result.properties.get(&PropertyId::Color),
        Some(&PropertyValue::Keyword("blue".to_string()))
    );
}

#[test]
fn test_cascade_origin_override() {
    let resolver = CascadeResolver::new();

    // User agent stylesheet
    let rule1 = StyleRule {
        declarations: vec![(
            PropertyId::Color,
            PropertyValue::Keyword("black".to_string()),
        )],
    };

    // Author stylesheet (should win even with lower specificity)
    let rule2 = StyleRule {
        declarations: vec![(PropertyId::Color, PropertyValue::Keyword("red".to_string()))],
    };

    let applicable_rules = vec![
        ApplicableRule {
            rule: rule1,
            specificity: Specificity::new(1, 0, 0), // Higher specificity
            origin: Origin::UserAgent,
            source_order: 0,
        },
        ApplicableRule {
            rule: rule2,
            specificity: Specificity::new(0, 1, 0), // Lower specificity
            origin: Origin::Author,
            source_order: 1,
        },
    ];

    let result = resolver.resolve(&applicable_rules);

    // Author origin should win
    assert_eq!(
        result.properties.get(&PropertyId::Color),
        Some(&PropertyValue::Keyword("red".to_string()))
    );
}

#[test]
fn test_cascade_important_flag() {
    let resolver = CascadeResolver::new();

    // Normal declaration
    let rule1 = StyleRule {
        declarations: vec![(PropertyId::Color, PropertyValue::Keyword("red".to_string()))],
    };

    // Important declaration (should win even with lower specificity)
    let rule2 = StyleRule {
        declarations: vec![(
            PropertyId::Color,
            PropertyValue::Important(Box::new(PropertyValue::Keyword("blue".to_string()))),
        )],
    };

    let applicable_rules = vec![
        ApplicableRule {
            rule: rule1,
            specificity: Specificity::new(1, 0, 0), // Higher specificity
            origin: Origin::Author,
            source_order: 1,
        },
        ApplicableRule {
            rule: rule2,
            specificity: Specificity::new(0, 1, 0), // Lower specificity
            origin: Origin::Author,
            source_order: 0,
        },
    ];

    let result = resolver.resolve(&applicable_rules);

    // Important declaration should win
    match result.properties.get(&PropertyId::Color) {
        Some(PropertyValue::Important(val)) => {
            assert_eq!(**val, PropertyValue::Keyword("blue".to_string()));
        }
        _ => panic!("Expected important value"),
    }
}

#[test]
fn test_cascade_merge_multiple_properties() {
    let resolver = CascadeResolver::new();

    let rule1 = StyleRule {
        declarations: vec![
            (PropertyId::Color, PropertyValue::Keyword("red".to_string())),
            (
                PropertyId::FontSize,
                PropertyValue::Length(14.0, "px".to_string()),
            ),
        ],
    };

    let rule2 = StyleRule {
        declarations: vec![
            (
                PropertyId::Color,
                PropertyValue::Keyword("blue".to_string()),
            ),
            (
                PropertyId::Margin,
                PropertyValue::Length(10.0, "px".to_string()),
            ),
        ],
    };

    let applicable_rules = vec![
        ApplicableRule {
            rule: rule1,
            specificity: Specificity::new(0, 1, 0),
            origin: Origin::Author,
            source_order: 0,
        },
        ApplicableRule {
            rule: rule2,
            specificity: Specificity::new(0, 1, 1),
            origin: Origin::Author,
            source_order: 1,
        },
    ];

    let result = resolver.resolve(&applicable_rules);

    // Color should be from rule2 (higher specificity)
    assert_eq!(
        result.properties.get(&PropertyId::Color),
        Some(&PropertyValue::Keyword("blue".to_string()))
    );
    // FontSize should be from rule1 (only rule with it)
    assert_eq!(
        result.properties.get(&PropertyId::FontSize),
        Some(&PropertyValue::Length(14.0, "px".to_string()))
    );
    // Margin should be from rule2 (only rule with it)
    assert_eq!(
        result.properties.get(&PropertyId::Margin),
        Some(&PropertyValue::Length(10.0, "px".to_string()))
    );
}

#[test]
fn test_empty_rules() {
    let resolver = CascadeResolver::new();
    let result = resolver.resolve(&[]);
    assert!(result.properties.is_empty());
}
