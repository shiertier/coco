from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..models.vector_backend_kind import VectorBackendKind
from ..types import UNSET, Unset

T = TypeVar("T", bound="VectorBackendConfig")


@_attrs_define
class VectorBackendConfig:
    """Configuration for selecting a vector backend.

    Attributes:
        kind (VectorBackendKind): Supported vector backend types.
        api_key (Union[None, Unset, str]): Optional API key or token.
        collection_prefix (Union[None, Unset, str]): Optional collection name prefix.
        url (Union[None, Unset, str]): Optional endpoint or connection URL.
    """

    kind: VectorBackendKind
    api_key: Union[None, Unset, str] = UNSET
    collection_prefix: Union[None, Unset, str] = UNSET
    url: Union[None, Unset, str] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        kind = self.kind.value

        api_key: Union[None, Unset, str]
        if isinstance(self.api_key, Unset):
            api_key = UNSET
        else:
            api_key = self.api_key

        collection_prefix: Union[None, Unset, str]
        if isinstance(self.collection_prefix, Unset):
            collection_prefix = UNSET
        else:
            collection_prefix = self.collection_prefix

        url: Union[None, Unset, str]
        if isinstance(self.url, Unset):
            url = UNSET
        else:
            url = self.url

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "kind": kind,
            }
        )
        if api_key is not UNSET:
            field_dict["api_key"] = api_key
        if collection_prefix is not UNSET:
            field_dict["collection_prefix"] = collection_prefix
        if url is not UNSET:
            field_dict["url"] = url

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        kind = VectorBackendKind(d.pop("kind"))

        def _parse_api_key(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        api_key = _parse_api_key(d.pop("api_key", UNSET))

        def _parse_collection_prefix(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        collection_prefix = _parse_collection_prefix(d.pop("collection_prefix", UNSET))

        def _parse_url(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        url = _parse_url(d.pop("url", UNSET))

        vector_backend_config = cls(
            kind=kind,
            api_key=api_key,
            collection_prefix=collection_prefix,
            url=url,
        )

        vector_backend_config.additional_properties = d
        return vector_backend_config

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
