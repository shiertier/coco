from __future__ import annotations

import json
from dataclasses import asdict, fields
from pathlib import Path
from typing import Dict, Optional

from .models import Manifest, ManifestDocument


def load_manifest(path: Path) -> Optional[Manifest]:
    if not path.exists():
        return None
    raw = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(raw, dict):
        raise ValueError(f"Invalid manifest format: {path}")

    documents_raw = raw.get("documents", {})
    if not isinstance(documents_raw, dict):
        raise ValueError(f"Invalid manifest documents format: {path}")

    documents: Dict[str, ManifestDocument] = {}
    allowed_doc_fields = {f.name for f in fields(ManifestDocument)}
    for url, doc in documents_raw.items():
        if not isinstance(doc, dict):
            continue
        filtered = {k: v for k, v in doc.items() if k in allowed_doc_fields}
        documents[url] = ManifestDocument(**filtered)

    return Manifest(
        version=int(raw.get("version", 1)),
        source_id=str(raw.get("source_id", "")),
        generated_at=str(raw.get("generated_at", "")),
        entrypoints=list(raw.get("entrypoints", [])),
        documents=documents,
        documents_order=list(raw.get("documents_order", [])),
    )


def save_manifest(path: Path, manifest: Manifest) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = asdict(manifest)
    payload["documents"] = {url: asdict(doc) for url, doc in manifest.documents.items()}
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def write_document(path: Path, content: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(content)


def delete_document(path: Path) -> None:
    try:
        path.unlink()
    except FileNotFoundError:
        return
