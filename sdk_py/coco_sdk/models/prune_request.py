from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="PruneRequest")


@_attrs_define
class PruneRequest:
    """
    Attributes:
        org_id (str):
        project_id (str):
        user_id (str):
        keep (Union[None, Unset, int]):
    """

    org_id: str
    project_id: str
    user_id: str
    keep: Union[None, Unset, int] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        org_id = self.org_id

        project_id = self.project_id

        user_id = self.user_id

        keep: Union[None, Unset, int]
        if isinstance(self.keep, Unset):
            keep = UNSET
        else:
            keep = self.keep

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "org_id": org_id,
                "project_id": project_id,
                "user_id": user_id,
            }
        )
        if keep is not UNSET:
            field_dict["keep"] = keep

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        org_id = d.pop("org_id")

        project_id = d.pop("project_id")

        user_id = d.pop("user_id")

        def _parse_keep(data: object) -> Union[None, Unset, int]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, int], data)

        keep = _parse_keep(d.pop("keep", UNSET))

        prune_request = cls(
            org_id=org_id,
            project_id=project_id,
            user_id=user_id,
            keep=keep,
        )

        prune_request.additional_properties = d
        return prune_request

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
