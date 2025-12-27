#!/usr/bin/env python3
import json
import re
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: set-sdk-version.py <version>")
        return 1
    version = sys.argv[1].lstrip("v")
    repo_root = Path(__file__).resolve().parents[1]

    package_json = repo_root / "sdk" / "package.json"
    package_data = json.loads(package_json.read_text())
    package_data["version"] = version
    package_json.write_text(json.dumps(package_data, indent=2) + "\n")

    pyproject = repo_root / "sdk_py" / "pyproject.toml"
    text = pyproject.read_text()
    text = re.sub(r'(?m)^version = ".*"$', f'version = "{version}"', text)
    pyproject.write_text(text)

    python_config = repo_root / "scripts" / "openapi-python-client.yaml"
    if python_config.exists():
        text = python_config.read_text()
        text = re.sub(
            r'(?m)^package_version_override: ".*"$',
            f'package_version_override: "{version}"',
            text,
        )
        python_config.write_text(text)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
