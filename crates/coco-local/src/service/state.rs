use std::sync::Arc;
use std::time::Duration;

use crate::embedder::LocalEmbedder;
use crate::metrics::LocalMetrics;
use crate::storage::lance::LanceStore;
use crate::storage::meta::LocalMetaStore;
use crate::watcher::WatchSender;

#[derive(Clone)]
pub(crate) struct ServiceState {
    pub(crate) meta: LocalMetaStore,
    pub(crate) vector: LanceStore,
    pub(crate) embedder: Option<Arc<LocalEmbedder>>,
    pub(crate) watcher: Option<WatchSender>,
    pub(crate) metrics: Arc<LocalMetrics>,
    pub(crate) live_retrieval_enabled: bool,
    pub(crate) live_retrieval_window_bytes: usize,
    pub(crate) live_grep: LiveGrepConfig,
}

#[derive(Debug, Clone)]
pub(crate) struct LiveGrepConfig {
    pub(crate) enabled: bool,
    pub(crate) max_results: usize,
    pub(crate) timeout: Duration,
}
