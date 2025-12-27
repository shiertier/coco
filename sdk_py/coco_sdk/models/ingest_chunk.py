from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="IngestChunk")


@_attrs_define
class IngestChunk:
    """
    Attributes:
        chunk_id (str):
        content (str):
        embedding (list[float]):
        end (int):
        start (int):
        quality_score (Union[None, Unset, float]):
        verified (Union[None, Unset, bool]):
    """

    chunk_id: str
    content: str
    embedding: list[float]
    end: int
    start: int
    quality_score: Union[None, Unset, float] = UNSET
    verified: Union[None, Unset, bool] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        chunk_id = self.chunk_id

        content = self.content

        embedding = self.embedding

        end = self.end

        start = self.start

        quality_score: Union[None, Unset, float]
        if isinstance(self.quality_score, Unset):
            quality_score = UNSET
        else:
            quality_score = self.quality_score

        verified: Union[None, Unset, bool]
        if isinstance(self.verified, Unset):
            verified = UNSET
        else:
            verified = self.verified

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "chunk_id": chunk_id,
                "content": content,
                "embedding": embedding,
                "end": end,
                "start": start,
            }
        )
        if quality_score is not UNSET:
            field_dict["quality_score"] = quality_score
        if verified is not UNSET:
            field_dict["verified"] = verified

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        chunk_id = d.pop("chunk_id")

        content = d.pop("content")

        embedding = cast(list[float], d.pop("embedding"))

        end = d.pop("end")

        start = d.pop("start")

        def _parse_quality_score(data: object) -> Union[None, Unset, float]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, float], data)

        quality_score = _parse_quality_score(d.pop("quality_score", UNSET))

        def _parse_verified(data: object) -> Union[None, Unset, bool]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, bool], data)

        verified = _parse_verified(d.pop("verified", UNSET))

        ingest_chunk = cls(
            chunk_id=chunk_id,
            content=content,
            embedding=embedding,
            end=end,
            start=start,
            quality_score=quality_score,
            verified=verified,
        )

        ingest_chunk.additional_properties = d
        return ingest_chunk

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
