//! Arena Allocator for CSS Style Computation
//!
//! This module provides arena-based allocation for efficient memory management
//! during style computation. Arena allocation reduces allocation overhead and
//! improves cache locality by grouping related allocations together.
//!
//! # Design
//!
//! The arena uses chunked allocation with a default chunk size of 4KB. When a
//! chunk fills up, a new chunk is allocated. This provides:
//! - O(1) allocation (amortized)
//! - No per-object deallocation overhead
//! - Cache-friendly memory layout
//! - Efficient bulk clearing
//!
//! # Example
//!
//! ```
//! use css_stylist_core::arena::StyleArena;
//!
//! let arena: StyleArena<u32> = StyleArena::new();
//! let idx1 = arena.alloc(42);
//! let idx2 = arena.alloc(100);
//!
//! assert_eq!(*arena.get(idx1), 42);
//! assert_eq!(*arena.get(idx2), 100);
//! ```

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem;

/// Default chunk size in number of elements
/// For 4KB chunks with typical CSS types (~64-256 bytes), this gives us
/// 16-64 elements per chunk
const DEFAULT_CHUNK_CAPACITY: usize = 64;

/// A typed index into a `StyleArena<T>`
///
/// This provides a safe, typed reference to an allocated value.
/// The index is only valid for the arena it was allocated from.
///
/// # Type Safety
///
/// `ArenaIndex<T>` uses `PhantomData<T>` to ensure type safety at compile time.
/// You cannot use an `ArenaIndex<u32>` to access a `StyleArena<String>`.
#[derive(Debug)]
pub struct ArenaIndex<T> {
    /// The chunk index
    chunk: u32,
    /// The index within the chunk
    index: u32,
    /// Phantom data for type safety
    _marker: PhantomData<T>,
}

impl<T> Clone for ArenaIndex<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ArenaIndex<T> {}

impl<T> PartialEq for ArenaIndex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.chunk == other.chunk && self.index == other.index
    }
}

impl<T> Eq for ArenaIndex<T> {}

impl<T> std::hash::Hash for ArenaIndex<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.chunk.hash(state);
        self.index.hash(state);
    }
}

impl<T> ArenaIndex<T> {
    /// Create a new arena index
    ///
    /// # Arguments
    /// * `chunk` - The chunk index
    /// * `index` - The index within the chunk
    #[inline]
    fn new(chunk: u32, index: u32) -> Self {
        Self {
            chunk,
            index,
            _marker: PhantomData,
        }
    }

    /// Get the chunk index
    #[inline]
    pub fn chunk(&self) -> u32 {
        self.chunk
    }

    /// Get the index within the chunk
    #[inline]
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Convert to a raw u64 for serialization
    #[inline]
    pub fn to_raw(&self) -> u64 {
        ((self.chunk as u64) << 32) | (self.index as u64)
    }

    /// Create from a raw u64
    #[inline]
    pub fn from_raw(raw: u64) -> Self {
        Self::new((raw >> 32) as u32, raw as u32)
    }
}

/// Internal storage for arena chunks
struct ArenaChunk<T> {
    /// The data storage
    data: Vec<T>,
    /// Capacity of this chunk
    capacity: usize,
}

impl<T> ArenaChunk<T> {
    /// Create a new chunk with the given capacity
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Check if the chunk is full
    #[inline]
    fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    /// Push a value to the chunk
    ///
    /// # Panics
    /// Panics if the chunk is full
    #[inline]
    fn push(&mut self, value: T) -> usize {
        debug_assert!(!self.is_full(), "Chunk is full");
        let index = self.data.len();
        self.data.push(value);
        index
    }

    /// Get a reference to a value
    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Get a mutable reference to a value
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Clear the chunk, keeping capacity
    fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the number of elements in this chunk
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}

/// A generic arena allocator for CSS style computation
///
/// `StyleArena<T>` provides efficient allocation of values of type `T`.
/// Allocations are grouped into chunks for better cache locality.
///
/// # Thread Safety
///
/// `StyleArena` uses interior mutability with `UnsafeCell` for allocation.
/// It is NOT thread-safe. For multi-threaded access, wrap in appropriate
/// synchronization primitives.
///
/// # Examples
///
/// ```
/// use css_stylist_core::arena::StyleArena;
///
/// let arena: StyleArena<String> = StyleArena::new();
/// let idx = arena.alloc("hello".to_string());
/// assert_eq!(arena.get(idx).as_str(), "hello");
/// ```
pub struct StyleArena<T> {
    /// The chunks of allocated data
    chunks: UnsafeCell<Vec<ArenaChunk<T>>>,
    /// Capacity of each chunk
    chunk_capacity: usize,
}

impl<T> StyleArena<T> {
    /// Create a new arena with default chunk size
    ///
    /// The default chunk capacity is 64 elements, which for typical
    /// CSS types gives approximately 4KB per chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let arena: StyleArena<u32> = StyleArena::new();
    /// assert!(arena.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_chunk_capacity(DEFAULT_CHUNK_CAPACITY)
    }

    /// Create a new arena with specified chunk capacity
    ///
    /// # Arguments
    /// * `chunk_capacity` - The number of elements per chunk
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(128);
    /// assert!(arena.is_empty());
    /// ```
    pub fn with_chunk_capacity(chunk_capacity: usize) -> Self {
        assert!(chunk_capacity > 0, "Chunk capacity must be positive");
        Self {
            chunks: UnsafeCell::new(vec![ArenaChunk::new(chunk_capacity)]),
            chunk_capacity,
        }
    }

    /// Allocate a value in the arena
    ///
    /// Returns an index that can be used to retrieve the value later.
    ///
    /// # Arguments
    /// * `value` - The value to allocate
    ///
    /// # Returns
    /// An `ArenaIndex<T>` that references the allocated value
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let arena: StyleArena<i32> = StyleArena::new();
    /// let idx1 = arena.alloc(10);
    /// let idx2 = arena.alloc(20);
    ///
    /// assert_eq!(*arena.get(idx1), 10);
    /// assert_eq!(*arena.get(idx2), 20);
    /// ```
    pub fn alloc(&self, value: T) -> ArenaIndex<T> {
        // SAFETY: We have &self, so no other mutable references exist.
        // The UnsafeCell allows us to mutate during allocation.
        let chunks = unsafe { &mut *self.chunks.get() };

        // Find or create a chunk with space
        let chunk_idx = if chunks.last().map_or(true, |c| c.is_full()) {
            chunks.push(ArenaChunk::new(self.chunk_capacity));
            chunks.len() - 1
        } else {
            chunks.len() - 1
        };

        let idx_in_chunk = chunks[chunk_idx].push(value);

        ArenaIndex::new(chunk_idx as u32, idx_in_chunk as u32)
    }

    /// Get a reference to an allocated value
    ///
    /// # Arguments
    /// * `index` - The index returned from `alloc`
    ///
    /// # Returns
    /// A reference to the allocated value
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let arena: StyleArena<&str> = StyleArena::new();
    /// let idx = arena.alloc("test");
    /// assert_eq!(*arena.get(idx), "test");
    /// ```
    #[inline]
    pub fn get(&self, index: ArenaIndex<T>) -> &T {
        // SAFETY: We have &self, and the index was created by this arena
        let chunks = unsafe { &*self.chunks.get() };
        chunks[index.chunk as usize]
            .get(index.index as usize)
            .expect("Invalid arena index")
    }

    /// Try to get a reference to an allocated value
    ///
    /// # Arguments
    /// * `index` - The index to look up
    ///
    /// # Returns
    /// `Some(&T)` if the index is valid, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::{StyleArena, ArenaIndex};
    ///
    /// let arena: StyleArena<u32> = StyleArena::new();
    /// let idx = arena.alloc(42);
    /// assert_eq!(arena.try_get(idx), Some(&42));
    ///
    /// // Invalid index
    /// let bad_idx: ArenaIndex<u32> = ArenaIndex::from_raw(999999);
    /// assert_eq!(arena.try_get(bad_idx), None);
    /// ```
    #[inline]
    pub fn try_get(&self, index: ArenaIndex<T>) -> Option<&T> {
        let chunks = unsafe { &*self.chunks.get() };
        chunks.get(index.chunk as usize)?.get(index.index as usize)
    }

    /// Get a mutable reference to an allocated value
    ///
    /// # Arguments
    /// * `index` - The index returned from `alloc`
    ///
    /// # Returns
    /// A mutable reference to the allocated value
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    ///
    /// # Safety
    /// This requires `&mut self` to ensure exclusive access
    #[inline]
    pub fn get_mut(&mut self, index: ArenaIndex<T>) -> &mut T {
        let chunks = self.chunks.get_mut();
        chunks[index.chunk as usize]
            .get_mut(index.index as usize)
            .expect("Invalid arena index")
    }

    /// Clear all allocations without deallocating memory
    ///
    /// This resets the arena to an empty state while keeping the
    /// allocated chunks for reuse. This is very efficient for
    /// scenarios where the arena is repeatedly filled and cleared.
    ///
    /// # Warning
    /// After calling `clear()`, all previously returned `ArenaIndex`
    /// values become invalid. Using them will result in undefined behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let mut arena: StyleArena<u32> = StyleArena::new();
    /// arena.alloc(1);
    /// arena.alloc(2);
    /// arena.alloc(3);
    ///
    /// assert_eq!(arena.len(), 3);
    ///
    /// arena.clear();
    ///
    /// assert_eq!(arena.len(), 0);
    /// assert!(arena.is_empty());
    /// ```
    pub fn clear(&mut self) {
        let chunks = self.chunks.get_mut();

        // Clear all chunks but keep them allocated
        for chunk in chunks.iter_mut() {
            chunk.clear();
        }

        // Keep only the first chunk to avoid memory fragmentation
        if chunks.len() > 1 {
            chunks.truncate(1);
        }
    }

    /// Get the total number of allocated elements
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let arena: StyleArena<u32> = StyleArena::new();
    /// assert_eq!(arena.len(), 0);
    ///
    /// arena.alloc(1);
    /// arena.alloc(2);
    /// assert_eq!(arena.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        let chunks = unsafe { &*self.chunks.get() };
        chunks.iter().map(|c| c.len()).sum()
    }

    /// Check if the arena is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let arena: StyleArena<u32> = StyleArena::new();
    /// assert!(arena.is_empty());
    ///
    /// arena.alloc(42);
    /// assert!(!arena.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the number of chunks allocated
    ///
    /// This is useful for monitoring memory usage and arena growth.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_stylist_core::arena::StyleArena;
    ///
    /// let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(2);
    /// assert_eq!(arena.chunk_count(), 1);
    ///
    /// arena.alloc(1);
    /// arena.alloc(2);
    /// arena.alloc(3); // Triggers new chunk
    ///
    /// assert_eq!(arena.chunk_count(), 2);
    /// ```
    pub fn chunk_count(&self) -> usize {
        let chunks = unsafe { &*self.chunks.get() };
        chunks.len()
    }

    /// Get the chunk capacity
    pub fn chunk_capacity(&self) -> usize {
        self.chunk_capacity
    }

    /// Estimate memory usage in bytes
    ///
    /// Returns an estimate of the total memory used by this arena,
    /// including allocated but unused capacity.
    pub fn memory_usage(&self) -> usize {
        let chunks = unsafe { &*self.chunks.get() };
        let element_size = mem::size_of::<T>();
        chunks.len() * self.chunk_capacity * element_size
    }
}

impl<T> Default for StyleArena<T> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: StyleArena can be sent between threads
// (but not shared without synchronization)
unsafe impl<T: Send> Send for StyleArena<T> {}

/// Type alias for rule tree node arena
///
/// Used for allocating rule tree nodes during style computation.
pub type RuleNodeArena = StyleArena<super::rule_tree::RuleNode>;

/// Type alias for computed style values arena
///
/// Used for allocating computed style values.
pub type StyleDataArena = StyleArena<super::types::ComputedValues>;

/// Selector data for arena storage
///
/// Represents a parsed selector that can be stored in an arena.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectorData {
    /// The raw selector string
    pub selector: String,
    /// Specificity of this selector
    pub specificity: css_types::Specificity,
    /// Whether this is a universal selector
    pub is_universal: bool,
}

impl SelectorData {
    /// Create a new selector data
    pub fn new(selector: String, specificity: css_types::Specificity) -> Self {
        let is_universal = selector.trim() == "*";
        Self {
            selector,
            specificity,
            is_universal,
        }
    }
}

/// Type alias for selector arena
///
/// Used for storing parsed selectors.
pub type SelectorArena = StyleArena<SelectorData>;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // ArenaIndex Tests
    // ============================================

    #[test]
    fn test_arena_index_new() {
        let idx: ArenaIndex<u32> = ArenaIndex::new(1, 2);
        assert_eq!(idx.chunk(), 1);
        assert_eq!(idx.index(), 2);
    }

    #[test]
    fn test_arena_index_clone() {
        let idx: ArenaIndex<u32> = ArenaIndex::new(5, 10);
        let cloned = idx;
        assert_eq!(idx, cloned);
    }

    #[test]
    fn test_arena_index_eq() {
        let idx1: ArenaIndex<u32> = ArenaIndex::new(1, 2);
        let idx2: ArenaIndex<u32> = ArenaIndex::new(1, 2);
        let idx3: ArenaIndex<u32> = ArenaIndex::new(1, 3);

        assert_eq!(idx1, idx2);
        assert_ne!(idx1, idx3);
    }

    #[test]
    fn test_arena_index_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let idx1: ArenaIndex<u32> = ArenaIndex::new(1, 2);
        let idx2: ArenaIndex<u32> = ArenaIndex::new(1, 2);
        let idx3: ArenaIndex<u32> = ArenaIndex::new(2, 2);

        set.insert(idx1);
        assert!(set.contains(&idx2));
        assert!(!set.contains(&idx3));
    }

    #[test]
    fn test_arena_index_to_from_raw() {
        let idx: ArenaIndex<u32> = ArenaIndex::new(100, 200);
        let raw = idx.to_raw();
        let recovered: ArenaIndex<u32> = ArenaIndex::from_raw(raw);

        assert_eq!(idx, recovered);
        assert_eq!(recovered.chunk(), 100);
        assert_eq!(recovered.index(), 200);
    }

    #[test]
    fn test_arena_index_raw_roundtrip_edge_cases() {
        // Test with maximum values
        let idx: ArenaIndex<u32> = ArenaIndex::new(u32::MAX, u32::MAX);
        let recovered: ArenaIndex<u32> = ArenaIndex::from_raw(idx.to_raw());
        assert_eq!(idx, recovered);

        // Test with zero
        let idx_zero: ArenaIndex<u32> = ArenaIndex::new(0, 0);
        let recovered_zero: ArenaIndex<u32> = ArenaIndex::from_raw(idx_zero.to_raw());
        assert_eq!(idx_zero, recovered_zero);
    }

    // ============================================
    // StyleArena Basic Tests
    // ============================================

    #[test]
    fn test_arena_new() {
        let arena: StyleArena<u32> = StyleArena::new();
        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.chunk_count(), 1);
    }

    #[test]
    fn test_arena_with_chunk_capacity() {
        let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(128);
        assert_eq!(arena.chunk_capacity(), 128);
    }

    #[test]
    #[should_panic(expected = "Chunk capacity must be positive")]
    fn test_arena_zero_capacity_panics() {
        let _arena: StyleArena<u32> = StyleArena::with_chunk_capacity(0);
    }

    #[test]
    fn test_arena_default() {
        let arena: StyleArena<u32> = StyleArena::default();
        assert!(arena.is_empty());
    }

    // ============================================
    // Allocation and Retrieval Tests
    // ============================================

    #[test]
    fn test_arena_alloc_single() {
        let arena: StyleArena<u32> = StyleArena::new();
        let idx = arena.alloc(42);

        assert_eq!(*arena.get(idx), 42);
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn test_arena_alloc_multiple() {
        let arena: StyleArena<i32> = StyleArena::new();
        let idx1 = arena.alloc(10);
        let idx2 = arena.alloc(20);
        let idx3 = arena.alloc(30);

        assert_eq!(*arena.get(idx1), 10);
        assert_eq!(*arena.get(idx2), 20);
        assert_eq!(*arena.get(idx3), 30);
        assert_eq!(arena.len(), 3);
    }

    #[test]
    fn test_arena_alloc_string() {
        let arena: StyleArena<String> = StyleArena::new();
        let idx1 = arena.alloc("hello".to_string());
        let idx2 = arena.alloc("world".to_string());

        assert_eq!(arena.get(idx1).as_str(), "hello");
        assert_eq!(arena.get(idx2).as_str(), "world");
    }

    #[test]
    fn test_arena_alloc_complex_type() {
        #[derive(Debug, PartialEq)]
        struct ComplexType {
            id: u64,
            name: String,
            values: Vec<f64>,
        }

        let arena: StyleArena<ComplexType> = StyleArena::new();
        let complex = ComplexType {
            id: 123,
            name: "test".to_string(),
            values: vec![1.0, 2.0, 3.0],
        };

        let idx = arena.alloc(complex);
        let retrieved = arena.get(idx);

        assert_eq!(retrieved.id, 123);
        assert_eq!(retrieved.name, "test");
        assert_eq!(retrieved.values, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_arena_try_get_valid() {
        let arena: StyleArena<u32> = StyleArena::new();
        let idx = arena.alloc(42);

        assert_eq!(arena.try_get(idx), Some(&42));
    }

    #[test]
    fn test_arena_try_get_invalid() {
        let arena: StyleArena<u32> = StyleArena::new();
        arena.alloc(42);

        let invalid_idx: ArenaIndex<u32> = ArenaIndex::new(999, 0);
        assert_eq!(arena.try_get(invalid_idx), None);

        let invalid_idx2: ArenaIndex<u32> = ArenaIndex::new(0, 999);
        assert_eq!(arena.try_get(invalid_idx2), None);
    }

    #[test]
    fn test_arena_get_mut() {
        let mut arena: StyleArena<u32> = StyleArena::new();
        let idx = arena.alloc(10);

        assert_eq!(*arena.get(idx), 10);

        *arena.get_mut(idx) = 20;

        assert_eq!(*arena.get(idx), 20);
    }

    // ============================================
    // Arena Growth Tests
    // ============================================

    #[test]
    fn test_arena_growth_small_capacity() {
        let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(2);
        assert_eq!(arena.chunk_count(), 1);

        arena.alloc(1);
        arena.alloc(2);
        assert_eq!(arena.chunk_count(), 1);

        // This should trigger a new chunk
        arena.alloc(3);
        assert_eq!(arena.chunk_count(), 2);

        arena.alloc(4);
        assert_eq!(arena.chunk_count(), 2);

        // Another new chunk
        arena.alloc(5);
        assert_eq!(arena.chunk_count(), 3);

        // Verify all values
        assert_eq!(arena.len(), 5);
    }

    #[test]
    fn test_arena_growth_many_elements() {
        let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(10);

        for i in 0..100 {
            let idx = arena.alloc(i);
            assert_eq!(*arena.get(idx), i);
        }

        assert_eq!(arena.len(), 100);
        assert_eq!(arena.chunk_count(), 10);
    }

    #[test]
    fn test_arena_indices_remain_valid_after_growth() {
        let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(2);

        let idx1 = arena.alloc(1);
        let idx2 = arena.alloc(2);
        let idx3 = arena.alloc(3); // New chunk
        let idx4 = arena.alloc(4);
        let idx5 = arena.alloc(5); // Another new chunk

        // All indices should still be valid
        assert_eq!(*arena.get(idx1), 1);
        assert_eq!(*arena.get(idx2), 2);
        assert_eq!(*arena.get(idx3), 3);
        assert_eq!(*arena.get(idx4), 4);
        assert_eq!(*arena.get(idx5), 5);
    }

    // ============================================
    // Clear and Reuse Tests
    // ============================================

    #[test]
    fn test_arena_clear_empty() {
        let mut arena: StyleArena<u32> = StyleArena::new();
        arena.clear();

        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn test_arena_clear_with_elements() {
        let mut arena: StyleArena<u32> = StyleArena::new();
        arena.alloc(1);
        arena.alloc(2);
        arena.alloc(3);

        assert_eq!(arena.len(), 3);

        arena.clear();

        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.chunk_count(), 1); // Keeps one chunk
    }

    #[test]
    fn test_arena_clear_multiple_chunks() {
        let mut arena: StyleArena<u32> = StyleArena::with_chunk_capacity(2);

        // Fill multiple chunks
        for i in 0..10 {
            arena.alloc(i);
        }

        assert_eq!(arena.chunk_count(), 5);

        arena.clear();

        // Should be back to 1 chunk
        assert_eq!(arena.chunk_count(), 1);
        assert!(arena.is_empty());
    }

    #[test]
    fn test_arena_reuse_after_clear() {
        let mut arena: StyleArena<u32> = StyleArena::new();

        // First use
        let idx1 = arena.alloc(10);
        assert_eq!(*arena.get(idx1), 10);

        arena.clear();

        // Reuse
        let idx2 = arena.alloc(20);
        assert_eq!(*arena.get(idx2), 20);
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn test_arena_clear_drops_elements() {
        use std::cell::Cell;
        use std::rc::Rc;

        let drop_count = Rc::new(Cell::new(0));

        struct DropCounter {
            count: Rc<Cell<u32>>,
        }

        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.count.set(self.count.get() + 1);
            }
        }

        let mut arena: StyleArena<DropCounter> = StyleArena::new();

        arena.alloc(DropCounter {
            count: drop_count.clone(),
        });
        arena.alloc(DropCounter {
            count: drop_count.clone(),
        });
        arena.alloc(DropCounter {
            count: drop_count.clone(),
        });

        assert_eq!(drop_count.get(), 0);

        arena.clear();

        assert_eq!(drop_count.get(), 3);
    }

    // ============================================
    // Memory Efficiency Tests
    // ============================================

    #[test]
    fn test_arena_memory_usage() {
        let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(100);

        // Should allocate space for 100 u32s = 400 bytes
        let expected = 100 * std::mem::size_of::<u32>();
        assert_eq!(arena.memory_usage(), expected);

        // After filling and triggering new chunk
        for i in 0..101 {
            arena.alloc(i);
        }

        // Now 2 chunks
        assert_eq!(arena.memory_usage(), 2 * expected);
    }

    #[test]
    fn test_arena_chunk_index_encoding() {
        let arena: StyleArena<u32> = StyleArena::with_chunk_capacity(10);

        let mut indices = Vec::new();
        for i in 0..50 {
            indices.push(arena.alloc(i));
        }

        // Verify chunk/index encoding is correct
        for (i, idx) in indices.iter().enumerate() {
            let expected_chunk = (i / 10) as u32;
            let expected_index = (i % 10) as u32;

            assert_eq!(idx.chunk(), expected_chunk);
            assert_eq!(idx.index(), expected_index);
        }
    }

    // ============================================
    // Type Alias Tests
    // ============================================

    #[test]
    fn test_selector_data_new() {
        use css_types::Specificity;

        let selector = SelectorData::new(".class".to_string(), Specificity::new(0, 1, 0));

        assert_eq!(selector.selector, ".class");
        assert!(!selector.is_universal);
    }

    #[test]
    fn test_selector_data_universal() {
        use css_types::Specificity;

        let selector = SelectorData::new("*".to_string(), Specificity::zero());

        assert!(selector.is_universal);
    }

    #[test]
    fn test_selector_arena() {
        use css_types::Specificity;

        let arena: SelectorArena = SelectorArena::new();

        let idx1 = arena.alloc(SelectorData::new(
            "div".to_string(),
            Specificity::new(0, 0, 1),
        ));

        let idx2 = arena.alloc(SelectorData::new(
            "#id".to_string(),
            Specificity::new(1, 0, 0),
        ));

        assert_eq!(arena.get(idx1).selector, "div");
        assert_eq!(arena.get(idx2).selector, "#id");
    }

    // ============================================
    // Edge Case Tests
    // ============================================

    #[test]
    fn test_arena_with_zero_sized_type() {
        let arena: StyleArena<()> = StyleArena::new();

        let idx1 = arena.alloc(());
        let idx2 = arena.alloc(());

        // Should work even with zero-sized types
        assert_eq!(arena.len(), 2);
        let _ = arena.get(idx1);
        let _ = arena.get(idx2);
    }

    #[test]
    fn test_arena_large_allocation() {
        #[derive(Clone)]
        struct LargeStruct {
            data: [u8; 1024],
        }

        let arena: StyleArena<LargeStruct> = StyleArena::with_chunk_capacity(4);

        let value = LargeStruct { data: [42; 1024] };

        let idx = arena.alloc(value.clone());

        assert_eq!(arena.get(idx).data[0], 42);
        assert_eq!(arena.get(idx).data[1023], 42);
    }

    #[test]
    fn test_arena_stress() {
        let arena: StyleArena<u64> = StyleArena::new();

        // Allocate many elements
        let count = 10000;
        let mut indices = Vec::with_capacity(count);

        for i in 0..count {
            indices.push(arena.alloc(i as u64));
        }

        // Verify all
        for (i, idx) in indices.iter().enumerate() {
            assert_eq!(*arena.get(*idx), i as u64);
        }

        assert_eq!(arena.len(), count);
    }

    // ============================================
    // Integration Tests with CSS Types
    // ============================================

    #[test]
    fn test_style_data_arena_with_computed_values() {
        use super::super::types::ComputedValues;

        let arena: StyleDataArena = StyleDataArena::new();

        let values1 = ComputedValues::default();
        let idx1 = arena.alloc(values1);

        let mut values2 = ComputedValues::default();
        values2.color = css_types::Color::rgb(255, 0, 0);
        let idx2 = arena.alloc(values2);

        // Verify different values
        assert_eq!(arena.get(idx1).color, css_types::Color::rgb(0, 0, 0));
        assert_eq!(arena.get(idx2).color, css_types::Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_rule_node_arena() {
        use super::super::rule_tree::{RuleNode, RuleSource};
        use css_types::Specificity;

        let arena: StyleArena<RuleNode> = StyleArena::new();

        let node = RuleNode {
            declarations: vec![],
            specificity: Specificity::new(0, 1, 0),
            source: RuleSource::UserAgent,
            parent: None,
            children: vec![],
            depth: 0,
        };

        let idx = arena.alloc(node);
        assert_eq!(arena.get(idx).depth, 0);
    }
}
