//! Integration tests for CSS media queries

use css_media_queries::*;

#[test]
fn test_complete_pipeline_parse_and_evaluate() {
    // Parse a media query
    let query = parse_media_query("screen and (min-width: 768px)").unwrap();

    // Create a desktop viewport
    let viewport = ViewportInfo::desktop();

    // Evaluate
    let result = evaluate_media_query(&query, &viewport);
    assert!(result);
}

#[test]
fn test_responsive_breakpoints() {
    // Mobile query
    let mobile_query = parse_media_query("(max-width: 767px)").unwrap();

    // Tablet query
    let tablet_query = parse_media_query("(min-width: 768px) and (max-width: 1023px)").unwrap();

    // Desktop query
    let desktop_query = parse_media_query("(min-width: 1024px)").unwrap();

    // Test mobile viewport
    let mobile = ViewportInfo::mobile();
    assert!(evaluate_media_query(&mobile_query, &mobile));
    assert!(!evaluate_media_query(&tablet_query, &mobile));
    assert!(!evaluate_media_query(&desktop_query, &mobile));

    // Test tablet viewport
    let tablet = ViewportInfo::tablet();
    assert!(!evaluate_media_query(&mobile_query, &tablet));
    assert!(evaluate_media_query(&tablet_query, &tablet));
    assert!(!evaluate_media_query(&desktop_query, &tablet));

    // Test desktop viewport
    let desktop = ViewportInfo::desktop();
    assert!(!evaluate_media_query(&mobile_query, &desktop));
    assert!(!evaluate_media_query(&tablet_query, &desktop));
    assert!(evaluate_media_query(&desktop_query, &desktop));
}

#[test]
fn test_media_query_list_evaluation() {
    let list = parse_media_query_list("print, (max-width: 767px)").unwrap();

    // Mobile should match second query
    let mobile = ViewportInfo::mobile();
    let evaluator = DefaultEvaluator;
    assert!(evaluator.evaluate_list(&list, &mobile));

    // Desktop should not match either query
    let desktop = ViewportInfo::desktop();
    assert!(!evaluator.evaluate_list(&list, &desktop));
}

#[test]
fn test_complex_query_with_multiple_conditions() {
    let query = parse_media_query(
        "screen and (min-width: 768px) and (max-width: 1023px) and (orientation: portrait)",
    )
    .unwrap();

    // Portrait tablet should match
    let viewport = ViewportInfo::new(768, 1024);
    assert!(evaluate_media_query(&query, &viewport));

    // Landscape tablet should not match (wrong orientation)
    let viewport = ViewportInfo::new(1024, 768);
    assert!(!evaluate_media_query(&query, &viewport));
}

#[test]
fn test_preference_features() {
    // Test color scheme preference
    let dark_query = parse_media_query("(prefers-color-scheme: dark)").unwrap();
    let light_query = parse_media_query("(prefers-color-scheme: light)").unwrap();

    let viewport = ViewportInfo::desktop();

    // Default should be light
    assert!(!evaluate_media_query(&dark_query, &viewport));
    assert!(evaluate_media_query(&light_query, &viewport));
}

#[test]
fn test_dynamic_viewport_updates() {
    let query = parse_media_query("(min-width: 768px)").unwrap();

    // Start with mobile
    let mut viewport = ViewportInfo::mobile();
    assert!(!evaluate_media_query(&query, &viewport));

    // Resize to desktop
    viewport.width = 1920;
    viewport.height = 1080;
    viewport.orientation = Orientation::Landscape;
    assert!(evaluate_media_query(&query, &viewport));
}

#[test]
fn test_resolution_queries() {
    let high_res_query = parse_media_query("(min-resolution: 192dpi)").unwrap();

    // Standard resolution viewport
    let standard = ViewportInfo::desktop();
    assert!(!evaluate_media_query(&high_res_query, &standard));

    // High resolution viewport (retina display)
    let mut retina = ViewportInfo::desktop();
    retina.device_pixel_ratio = 2.0;
    retina.resolution_dpi = 192.0;
    assert!(evaluate_media_query(&high_res_query, &retina));
}

#[test]
fn test_aspect_ratio_queries() {
    let widescreen_query = parse_media_query("(aspect-ratio: 16/9)").unwrap();

    // 16:9 viewport
    let widescreen = ViewportInfo::new(1920, 1080);
    assert!(evaluate_media_query(&widescreen_query, &widescreen));

    // 4:3 viewport
    let standard = ViewportInfo::new(1024, 768);
    assert!(!evaluate_media_query(&widescreen_query, &standard));
}

#[test]
fn test_hover_and_pointer_queries() {
    let hover_query = parse_media_query("(hover: hover)").unwrap();
    let no_hover_query = parse_media_query("(hover: none)").unwrap();

    // Desktop has hover
    let desktop = ViewportInfo::desktop();
    assert!(evaluate_media_query(&hover_query, &desktop));
    assert!(!evaluate_media_query(&no_hover_query, &desktop));

    // Mobile has no hover
    let mobile = ViewportInfo::mobile();
    assert!(!evaluate_media_query(&hover_query, &mobile));
    assert!(evaluate_media_query(&no_hover_query, &mobile));
}

#[test]
fn test_not_operator() {
    let not_mobile_query = parse_media_query("not all and (max-width: 767px)").unwrap();

    // Desktop should match (NOT mobile)
    let desktop = ViewportInfo::desktop();
    assert!(evaluate_media_query(&not_mobile_query, &desktop));

    // Mobile should not match
    let mobile = ViewportInfo::mobile();
    assert!(!evaluate_media_query(&not_mobile_query, &mobile));
}

#[test]
fn test_or_operator_in_list() {
    // Comma-separated queries act as OR
    let list = parse_media_query_list("(max-width: 767px), (min-width: 1024px)").unwrap();

    let evaluator = DefaultEvaluator;

    // Mobile matches first query
    let mobile = ViewportInfo::mobile();
    assert!(evaluator.evaluate_list(&list, &mobile));

    // Tablet matches neither
    let tablet = ViewportInfo::tablet();
    assert!(!evaluator.evaluate_list(&list, &tablet));

    // Desktop matches second query
    let desktop = ViewportInfo::desktop();
    assert!(evaluator.evaluate_list(&list, &desktop));
}

#[test]
fn test_boolean_features() {
    let color_query = parse_media_query("(color)").unwrap();
    let monochrome_query = parse_media_query("(monochrome)").unwrap();

    let viewport = ViewportInfo::desktop();

    // Desktop has color
    assert!(evaluate_media_query(&color_query, &viewport));

    // Desktop is not monochrome
    assert!(!evaluate_media_query(&monochrome_query, &viewport));
}

#[test]
fn test_real_world_responsive_design() {
    // Common responsive breakpoints
    let mobile_first =
        parse_media_query_list("(max-width: 575px), (min-width: 576px) and (max-width: 767px)")
            .unwrap();

    let tablet = parse_media_query_list(
        "(min-width: 768px) and (max-width: 991px), (min-width: 992px) and (max-width: 1199px)",
    )
    .unwrap();

    let desktop = parse_media_query_list("(min-width: 1200px)").unwrap();

    let evaluator = DefaultEvaluator;

    // Test iPhone SE (375x667)
    let iphone = ViewportInfo::new(375, 667);
    assert!(evaluator.evaluate_list(&mobile_first, &iphone));
    assert!(!evaluator.evaluate_list(&tablet, &iphone));
    assert!(!evaluator.evaluate_list(&desktop, &iphone));

    // Test iPad (768x1024)
    let ipad = ViewportInfo::new(768, 1024);
    assert!(!evaluator.evaluate_list(&mobile_first, &ipad));
    assert!(evaluator.evaluate_list(&tablet, &ipad));
    assert!(!evaluator.evaluate_list(&desktop, &ipad));

    // Test desktop (1920x1080)
    let desktop_vp = ViewportInfo::new(1920, 1080);
    assert!(!evaluator.evaluate_list(&mobile_first, &desktop_vp));
    assert!(!evaluator.evaluate_list(&tablet, &desktop_vp));
    assert!(evaluator.evaluate_list(&desktop, &desktop_vp));
}
