from collections.abc import Mapping
from typing import Any, TypeVar

from attrs import define as _attrs_define
from attrs import field as _attrs_field

T = TypeVar("T", bound="RerankerConfig")


@_attrs_define
class RerankerConfig:
    """Reranker configuration for post-retrieval scoring.

    Attributes:
        model_name (str): Model name for reranking.
        rerank_top_n (int): Number of top candidates to rerank.
    """

    model_name: str
    rerank_top_n: int
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        model_name = self.model_name

        rerank_top_n = self.rerank_top_n

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "model_name": model_name,
                "rerank_top_n": rerank_top_n,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        model_name = d.pop("model_name")

        rerank_top_n = d.pop("rerank_top_n")

        reranker_config = cls(
            model_name=model_name,
            rerank_top_n=rerank_top_n,
        )

        reranker_config.additional_properties = d
        return reranker_config

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
