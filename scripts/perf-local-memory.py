#!/usr/bin/env python3
import os
import platform
import shutil
import socket
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Optional
from urllib import request


def ensure_command(cmd: str) -> None:
    if shutil.which(cmd) is None:
        raise SystemExit(f"{cmd} is required for perf tests")


def write_project(root: Path, lines: int) -> None:
    root.mkdir(parents=True, exist_ok=True)
    target = root / "main.rs"
    with target.open("w", encoding="utf-8") as handle:
        for i in range(lines):
            handle.write(f"// line {i}\n")


def pick_port() -> int:
    sock = socket.socket()
    sock.bind(("127.0.0.1", 0))
    port = sock.getsockname()[1]
    sock.close()
    return port


def wait_ready(port: int) -> bool:
    url = f"http://127.0.0.1:{port}/v1/sys/health"
    for _ in range(40):
        try:
            with request.urlopen(url, timeout=1) as resp:
                if resp.status == 200:
                    return True
        except Exception:
            pass
        time.sleep(0.25)
    return False


def build_env(tmp_root: Path, port: int, embedder_mode: str) -> dict[str, str]:
    env = os.environ.copy()
    env["HOME"] = str(tmp_root / "home")
    env["COCO_HOST"] = "127.0.0.1"
    env["COCO_PORT"] = str(port)
    env["COCO_META_DB"] = str(tmp_root / "meta.db")
    env["COCO_VECTOR_DIR"] = str(tmp_root / "vectors")
    env["COCO_WATCH_ENABLED"] = "0"
    if embedder_mode == "stub":
        env["COCO_EMBEDDER_MODE"] = "stub"
    else:
        env.pop("COCO_EMBEDDER_MODE", None)
    return env


def read_rss_kb(pid: int) -> Optional[int]:
    system = platform.system().lower()
    if system == "windows":
        return read_rss_kb_windows(pid)
    return read_rss_kb_unix(pid)


def read_rss_kb_unix(pid: int) -> Optional[int]:
    result = subprocess.run(
        ["ps", "-o", "rss=", "-p", str(pid)],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        return None
    raw = result.stdout.strip()
    if not raw:
        return None
    try:
        return int(raw)
    except ValueError:
        return None


def read_rss_kb_windows(pid: int) -> Optional[int]:
    shell = shutil.which("pwsh") or shutil.which("powershell")
    if shell:
        result = subprocess.run(
            [
                shell,
                "-NoProfile",
                "-Command",
                f"(Get-Process -Id {pid}).WorkingSet64",
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            raw = result.stdout.strip()
            if raw:
                try:
                    return int(raw) // 1024
                except ValueError:
                    return None
    if shutil.which("wmic"):
        result = subprocess.run(
            [
                "wmic",
                "process",
                "where",
                f"ProcessId={pid}",
                "get",
                "WorkingSetSize",
                "/value",
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            for line in result.stdout.splitlines():
                if line.strip().startswith("WorkingSetSize="):
                    value = line.split("=", 1)[1].strip()
                    if value:
                        try:
                            return int(value) // 1024
                        except ValueError:
                            return None
    return None


def stop_process(proc: subprocess.Popen) -> None:
    if proc.poll() is not None:
        return
    proc.terminate()
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait(timeout=5)


def main() -> int:
    ensure_command("cargo")
    if platform.system().lower() != "windows":
        ensure_command("ps")

    lines = int(os.environ.get("COCO_PERF_LINES", "100000"))
    embedder_mode = os.environ.get("COCO_PERF_EMBEDDER_MODE", "stub")
    sample_interval = float(os.environ.get("COCO_PERF_SAMPLE_INTERVAL", "0.1"))
    settle_seconds = float(os.environ.get("COCO_PERF_IDLE_SECONDS", "0.5"))

    repo_root = Path(__file__).resolve().parents[1]
    tmp_root = Path(tempfile.mkdtemp(prefix="coco-local-perf-"))
    service_proc: Optional[subprocess.Popen] = None
    service_log = None
    try:
        project_root = tmp_root / "project"
        write_project(project_root, lines)

        port = pick_port()
        env = build_env(tmp_root, port, embedder_mode)
        (tmp_root / "home").mkdir(parents=True, exist_ok=True)

        service_log = (tmp_root / "service.log").open("wb")
        service_proc = subprocess.Popen(
            [
                "cargo",
                "run",
                "-p",
                "coco-local",
                "--features",
                "local-storage",
                "--",
                "start",
                "--headless",
            ],
            cwd=repo_root,
            env=env,
            stdout=service_log,
            stderr=subprocess.STDOUT,
        )

        if not wait_ready(port):
            raise SystemExit("local service did not become ready")

        time.sleep(settle_seconds)

        idle_kb = read_rss_kb(service_proc.pid)
        if idle_kb is None:
            raise SystemExit("failed to read service memory")
        max_kb = idle_kb

        import_log = (tmp_root / "import.log").open("wb")
        import_proc = subprocess.Popen(
            [
                "cargo",
                "run",
                "-p",
                "coco-local",
                "--features",
                "local-storage",
                "--",
                "import",
                str(project_root),
                "--recursive",
            ],
            cwd=repo_root,
            env=env,
            stdout=import_log,
            stderr=subprocess.STDOUT,
        )

        while import_proc.poll() is None:
            current_kb = read_rss_kb(service_proc.pid)
            if current_kb is not None and current_kb > max_kb:
                max_kb = current_kb
            time.sleep(sample_interval)

        import_proc.wait()
        import_log.close()

        current_kb = read_rss_kb(service_proc.pid)
        if current_kb is not None and current_kb > max_kb:
            max_kb = current_kb

        print(f"lines={lines}")
        print(f"idle_mb={idle_kb / 1024:.2f}")
        print(f"peak_mb={max_kb / 1024:.2f}")
    finally:
        if service_proc is not None:
            stop_process(service_proc)
        if service_log is not None:
            service_log.close()
        if tmp_root.exists():
            shutil.rmtree(tmp_root)
    return 0


if __name__ == "__main__":
    sys.exit(main())
