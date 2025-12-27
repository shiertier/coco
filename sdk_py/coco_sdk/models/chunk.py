from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.text_span import TextSpan


T = TypeVar("T", bound="Chunk")


@_attrs_define
class Chunk:
    """A chunk extracted from a document.

    Attributes:
        content (str): Chunk content.
        doc_id (str): Source document identifier.
        id (str): Chunk identifier.
        span (TextSpan): A span inside a document, expressed as byte offsets.
        embedding (Union[None, Unset, list[float]]): Optional embedding vector.
        quality_score (Union[None, Unset, float]): Optional quality score for the chunk.
        verified (Union[None, Unset, bool]): Whether the chunk has been verified.
    """

    content: str
    doc_id: str
    id: str
    span: "TextSpan"
    embedding: Union[None, Unset, list[float]] = UNSET
    quality_score: Union[None, Unset, float] = UNSET
    verified: Union[None, Unset, bool] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        content = self.content

        doc_id = self.doc_id

        id = self.id

        span = self.span.to_dict()

        embedding: Union[None, Unset, list[float]]
        if isinstance(self.embedding, Unset):
            embedding = UNSET
        elif isinstance(self.embedding, list):
            embedding = self.embedding

        else:
            embedding = self.embedding

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
                "content": content,
                "doc_id": doc_id,
                "id": id,
                "span": span,
            }
        )
        if embedding is not UNSET:
            field_dict["embedding"] = embedding
        if quality_score is not UNSET:
            field_dict["quality_score"] = quality_score
        if verified is not UNSET:
            field_dict["verified"] = verified

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.text_span import TextSpan

        d = dict(src_dict)
        content = d.pop("content")

        doc_id = d.pop("doc_id")

        id = d.pop("id")

        span = TextSpan.from_dict(d.pop("span"))

        def _parse_embedding(data: object) -> Union[None, Unset, list[float]]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, list):
                    raise TypeError()
                embedding_type_0 = cast(list[float], data)

                return embedding_type_0
            except:  # noqa: E722
                pass
            return cast(Union[None, Unset, list[float]], data)

        embedding = _parse_embedding(d.pop("embedding", UNSET))

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

        chunk = cls(
            content=content,
            doc_id=doc_id,
            id=id,
            span=span,
            embedding=embedding,
            quality_score=quality_score,
            verified=verified,
        )

        chunk.additional_properties = d
        return chunk

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
