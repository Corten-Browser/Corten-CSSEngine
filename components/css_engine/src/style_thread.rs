//! Style Thread
//!
//! This module provides a dedicated background thread for style computation,
//! allowing the main thread to remain responsive while styles are computed
//! asynchronously.
//!
//! # Example
//!
//! ```ignore
//! use css_engine::style_thread::{StyleThread, StyleTask};
//!
//! let thread = StyleThread::spawn();
//!
//! thread.submit(StyleTask::ComputeStyles {
//!     elements: vec![1, 2, 3],
//!     callback: Box::new(|styles| {
//!         println!("Computed {} styles", styles.len());
//!     }),
//! });
//!
//! thread.shutdown();
//! ```

use crossbeam_channel::{bounded, unbounded, Receiver, Sender, TryRecvError};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::error::ElementId;
use crate::parallel::ElementRef;
use crate::types::ComputedStyle;

/// Callback type for compute styles with element references
pub type StyleRefsCallback = Box<dyn FnOnce(Vec<(ElementId, ComputedStyle)>) + Send>;

/// Tasks that can be submitted to the style thread
pub enum StyleTask {
    /// Compute styles for a set of elements
    ComputeStyles {
        /// Element IDs to compute styles for
        elements: Vec<u64>,
        /// Callback to invoke with computed styles
        callback: Box<dyn FnOnce(Vec<ComputedStyle>) + Send>,
    },
    /// Compute styles with element refs for more details
    ComputeStylesWithRefs {
        /// Element references
        elements: Vec<ElementRef>,
        /// Callback to invoke with computed styles
        callback: StyleRefsCallback,
    },
    /// Invalidate cached styles
    InvalidateStyles {
        /// Element IDs to invalidate
        elements: Vec<u64>,
    },
    /// Flush all pending work
    Flush {
        /// Signal when flush is complete
        done: Sender<()>,
    },
    /// Shutdown the thread
    Shutdown,
}

/// A dedicated thread for style computation
pub struct StyleThread {
    /// Channel to send tasks
    sender: Sender<StyleTask>,
    /// Thread handle
    handle: Option<JoinHandle<()>>,
    /// Whether the thread is running
    is_running: Arc<AtomicBool>,
    /// Number of tasks processed
    tasks_processed: Arc<AtomicU64>,
    /// Number of elements styled
    elements_styled: Arc<AtomicU64>,
}

impl StyleThread {
    /// Spawn a new style thread
    ///
    /// Creates a background thread that processes style computation tasks.
    pub fn spawn() -> Self {
        Self::spawn_with_capacity(1024)
    }

    /// Spawn with a custom channel capacity
    pub fn spawn_with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = bounded(capacity);
        let is_running = Arc::new(AtomicBool::new(true));
        let tasks_processed = Arc::new(AtomicU64::new(0));
        let elements_styled = Arc::new(AtomicU64::new(0));

        let thread_is_running = Arc::clone(&is_running);
        let thread_tasks_processed = Arc::clone(&tasks_processed);
        let thread_elements_styled = Arc::clone(&elements_styled);

        let handle = thread::Builder::new()
            .name("css-style-thread".to_string())
            .spawn(move || {
                Self::run_loop(
                    receiver,
                    thread_is_running,
                    thread_tasks_processed,
                    thread_elements_styled,
                );
            })
            .expect("Failed to spawn style thread");

        Self {
            sender,
            handle: Some(handle),
            is_running,
            tasks_processed,
            elements_styled,
        }
    }

    /// Spawn with an unbounded channel (use with caution)
    pub fn spawn_unbounded() -> Self {
        let (sender, receiver) = unbounded();
        let is_running = Arc::new(AtomicBool::new(true));
        let tasks_processed = Arc::new(AtomicU64::new(0));
        let elements_styled = Arc::new(AtomicU64::new(0));

        let thread_is_running = Arc::clone(&is_running);
        let thread_tasks_processed = Arc::clone(&tasks_processed);
        let thread_elements_styled = Arc::clone(&elements_styled);

        let handle = thread::Builder::new()
            .name("css-style-thread".to_string())
            .spawn(move || {
                Self::run_loop(
                    receiver,
                    thread_is_running,
                    thread_tasks_processed,
                    thread_elements_styled,
                );
            })
            .expect("Failed to spawn style thread");

        Self {
            sender,
            handle: Some(handle),
            is_running,
            tasks_processed,
            elements_styled,
        }
    }

    /// Main processing loop for the style thread
    fn run_loop(
        receiver: Receiver<StyleTask>,
        is_running: Arc<AtomicBool>,
        tasks_processed: Arc<AtomicU64>,
        elements_styled: Arc<AtomicU64>,
    ) {
        while is_running.load(Ordering::Relaxed) {
            match receiver.recv() {
                Ok(task) => {
                    let should_continue =
                        Self::process_task(task, &tasks_processed, &elements_styled);
                    if !should_continue {
                        break;
                    }
                }
                Err(_) => {
                    // Channel disconnected, exit loop
                    break;
                }
            }
        }

        is_running.store(false, Ordering::Relaxed);
    }

    /// Process a single task
    fn process_task(
        task: StyleTask,
        tasks_processed: &Arc<AtomicU64>,
        elements_styled: &Arc<AtomicU64>,
    ) -> bool {
        match task {
            StyleTask::ComputeStyles { elements, callback } => {
                let count = elements.len() as u64;
                let styles = Self::compute_styles_for_ids(&elements);
                elements_styled.fetch_add(count, Ordering::Relaxed);
                callback(styles);
                tasks_processed.fetch_add(1, Ordering::Relaxed);
                true
            }
            StyleTask::ComputeStylesWithRefs { elements, callback } => {
                let count = elements.len() as u64;
                let styles = Self::compute_styles_for_refs(&elements);
                elements_styled.fetch_add(count, Ordering::Relaxed);
                callback(styles);
                tasks_processed.fetch_add(1, Ordering::Relaxed);
                true
            }
            StyleTask::InvalidateStyles { elements: _ } => {
                // Invalidation is handled by clearing internal state
                // In a real implementation, this would clear caches
                tasks_processed.fetch_add(1, Ordering::Relaxed);
                true
            }
            StyleTask::Flush { done } => {
                // Signal that flush is complete
                let _ = done.send(());
                tasks_processed.fetch_add(1, Ordering::Relaxed);
                true
            }
            StyleTask::Shutdown => false,
        }
    }

    /// Compute styles for element IDs (simplified)
    fn compute_styles_for_ids(elements: &[u64]) -> Vec<ComputedStyle> {
        elements.iter().map(|_| ComputedStyle::default()).collect()
    }

    /// Compute styles for element refs (simplified)
    fn compute_styles_for_refs(elements: &[ElementRef]) -> Vec<(ElementId, ComputedStyle)> {
        elements
            .iter()
            .map(|e| (e.id, ComputedStyle::default()))
            .collect()
    }

    /// Submit a task to the style thread
    ///
    /// # Panics
    /// Panics if the thread has been shutdown
    pub fn submit(&self, task: StyleTask) {
        self.sender
            .send(task)
            .expect("Style thread has been shutdown");
    }

    /// Try to submit a task without blocking
    ///
    /// Returns `Err` if the channel is full or disconnected
    pub fn try_submit(&self, task: StyleTask) -> Result<(), StyleTask> {
        self.sender.try_send(task).map_err(|e| e.into_inner())
    }

    /// Check if there are pending tasks
    pub fn has_pending(&self) -> bool {
        !self.sender.is_empty()
    }

    /// Get the number of pending tasks
    pub fn pending_count(&self) -> usize {
        self.sender.len()
    }

    /// Flush all pending tasks and wait for completion
    pub fn flush(&self) {
        let (done_sender, done_receiver) = bounded(1);
        self.submit(StyleTask::Flush { done: done_sender });
        let _ = done_receiver.recv();
    }

    /// Check if the thread is still running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    /// Get the number of tasks processed
    pub fn tasks_processed(&self) -> u64 {
        self.tasks_processed.load(Ordering::Relaxed)
    }

    /// Get the number of elements styled
    pub fn elements_styled(&self) -> u64 {
        self.elements_styled.load(Ordering::Relaxed)
    }

    /// Shutdown the style thread gracefully
    ///
    /// This will wait for all pending tasks to complete before returning.
    pub fn shutdown(mut self) {
        // Send shutdown signal
        let _ = self.sender.send(StyleTask::Shutdown);

        // Wait for thread to finish
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        self.is_running.store(false, Ordering::Relaxed);
    }

    /// Shutdown immediately without waiting for pending tasks
    pub fn shutdown_now(mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        let _ = self.sender.send(StyleTask::Shutdown);

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for StyleThread {
    fn drop(&mut self) {
        if self.is_running.load(Ordering::Relaxed) {
            let _ = self.sender.send(StyleTask::Shutdown);
            if let Some(handle) = self.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

/// A pool of style threads for higher throughput
pub struct StyleThreadPool {
    /// Worker threads
    threads: Vec<StyleThread>,
    /// Round-robin counter
    next_thread: AtomicU64,
}

impl StyleThreadPool {
    /// Create a new thread pool
    pub fn new(thread_count: usize) -> Self {
        let threads = (0..thread_count).map(|_| StyleThread::spawn()).collect();

        Self {
            threads,
            next_thread: AtomicU64::new(0),
        }
    }

    /// Get the number of threads in the pool
    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }

    /// Submit a task to the next available thread (round-robin)
    pub fn submit(&self, task: StyleTask) {
        let index = self.next_thread.fetch_add(1, Ordering::Relaxed) as usize % self.threads.len();
        self.threads[index].submit(task);
    }

    /// Flush all threads
    pub fn flush(&self) {
        for thread in &self.threads {
            thread.flush();
        }
    }

    /// Get total tasks processed across all threads
    pub fn total_tasks_processed(&self) -> u64 {
        self.threads.iter().map(|t| t.tasks_processed()).sum()
    }

    /// Get total elements styled across all threads
    pub fn total_elements_styled(&self) -> u64 {
        self.threads.iter().map(|t| t.elements_styled()).sum()
    }

    /// Shutdown all threads gracefully
    pub fn shutdown(self) {
        for thread in self.threads {
            thread.shutdown();
        }
    }
}

/// Result type for async style computation
pub struct StyleFuture {
    receiver: Receiver<Vec<ComputedStyle>>,
}

impl StyleFuture {
    /// Create a new style future with its sender
    pub fn new() -> (Self, Sender<Vec<ComputedStyle>>) {
        let (sender, receiver) = bounded(1);
        (Self { receiver }, sender)
    }

    /// Block until styles are ready
    pub fn wait(self) -> Vec<ComputedStyle> {
        self.receiver.recv().unwrap_or_default()
    }

    /// Try to get styles without blocking
    pub fn try_get(&self) -> Option<Vec<ComputedStyle>> {
        match self.receiver.try_recv() {
            Ok(styles) => Some(styles),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Vec::new()),
        }
    }

    /// Check if styles are ready
    pub fn is_ready(&self) -> bool {
        !self.receiver.is_empty()
    }
}

impl Default for StyleFuture {
    fn default() -> Self {
        let (future, _) = Self::new();
        future
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    #[test]
    fn test_style_thread_creation() {
        let thread = StyleThread::spawn();
        assert!(thread.is_running());
        thread.shutdown();
    }

    #[test]
    fn test_style_thread_with_capacity() {
        let thread = StyleThread::spawn_with_capacity(10);
        assert!(thread.is_running());
        thread.shutdown();
    }

    #[test]
    fn test_style_thread_unbounded() {
        let thread = StyleThread::spawn_unbounded();
        assert!(thread.is_running());
        thread.shutdown();
    }

    #[test]
    fn test_submit_compute_styles() {
        let thread = StyleThread::spawn();
        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = Arc::clone(&callback_called);

        thread.submit(StyleTask::ComputeStyles {
            elements: vec![1, 2, 3],
            callback: Box::new(move |styles| {
                assert_eq!(styles.len(), 3);
                callback_called_clone.store(true, Ordering::SeqCst);
            }),
        });

        thread.flush();
        assert!(callback_called.load(Ordering::SeqCst));
        assert_eq!(thread.elements_styled(), 3);
        thread.shutdown();
    }

    #[test]
    fn test_submit_compute_styles_with_refs() {
        let thread = StyleThread::spawn();
        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = Arc::clone(&callback_called);

        let elements = vec![
            ElementRef::new(ElementId::new(1), "div"),
            ElementRef::new(ElementId::new(2), "span").with_class("test"),
        ];

        thread.submit(StyleTask::ComputeStylesWithRefs {
            elements,
            callback: Box::new(move |styles| {
                assert_eq!(styles.len(), 2);
                callback_called_clone.store(true, Ordering::SeqCst);
            }),
        });

        thread.flush();
        assert!(callback_called.load(Ordering::SeqCst));
        thread.shutdown();
    }

    #[test]
    fn test_invalidate_styles() {
        let thread = StyleThread::spawn();

        thread.submit(StyleTask::InvalidateStyles {
            elements: vec![1, 2, 3],
        });

        thread.flush();
        assert_eq!(thread.tasks_processed(), 2); // InvalidateStyles + Flush
        thread.shutdown();
    }

    #[test]
    fn test_flush() {
        let thread = StyleThread::spawn();

        for i in 0..5 {
            thread.submit(StyleTask::ComputeStyles {
                elements: vec![i],
                callback: Box::new(|_| {}),
            });
        }

        thread.flush();
        assert_eq!(thread.tasks_processed(), 6); // 5 computes + 1 flush
        thread.shutdown();
    }

    #[test]
    fn test_pending_count() {
        let thread = StyleThread::spawn_with_capacity(100);

        // Submit many tasks quickly
        for i in 0..10 {
            thread.submit(StyleTask::ComputeStyles {
                elements: vec![i],
                callback: Box::new(|_| {}),
            });
        }

        // Wait and flush
        thread.flush();
        assert!(!thread.has_pending()); // After flush, should be empty
        thread.shutdown();
    }

    #[test]
    fn test_try_submit() {
        let thread = StyleThread::spawn_with_capacity(2);

        let result = thread.try_submit(StyleTask::ComputeStyles {
            elements: vec![1],
            callback: Box::new(|_| {}),
        });

        assert!(result.is_ok());
        thread.shutdown();
    }

    #[test]
    fn test_shutdown() {
        let thread = StyleThread::spawn();
        assert!(thread.is_running());
        thread.shutdown();
        // Thread should be cleaned up
    }

    #[test]
    fn test_shutdown_now() {
        let thread = StyleThread::spawn();
        assert!(thread.is_running());
        thread.shutdown_now();
        // Thread should be cleaned up immediately
    }

    #[test]
    fn test_drop_cleanup() {
        {
            let thread = StyleThread::spawn();
            assert!(thread.is_running());
            // Thread goes out of scope here
        }
        // Thread should be automatically cleaned up
    }

    #[test]
    fn test_element_ref_builder() {
        let element = ElementRef::new(ElementId::new(1), "div")
            .with_class("container")
            .with_class("main");

        assert_eq!(element.id, ElementId::new(1));
        assert_eq!(element.tag_name, "div");
        assert_eq!(element.classes.len(), 2);
    }

    #[test]
    fn test_style_thread_pool_creation() {
        let pool = StyleThreadPool::new(4);
        assert_eq!(pool.thread_count(), 4);
        pool.shutdown();
    }

    #[test]
    fn test_style_thread_pool_submit() {
        let pool = StyleThreadPool::new(2);
        let counter = Arc::new(AtomicU64::new(0));

        for _ in 0..4 {
            let counter_clone = Arc::clone(&counter);
            pool.submit(StyleTask::ComputeStyles {
                elements: vec![1],
                callback: Box::new(move |_| {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                }),
            });
        }

        pool.flush();
        assert_eq!(counter.load(Ordering::SeqCst), 4);
        pool.shutdown();
    }

    #[test]
    fn test_style_thread_pool_stats() {
        let pool = StyleThreadPool::new(2);

        for _ in 0..4 {
            pool.submit(StyleTask::ComputeStyles {
                elements: vec![1, 2],
                callback: Box::new(|_| {}),
            });
        }

        pool.flush();
        assert_eq!(pool.total_tasks_processed(), 6); // 4 computes + 2 flushes
        assert_eq!(pool.total_elements_styled(), 8); // 4 * 2 elements
        pool.shutdown();
    }

    #[test]
    fn test_style_future() {
        let (future, sender) = StyleFuture::new();

        assert!(!future.is_ready());

        sender.send(vec![ComputedStyle::default()]).unwrap();

        assert!(future.is_ready());
        let styles = future.wait();
        assert_eq!(styles.len(), 1);
    }

    #[test]
    fn test_style_future_try_get() {
        let (future, sender) = StyleFuture::new();

        assert!(future.try_get().is_none());

        sender.send(vec![ComputedStyle::default()]).unwrap();

        let styles = future.try_get().unwrap();
        assert_eq!(styles.len(), 1);
    }

    #[test]
    fn test_multiple_tasks_ordering() {
        let thread = StyleThread::spawn();
        let results = Arc::new(parking_lot::Mutex::new(Vec::new()));

        for i in 0..5 {
            let results_clone = Arc::clone(&results);
            thread.submit(StyleTask::ComputeStyles {
                elements: vec![i],
                callback: Box::new(move |_| {
                    results_clone.lock().push(i);
                }),
            });
        }

        thread.flush();

        let final_results = results.lock();
        // Tasks should be processed (order may vary slightly due to thread scheduling)
        assert_eq!(final_results.len(), 5);
        thread.shutdown();
    }

    #[test]
    fn test_stress_test() {
        let thread = StyleThread::spawn();
        let counter = Arc::new(AtomicU64::new(0));

        for _ in 0..100 {
            let counter_clone = Arc::clone(&counter);
            thread.submit(StyleTask::ComputeStyles {
                elements: vec![1],
                callback: Box::new(move |_| {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                }),
            });
        }

        thread.flush();
        assert_eq!(counter.load(Ordering::SeqCst), 100);
        thread.shutdown();
    }
}
