// Browser shared types for CSS Engine integration
// Mock implementation for standalone CSS engine development

use serde::{Deserialize, Serialize};

/// DOM Node representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomNode {
    pub id: u64,
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<DomNode>,
}

/// URL type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub path: String,
}

impl Url {
    pub fn parse(url: &str) -> Result<Self, String> {
        // Simple URL parsing for testing
        Ok(Url {
            scheme: "http".to_string(),
            host: "example.com".to_string(),
            path: url.to_string(),
        })
    }
}

/// Viewport information
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ViewportInfo {
    pub width: u32,
    pub height: u32,
    pub device_pixel_ratio: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_node_creation() {
        let node = DomNode {
            id: 1,
            tag_name: "div".to_string(),
            attributes: vec![],
            children: vec![],
        };
        assert_eq!(node.tag_name, "div");
    }
}
