//! Optional Redis queue for ingest jobs.

use std::sync::Arc;
use std::time::Duration;

use coco_protocol::{CocoError, CocoResult};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};
use tokio::sync::Mutex;

/// Redis-backed queue for ingest job notifications.
#[derive(Clone)]
pub struct RedisQueue {
    manager: Arc<Mutex<ConnectionManager>>,
    key: String,
}

impl RedisQueue {
    /// Connects to Redis and prepares a queue.
    pub async fn connect(url: &str, key: impl Into<String>) -> CocoResult<Self> {
        let client = Client::open(url)
            .map_err(|err| CocoError::network(format!("redis client error: {err}")))?;
        let manager = ConnectionManager::new(client)
            .await
            .map_err(|err| CocoError::network(format!("redis connection error: {err}")))?;
        Ok(Self {
            manager: Arc::new(Mutex::new(manager)),
            key: key.into(),
        })
    }

    /// Pushes a job id onto the queue.
    pub async fn enqueue(&self, job_id: &str) -> CocoResult<()> {
        let mut guard = self.manager.lock().await;
        guard
            .rpush::<_, _, ()>(&self.key, job_id)
            .await
            .map_err(|err| CocoError::network(format!("redis enqueue error: {err}")))?;
        Ok(())
    }

    /// Checks connectivity by issuing a ping to Redis.
    pub async fn ping(&self) -> CocoResult<()> {
        let mut guard = self.manager.lock().await;
        let pong: String = redis::cmd("PING")
            .query_async(&mut *guard)
            .await
            .map_err(|err| CocoError::network(format!("redis ping error: {err}")))?;
        if !pong.eq_ignore_ascii_case("PONG") {
            return Err(CocoError::network(
                "redis ping returned unexpected response",
            ));
        }
        Ok(())
    }

    /// Pops a job id from the queue with a blocking timeout.
    pub async fn pop(&self, timeout: Duration) -> CocoResult<Option<String>> {
        let timeout_secs = usize::try_from(timeout.as_secs()).unwrap_or(0);
        let mut guard = self.manager.lock().await;
        let result: Option<(String, String)> = guard
            .blpop(&self.key, timeout_secs as f64)
            .await
            .map_err(|err| CocoError::network(format!("redis pop error: {err}")))?;
        Ok(result.map(|(_, value)| value))
    }
}
