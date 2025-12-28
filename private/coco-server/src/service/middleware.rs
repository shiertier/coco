use std::time::Instant;

use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use serde_json::json;
use tracing::info;
use uuid::Uuid;

use coco_protocol::{CocoError, CocoResult};

use super::constants::ACCESS_LOG_SCHEMA_VERSION;
use super::state::AppState;

#[derive(Debug, Clone)]
struct TraceId(String);

pub(crate) async fn access_log(mut request: Request, next: Next) -> Response {
    let trace_id = TraceId(generate_trace_id());
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let start = Instant::now();
    request.extensions_mut().insert(trace_id.clone());

    let response = next.run(request).await;
    let status = response.status().as_u16();
    let latency_ms = start.elapsed().as_millis();
    let ts = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let log = build_access_log_entry(&method, &path, status, latency_ms, &trace_id.0, &ts);
    info!("{log}");
    response
}

pub(crate) fn build_access_log_entry(
    method: &str,
    path: &str,
    status: u16,
    latency_ms: u128,
    trace_id: &str,
    ts: &str,
) -> serde_json::Value {
    json!({
        "schema_version": ACCESS_LOG_SCHEMA_VERSION,
        "ts": ts,
        "trace_id": trace_id,
        "method": method,
        "path": path,
        "status": status,
        "latency_ms": latency_ms,
    })
}

fn generate_trace_id() -> String {
    Uuid::now_v7().to_string()
}

#[cfg(test)]
mod tests {
    use super::{build_access_log_entry, generate_trace_id};
    use uuid::Uuid;

    fn is_rfc3339_millis(value: &str) -> bool {
        let Some((_, suffix)) = value.split_once('.') else {
            return false;
        };
        let Some((millis, tz)) = suffix.split_once('Z') else {
            return false;
        };
        if tz != "" {
            return false;
        }
        if millis.len() != 3 {
            return false;
        }
        if !millis.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        chrono::DateTime::parse_from_rfc3339(value).is_ok()
    }

    #[test]
    fn access_log_schema() {
        let ts = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let trace_id = generate_trace_id();
        let entry = build_access_log_entry("GET", "/v1/sys/health", 200, 5, &trace_id, &ts);

        assert_eq!(
            entry.get("schema_version").and_then(|v| v.as_u64()),
            Some(1)
        );
        assert_eq!(entry.get("method").and_then(|v| v.as_str()), Some("GET"));
        assert_eq!(
            entry.get("path").and_then(|v| v.as_str()),
            Some("/v1/sys/health")
        );
        assert_eq!(entry.get("status").and_then(|v| v.as_u64()), Some(200));
        assert_eq!(entry.get("latency_ms").and_then(|v| v.as_u64()), Some(5));
        assert!(entry.get("request_id").is_none());

        let ts_value = entry.get("ts").and_then(|v| v.as_str()).unwrap_or("");
        assert!(is_rfc3339_millis(ts_value));

        let trace_value = entry.get("trace_id").and_then(|v| v.as_str()).unwrap_or("");
        let parsed = Uuid::parse_str(trace_value).expect("trace_id uuid");
        assert_eq!(parsed.get_version(), Some(uuid::Version::SortRand));
    }
}

pub(crate) async fn require_admin(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let Some(token) = bearer_token(request.headers()) else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    if token != state.admin_key {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}

pub(crate) async fn require_api(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let Some(token) = bearer_token(request.headers()) else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    if token != state.admin_key && token != state.api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let org_id = request
        .headers()
        .get("x-coco-org-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("global");
    let user_id = request
        .headers()
        .get("x-coco-user-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("anonymous");
    let project_id = request
        .headers()
        .get("x-coco-project-id")
        .and_then(|value| value.to_str().ok());
    let key = match project_id {
        Some(project_id) => format!("{org_id}:{user_id}:{project_id}"),
        None => format!("{org_id}:{user_id}"),
    };
    if !state.limiter.allow(&key) {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    Ok(next.run(request).await)
}

fn bearer_token(headers: &HeaderMap) -> Option<String> {
    let value = headers.get("authorization")?;
    let value = value.to_str().ok()?;
    let value = value.strip_prefix("Bearer ")?;
    Some(value.to_string())
}

fn header_value(headers: &HeaderMap, name: &str) -> CocoResult<Option<String>> {
    let Some(value) = headers.get(name) else {
        return Ok(None);
    };
    let value = value
        .to_str()
        .map_err(|_| CocoError::user(format!("{name} header must be valid utf-8")))?;
    if value.trim().is_empty() {
        return Err(CocoError::user(format!("{name} header must not be empty")));
    }
    Ok(Some(value.to_string()))
}

pub(crate) fn header_required(headers: &HeaderMap, name: &str) -> CocoResult<String> {
    header_value(headers, name)?.ok_or_else(|| CocoError::user(format!("{name} header required")))
}

pub(crate) fn header_optional(headers: &HeaderMap, name: &str) -> CocoResult<Option<String>> {
    header_value(headers, name)
}

pub(crate) fn org_user_from_headers(headers: &HeaderMap) -> CocoResult<(String, String)> {
    let org_id = header_required(headers, "x-coco-org-id")?;
    let user_id = header_required(headers, "x-coco-user-id")?;
    Ok((org_id, user_id))
}

pub(crate) fn org_user_project_from_headers(
    headers: &HeaderMap,
) -> CocoResult<(String, String, String)> {
    let org_id = header_required(headers, "x-coco-org-id")?;
    let user_id = header_required(headers, "x-coco-user-id")?;
    let project_id = header_required(headers, "x-coco-project-id")?;
    Ok((org_id, user_id, project_id))
}
