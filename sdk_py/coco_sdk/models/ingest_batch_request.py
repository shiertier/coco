from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.ingest_document import IngestDocument


T = TypeVar("T", bound="IngestBatchRequest")


@_attrs_define
class IngestBatchRequest:
    """
    Attributes:
        documents (list['IngestDocument']):
        activate (Union[Unset, bool]):
        indexing_config_id (Union[None, Unset, str]):
    """

    documents: list["IngestDocument"]
    activate: Union[Unset, bool] = UNSET
    indexing_config_id: Union[None, Unset, str] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        documents = []
        for documents_item_data in self.documents:
            documents_item = documents_item_data.to_dict()
            documents.append(documents_item)

        activate = self.activate

        indexing_config_id: Union[None, Unset, str]
        if isinstance(self.indexing_config_id, Unset):
            indexing_config_id = UNSET
        else:
            indexing_config_id = self.indexing_config_id

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "documents": documents,
            }
        )
        if activate is not UNSET:
            field_dict["activate"] = activate
        if indexing_config_id is not UNSET:
            field_dict["indexing_config_id"] = indexing_config_id

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.ingest_document import IngestDocument

        d = dict(src_dict)
        documents = []
        _documents = d.pop("documents")
        for documents_item_data in _documents:
            documents_item = IngestDocument.from_dict(documents_item_data)

            documents.append(documents_item)

        activate = d.pop("activate", UNSET)

        def _parse_indexing_config_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        indexing_config_id = _parse_indexing_config_id(d.pop("indexing_config_id", UNSET))

        ingest_batch_request = cls(
            documents=documents,
            activate=activate,
            indexing_config_id=indexing_config_id,
        )

        ingest_batch_request.additional_properties = d
        return ingest_batch_request

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
