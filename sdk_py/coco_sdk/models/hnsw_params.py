from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="HnswParams")


@_attrs_define
class HnswParams:
    """HNSW index parameters.

    Attributes:
        ef_construction (Union[None, Unset, int]): Construction ef parameter.
        m (Union[None, Unset, int]): Number of connections per node.
    """

    ef_construction: Union[None, Unset, int] = UNSET
    m: Union[None, Unset, int] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        ef_construction: Union[None, Unset, int]
        if isinstance(self.ef_construction, Unset):
            ef_construction = UNSET
        else:
            ef_construction = self.ef_construction

        m: Union[None, Unset, int]
        if isinstance(self.m, Unset):
            m = UNSET
        else:
            m = self.m

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if ef_construction is not UNSET:
            field_dict["ef_construction"] = ef_construction
        if m is not UNSET:
            field_dict["m"] = m

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)

        def _parse_ef_construction(data: object) -> Union[None, Unset, int]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, int], data)

        ef_construction = _parse_ef_construction(d.pop("ef_construction", UNSET))

        def _parse_m(data: object) -> Union[None, Unset, int]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, int], data)

        m = _parse_m(d.pop("m", UNSET))

        hnsw_params = cls(
            ef_construction=ef_construction,
            m=m,
        )

        hnsw_params.additional_properties = d
        return hnsw_params

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
