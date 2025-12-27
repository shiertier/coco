#!/usr/bin/env python3
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

DEFAULT_VERSION = "0.27.0"


def ensure_node_tools() -> tuple[str, str]:
    node = shutil.which("node")
    npm = shutil.which("npm")
    if not node or not npm:
        raise SystemExit("node and npm are required to generate the TypeScript SDK")
    return node, npm


def installed_version(package_json: Path) -> str:
    if not package_json.exists():
        return ""
    try:
        data = json.loads(package_json.read_text())
    except json.JSONDecodeError:
        return ""
    return str(data.get("version", ""))


def ensure_codegen(cache_dir: Path, npm: str, version: str) -> Path:
    package_json = (
        cache_dir
        / "node_modules"
        / "openapi-typescript-codegen"
        / "package.json"
    )
    desired = version.strip().lower()
    current = installed_version(package_json)
    needs_install = not package_json.exists()
    if desired and desired != "latest" and current != desired:
        needs_install = True
    if needs_install:
        cache_dir.mkdir(parents=True, exist_ok=True)
        package = "openapi-typescript-codegen"
        if desired and desired != "latest":
            package = f"openapi-typescript-codegen@{version}"
        subprocess.run(
            [npm, "install", "--prefix", str(cache_dir), package],
            check=True,
        )
    bin_dir = cache_dir / "node_modules" / ".bin"
    exe_name = "openapi.cmd" if os.name == "nt" else "openapi"
    exe_path = bin_dir / exe_name
    if not exe_path.exists():
        raise SystemExit("openapi-typescript-codegen binary not found after install")
    return exe_path


def clean_output(output_dir: Path) -> None:
    generated = [
        "core",
        "models",
        "services",
        "client.ts",
        "types.ts",
        "index.ts",
        "OpenAPI.ts",
        "request.ts",
    ]
    for name in generated:
        path = output_dir / name
        if not path.exists():
            continue
        if path.is_dir():
            shutil.rmtree(path)
        else:
            path.unlink()


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    openapi_path = repo_root / "openapi.json"
    output_dir = repo_root / "sdk"

    _node, npm = ensure_node_tools()
    version = os.environ.get("OPENAPI_TS_CODEGEN_VERSION", DEFAULT_VERSION)
    cache_dir = repo_root / "scripts" / ".cache" / "openapi-typescript-codegen"
    exe_path = ensure_codegen(cache_dir, npm, version)

    clean_output(output_dir)
    subprocess.run(
        [
            str(exe_path),
            "--input",
            str(openapi_path),
            "--output",
            str(output_dir),
            "--client",
            "fetch",
            "--useUnionTypes",
        ],
        check=True,
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
