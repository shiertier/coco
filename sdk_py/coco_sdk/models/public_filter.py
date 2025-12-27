from collections.abc import Mapping
from typing import Any, TypeVar

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..models.public_filter_op import PublicFilterOp

T = TypeVar("T", bound="PublicFilter")


@_attrs_define
class PublicFilter:
    """Filter constraint accepted by the public server API.

    Attributes:
        field (str): Field name to filter on (server allows `doc_id` and `chunk_id`).
        op (PublicFilterOp): Filter operator documented for the public server API.
        value (str): String-encoded filter value.
    """

    field: str
    op: PublicFilterOp
    value: str
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        field = self.field

        op = self.op.value

        value = self.value

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "field": field,
                "op": op,
                "value": value,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        field = d.pop("field")

        op = PublicFilterOp(d.pop("op"))

        value = d.pop("value")

        public_filter = cls(
            field=field,
            op=op,
            value=value,
        )

        public_filter.additional_properties = d
        return public_filter

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
