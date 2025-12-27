#!/usr/bin/env python3
import os
import shutil
import subprocess
import sys
from pathlib import Path

DEFAULT_VERSION = "0.26.2"


def venv_paths(venv_dir: Path) -> tuple[Path, Path]:
    if os.name == "nt":
        return (
            venv_dir / "Scripts" / "python.exe",
            venv_dir / "Scripts" / "pip.exe",
        )
    return venv_dir / "bin" / "python", venv_dir / "bin" / "pip"


def ensure_venv(venv_dir: Path) -> tuple[Path, Path]:
    python_bin, pip_bin = venv_paths(venv_dir)
    if not python_bin.exists():
        subprocess.run([sys.executable, "-m", "venv", str(venv_dir)], check=True)
    if not pip_bin.exists():
        raise SystemExit("pip not found inside openapi-python-client venv")
    return python_bin, pip_bin


def installed_version(python_bin: Path) -> str:
    try:
        result = subprocess.run(
            [
                str(python_bin),
                "-c",
                (
                    "import importlib.metadata as m; "
                    "print(m.version('openapi-python-client'))"
                ),
            ],
            check=True,
            capture_output=True,
            text=True,
        )
    except subprocess.CalledProcessError:
        return ""
    return result.stdout.strip()


def ensure_codegen(python_bin: Path, pip_bin: Path, version: str) -> None:
    desired = version.strip().lower()
    package = "openapi-python-client"
    current = installed_version(python_bin)
    needs_install = not current
    if desired and desired != "latest" and current != desired:
        needs_install = True
    if desired in ("", "latest"):
        needs_install = True
    if not needs_install:
        return
    if desired and desired != "latest":
        package = f"openapi-python-client=={version}"
    subprocess.run(
        [str(pip_bin), "install", "--upgrade", package],
        check=True,
    )


def read_dependencies(text: str) -> list[str]:
    deps: list[str] = []
    in_project = False
    in_deps = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == "[project]":
            in_project = True
            continue
        if in_project and stripped.startswith("[") and stripped != "[project]":
            break
        if in_project and stripped.startswith("dependencies"):
            parts = stripped.split("=", 1)
            if len(parts) != 2:
                in_deps = True
                continue
            tail = parts[1].strip()
            if tail.startswith("[") and tail.endswith("]"):
                inner = tail[1:-1].strip()
                if inner:
                    for part in inner.split(","):
                        value = part.strip().strip("\"'")
                        if value:
                            deps.append(value)
                break
            if tail.startswith("["):
                in_deps = True
                continue
        if in_deps:
            if stripped.startswith("]"):
                break
            if stripped and not stripped.startswith("#"):
                value = stripped.rstrip(",").strip().strip("\"'")
                if value:
                    deps.append(value)
    return deps


def read_poetry_dependencies(text: str) -> list[str]:
    deps: list[str] = []
    in_deps = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == "[tool.poetry.dependencies]":
            in_deps = True
            continue
        if in_deps and stripped.startswith("[") and stripped != "[tool.poetry.dependencies]":
            break
        if in_deps:
            if "=" not in stripped:
                continue
            name, value = stripped.split("=", 1)
            name = name.strip()
            if name == "python":
                continue
            value = value.strip().strip("\"'")
            if not value or value == "*":
                deps.append(name)
                continue
            if value.startswith("^"):
                dep = pep440_from_caret(name, value[1:])
                deps.append(dep)
                continue
            if value[0] in ("<", ">", "=", "~"):
                deps.append(f"{name}{value}")
                continue
            deps.append(f"{name}=={value}")
    return deps


def pep440_from_caret(name: str, version: str) -> str:
    parts = version.split(".")
    major = int(parts[0]) if parts and parts[0].isdigit() else 0
    if major > 0:
        upper = f"{major + 1}.0.0"
    else:
        upper = "1.0.0"
    return f"{name}>={version},<{upper}"


def render_dependencies(deps: list[str]) -> list[str]:
    lines = ["dependencies = ["]
    for dep in deps:
        lines.append(f"  \"{dep}\",")
    lines.append("]")
    return lines


def update_dependencies(pyproject: Path, deps: list[str]) -> None:
    if not deps:
        return
    lines = pyproject.read_text().splitlines()
    output: list[str] = []
    in_project = False
    in_deps = False
    deps_written = False
    for line in lines:
        stripped = line.strip()
        if stripped == "[project]":
            in_project = True
            output.append(line)
            continue
        if in_project and stripped.startswith("[") and stripped != "[project]":
            if not deps_written:
                output.extend(render_dependencies(deps))
                deps_written = True
            in_project = False
        if in_project and stripped.startswith("dependencies"):
            in_deps = True
            if not deps_written:
                output.extend(render_dependencies(deps))
                deps_written = True
            continue
        if in_deps:
            if stripped.startswith("]"):
                in_deps = False
            continue
        output.append(line)
    if in_project and not deps_written:
        output.extend(render_dependencies(deps))
    pyproject.write_text("\n".join(output) + "\n")


def clean_output(target: Path) -> None:
    if target.exists():
        shutil.rmtree(target)


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    openapi_path = repo_root / "openapi.json"
    output_dir = repo_root / "sdk_py"
    package_dir = output_dir / "coco_sdk"
    config_path = repo_root / "scripts" / "openapi-python-client.yaml"

    cache_dir = repo_root / "scripts" / ".cache" / "openapi-python-client"
    venv_dir = cache_dir / "venv"
    python_bin, pip_bin = ensure_venv(venv_dir)

    version = os.environ.get("OPENAPI_PYTHON_CLIENT_VERSION", DEFAULT_VERSION)
    ensure_codegen(python_bin, pip_bin, version)

    tmp_dir = cache_dir / "output"
    if tmp_dir.exists():
        shutil.rmtree(tmp_dir)

    subprocess.run(
        [
            str(python_bin),
            "-m",
            "openapi_python_client",
            "generate",
            "--path",
            str(openapi_path),
            "--config",
            str(config_path),
            "--output-path",
            str(tmp_dir),
        ],
        check=True,
    )

    generated_package = tmp_dir / "coco_sdk"
    if not generated_package.exists():
        raise SystemExit("openapi-python-client did not generate coco_sdk")

    clean_output(package_dir)
    shutil.copytree(generated_package, package_dir)

    generated_pyproject = tmp_dir / "pyproject.toml"
    target_pyproject = output_dir / "pyproject.toml"
    if generated_pyproject.exists() and target_pyproject.exists():
        text = generated_pyproject.read_text()
        deps = read_dependencies(text)
        if not deps:
            deps = read_poetry_dependencies(text)
        update_dependencies(target_pyproject, deps)

    return 0


if __name__ == "__main__":
    sys.exit(main())
