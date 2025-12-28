use tonic::transport::Endpoint;

use coco_protocol::{CocoError, CocoResult};

use super::constants::WORKER_IPC_TIMEOUT_SECS;
use super::worker_ipc;

fn worker_endpoint(worker_addr: &str) -> CocoResult<Endpoint> {
    let addr = format!("http://{worker_addr}");
    Endpoint::from_shared(addr)
        .map_err(|_| CocoError::user("COCO_WORKER_ADDR must be host:port"))
        .map(|endpoint| {
            endpoint.connect_timeout(std::time::Duration::from_secs(WORKER_IPC_TIMEOUT_SECS))
        })
}

pub(crate) async fn worker_client(
    worker_addr: &str,
) -> CocoResult<worker_ipc::worker_service_client::WorkerServiceClient<tonic::transport::Channel>> {
    let endpoint = worker_endpoint(worker_addr)?;
    let channel = endpoint
        .connect()
        .await
        .map_err(|err| CocoError::network(format!("worker IPC connect failed: {err}")))?;
    Ok(worker_ipc::worker_service_client::WorkerServiceClient::new(
        channel,
    ))
}

pub(crate) async fn check_worker_connectivity(worker_addr: &str) -> CocoResult<()> {
    let mut client = worker_client(worker_addr).await?;
    let response = client
        .ping(worker_ipc::PingRequest {})
        .await
        .map_err(|err| CocoError::network(format!("worker IPC ping failed: {err}")))?;
    let worker_version = response.into_inner().version;
    ensure_worker_version(env!("CARGO_PKG_VERSION"), &worker_version)?;
    Ok(())
}

pub(crate) async fn submit_ingest_ipc(
    worker_addr: &str,
    request: worker_ipc::SubmitIngestRequest,
) -> CocoResult<()> {
    let mut client = worker_client(worker_addr).await?;
    client
        .submit_ingest(request)
        .await
        .map_err(|err| CocoError::network(format!("worker IPC submit failed: {err}")))?;
    Ok(())
}

fn ensure_worker_version(api_version: &str, worker_version: &str) -> CocoResult<()> {
    let api_major = parse_major_version(api_version)?;
    let worker_major = parse_major_version(worker_version)?;
    if api_major != worker_major {
        return Err(CocoError::user(format!(
            "version mismatch: api={api_version} worker={worker_version}"
        )));
    }
    Ok(())
}

fn parse_major_version(version: &str) -> CocoResult<u64> {
    let major = version
        .split('.')
        .next()
        .ok_or_else(|| CocoError::user("invalid version string"))?;
    major
        .parse::<u64>()
        .map_err(|_| CocoError::user("invalid version string"))
}
