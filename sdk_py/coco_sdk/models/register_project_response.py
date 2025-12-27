from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="RegisterProjectResponse")


@_attrs_define
class RegisterProjectResponse:
    """
    Attributes:
        active_config_id (str):
        name (str):
        org_id (str):
        project_id (str):
        active_version_id (Union[None, Unset, str]):
    """

    active_config_id: str
    name: str
    org_id: str
    project_id: str
    active_version_id: Union[None, Unset, str] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        active_config_id = self.active_config_id

        name = self.name

        org_id = self.org_id

        project_id = self.project_id

        active_version_id: Union[None, Unset, str]
        if isinstance(self.active_version_id, Unset):
            active_version_id = UNSET
        else:
            active_version_id = self.active_version_id

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "active_config_id": active_config_id,
                "name": name,
                "org_id": org_id,
                "project_id": project_id,
            }
        )
        if active_version_id is not UNSET:
            field_dict["active_version_id"] = active_version_id

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        active_config_id = d.pop("active_config_id")

        name = d.pop("name")

        org_id = d.pop("org_id")

        project_id = d.pop("project_id")

        def _parse_active_version_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        active_version_id = _parse_active_version_id(d.pop("active_version_id", UNSET))

        register_project_response = cls(
            active_config_id=active_config_id,
            name=name,
            org_id=org_id,
            project_id=project_id,
            active_version_id=active_version_id,
        )

        register_project_response.additional_properties = d
        return register_project_response

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
