//! Service lifecycle helpers for local mode.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use chrono::Utc;
use coco_protocol::{CocoError, CocoResult};
use serde::{Deserialize, Serialize};

use crate::paths;

const CONNECT_TIMEOUT_MS: u64 = 300;
const IO_TIMEOUT_MS: u64 = 500;

/// Result of checking whether another service is running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceStatus {
    /// Another CoCo service is already running.
    Running,
    /// No service detected.
    NotRunning,
    /// The port is occupied by another process.
    PortInUse,
}

/// Lock record stored on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LockRecord {
    pid: u32,
    started_at: String,
}

/// Guard that removes the lock file on drop.
#[derive(Debug)]
pub struct ServiceLock {
    path: PathBuf,
}

impl ServiceLock {
    /// Creates the lock file and returns a guard.
    pub fn acquire() -> CocoResult<Self> {
        let path = paths::lock_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(CocoError::system)?;
        }
        let record = LockRecord {
            pid: std::process::id(),
            started_at: Utc::now().to_rfc3339(),
        };
        let payload =
            serde_json::to_vec(&record).map_err(|err| CocoError::system(format!("{err}")))?;
        std::fs::write(&path, payload).map_err(CocoError::system)?;
        Ok(Self { path })
    }
}

impl Drop for ServiceLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Ensures only a single local instance is running.
pub fn ensure_single_instance(host: &str, port: u16) -> CocoResult<(InstanceStatus, Option<ServiceLock>)> {
    let lock_path = paths::lock_path()?;

    if let Some(record) = read_lock(&lock_path)? {
        if is_pid_running(record.pid)? {
            match check_health(host, port)? {
                InstanceStatus::Running => return Ok((InstanceStatus::Running, None)),
                InstanceStatus::PortInUse => {
                    return Err(CocoError::system(
                        "port already in use by another process",
                    ))
                }
                InstanceStatus::NotRunning => {
                    let _ = std::fs::remove_file(&lock_path);
                }
            }
        } else {
            let _ = std::fs::remove_file(&lock_path);
        }
    }

    match check_health(host, port)? {
        InstanceStatus::Running => return Ok((InstanceStatus::Running, None)),
        InstanceStatus::PortInUse => {
            return Err(CocoError::system(
                "port already in use by another process",
            ))
        }
        InstanceStatus::NotRunning => {}
    }

    let lock = ServiceLock::acquire()?;
    Ok((InstanceStatus::NotRunning, Some(lock)))
}

fn read_lock(path: &Path) -> CocoResult<Option<LockRecord>> {
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read(path).map_err(CocoError::system)?;
    let record = serde_json::from_slice(&data).map_err(|_| {
        let _ = std::fs::remove_file(path);
        CocoError::system("invalid lock file")
    })?;
    Ok(Some(record))
}

fn check_health(host: &str, port: u16) -> CocoResult<InstanceStatus> {
    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .map_err(|_| CocoError::user("invalid host or port"))?;
    let timeout = Duration::from_millis(CONNECT_TIMEOUT_MS);
    let mut stream = match TcpStream::connect_timeout(&addr, timeout) {
        Ok(stream) => stream,
        Err(_) => return Ok(InstanceStatus::NotRunning),
    };
    stream
        .set_read_timeout(Some(Duration::from_millis(IO_TIMEOUT_MS)))
        .map_err(CocoError::system)?;
    stream
        .set_write_timeout(Some(Duration::from_millis(IO_TIMEOUT_MS)))
        .map_err(CocoError::system)?;

    let request = format!(
        "GET /v1/sys/health HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).map_err(CocoError::system)?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).map_err(CocoError::system)?;
    let response_text = String::from_utf8_lossy(&response);

    if !response_text.contains("HTTP/1.1 200") && !response_text.contains("HTTP/1.0 200") {
        return Ok(InstanceStatus::PortInUse);
    }

    let body = response_text
        .split("\r\n\r\n")
        .nth(1)
        .unwrap_or_default();
    if is_coco_health(body) {
        Ok(InstanceStatus::Running)
    } else {
        Ok(InstanceStatus::PortInUse)
    }
}

fn is_coco_health(body: &str) -> bool {
    #[derive(Deserialize)]
    struct Health {
        status: String,
        service: Option<String>,
    }

    let parsed: Health = match serde_json::from_str(body) {
        Ok(parsed) => parsed,
        Err(_) => return false,
    };
    parsed.status == "ok" && parsed.service.as_deref() == Some("coco")
}

fn is_pid_running(pid: u32) -> CocoResult<bool> {
    let pid_str = pid.to_string();
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid_str}")])
            .output()
            .map_err(|err| CocoError::system(format!("failed to execute tasklist: {err}")))?;
        if !output.status.success() {
            return Ok(false);
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Ok(stdout.contains(&pid_str));
    }
    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("ps")
            .args(["-p", &pid_str])
            .output()
            .map_err(|err| CocoError::system(format!("failed to execute ps: {err}")))?;
        if !output.status.success() {
            return Ok(false);
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            if line.split_whitespace().next() == Some(pid_str.as_str()) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
