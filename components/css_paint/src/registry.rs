//! Paint Worklet Registry
//!
//! This module provides the registry for CSS Paint worklets, equivalent to
//! the `CSS.paintWorklet.addModule()` and `registerPaint()` APIs.

use crate::{PaintArgumentType, PaintContext, PaintError, PaintSize, PaintValue, PaintWorklet};
use std::collections::HashMap;
use std::sync::Arc;

/// A boxed paint worklet that can be stored in the registry
pub type BoxedWorklet = Arc<dyn PaintWorklet + Send + Sync>;

/// Registry for CSS Paint worklets
///
/// This registry stores paint worklets that can be used in CSS via the
/// `paint()` function.
///
/// # Example
///
/// ```rust
/// use css_paint::{PaintWorkletRegistry, PaintWorklet, PaintContext, PaintSize, PaintValue, PaintArgumentType};
/// use std::sync::Arc;
///
/// struct MyWorklet;
///
/// impl PaintWorklet for MyWorklet {
///     fn name(&self) -> &str { "my-worklet" }
///     fn input_properties(&self) -> Vec<String> { vec![] }
///     fn input_arguments(&self) -> Vec<PaintArgumentType> { vec![] }
///     fn paint(&self, ctx: &mut PaintContext, size: PaintSize, args: &[PaintValue]) {
///         ctx.fill_rect(0.0, 0.0, size.width, size.height);
///     }
/// }
///
/// let mut registry = PaintWorkletRegistry::new();
/// registry.register(Arc::new(MyWorklet)).unwrap();
///
/// let worklet = registry.get("my-worklet").unwrap();
/// ```
#[derive(Default)]
pub struct PaintWorkletRegistry {
    /// Registered worklets by name
    worklets: HashMap<String, BoxedWorklet>,
}

impl PaintWorkletRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            worklets: HashMap::new(),
        }
    }

    /// Create a registry with built-in worklets pre-registered
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register_builtins();
        registry
    }

    /// Register a paint worklet
    ///
    /// Equivalent to `registerPaint(name, class)` in JavaScript.
    ///
    /// # Errors
    ///
    /// Returns an error if a worklet with the same name is already registered.
    pub fn register(&mut self, worklet: BoxedWorklet) -> Result<(), PaintError> {
        let name = worklet.name().to_string();

        if self.worklets.contains_key(&name) {
            return Err(PaintError::WorkletAlreadyRegistered(name));
        }

        self.worklets.insert(name, worklet);
        Ok(())
    }

    /// Register a paint worklet, replacing any existing worklet with the same name
    pub fn register_or_replace(&mut self, worklet: BoxedWorklet) {
        let name = worklet.name().to_string();
        self.worklets.insert(name, worklet);
    }

    /// Unregister a paint worklet by name
    ///
    /// Returns `true` if a worklet was removed, `false` if no worklet was found.
    pub fn unregister(&mut self, name: &str) -> bool {
        self.worklets.remove(name).is_some()
    }

    /// Get a paint worklet by name
    pub fn get(&self, name: &str) -> Option<&BoxedWorklet> {
        self.worklets.get(name)
    }

    /// Check if a worklet is registered
    pub fn contains(&self, name: &str) -> bool {
        self.worklets.contains_key(name)
    }

    /// Get all registered worklet names
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.worklets.keys().map(|s| s.as_str())
    }

    /// Get the number of registered worklets
    pub fn len(&self) -> usize {
        self.worklets.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.worklets.is_empty()
    }

    /// Clear all registered worklets
    pub fn clear(&mut self) {
        self.worklets.clear();
    }

    /// Execute a paint worklet
    ///
    /// # Arguments
    ///
    /// * `name` - The worklet name
    /// * `size` - The size of the paint area
    /// * `args` - The arguments passed to the paint() function
    ///
    /// # Returns
    ///
    /// A `PaintContext` with the recorded operations, or an error.
    pub fn execute(
        &self,
        name: &str,
        size: PaintSize,
        args: &[PaintValue],
    ) -> Result<PaintContext, PaintError> {
        let worklet = self
            .get(name)
            .ok_or_else(|| PaintError::WorkletNotFound(name.to_string()))?;

        // Validate argument count
        let expected_args = worklet.input_arguments();
        if args.len() != expected_args.len() {
            return Err(PaintError::ArgumentCountMismatch {
                expected: expected_args.len(),
                got: args.len(),
            });
        }

        // Validate argument types
        for (i, (arg, expected_type)) in args.iter().zip(expected_args.iter()).enumerate() {
            if arg.argument_type() != *expected_type {
                return Err(PaintError::InvalidArgumentType {
                    expected: *expected_type,
                    got: format!("argument {} has type {:?}", i, arg.argument_type()),
                });
            }
        }

        // Execute the paint worklet
        let mut ctx = PaintContext::new();
        worklet.paint(&mut ctx, size, args);

        Ok(ctx)
    }

    /// Register built-in worklets
    fn register_builtins(&mut self) {
        // Register built-in example worklets
        let _ = self.register(Arc::new(CheckerboardWorklet));
        let _ = self.register(Arc::new(CircleWorklet));
        let _ = self.register(Arc::new(RoughBackgroundWorklet));
    }
}

impl std::fmt::Debug for PaintWorkletRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaintWorkletRegistry")
            .field("worklet_count", &self.worklets.len())
            .field("worklet_names", &self.names().collect::<Vec<_>>())
            .finish()
    }
}

// ============================================================================
// Built-in Worklets
// ============================================================================

/// A checkerboard pattern worklet
///
/// Usage: `background: paint(checkerboard, 20, #ff0000, #0000ff)`
/// Arguments: size (number), color1, color2
pub struct CheckerboardWorklet;

impl PaintWorklet for CheckerboardWorklet {
    fn name(&self) -> &str {
        "checkerboard"
    }

    fn input_properties(&self) -> Vec<String> {
        vec![
            "--checkerboard-size".to_string(),
            "--checkerboard-color1".to_string(),
            "--checkerboard-color2".to_string(),
        ]
    }

    fn input_arguments(&self) -> Vec<PaintArgumentType> {
        vec![
            PaintArgumentType::Number, // cell size
            PaintArgumentType::Color,  // color 1
            PaintArgumentType::Color,  // color 2
        ]
    }

    fn paint(&self, ctx: &mut PaintContext, size: PaintSize, args: &[PaintValue]) {
        let cell_size = args.first().and_then(|a| a.as_number()).unwrap_or(20.0);
        let color1 = args
            .get(1)
            .and_then(|a| a.as_color())
            .cloned()
            .unwrap_or_else(|| css_types::Color::rgb(255, 255, 255));
        let color2 = args
            .get(2)
            .and_then(|a| a.as_color())
            .cloned()
            .unwrap_or_else(|| css_types::Color::rgb(0, 0, 0));

        let cols = (size.width / cell_size).ceil() as i32;
        let rows = (size.height / cell_size).ceil() as i32;

        for row in 0..rows {
            for col in 0..cols {
                let color = if (row + col) % 2 == 0 { color1 } else { color2 };
                ctx.set_fill_color(color);
                ctx.fill_rect(
                    col as f32 * cell_size,
                    row as f32 * cell_size,
                    cell_size,
                    cell_size,
                );
            }
        }
    }
}

/// A simple circle worklet
///
/// Usage: `background: paint(circle, #ff0000)`
/// Arguments: color
pub struct CircleWorklet;

impl PaintWorklet for CircleWorklet {
    fn name(&self) -> &str {
        "circle"
    }

    fn input_properties(&self) -> Vec<String> {
        vec!["--circle-color".to_string()]
    }

    fn input_arguments(&self) -> Vec<PaintArgumentType> {
        vec![PaintArgumentType::Color]
    }

    fn paint(&self, ctx: &mut PaintContext, size: PaintSize, args: &[PaintValue]) {
        let color = args
            .first()
            .and_then(|a| a.as_color())
            .cloned()
            .unwrap_or_else(|| css_types::Color::rgb(0, 0, 0));

        let center_x = size.width / 2.0;
        let center_y = size.height / 2.0;
        let radius = size.width.min(size.height) / 2.0;

        ctx.set_fill_color(color);
        ctx.fill_circle(center_x, center_y, radius);
    }
}

/// A rough/hand-drawn background worklet
///
/// Usage: `background: paint(rough-background)`
/// Creates a rough, sketchy border effect
pub struct RoughBackgroundWorklet;

impl PaintWorklet for RoughBackgroundWorklet {
    fn name(&self) -> &str {
        "rough-background"
    }

    fn input_properties(&self) -> Vec<String> {
        vec![
            "--rough-stroke-color".to_string(),
            "--rough-stroke-width".to_string(),
        ]
    }

    fn input_arguments(&self) -> Vec<PaintArgumentType> {
        vec![]
    }

    fn paint(&self, ctx: &mut PaintContext, size: PaintSize, _args: &[PaintValue]) {
        // Draw a simple rough rectangle outline
        ctx.set_stroke_color(css_types::Color::rgb(0, 0, 0));
        ctx.set_line_width(2.0);

        // Use slight variations to create rough effect
        let margin = 5.0;
        ctx.begin_path();
        ctx.move_to(margin, margin);
        ctx.line_to(size.width - margin, margin + 1.0);
        ctx.line_to(size.width - margin - 1.0, size.height - margin);
        ctx.line_to(margin + 1.0, size.height - margin + 1.0);
        ctx.close_path();
        ctx.stroke();
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use css_types::Color;

    struct SimpleWorklet;

    impl PaintWorklet for SimpleWorklet {
        fn name(&self) -> &str {
            "simple"
        }

        fn input_properties(&self) -> Vec<String> {
            vec![]
        }

        fn input_arguments(&self) -> Vec<PaintArgumentType> {
            vec![]
        }

        fn paint(&self, ctx: &mut PaintContext, size: PaintSize, _args: &[PaintValue]) {
            ctx.fill_rect(0.0, 0.0, size.width, size.height);
        }
    }

    struct ArgWorklet;

    impl PaintWorklet for ArgWorklet {
        fn name(&self) -> &str {
            "arg-worklet"
        }

        fn input_properties(&self) -> Vec<String> {
            vec![]
        }

        fn input_arguments(&self) -> Vec<PaintArgumentType> {
            vec![PaintArgumentType::Number, PaintArgumentType::Color]
        }

        fn paint(&self, ctx: &mut PaintContext, _size: PaintSize, args: &[PaintValue]) {
            if let Some(num) = args.first().and_then(|a| a.as_number()) {
                ctx.set_line_width(num);
            }
            if let Some(color) = args.get(1).and_then(|a| a.as_color()) {
                ctx.set_fill_color(*color);
            }
        }
    }

    #[test]
    fn test_registry_new() {
        let registry = PaintWorkletRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_register() {
        let mut registry = PaintWorkletRegistry::new();
        let worklet: BoxedWorklet = Arc::new(SimpleWorklet);

        assert!(registry.register(worklet).is_ok());
        assert!(registry.contains("simple"));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_register_duplicate() {
        let mut registry = PaintWorkletRegistry::new();
        let worklet1: BoxedWorklet = Arc::new(SimpleWorklet);
        let worklet2: BoxedWorklet = Arc::new(SimpleWorklet);

        assert!(registry.register(worklet1).is_ok());
        let result = registry.register(worklet2);

        assert!(matches!(
            result,
            Err(PaintError::WorkletAlreadyRegistered(_))
        ));
    }

    #[test]
    fn test_registry_register_or_replace() {
        let mut registry = PaintWorkletRegistry::new();
        let worklet1: BoxedWorklet = Arc::new(SimpleWorklet);
        let worklet2: BoxedWorklet = Arc::new(SimpleWorklet);

        registry.register_or_replace(worklet1);
        registry.register_or_replace(worklet2);

        assert_eq!(registry.len(), 1);
        assert!(registry.contains("simple"));
    }

    #[test]
    fn test_registry_get() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(SimpleWorklet)).unwrap();

        let worklet = registry.get("simple");
        assert!(worklet.is_some());
        assert_eq!(worklet.unwrap().name(), "simple");

        let nonexistent = registry.get("nonexistent");
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_registry_unregister() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(SimpleWorklet)).unwrap();

        assert!(registry.unregister("simple"));
        assert!(!registry.contains("simple"));
        assert!(!registry.unregister("simple")); // Already removed
    }

    #[test]
    fn test_registry_names() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(SimpleWorklet)).unwrap();
        registry.register(Arc::new(ArgWorklet)).unwrap();

        let names: Vec<_> = registry.names().collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"simple"));
        assert!(names.contains(&"arg-worklet"));
    }

    #[test]
    fn test_registry_clear() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(SimpleWorklet)).unwrap();
        registry.register(Arc::new(ArgWorklet)).unwrap();

        registry.clear();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_execute() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(SimpleWorklet)).unwrap();

        let size = PaintSize::new(100.0, 50.0);
        let result = registry.execute("simple", size, &[]);

        assert!(result.is_ok());
        let ctx = result.unwrap();
        assert!(!ctx.operations().is_empty());
    }

    #[test]
    fn test_registry_execute_not_found() {
        let registry = PaintWorkletRegistry::new();
        let size = PaintSize::new(100.0, 50.0);
        let result = registry.execute("nonexistent", size, &[]);

        assert!(matches!(result, Err(PaintError::WorkletNotFound(_))));
    }

    #[test]
    fn test_registry_execute_wrong_arg_count() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(ArgWorklet)).unwrap();

        let size = PaintSize::new(100.0, 50.0);
        let result = registry.execute("arg-worklet", size, &[]);

        assert!(matches!(
            result,
            Err(PaintError::ArgumentCountMismatch {
                expected: 2,
                got: 0
            })
        ));
    }

    #[test]
    fn test_registry_execute_wrong_arg_type() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(ArgWorklet)).unwrap();

        let size = PaintSize::new(100.0, 50.0);
        let args = vec![
            PaintValue::Color(Color::rgb(255, 0, 0)), // Should be Number
            PaintValue::Number(10.0),                 // Should be Color
        ];
        let result = registry.execute("arg-worklet", size, &args);

        assert!(matches!(
            result,
            Err(PaintError::InvalidArgumentType { .. })
        ));
    }

    #[test]
    fn test_registry_execute_with_args() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(ArgWorklet)).unwrap();

        let size = PaintSize::new(100.0, 50.0);
        let args = vec![
            PaintValue::Number(5.0),
            PaintValue::Color(Color::rgb(255, 0, 0)),
        ];
        let result = registry.execute("arg-worklet", size, &args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_registry_with_builtins() {
        let registry = PaintWorkletRegistry::with_builtins();

        assert!(registry.contains("checkerboard"));
        assert!(registry.contains("circle"));
        assert!(registry.contains("rough-background"));
    }

    #[test]
    fn test_checkerboard_worklet() {
        let worklet = CheckerboardWorklet;
        let mut ctx = PaintContext::new();
        let size = PaintSize::new(100.0, 100.0);
        let args = vec![
            PaintValue::Number(20.0),
            PaintValue::Color(Color::rgb(255, 255, 255)),
            PaintValue::Color(Color::rgb(0, 0, 0)),
        ];

        worklet.paint(&mut ctx, size, &args);

        // Should have drawn multiple rectangles
        assert!(!ctx.operations().is_empty());
    }

    #[test]
    fn test_circle_worklet() {
        let worklet = CircleWorklet;
        let mut ctx = PaintContext::new();
        let size = PaintSize::new(100.0, 100.0);
        let args = vec![PaintValue::Color(Color::rgb(255, 0, 0))];

        worklet.paint(&mut ctx, size, &args);

        // Should have operations for drawing a circle
        assert!(!ctx.operations().is_empty());
    }

    #[test]
    fn test_rough_background_worklet() {
        let worklet = RoughBackgroundWorklet;
        let mut ctx = PaintContext::new();
        let size = PaintSize::new(200.0, 100.0);

        worklet.paint(&mut ctx, size, &[]);

        // Should have drawn a rough rectangle
        assert!(!ctx.operations().is_empty());
    }

    #[test]
    fn test_registry_debug() {
        let mut registry = PaintWorkletRegistry::new();
        registry.register(Arc::new(SimpleWorklet)).unwrap();

        let debug_str = format!("{:?}", registry);
        assert!(debug_str.contains("PaintWorkletRegistry"));
        assert!(debug_str.contains("1")); // worklet_count
    }
}
