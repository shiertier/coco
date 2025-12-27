from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar

from attrs import define as _attrs_define
from attrs import field as _attrs_field

if TYPE_CHECKING:
    from ..models.indexing_config import IndexingConfig


T = TypeVar("T", bound="UpsertConfigRequest")


@_attrs_define
class UpsertConfigRequest:
    """
    Attributes:
        config (IndexingConfig): Indexing-time configuration for a project.
        org_id (str):
    """

    config: "IndexingConfig"
    org_id: str
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        config = self.config.to_dict()

        org_id = self.org_id

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "config": config,
                "org_id": org_id,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.indexing_config import IndexingConfig

        d = dict(src_dict)
        config = IndexingConfig.from_dict(d.pop("config"))

        org_id = d.pop("org_id")

        upsert_config_request = cls(
            config=config,
            org_id=org_id,
        )

        upsert_config_request.additional_properties = d
        return upsert_config_request

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
