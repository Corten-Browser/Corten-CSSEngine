// Browser component interfaces
// Mock implementation for standalone CSS engine development

use async_trait::async_trait;

/// Base trait for browser components
#[async_trait]
pub trait BrowserComponent: Send + Sync {
    /// Component name
    fn name(&self) -> &str;

    /// Initialize the component
    async fn initialize(&mut self) -> Result<(), String>;

    /// Shutdown the component
    async fn shutdown(&mut self) -> Result<(), String>;

    /// Health check
    fn is_healthy(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        name: String,
    }

    #[async_trait]
    impl BrowserComponent for TestComponent {
        fn name(&self) -> &str {
            &self.name
        }

        async fn initialize(&mut self) -> Result<(), String> {
            Ok(())
        }

        async fn shutdown(&mut self) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_component_interface() {
        let mut component = TestComponent {
            name: "test".to_string(),
        };
        assert_eq!(component.name(), "test");
        assert!(component.initialize().await.is_ok());
    }
}
