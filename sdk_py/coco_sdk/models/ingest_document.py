from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.ingest_chunk import IngestChunk


T = TypeVar("T", bound="IngestDocument")


@_attrs_define
class IngestDocument:
    """
    Attributes:
        chunks (list['IngestChunk']):
        doc_id (str):
        source_ref (str):
        content_hash (Union[None, Unset, str]):
        quality_score (Union[None, Unset, float]):
        title (Union[None, Unset, str]):
        verified (Union[None, Unset, bool]):
    """

    chunks: list["IngestChunk"]
    doc_id: str
    source_ref: str
    content_hash: Union[None, Unset, str] = UNSET
    quality_score: Union[None, Unset, float] = UNSET
    title: Union[None, Unset, str] = UNSET
    verified: Union[None, Unset, bool] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        chunks = []
        for chunks_item_data in self.chunks:
            chunks_item = chunks_item_data.to_dict()
            chunks.append(chunks_item)

        doc_id = self.doc_id

        source_ref = self.source_ref

        content_hash: Union[None, Unset, str]
        if isinstance(self.content_hash, Unset):
            content_hash = UNSET
        else:
            content_hash = self.content_hash

        quality_score: Union[None, Unset, float]
        if isinstance(self.quality_score, Unset):
            quality_score = UNSET
        else:
            quality_score = self.quality_score

        title: Union[None, Unset, str]
        if isinstance(self.title, Unset):
            title = UNSET
        else:
            title = self.title

        verified: Union[None, Unset, bool]
        if isinstance(self.verified, Unset):
            verified = UNSET
        else:
            verified = self.verified

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "chunks": chunks,
                "doc_id": doc_id,
                "source_ref": source_ref,
            }
        )
        if content_hash is not UNSET:
            field_dict["content_hash"] = content_hash
        if quality_score is not UNSET:
            field_dict["quality_score"] = quality_score
        if title is not UNSET:
            field_dict["title"] = title
        if verified is not UNSET:
            field_dict["verified"] = verified

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.ingest_chunk import IngestChunk

        d = dict(src_dict)
        chunks = []
        _chunks = d.pop("chunks")
        for chunks_item_data in _chunks:
            chunks_item = IngestChunk.from_dict(chunks_item_data)

            chunks.append(chunks_item)

        doc_id = d.pop("doc_id")

        source_ref = d.pop("source_ref")

        def _parse_content_hash(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        content_hash = _parse_content_hash(d.pop("content_hash", UNSET))

        def _parse_quality_score(data: object) -> Union[None, Unset, float]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, float], data)

        quality_score = _parse_quality_score(d.pop("quality_score", UNSET))

        def _parse_title(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        title = _parse_title(d.pop("title", UNSET))

        def _parse_verified(data: object) -> Union[None, Unset, bool]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, bool], data)

        verified = _parse_verified(d.pop("verified", UNSET))

        ingest_document = cls(
            chunks=chunks,
            doc_id=doc_id,
            source_ref=source_ref,
            content_hash=content_hash,
            quality_score=quality_score,
            title=title,
            verified=verified,
        )

        ingest_document.additional_properties = d
        return ingest_document

    @property
    def additional_keys(self) -> list[str]:
        return list(self.additional_properties.keys())

    def __getitem__(self, key: str) -> Any:
        return self.additional_properties[key]

    def __setitem__(self, key: str, value: Any) -> None:
        self.additional_properties[key] = value

    def __delitem__(self, key: str) -> None:
        del self.additional_properties[key]

    def __contains__(self, key: str) -> bool:
        return key in self.additional_properties
