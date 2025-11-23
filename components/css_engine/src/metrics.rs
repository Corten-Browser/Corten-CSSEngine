//! CSS Performance Metrics
//!
//! This module provides performance metrics collection for the CSS engine.
//! It tracks timing information, cache statistics, and resource usage.
//!
//! # Example
//!
//! ```ignore
//! use css_engine::metrics::MetricsCollector;
//!
//! let mut collector = MetricsCollector::new();
//!
//! {
//!     let _guard = collector.start_parse();
//!     // ... parse stylesheet ...
//! }
//!
//! let snapshot = collector.snapshot();
//! println!("Parse time: {:?}", snapshot.parse_time);
//! ```

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use parking_lot::Mutex;

/// Comprehensive metrics for CSS engine operations
#[derive(Debug, Clone, Default)]
pub struct CssEngineMetrics {
    /// Time spent parsing CSS
    pub parse_time: Duration,
    /// Time spent matching selectors
    pub match_time: Duration,
    /// Time spent in cascade resolution
    pub cascade_time: Duration,
    /// Total computation time
    pub total_time: Duration,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Number of rules matched
    pub rules_matched: u64,
    /// Number of elements styled
    pub elements_styled: u64,
    /// Memory usage in bytes
    pub memory_bytes: usize,
}

impl CssEngineMetrics {
    /// Create new empty metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Get cache hit rate (0.0 to 1.0)
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    /// Get average time per element
    pub fn time_per_element(&self) -> Duration {
        if self.elements_styled == 0 {
            Duration::ZERO
        } else {
            self.total_time / self.elements_styled as u32
        }
    }

    /// Get average rules matched per element
    pub fn rules_per_element(&self) -> f64 {
        if self.elements_styled == 0 {
            0.0
        } else {
            self.rules_matched as f64 / self.elements_styled as f64
        }
    }

    /// Merge another metrics instance into this one
    pub fn merge(&mut self, other: &CssEngineMetrics) {
        self.parse_time += other.parse_time;
        self.match_time += other.match_time;
        self.cascade_time += other.cascade_time;
        self.total_time += other.total_time;
        self.cache_hits += other.cache_hits;
        self.cache_misses += other.cache_misses;
        self.rules_matched += other.rules_matched;
        self.elements_styled += other.elements_styled;
        self.memory_bytes = self.memory_bytes.max(other.memory_bytes);
    }

    /// Reset all metrics to zero
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// A guard that records elapsed time when dropped
pub struct TimingGuard<'a> {
    /// Reference to the collector
    collector: &'a MetricsCollector,
    /// Which timing to update
    timing_type: TimingType,
    /// When this guard was created
    start: Instant,
}

impl<'a> Drop for TimingGuard<'a> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let mut current = self.collector.current.lock();

        match self.timing_type {
            TimingType::Parse => current.parse_time += elapsed,
            TimingType::Match => current.match_time += elapsed,
            TimingType::Cascade => current.cascade_time += elapsed,
            TimingType::Total => current.total_time += elapsed,
        }
    }
}

/// Type of timing being measured
#[derive(Debug, Clone, Copy)]
enum TimingType {
    Parse,
    Match,
    Cascade,
    Total,
}

/// Metrics collector for tracking CSS engine performance
#[derive(Debug)]
pub struct MetricsCollector {
    /// Current metrics being collected
    current: Mutex<CssEngineMetrics>,
    /// History of snapshots
    history: Mutex<Vec<CssEngineMetrics>>,
    /// Atomic counters for thread-safe increments
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    rules_matched: AtomicU64,
    elements_styled: AtomicU64,
    /// Maximum history size
    max_history: usize,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            current: Mutex::new(CssEngineMetrics::new()),
            history: Mutex::new(Vec::new()),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            rules_matched: AtomicU64::new(0),
            elements_styled: AtomicU64::new(0),
            max_history: 100,
        }
    }

    /// Create a collector with custom history size
    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            max_history,
            ..Self::new()
        }
    }

    /// Start timing CSS parsing
    ///
    /// Returns a guard that automatically records the elapsed time when dropped.
    pub fn start_parse(&self) -> TimingGuard<'_> {
        TimingGuard {
            collector: self,
            timing_type: TimingType::Parse,
            start: Instant::now(),
        }
    }

    /// Start timing selector matching
    ///
    /// Returns a guard that automatically records the elapsed time when dropped.
    pub fn start_match(&self) -> TimingGuard<'_> {
        TimingGuard {
            collector: self,
            timing_type: TimingType::Match,
            start: Instant::now(),
        }
    }

    /// Start timing cascade resolution
    ///
    /// Returns a guard that automatically records the elapsed time when dropped.
    pub fn start_cascade(&self) -> TimingGuard<'_> {
        TimingGuard {
            collector: self,
            timing_type: TimingType::Cascade,
            start: Instant::now(),
        }
    }

    /// Start timing total computation
    ///
    /// Returns a guard that automatically records the elapsed time when dropped.
    pub fn start_total(&self) -> TimingGuard<'_> {
        TimingGuard {
            collector: self,
            timing_type: TimingType::Total,
            start: Instant::now(),
        }
    }

    /// Record a cache hit (thread-safe)
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache miss (thread-safe)
    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a rule match (thread-safe)
    pub fn record_rule_matched(&self) {
        self.rules_matched.fetch_add(1, Ordering::Relaxed);
    }

    /// Record multiple rule matches (thread-safe)
    pub fn record_rules_matched(&self, count: u64) {
        self.rules_matched.fetch_add(count, Ordering::Relaxed);
    }

    /// Record an element being styled (thread-safe)
    pub fn record_element_styled(&self) {
        self.elements_styled.fetch_add(1, Ordering::Relaxed);
    }

    /// Record multiple elements being styled (thread-safe)
    pub fn record_elements_styled(&self, count: u64) {
        self.elements_styled.fetch_add(count, Ordering::Relaxed);
    }

    /// Record memory usage
    pub fn record_memory(&self, bytes: usize) {
        let mut current = self.current.lock();
        current.memory_bytes = bytes;
    }

    /// Take a snapshot of current metrics
    ///
    /// This syncs atomic counters, stores in history, and returns a copy.
    pub fn snapshot(&self) -> CssEngineMetrics {
        let mut current = self.current.lock();

        // Sync atomic counters
        current.cache_hits = self.cache_hits.load(Ordering::Relaxed);
        current.cache_misses = self.cache_misses.load(Ordering::Relaxed);
        current.rules_matched = self.rules_matched.load(Ordering::Relaxed);
        current.elements_styled = self.elements_styled.load(Ordering::Relaxed);

        let snapshot = current.clone();

        // Store in history
        let mut history = self.history.lock();
        history.push(snapshot.clone());

        // Trim history if needed
        while history.len() > self.max_history {
            history.remove(0);
        }

        snapshot
    }

    /// Get the current metrics without taking a snapshot
    pub fn current_metrics(&self) -> CssEngineMetrics {
        let mut current = self.current.lock();

        // Sync atomic counters
        current.cache_hits = self.cache_hits.load(Ordering::Relaxed);
        current.cache_misses = self.cache_misses.load(Ordering::Relaxed);
        current.rules_matched = self.rules_matched.load(Ordering::Relaxed);
        current.elements_styled = self.elements_styled.load(Ordering::Relaxed);

        current.clone()
    }

    /// Get metrics history
    pub fn history(&self) -> Vec<CssEngineMetrics> {
        self.history.lock().clone()
    }

    /// Get the number of snapshots in history
    pub fn history_len(&self) -> usize {
        self.history.lock().len()
    }

    /// Clear history
    pub fn clear_history(&self) {
        self.history.lock().clear();
    }

    /// Reset all metrics (current and atomic counters)
    pub fn reset(&self) {
        let mut current = self.current.lock();
        current.reset();

        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
        self.rules_matched.store(0, Ordering::Relaxed);
        self.elements_styled.store(0, Ordering::Relaxed);
    }

    /// Get aggregate statistics from history
    pub fn aggregate(&self) -> Option<AggregateMetrics> {
        let history = self.history.lock();
        if history.is_empty() {
            return None;
        }

        let mut total_parse = Duration::ZERO;
        let mut total_match = Duration::ZERO;
        let mut total_cascade = Duration::ZERO;
        let mut total_time = Duration::ZERO;
        let mut total_cache_hits: u64 = 0;
        let mut total_cache_misses: u64 = 0;
        let mut total_elements: u64 = 0;
        let mut max_memory: usize = 0;

        for metrics in history.iter() {
            total_parse += metrics.parse_time;
            total_match += metrics.match_time;
            total_cascade += metrics.cascade_time;
            total_time += metrics.total_time;
            total_cache_hits += metrics.cache_hits;
            total_cache_misses += metrics.cache_misses;
            total_elements += metrics.elements_styled;
            max_memory = max_memory.max(metrics.memory_bytes);
        }

        let count = history.len() as u32;
        Some(AggregateMetrics {
            sample_count: history.len(),
            avg_parse_time: total_parse / count,
            avg_match_time: total_match / count,
            avg_cascade_time: total_cascade / count,
            avg_total_time: total_time / count,
            total_cache_hits,
            total_cache_misses,
            total_elements_styled: total_elements,
            peak_memory_bytes: max_memory,
        })
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregate metrics from history
#[derive(Debug, Clone)]
pub struct AggregateMetrics {
    /// Number of samples in aggregate
    pub sample_count: usize,
    /// Average parse time
    pub avg_parse_time: Duration,
    /// Average match time
    pub avg_match_time: Duration,
    /// Average cascade time
    pub avg_cascade_time: Duration,
    /// Average total time
    pub avg_total_time: Duration,
    /// Total cache hits across all samples
    pub total_cache_hits: u64,
    /// Total cache misses across all samples
    pub total_cache_misses: u64,
    /// Total elements styled across all samples
    pub total_elements_styled: u64,
    /// Peak memory usage
    pub peak_memory_bytes: usize,
}

impl AggregateMetrics {
    /// Get overall cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.total_cache_hits + self.total_cache_misses;
        if total == 0 {
            0.0
        } else {
            self.total_cache_hits as f64 / total as f64
        }
    }
}

/// Scoped metrics for a specific operation
pub struct ScopedMetrics<'a> {
    collector: &'a MetricsCollector,
    _total_guard: TimingGuard<'a>,
}

impl<'a> ScopedMetrics<'a> {
    /// Create scoped metrics that track total time
    pub fn new(collector: &'a MetricsCollector) -> Self {
        Self {
            collector,
            _total_guard: collector.start_total(),
        }
    }

    /// Start parse timing within this scope
    pub fn start_parse(&self) -> TimingGuard<'_> {
        self.collector.start_parse()
    }

    /// Start match timing within this scope
    pub fn start_match(&self) -> TimingGuard<'_> {
        self.collector.start_match()
    }

    /// Start cascade timing within this scope
    pub fn start_cascade(&self) -> TimingGuard<'_> {
        self.collector.start_cascade()
    }

    /// Record a cache hit
    pub fn record_cache_hit(&self) {
        self.collector.record_cache_hit();
    }

    /// Record a cache miss
    pub fn record_cache_miss(&self) {
        self.collector.record_cache_miss();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_metrics_creation() {
        let metrics = CssEngineMetrics::new();
        assert_eq!(metrics.cache_hits, 0);
        assert_eq!(metrics.cache_misses, 0);
        assert_eq!(metrics.parse_time, Duration::ZERO);
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut metrics = CssEngineMetrics::new();
        metrics.cache_hits = 75;
        metrics.cache_misses = 25;

        assert!((metrics.cache_hit_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_cache_hit_rate_zero_total() {
        let metrics = CssEngineMetrics::new();
        assert_eq!(metrics.cache_hit_rate(), 0.0);
    }

    #[test]
    fn test_time_per_element() {
        let mut metrics = CssEngineMetrics::new();
        metrics.total_time = Duration::from_millis(100);
        metrics.elements_styled = 10;

        assert_eq!(metrics.time_per_element(), Duration::from_millis(10));
    }

    #[test]
    fn test_time_per_element_zero() {
        let metrics = CssEngineMetrics::new();
        assert_eq!(metrics.time_per_element(), Duration::ZERO);
    }

    #[test]
    fn test_rules_per_element() {
        let mut metrics = CssEngineMetrics::new();
        metrics.rules_matched = 50;
        metrics.elements_styled = 10;

        assert!((metrics.rules_per_element() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_metrics_merge() {
        let mut metrics1 = CssEngineMetrics::new();
        metrics1.cache_hits = 10;
        metrics1.parse_time = Duration::from_millis(50);

        let mut metrics2 = CssEngineMetrics::new();
        metrics2.cache_hits = 20;
        metrics2.parse_time = Duration::from_millis(30);

        metrics1.merge(&metrics2);
        assert_eq!(metrics1.cache_hits, 30);
        assert_eq!(metrics1.parse_time, Duration::from_millis(80));
    }

    #[test]
    fn test_metrics_reset() {
        let mut metrics = CssEngineMetrics::new();
        metrics.cache_hits = 100;
        metrics.parse_time = Duration::from_secs(1);

        metrics.reset();
        assert_eq!(metrics.cache_hits, 0);
        assert_eq!(metrics.parse_time, Duration::ZERO);
    }

    #[test]
    fn test_collector_creation() {
        let collector = MetricsCollector::new();
        let metrics = collector.current_metrics();
        assert_eq!(metrics.cache_hits, 0);
    }

    #[test]
    fn test_collector_with_max_history() {
        let collector = MetricsCollector::with_max_history(5);
        assert_eq!(collector.max_history, 5);
    }

    #[test]
    fn test_timing_guard_parse() {
        let collector = MetricsCollector::new();

        {
            let _guard = collector.start_parse();
            thread::sleep(StdDuration::from_millis(10));
        }

        let metrics = collector.current_metrics();
        assert!(metrics.parse_time >= Duration::from_millis(10));
    }

    #[test]
    fn test_timing_guard_match() {
        let collector = MetricsCollector::new();

        {
            let _guard = collector.start_match();
            thread::sleep(StdDuration::from_millis(10));
        }

        let metrics = collector.current_metrics();
        assert!(metrics.match_time >= Duration::from_millis(10));
    }

    #[test]
    fn test_record_cache_hit() {
        let collector = MetricsCollector::new();

        collector.record_cache_hit();
        collector.record_cache_hit();

        let metrics = collector.current_metrics();
        assert_eq!(metrics.cache_hits, 2);
    }

    #[test]
    fn test_record_cache_miss() {
        let collector = MetricsCollector::new();

        collector.record_cache_miss();

        let metrics = collector.current_metrics();
        assert_eq!(metrics.cache_misses, 1);
    }

    #[test]
    fn test_record_element_styled() {
        let collector = MetricsCollector::new();

        collector.record_element_styled();
        collector.record_elements_styled(9);

        let metrics = collector.current_metrics();
        assert_eq!(metrics.elements_styled, 10);
    }

    #[test]
    fn test_record_rules_matched() {
        let collector = MetricsCollector::new();

        collector.record_rule_matched();
        collector.record_rules_matched(4);

        let metrics = collector.current_metrics();
        assert_eq!(metrics.rules_matched, 5);
    }

    #[test]
    fn test_record_memory() {
        let collector = MetricsCollector::new();

        collector.record_memory(1024);

        let metrics = collector.current_metrics();
        assert_eq!(metrics.memory_bytes, 1024);
    }

    #[test]
    fn test_snapshot() {
        let collector = MetricsCollector::new();

        collector.record_cache_hit();
        let snapshot = collector.snapshot();

        assert_eq!(snapshot.cache_hits, 1);
        assert_eq!(collector.history_len(), 1);
    }

    #[test]
    fn test_history_limit() {
        let collector = MetricsCollector::with_max_history(3);

        for _ in 0..5 {
            collector.snapshot();
        }

        assert_eq!(collector.history_len(), 3);
    }

    #[test]
    fn test_clear_history() {
        let collector = MetricsCollector::new();

        collector.snapshot();
        collector.snapshot();
        assert_eq!(collector.history_len(), 2);

        collector.clear_history();
        assert_eq!(collector.history_len(), 0);
    }

    #[test]
    fn test_reset() {
        let collector = MetricsCollector::new();

        collector.record_cache_hit();
        collector.record_cache_miss();
        collector.record_element_styled();

        collector.reset();

        let metrics = collector.current_metrics();
        assert_eq!(metrics.cache_hits, 0);
        assert_eq!(metrics.cache_misses, 0);
        assert_eq!(metrics.elements_styled, 0);
    }

    #[test]
    fn test_aggregate_empty() {
        let collector = MetricsCollector::new();
        assert!(collector.aggregate().is_none());
    }

    #[test]
    fn test_aggregate() {
        let collector = MetricsCollector::new();

        collector.record_cache_hit();
        collector.snapshot();

        collector.record_cache_hit();
        collector.snapshot();

        let aggregate = collector.aggregate().unwrap();
        assert_eq!(aggregate.sample_count, 2);
        assert_eq!(aggregate.total_cache_hits, 3); // 1 + 2 (cumulative)
    }

    #[test]
    fn test_aggregate_cache_hit_rate() {
        let collector = MetricsCollector::new();

        collector.record_cache_hit();
        collector.record_cache_hit();
        collector.record_cache_hit();
        collector.record_cache_miss();
        collector.snapshot();

        let aggregate = collector.aggregate().unwrap();
        assert!((aggregate.cache_hit_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_scoped_metrics() {
        let collector = MetricsCollector::new();

        {
            let scoped = ScopedMetrics::new(&collector);
            {
                let _parse = scoped.start_parse();
                thread::sleep(StdDuration::from_millis(5));
            }
            scoped.record_cache_hit();
        }

        let metrics = collector.current_metrics();
        assert!(metrics.total_time >= Duration::from_millis(5));
        assert!(metrics.parse_time >= Duration::from_millis(5));
        assert_eq!(metrics.cache_hits, 1);
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;

        let collector = Arc::new(MetricsCollector::new());
        let mut handles = vec![];

        for _ in 0..4 {
            let collector = Arc::clone(&collector);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    collector.record_cache_hit();
                    collector.record_element_styled();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let metrics = collector.current_metrics();
        assert_eq!(metrics.cache_hits, 400);
        assert_eq!(metrics.elements_styled, 400);
    }
}
