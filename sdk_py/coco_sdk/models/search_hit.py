from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar

from attrs import define as _attrs_define
from attrs import field as _attrs_field

if TYPE_CHECKING:
    from ..models.chunk import Chunk
    from ..models.search_hit_meta import SearchHitMeta


T = TypeVar("T", bound="SearchHit")


@_attrs_define
class SearchHit:
    """A search hit with metadata and chunk payload.

    Attributes:
        chunk (Chunk): A chunk extracted from a document.
        meta (SearchHitMeta): Metadata associated with a search hit.
    """

    chunk: "Chunk"
    meta: "SearchHitMeta"
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        chunk = self.chunk.to_dict()

        meta = self.meta.to_dict()

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "chunk": chunk,
                "meta": meta,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.chunk import Chunk
        from ..models.search_hit_meta import SearchHitMeta

        d = dict(src_dict)
        chunk = Chunk.from_dict(d.pop("chunk"))

        meta = SearchHitMeta.from_dict(d.pop("meta"))

        search_hit = cls(
            chunk=chunk,
            meta=meta,
        )

        search_hit.additional_properties = d
        return search_hit

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
