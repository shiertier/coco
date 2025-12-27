from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="SearchHitMeta")


@_attrs_define
class SearchHitMeta:
    """Metadata associated with a search hit.

    Attributes:
        score (float): Similarity or relevance score.
        quality (Union[None, Unset, float]): Optional quality indicator.
        verified (Union[None, Unset, bool]): Whether the result has been verified.
    """

    score: float
    quality: Union[None, Unset, float] = UNSET
    verified: Union[None, Unset, bool] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        score = self.score

        quality: Union[None, Unset, float]
        if isinstance(self.quality, Unset):
            quality = UNSET
        else:
            quality = self.quality

        verified: Union[None, Unset, bool]
        if isinstance(self.verified, Unset):
            verified = UNSET
        else:
            verified = self.verified

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "score": score,
            }
        )
        if quality is not UNSET:
            field_dict["quality"] = quality
        if verified is not UNSET:
            field_dict["verified"] = verified

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        score = d.pop("score")

        def _parse_quality(data: object) -> Union[None, Unset, float]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, float], data)

        quality = _parse_quality(d.pop("quality", UNSET))

        def _parse_verified(data: object) -> Union[None, Unset, bool]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, bool], data)

        verified = _parse_verified(d.pop("verified", UNSET))

        search_hit_meta = cls(
            score=score,
            quality=quality,
            verified=verified,
        )

        search_hit_meta.additional_properties = d
        return search_hit_meta

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
