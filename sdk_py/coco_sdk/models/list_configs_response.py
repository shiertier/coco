from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.indexing_config import IndexingConfig


T = TypeVar("T", bound="ListConfigsResponse")


@_attrs_define
class ListConfigsResponse:
    """
    Attributes:
        configs (list['IndexingConfig']):
        active_config_id (Union[None, Unset, str]):
    """

    configs: list["IndexingConfig"]
    active_config_id: Union[None, Unset, str] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        configs = []
        for configs_item_data in self.configs:
            configs_item = configs_item_data.to_dict()
            configs.append(configs_item)

        active_config_id: Union[None, Unset, str]
        if isinstance(self.active_config_id, Unset):
            active_config_id = UNSET
        else:
            active_config_id = self.active_config_id

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "configs": configs,
            }
        )
        if active_config_id is not UNSET:
            field_dict["active_config_id"] = active_config_id

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.indexing_config import IndexingConfig

        d = dict(src_dict)
        configs = []
        _configs = d.pop("configs")
        for configs_item_data in _configs:
            configs_item = IndexingConfig.from_dict(configs_item_data)

            configs.append(configs_item)

        def _parse_active_config_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        active_config_id = _parse_active_config_id(d.pop("active_config_id", UNSET))

        list_configs_response = cls(
            configs=configs,
            active_config_id=active_config_id,
        )

        list_configs_response.additional_properties = d
        return list_configs_response

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
