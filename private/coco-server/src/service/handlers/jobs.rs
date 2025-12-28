use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State};
use futures::stream;

use coco_protocol::CocoError;

use super::super::middleware::{header_optional, org_user_from_headers};
use super::super::state::AppState;
use super::super::types::{ApiResult, JobStatusResponse};

#[utoipa::path(
    get,
    path = "/v1/jobs/{id}",
    tag = "Jobs",
    params(
        ("id" = String, Path, description = "Job identifier"),
        ("x-coco-org-id" = String, Header, description = "Organization identifier"),
        ("x-coco-user-id" = String, Header, description = "User identifier"),
        ("x-coco-project-id" = Option<String>, Header, description = "Project identifier")
    ),
    responses(
        (status = 200, description = "Job status", body = JobStatusResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse),
        (status = 404, description = "Job not found", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn get_job(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Path(job_id): axum::extract::Path<String>,
) -> ApiResult<Json<JobStatusResponse>> {
    let (org_id, user_id) = org_user_from_headers(&headers)?;
    let project_id = header_optional(&headers, "x-coco-project-id")?;
    let Some(job) = state.meta.get_ingest_job(&job_id).await? else {
        return Err(CocoError::user("job not found").into());
    };
    if job.org_id != org_id || job.user_id != user_id {
        return Err(CocoError::user("job not found").into());
    }
    if let Some(project_id) = project_id.as_deref() {
        if job.project_id != project_id {
            return Err(CocoError::user("job not found").into());
        }
    }
    Ok(Json(JobStatusResponse::from(job)))
}

#[utoipa::path(
    get,
    path = "/v1/jobs/{id}/events",
    tag = "Jobs",
    params(
        ("id" = String, Path, description = "Job identifier"),
        ("x-coco-org-id" = String, Header, description = "Organization identifier"),
        ("x-coco-user-id" = String, Header, description = "User identifier"),
        ("x-coco-project-id" = Option<String>, Header, description = "Project identifier")
    ),
    responses((
        status = 200,
        description = "Job event stream",
        body = JobStatusResponse,
        content_type = "text/event-stream"
    ))
)]
pub(crate) async fn job_events(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Path(job_id): axum::extract::Path<String>,
) -> ApiResult<Response> {
    let (org_id, user_id) = org_user_from_headers(&headers)?;
    let project_id = header_optional(&headers, "x-coco-project-id")?;
    let Some(job) = state.meta.get_ingest_job(&job_id).await? else {
        return Err(CocoError::user("job not found").into());
    };
    if job.org_id != org_id || job.user_id != user_id {
        return Err(CocoError::user("job not found").into());
    }
    if let Some(project_id) = project_id.as_deref() {
        if job.project_id != project_id {
            return Err(CocoError::user("job not found").into());
        }
    }
    let payload =
        serde_json::to_string(&JobStatusResponse::from(job)).map_err(CocoError::system)?;
    let stream = stream::once(async move {
        Ok::<_, std::convert::Infallible>(axum::response::sse::Event::default().data(payload))
    });
    let response =
        axum::response::sse::Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default());
    Ok(response.into_response())
}
