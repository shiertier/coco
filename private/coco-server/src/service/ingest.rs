use std::fs;
use std::io::{self, Write};
use std::path::Path;

use coco_protocol::{CocoError, CocoResult};
use uuid::Uuid;

use super::types::{IngestBatchRequest, IngestJobPayload};
use super::worker_ipc;

pub(crate) fn build_ipc_request(
    job_id: &str,
    org_id: &str,
    user_id: &str,
    project_id: &str,
    payload: &IngestJobPayload,
) -> CocoResult<worker_ipc::SubmitIngestRequest> {
    let (request, blob_ref) = match (&payload.request, &payload.blob_ref) {
        (Some(request), _) => (Some(request.to_ipc()?), None),
        (None, Some(blob_ref)) => (None, Some(blob_ref.clone())),
        (None, None) => {
            return Err(CocoError::user(
                "ingest payload requires request or blob_ref",
            ))
        }
    };
    Ok(worker_ipc::SubmitIngestRequest {
        job_id: job_id.to_string(),
        org_id: org_id.to_string(),
        project_id: project_id.to_string(),
        api_version: env!("CARGO_PKG_VERSION").to_string(),
        request,
        blob_ref,
        user_id: user_id.to_string(),
    })
}

pub(crate) fn write_ingest_blob(dir: &Path, payload: &IngestBatchRequest) -> CocoResult<String> {
    fs::create_dir_all(dir).map_err(CocoError::storage)?;
    let file_name = format!("ingest-{}.json", Uuid::now_v7());
    let path = dir.join(file_name);
    let file = fs::File::create(&path).map_err(CocoError::storage)?;
    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer(&mut writer, payload).map_err(CocoError::storage)?;
    writer.flush().map_err(CocoError::storage)?;
    Ok(path.to_string_lossy().to_string())
}
