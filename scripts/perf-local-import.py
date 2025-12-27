#!/usr/bin/env python3
import os
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

    lines = int(os.environ.get("COCO_PERF_LINES", "100000"))
    embedder_mode = os.environ.get("COCO_PERF_EMBEDDER_MODE", "stub")

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

        cmd = [
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
        ]
        start = time.perf_counter()
        subprocess.run(cmd, check=True, cwd=repo_root, env=env)
        elapsed = time.perf_counter() - start

        rate = lines / elapsed if elapsed > 0 else 0.0
        print(f"lines={lines}")
        print(f"elapsed_sec={elapsed:.3f}")
        print(f"lines_per_sec={rate:.2f}")
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
