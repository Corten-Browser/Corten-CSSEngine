//! Mock DOM Implementation for Testing
//!
//! Provides MockDomTree and MockDomNode structures for testing CSS engine
//! components without requiring a real browser DOM.

use std::collections::HashMap;

/// Represents a mock DOM node for testing purposes.
///
/// This structure mimics the essential properties of a DOM node needed
/// for CSS selector matching and style computation.
#[derive(Debug, Clone, PartialEq)]
pub struct MockDomNode {
    /// Unique identifier for this node
    pub id: u64,
    /// HTML tag name (e.g., "div", "span", "p")
    pub tag_name: String,
    /// CSS class names applied to this node
    pub classes: Vec<String>,
    /// HTML attributes as key-value pairs
    pub attributes: HashMap<String, String>,
    /// Parent node ID (None for root node)
    pub parent: Option<u64>,
    /// IDs of child nodes
    pub children: Vec<u64>,
}

impl MockDomNode {
    /// Creates a new MockDomNode with the given ID and tag name.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this node
    /// * `tag_name` - HTML tag name
    ///
    /// # Example
    /// ```
    /// use testing_mocks::mock_dom::MockDomNode;
    /// let node = MockDomNode::new(1, "div");
    /// assert_eq!(node.tag_name, "div");
    /// ```
    pub fn new(id: u64, tag_name: &str) -> Self {
        Self {
            id,
            tag_name: tag_name.to_string(),
            classes: Vec::new(),
            attributes: HashMap::new(),
            parent: None,
            children: Vec::new(),
        }
    }

    /// Creates a builder for constructing a MockDomNode with fluent API.
    pub fn builder(id: u64, tag_name: &str) -> MockDomNodeBuilder {
        MockDomNodeBuilder::new(id, tag_name)
    }

    /// Adds a CSS class to this node.
    pub fn add_class(&mut self, class: &str) {
        if !self.classes.contains(&class.to_string()) {
            self.classes.push(class.to_string());
        }
    }

    /// Removes a CSS class from this node.
    pub fn remove_class(&mut self, class: &str) {
        self.classes.retain(|c| c != class);
    }

    /// Checks if this node has a specific CSS class.
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains(&class.to_string())
    }

    /// Sets an attribute on this node.
    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    /// Gets an attribute value from this node.
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Removes an attribute from this node.
    pub fn remove_attribute(&mut self, key: &str) {
        self.attributes.remove(key);
    }

    /// Checks if this node has a specific attribute.
    pub fn has_attribute(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    /// Gets the ID attribute if present.
    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    /// Checks if this is a root node (no parent).
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Checks if this is a leaf node (no children).
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// Builder for constructing MockDomNode with a fluent API.
#[derive(Debug)]
pub struct MockDomNodeBuilder {
    node: MockDomNode,
}

impl MockDomNodeBuilder {
    /// Creates a new builder with the given ID and tag name.
    pub fn new(id: u64, tag_name: &str) -> Self {
        Self {
            node: MockDomNode::new(id, tag_name),
        }
    }

    /// Adds a CSS class to the node.
    pub fn class(mut self, class: &str) -> Self {
        self.node.add_class(class);
        self
    }

    /// Adds multiple CSS classes to the node.
    pub fn classes(mut self, classes: &[&str]) -> Self {
        for class in classes {
            self.node.add_class(class);
        }
        self
    }

    /// Sets an attribute on the node.
    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        self.node.set_attribute(key, value);
        self
    }

    /// Sets the ID attribute on the node.
    pub fn id(self, id: &str) -> Self {
        self.attribute("id", id)
    }

    /// Sets the parent node ID.
    pub fn parent(mut self, parent_id: u64) -> Self {
        self.node.parent = Some(parent_id);
        self
    }

    /// Builds the MockDomNode.
    pub fn build(self) -> MockDomNode {
        self.node
    }
}

/// A mock DOM tree for testing purposes.
///
/// Provides tree navigation methods like getting parent/children nodes
/// and depth-first traversal.
#[derive(Debug, Clone)]
pub struct MockDomTree {
    nodes: HashMap<u64, MockDomNode>,
    root: Option<u64>,
    next_id: u64,
}

impl Default for MockDomTree {
    fn default() -> Self {
        Self::new()
    }
}

impl MockDomTree {
    /// Creates a new empty MockDomTree.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root: None,
            next_id: 1,
        }
    }

    /// Creates a tree with a root node.
    ///
    /// # Arguments
    /// * `tag_name` - HTML tag name for the root node
    ///
    /// # Returns
    /// A tuple of (tree, root_id)
    pub fn with_root(tag_name: &str) -> (Self, u64) {
        let mut tree = Self::new();
        let root_id = tree.add_node(MockDomNode::new(0, tag_name));
        tree.root = Some(root_id);
        (tree, root_id)
    }

    /// Adds a node to the tree.
    ///
    /// If this is the first node, it becomes the root.
    /// Returns the node's ID.
    pub fn add_node(&mut self, node: MockDomNode) -> u64 {
        let id = node.id;

        // If this is the first node, make it the root
        if self.nodes.is_empty() {
            self.root = Some(id);
        }

        self.nodes.insert(id, node);

        // Update next_id to avoid collisions
        if id >= self.next_id {
            self.next_id = id + 1;
        }

        id
    }

    /// Adds a child node to a parent node.
    ///
    /// # Arguments
    /// * `parent_id` - ID of the parent node
    /// * `child` - The child node to add
    ///
    /// # Returns
    /// The child node's ID if successful, None if parent doesn't exist
    pub fn add_child(&mut self, parent_id: u64, mut child: MockDomNode) -> Option<u64> {
        if !self.nodes.contains_key(&parent_id) {
            return None;
        }

        let child_id = child.id;
        child.parent = Some(parent_id);

        // Add child to parent's children list
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id);
        }

        self.add_node(child);
        Some(child_id)
    }

    /// Creates and adds a child node with auto-generated ID.
    ///
    /// # Arguments
    /// * `parent_id` - ID of the parent node
    /// * `tag_name` - HTML tag name for the child
    ///
    /// # Returns
    /// The child node's ID if successful, None if parent doesn't exist
    pub fn create_child(&mut self, parent_id: u64, tag_name: &str) -> Option<u64> {
        let child = MockDomNode::new(self.next_id, tag_name);
        self.add_child(parent_id, child)
    }

    /// Gets a reference to a node by ID.
    pub fn get_node(&self, id: u64) -> Option<&MockDomNode> {
        self.nodes.get(&id)
    }

    /// Gets a mutable reference to a node by ID.
    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut MockDomNode> {
        self.nodes.get_mut(&id)
    }

    /// Gets the parent of a node.
    pub fn get_parent(&self, id: u64) -> Option<&MockDomNode> {
        self.nodes
            .get(&id)
            .and_then(|node| node.parent)
            .and_then(|parent_id| self.nodes.get(&parent_id))
    }

    /// Gets the children of a node.
    pub fn get_children(&self, id: u64) -> Vec<&MockDomNode> {
        self.nodes
            .get(&id)
            .map(|node| {
                node.children
                    .iter()
                    .filter_map(|child_id| self.nodes.get(child_id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets the root node.
    pub fn get_root(&self) -> Option<&MockDomNode> {
        self.root.and_then(|id| self.nodes.get(&id))
    }

    /// Gets the root node ID.
    pub fn root_id(&self) -> Option<u64> {
        self.root
    }

    /// Returns the number of nodes in the tree.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Checks if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns an iterator that traverses the tree in depth-first order.
    pub fn traverse_depth_first(&self) -> DepthFirstIterator<'_> {
        DepthFirstIterator::new(self)
    }

    /// Returns an iterator that traverses the tree in breadth-first order.
    pub fn traverse_breadth_first(&self) -> BreadthFirstIterator<'_> {
        BreadthFirstIterator::new(self)
    }

    /// Gets all ancestors of a node (parent, grandparent, etc.).
    pub fn get_ancestors(&self, id: u64) -> Vec<&MockDomNode> {
        let mut ancestors = Vec::new();
        let mut current_id = self.nodes.get(&id).and_then(|n| n.parent);

        while let Some(ancestor_id) = current_id {
            if let Some(ancestor) = self.nodes.get(&ancestor_id) {
                ancestors.push(ancestor);
                current_id = ancestor.parent;
            } else {
                break;
            }
        }

        ancestors
    }

    /// Gets all descendants of a node (children, grandchildren, etc.).
    pub fn get_descendants(&self, id: u64) -> Vec<&MockDomNode> {
        let mut descendants = Vec::new();
        let mut stack = vec![id];

        // Skip the initial node itself
        if let Some(node) = self.nodes.get(&id) {
            stack = node.children.clone();
        }

        while let Some(current_id) = stack.pop() {
            if let Some(node) = self.nodes.get(&current_id) {
                descendants.push(node);
                stack.extend(node.children.iter().rev());
            }
        }

        descendants
    }

    /// Gets siblings of a node (excluding the node itself).
    pub fn get_siblings(&self, id: u64) -> Vec<&MockDomNode> {
        self.nodes
            .get(&id)
            .and_then(|node| node.parent)
            .map(|parent_id| {
                self.get_children(parent_id)
                    .into_iter()
                    .filter(|child| child.id != id)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets the depth of a node (root is depth 0).
    pub fn get_depth(&self, id: u64) -> Option<usize> {
        if !self.nodes.contains_key(&id) {
            return None;
        }
        Some(self.get_ancestors(id).len())
    }

    /// Finds nodes by tag name.
    pub fn find_by_tag(&self, tag_name: &str) -> Vec<&MockDomNode> {
        self.nodes
            .values()
            .filter(|node| node.tag_name == tag_name)
            .collect()
    }

    /// Finds nodes by CSS class.
    pub fn find_by_class(&self, class: &str) -> Vec<&MockDomNode> {
        self.nodes
            .values()
            .filter(|node| node.has_class(class))
            .collect()
    }

    /// Finds a node by ID attribute.
    pub fn find_by_id(&self, id: &str) -> Option<&MockDomNode> {
        self.nodes
            .values()
            .find(|node| node.get_id().map(|s| s.as_str()) == Some(id))
    }

    /// Removes a node and all its descendants from the tree.
    pub fn remove_node(&mut self, id: u64) -> Option<MockDomNode> {
        // Get descendants first (to remove them)
        let descendant_ids: Vec<u64> = self
            .get_descendants(id)
            .iter()
            .map(|n| n.id)
            .collect();

        // Remove from parent's children list
        if let Some(node) = self.nodes.get(&id) {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.retain(|&child_id| child_id != id);
                }
            }
        }

        // Remove descendants
        for desc_id in descendant_ids {
            self.nodes.remove(&desc_id);
        }

        // Remove the node itself
        let removed = self.nodes.remove(&id);

        // Update root if we removed it
        if self.root == Some(id) {
            self.root = None;
        }

        removed
    }
}

/// Iterator for depth-first traversal of MockDomTree.
pub struct DepthFirstIterator<'a> {
    tree: &'a MockDomTree,
    stack: Vec<u64>,
}

impl<'a> DepthFirstIterator<'a> {
    fn new(tree: &'a MockDomTree) -> Self {
        let stack = tree.root.map(|id| vec![id]).unwrap_or_default();
        Self { tree, stack }
    }
}

impl<'a> Iterator for DepthFirstIterator<'a> {
    type Item = &'a MockDomNode;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.stack.pop()?;
        let node = self.tree.nodes.get(&id)?;

        // Push children in reverse order so leftmost is processed first
        self.stack.extend(node.children.iter().rev());

        Some(node)
    }
}

/// Iterator for breadth-first traversal of MockDomTree.
pub struct BreadthFirstIterator<'a> {
    tree: &'a MockDomTree,
    queue: std::collections::VecDeque<u64>,
}

impl<'a> BreadthFirstIterator<'a> {
    fn new(tree: &'a MockDomTree) -> Self {
        let mut queue = std::collections::VecDeque::new();
        if let Some(root_id) = tree.root {
            queue.push_back(root_id);
        }
        Self { tree, queue }
    }
}

impl<'a> Iterator for BreadthFirstIterator<'a> {
    type Item = &'a MockDomNode;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.queue.pop_front()?;
        let node = self.tree.nodes.get(&id)?;

        // Add children to the queue
        for child_id in &node.children {
            self.queue.push_back(*child_id);
        }

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== MockDomNode Tests ====================

    #[test]
    fn test_mock_dom_node_new() {
        let node = MockDomNode::new(1, "div");
        assert_eq!(node.id, 1);
        assert_eq!(node.tag_name, "div");
        assert!(node.classes.is_empty());
        assert!(node.attributes.is_empty());
        assert!(node.parent.is_none());
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_mock_dom_node_builder() {
        let node = MockDomNode::builder(1, "div")
            .id("main")
            .class("container")
            .class("active")
            .attribute("data-role", "main")
            .build();

        assert_eq!(node.id, 1);
        assert_eq!(node.tag_name, "div");
        assert!(node.has_class("container"));
        assert!(node.has_class("active"));
        assert_eq!(node.get_id(), Some(&"main".to_string()));
        assert_eq!(node.get_attribute("data-role"), Some(&"main".to_string()));
    }

    #[test]
    fn test_mock_dom_node_classes() {
        let mut node = MockDomNode::new(1, "div");

        node.add_class("foo");
        assert!(node.has_class("foo"));
        assert!(!node.has_class("bar"));

        node.add_class("bar");
        assert!(node.has_class("bar"));

        // Adding duplicate class should not add again
        node.add_class("foo");
        assert_eq!(node.classes.len(), 2);

        node.remove_class("foo");
        assert!(!node.has_class("foo"));
        assert!(node.has_class("bar"));
    }

    #[test]
    fn test_mock_dom_node_attributes() {
        let mut node = MockDomNode::new(1, "input");

        node.set_attribute("type", "text");
        node.set_attribute("name", "username");

        assert!(node.has_attribute("type"));
        assert_eq!(node.get_attribute("type"), Some(&"text".to_string()));
        assert_eq!(node.get_attribute("name"), Some(&"username".to_string()));
        assert_eq!(node.get_attribute("missing"), None);

        node.remove_attribute("type");
        assert!(!node.has_attribute("type"));
    }

    #[test]
    fn test_mock_dom_node_is_root_and_leaf() {
        let mut node = MockDomNode::new(1, "div");

        assert!(node.is_root());
        assert!(node.is_leaf());

        node.parent = Some(0);
        assert!(!node.is_root());

        node.children.push(2);
        assert!(!node.is_leaf());
    }

    // ==================== MockDomTree Tests ====================

    #[test]
    fn test_mock_dom_tree_new() {
        let tree = MockDomTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(tree.get_root().is_none());
    }

    #[test]
    fn test_mock_dom_tree_with_root() {
        let (tree, root_id) = MockDomTree::with_root("html");

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.root_id(), Some(root_id));

        let root = tree.get_root().unwrap();
        assert_eq!(root.tag_name, "html");
    }

    #[test]
    fn test_mock_dom_tree_add_node() {
        let mut tree = MockDomTree::new();

        let node1 = MockDomNode::new(1, "html");
        let id1 = tree.add_node(node1);

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.root_id(), Some(id1));

        let node2 = MockDomNode::new(2, "body");
        tree.add_node(node2);

        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_mock_dom_tree_add_child() {
        let mut tree = MockDomTree::new();

        let root = MockDomNode::new(1, "html");
        tree.add_node(root);

        let body = MockDomNode::new(2, "body");
        let body_id = tree.add_child(1, body).unwrap();

        let div = MockDomNode::new(3, "div");
        let div_id = tree.add_child(body_id, div).unwrap();

        // Verify parent-child relationships
        let body_node = tree.get_node(body_id).unwrap();
        assert_eq!(body_node.parent, Some(1));

        let div_node = tree.get_node(div_id).unwrap();
        assert_eq!(div_node.parent, Some(body_id));

        let root_children = tree.get_children(1);
        assert_eq!(root_children.len(), 1);
        assert_eq!(root_children[0].id, body_id);
    }

    #[test]
    fn test_mock_dom_tree_create_child() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));

        let body_id = tree.create_child(1, "body").unwrap();
        let div_id = tree.create_child(body_id, "div").unwrap();

        assert!(body_id > 1);
        assert!(div_id > body_id);

        let body = tree.get_node(body_id).unwrap();
        assert_eq!(body.tag_name, "body");
        assert_eq!(body.parent, Some(1));
    }

    #[test]
    fn test_mock_dom_tree_get_parent() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "body"));
        tree.add_child(2, MockDomNode::new(3, "div"));

        let parent = tree.get_parent(3).unwrap();
        assert_eq!(parent.tag_name, "body");

        let grandparent = tree.get_parent(2).unwrap();
        assert_eq!(grandparent.tag_name, "html");

        assert!(tree.get_parent(1).is_none()); // Root has no parent
    }

    #[test]
    fn test_mock_dom_tree_get_children() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "ul"));
        tree.add_child(1, MockDomNode::new(2, "li"));
        tree.add_child(1, MockDomNode::new(3, "li"));
        tree.add_child(1, MockDomNode::new(4, "li"));

        let children = tree.get_children(1);
        assert_eq!(children.len(), 3);
        assert!(children.iter().all(|c| c.tag_name == "li"));
    }

    #[test]
    fn test_mock_dom_tree_depth_first_traversal() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "head"));
        tree.add_child(1, MockDomNode::new(3, "body"));
        tree.add_child(3, MockDomNode::new(4, "div"));
        tree.add_child(3, MockDomNode::new(5, "footer"));

        let nodes: Vec<_> = tree.traverse_depth_first().collect();
        let tags: Vec<_> = nodes.iter().map(|n| n.tag_name.as_str()).collect();

        // Depth-first: html -> head -> body -> div -> footer
        assert_eq!(tags, vec!["html", "head", "body", "div", "footer"]);
    }

    #[test]
    fn test_mock_dom_tree_breadth_first_traversal() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "head"));
        tree.add_child(1, MockDomNode::new(3, "body"));
        tree.add_child(3, MockDomNode::new(4, "div"));
        tree.add_child(3, MockDomNode::new(5, "footer"));

        let nodes: Vec<_> = tree.traverse_breadth_first().collect();
        let tags: Vec<_> = nodes.iter().map(|n| n.tag_name.as_str()).collect();

        // Breadth-first: html -> head -> body -> div -> footer
        assert_eq!(tags, vec!["html", "head", "body", "div", "footer"]);
    }

    #[test]
    fn test_mock_dom_tree_get_ancestors() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "body"));
        tree.add_child(2, MockDomNode::new(3, "div"));
        tree.add_child(3, MockDomNode::new(4, "span"));

        let ancestors = tree.get_ancestors(4);
        let tags: Vec<_> = ancestors.iter().map(|n| n.tag_name.as_str()).collect();

        assert_eq!(tags, vec!["div", "body", "html"]);
    }

    #[test]
    fn test_mock_dom_tree_get_descendants() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "head"));
        tree.add_child(1, MockDomNode::new(3, "body"));
        tree.add_child(3, MockDomNode::new(4, "div"));
        tree.add_child(4, MockDomNode::new(5, "span"));

        let descendants = tree.get_descendants(1);
        assert_eq!(descendants.len(), 4);

        let body_descendants = tree.get_descendants(3);
        assert_eq!(body_descendants.len(), 2);
    }

    #[test]
    fn test_mock_dom_tree_get_siblings() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "ul"));
        tree.add_child(1, MockDomNode::new(2, "li"));
        tree.add_child(1, MockDomNode::new(3, "li"));
        tree.add_child(1, MockDomNode::new(4, "li"));

        let siblings = tree.get_siblings(3);
        assert_eq!(siblings.len(), 2);
        let ids: Vec<_> = siblings.iter().map(|n| n.id).collect();
        assert!(ids.contains(&2));
        assert!(ids.contains(&4));
        assert!(!ids.contains(&3)); // Should not include itself
    }

    #[test]
    fn test_mock_dom_tree_get_depth() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "body"));
        tree.add_child(2, MockDomNode::new(3, "div"));
        tree.add_child(3, MockDomNode::new(4, "span"));

        assert_eq!(tree.get_depth(1), Some(0)); // Root
        assert_eq!(tree.get_depth(2), Some(1));
        assert_eq!(tree.get_depth(3), Some(2));
        assert_eq!(tree.get_depth(4), Some(3));
        assert_eq!(tree.get_depth(999), None); // Non-existent
    }

    #[test]
    fn test_mock_dom_tree_find_by_tag() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "div"));
        tree.add_child(1, MockDomNode::new(3, "div"));
        tree.add_child(1, MockDomNode::new(4, "span"));

        let divs = tree.find_by_tag("div");
        assert_eq!(divs.len(), 2);

        let spans = tree.find_by_tag("span");
        assert_eq!(spans.len(), 1);

        let buttons = tree.find_by_tag("button");
        assert!(buttons.is_empty());
    }

    #[test]
    fn test_mock_dom_tree_find_by_class() {
        let mut tree = MockDomTree::new();

        let mut node1 = MockDomNode::new(1, "div");
        node1.add_class("container");
        tree.add_node(node1);

        let mut node2 = MockDomNode::new(2, "div");
        node2.add_class("container");
        node2.add_class("active");
        tree.add_child(1, node2);

        let mut node3 = MockDomNode::new(3, "span");
        node3.add_class("active");
        tree.add_child(1, node3);

        let containers = tree.find_by_class("container");
        assert_eq!(containers.len(), 2);

        let active = tree.find_by_class("active");
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_mock_dom_tree_find_by_id() {
        let mut tree = MockDomTree::new();

        let node1 = MockDomNode::builder(1, "div")
            .id("header")
            .build();
        tree.add_node(node1);

        let node2 = MockDomNode::builder(2, "div")
            .id("content")
            .build();
        tree.add_child(1, node2);

        let header = tree.find_by_id("header");
        assert!(header.is_some());
        assert_eq!(header.unwrap().id, 1);

        let content = tree.find_by_id("content");
        assert!(content.is_some());
        assert_eq!(content.unwrap().id, 2);

        let missing = tree.find_by_id("nonexistent");
        assert!(missing.is_none());
    }

    #[test]
    fn test_mock_dom_tree_remove_node() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "body"));
        tree.add_child(2, MockDomNode::new(3, "div"));
        tree.add_child(3, MockDomNode::new(4, "span"));

        // Remove div and its descendants
        let removed = tree.remove_node(3);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().tag_name, "div");

        // Verify div and span are gone
        assert!(tree.get_node(3).is_none());
        assert!(tree.get_node(4).is_none());

        // Verify body no longer has div as child
        let body = tree.get_node(2).unwrap();
        assert!(body.children.is_empty());

        // html and body should still exist
        assert!(tree.get_node(1).is_some());
        assert!(tree.get_node(2).is_some());
    }

    #[test]
    fn test_mock_dom_tree_remove_root() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "html"));
        tree.add_child(1, MockDomNode::new(2, "body"));

        tree.remove_node(1);

        assert!(tree.get_root().is_none());
        assert!(tree.is_empty());
    }

    #[test]
    fn test_mock_dom_tree_mutable_access() {
        let mut tree = MockDomTree::new();
        tree.add_node(MockDomNode::new(1, "div"));

        // Modify node through mutable access
        if let Some(node) = tree.get_node_mut(1) {
            node.add_class("modified");
            node.set_attribute("data-updated", "true");
        }

        let node = tree.get_node(1).unwrap();
        assert!(node.has_class("modified"));
        assert_eq!(node.get_attribute("data-updated"), Some(&"true".to_string()));
    }

    #[test]
    fn test_builder_with_multiple_classes() {
        let node = MockDomNode::builder(1, "div")
            .classes(&["foo", "bar", "baz"])
            .build();

        assert!(node.has_class("foo"));
        assert!(node.has_class("bar"));
        assert!(node.has_class("baz"));
    }
}
