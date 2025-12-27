use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Debug)]
pub(crate) struct RateLimiter {
    capacity: f64,
    refill_per_sec: f64,
    buckets: Mutex<HashMap<String, Bucket>>,
}

impl RateLimiter {
    pub(crate) fn new(rate_per_min: u32, burst: u32) -> Self {
        let rate_per_min = rate_per_min.max(1);
        let burst = burst.max(1);
        let refill_per_sec = rate_per_min as f64 / 60.0;
        Self {
            capacity: burst as f64,
            refill_per_sec,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn allow(&self, key: &str) -> bool {
        let mut buckets = match self.buckets.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let now = Instant::now();
        let bucket = buckets
            .entry(key.to_string())
            .or_insert_with(|| Bucket::new(self.capacity));
        bucket.refill(now, self.refill_per_sec, self.capacity);
        bucket.take_one()
    }
}

#[derive(Debug)]
struct Bucket {
    tokens: f64,
    last: Instant,
}

impl Bucket {
    fn new(capacity: f64) -> Self {
        Self {
            tokens: capacity,
            last: Instant::now(),
        }
    }

    fn refill(&mut self, now: Instant, refill_per_sec: f64, capacity: f64) {
        let elapsed = now.duration_since(self.last);
        let tokens = elapsed.as_secs_f64() * refill_per_sec;
        self.tokens = (self.tokens + tokens).min(capacity);
        self.last = now;
    }

    fn take_one(&mut self) -> bool {
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}
