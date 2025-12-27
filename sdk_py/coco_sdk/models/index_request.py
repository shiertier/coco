from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="IndexRequest")


@_attrs_define
class IndexRequest:
    """
    Attributes:
        indexing_config_id (Union[None, Unset, str]):
    """

    indexing_config_id: Union[None, Unset, str] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        indexing_config_id: Union[None, Unset, str]
        if isinstance(self.indexing_config_id, Unset):
            indexing_config_id = UNSET
        else:
            indexing_config_id = self.indexing_config_id

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if indexing_config_id is not UNSET:
            field_dict["indexing_config_id"] = indexing_config_id

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)

        def _parse_indexing_config_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        indexing_config_id = _parse_indexing_config_id(d.pop("indexing_config_id", UNSET))

        index_request = cls(
            indexing_config_id=indexing_config_id,
        )

        index_request.additional_properties = d
        return index_request

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
