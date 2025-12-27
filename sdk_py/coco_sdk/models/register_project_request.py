from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="RegisterProjectRequest")


@_attrs_define
class RegisterProjectRequest:
    """
    Attributes:
        name (str):
        org_id (str):
        source_ref (str):
        user_id (str):
        org_name (Union[None, Unset, str]):
        platform (Union[None, Unset, str]):
        project_id (Union[None, Unset, str]):
    """

    name: str
    org_id: str
    source_ref: str
    user_id: str
    org_name: Union[None, Unset, str] = UNSET
    platform: Union[None, Unset, str] = UNSET
    project_id: Union[None, Unset, str] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        name = self.name

        org_id = self.org_id

        source_ref = self.source_ref

        user_id = self.user_id

        org_name: Union[None, Unset, str]
        if isinstance(self.org_name, Unset):
            org_name = UNSET
        else:
            org_name = self.org_name

        platform: Union[None, Unset, str]
        if isinstance(self.platform, Unset):
            platform = UNSET
        else:
            platform = self.platform

        project_id: Union[None, Unset, str]
        if isinstance(self.project_id, Unset):
            project_id = UNSET
        else:
            project_id = self.project_id

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "name": name,
                "org_id": org_id,
                "source_ref": source_ref,
                "user_id": user_id,
            }
        )
        if org_name is not UNSET:
            field_dict["org_name"] = org_name
        if platform is not UNSET:
            field_dict["platform"] = platform
        if project_id is not UNSET:
            field_dict["project_id"] = project_id

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        name = d.pop("name")

        org_id = d.pop("org_id")

        source_ref = d.pop("source_ref")

        user_id = d.pop("user_id")

        def _parse_org_name(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        org_name = _parse_org_name(d.pop("org_name", UNSET))

        def _parse_platform(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        platform = _parse_platform(d.pop("platform", UNSET))

        def _parse_project_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        project_id = _parse_project_id(d.pop("project_id", UNSET))

        register_project_request = cls(
            name=name,
            org_id=org_id,
            source_ref=source_ref,
            user_id=user_id,
            org_name=org_name,
            platform=platform,
            project_id=project_id,
        )

        register_project_request.additional_properties = d
        return register_project_request

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
