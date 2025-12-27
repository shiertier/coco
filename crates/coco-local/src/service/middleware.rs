use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use chrono::{SecondsFormat, Utc};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

use super::constants::ACCESS_LOG_SCHEMA_VERSION;

#[derive(Debug, Clone)]
struct TraceId(String);

pub(crate) async fn access_log(mut request: Request<Body>, next: Next) -> Response {
    let trace_id = TraceId(generate_trace_id());
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let start = std::time::Instant::now();
    request.extensions_mut().insert(trace_id.clone());

    let response = next.run(request).await;
    let status = response.status().as_u16();
    let latency_ms = start.elapsed().as_millis();
    let ts = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let log = json!({
        "schema_version": ACCESS_LOG_SCHEMA_VERSION,
        "ts": ts,
        "trace_id": trace_id.0,
        "method": method,
        "path": path,
        "status": status,
        "latency_ms": latency_ms,
    });
    info!("{log}");
    response
}

fn generate_trace_id() -> String {
    Uuid::now_v7().to_string()
}
