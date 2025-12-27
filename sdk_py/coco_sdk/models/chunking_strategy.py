from collections.abc import Mapping
from typing import Any, TypeVar

from attrs import define as _attrs_define
from attrs import field as _attrs_field

T = TypeVar("T", bound="ChunkingStrategy")


@_attrs_define
class ChunkingStrategy:
    """Chunking strategy parameters.

    Attributes:
        chunk_overlap (int): Overlap size between adjacent chunks.
        chunk_size (int): Target chunk size in tokens or characters.
        strategy_name (str): Strategy name (e.g., markdown_header, fixed_token).
    """

    chunk_overlap: int
    chunk_size: int
    strategy_name: str
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        chunk_overlap = self.chunk_overlap

        chunk_size = self.chunk_size

        strategy_name = self.strategy_name

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "chunk_overlap": chunk_overlap,
                "chunk_size": chunk_size,
                "strategy_name": strategy_name,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        chunk_overlap = d.pop("chunk_overlap")

        chunk_size = d.pop("chunk_size")

        strategy_name = d.pop("strategy_name")

        chunking_strategy = cls(
            chunk_overlap=chunk_overlap,
            chunk_size=chunk_size,
            strategy_name=strategy_name,
        )

        chunking_strategy.additional_properties = d
        return chunking_strategy

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
