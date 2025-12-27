//! Runtime metrics for local mode.

use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const RECENT_LIMIT: usize = 8;
const MIN_ELAPSED: Duration = Duration::from_millis(1);

/// Snapshot of runtime metrics for rendering.
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    /// Queries per second over the last sampling window.
    pub query_qps: f64,
    /// Ingests per second over the last sampling window.
    pub import_qps: f64,
    /// Total number of queries served.
    pub queries_total: u64,
    /// Total number of ingests performed.
    pub imports_total: u64,
    /// Pending filesystem events in the debounce queue.
    pub pending_events: usize,
    /// Recently indexed file paths.
    pub recent_files: Vec<String>,
}

/// Shared runtime metrics for local mode.
#[derive(Debug)]
pub struct LocalMetrics {
    inner: Mutex<MetricsInner>,
}

impl LocalMetrics {
    /// Creates a new metrics container.
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(MetricsInner::new()),
        }
    }

    /// Records a query request.
    pub fn record_query(&self) {
        let mut inner = lock_inner(&self.inner);
        inner.queries_total = inner.queries_total.saturating_add(1);
    }

    /// Records an ingest request and optional file path.
    pub fn record_import(&self, path: Option<&str>) {
        let mut inner = lock_inner(&self.inner);
        inner.imports_total = inner.imports_total.saturating_add(1);
        if let Some(path) = path {
            inner.push_recent(path);
        }
    }

    /// Updates the pending filesystem event count.
    pub fn set_pending_events(&self, pending: usize) {
        let mut inner = lock_inner(&self.inner);
        inner.pending_events = pending;
    }

    /// Returns a snapshot and updates sampling state.
    pub fn snapshot(&self) -> MetricsSnapshot {
        let mut inner = lock_inner(&self.inner);
        inner.snapshot()
    }
}

impl Default for LocalMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct MetricsInner {
    last_tick: Instant,
    last_queries: u64,
    last_imports: u64,
    queries_total: u64,
    imports_total: u64,
    pending_events: usize,
    recent_files: VecDeque<String>,
}

impl MetricsInner {
    fn new() -> Self {
        Self {
            last_tick: Instant::now(),
            last_queries: 0,
            last_imports: 0,
            queries_total: 0,
            imports_total: 0,
            pending_events: 0,
            recent_files: VecDeque::new(),
        }
    }

    fn push_recent(&mut self, path: &str) {
        if self.recent_files.front().map(|front| front == path) == Some(true) {
            return;
        }
        self.recent_files.push_front(path.to_string());
        while self.recent_files.len() > RECENT_LIMIT {
            self.recent_files.pop_back();
        }
    }

    fn snapshot(&mut self) -> MetricsSnapshot {
        let now = Instant::now();
        let elapsed = now
            .checked_duration_since(self.last_tick)
            .unwrap_or(MIN_ELAPSED);
        let elapsed = if elapsed < MIN_ELAPSED { MIN_ELAPSED } else { elapsed };
        let seconds = elapsed.as_secs_f64();

        let queries_delta = self.queries_total.saturating_sub(self.last_queries);
        let imports_delta = self.imports_total.saturating_sub(self.last_imports);
        let query_qps = queries_delta as f64 / seconds;
        let import_qps = imports_delta as f64 / seconds;

        self.last_tick = now;
        self.last_queries = self.queries_total;
        self.last_imports = self.imports_total;

        MetricsSnapshot {
            query_qps,
            import_qps,
            queries_total: self.queries_total,
            imports_total: self.imports_total,
            pending_events: self.pending_events,
            recent_files: self.recent_files.iter().cloned().collect(),
        }
    }
}

fn lock_inner(inner: &Mutex<MetricsInner>) -> std::sync::MutexGuard<'_, MetricsInner> {
    match inner.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}
