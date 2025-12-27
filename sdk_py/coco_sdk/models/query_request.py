from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.indexing_config import IndexingConfig
    from ..models.public_search_intent import PublicSearchIntent
    from ..models.retrieval_config import RetrievalConfig


T = TypeVar("T", bound="QueryRequest")


@_attrs_define
class QueryRequest:
    """
    Attributes:
        intent (PublicSearchIntent): Search intent describing how retrieval should run.
        indexing_config (Union['IndexingConfig', None, Unset]):
        indexing_config_id (Union[None, Unset, str]):
        retrieval_config (Union['RetrievalConfig', None, Unset]):
    """

    intent: "PublicSearchIntent"
    indexing_config: Union["IndexingConfig", None, Unset] = UNSET
    indexing_config_id: Union[None, Unset, str] = UNSET
    retrieval_config: Union["RetrievalConfig", None, Unset] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        from ..models.indexing_config import IndexingConfig
        from ..models.retrieval_config import RetrievalConfig

        intent = self.intent.to_dict()

        indexing_config: Union[None, Unset, dict[str, Any]]
        if isinstance(self.indexing_config, Unset):
            indexing_config = UNSET
        elif isinstance(self.indexing_config, IndexingConfig):
            indexing_config = self.indexing_config.to_dict()
        else:
            indexing_config = self.indexing_config

        indexing_config_id: Union[None, Unset, str]
        if isinstance(self.indexing_config_id, Unset):
            indexing_config_id = UNSET
        else:
            indexing_config_id = self.indexing_config_id

        retrieval_config: Union[None, Unset, dict[str, Any]]
        if isinstance(self.retrieval_config, Unset):
            retrieval_config = UNSET
        elif isinstance(self.retrieval_config, RetrievalConfig):
            retrieval_config = self.retrieval_config.to_dict()
        else:
            retrieval_config = self.retrieval_config

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "intent": intent,
            }
        )
        if indexing_config is not UNSET:
            field_dict["indexing_config"] = indexing_config
        if indexing_config_id is not UNSET:
            field_dict["indexing_config_id"] = indexing_config_id
        if retrieval_config is not UNSET:
            field_dict["retrieval_config"] = retrieval_config

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.indexing_config import IndexingConfig
        from ..models.public_search_intent import PublicSearchIntent
        from ..models.retrieval_config import RetrievalConfig

        d = dict(src_dict)
        intent = PublicSearchIntent.from_dict(d.pop("intent"))

        def _parse_indexing_config(data: object) -> Union["IndexingConfig", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                indexing_config_type_1 = IndexingConfig.from_dict(data)

                return indexing_config_type_1
            except:  # noqa: E722
                pass
            return cast(Union["IndexingConfig", None, Unset], data)

        indexing_config = _parse_indexing_config(d.pop("indexing_config", UNSET))

        def _parse_indexing_config_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        indexing_config_id = _parse_indexing_config_id(d.pop("indexing_config_id", UNSET))

        def _parse_retrieval_config(data: object) -> Union["RetrievalConfig", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                retrieval_config_type_1 = RetrievalConfig.from_dict(data)

                return retrieval_config_type_1
            except:  # noqa: E722
                pass
            return cast(Union["RetrievalConfig", None, Unset], data)

        retrieval_config = _parse_retrieval_config(d.pop("retrieval_config", UNSET))

        query_request = cls(
            intent=intent,
            indexing_config=indexing_config,
            indexing_config_id=indexing_config_id,
            retrieval_config=retrieval_config,
        )

        query_request.additional_properties = d
        return query_request

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
