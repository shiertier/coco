from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..models.retrieval_mode import RetrievalMode
from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.public_filter import PublicFilter
    from ..models.reranker_config import RerankerConfig


T = TypeVar("T", bound="PublicSearchIntent")


@_attrs_define
class PublicSearchIntent:
    """Search intent describing how retrieval should run.

    Attributes:
        filters (list['PublicFilter']): Optional filter list (server allows `doc_id`/`chunk_id` with `eq`/`contains`).
        hybrid_alpha (float): Hybrid weight for vector vs. keyword scoring.
        retrieval_mode (RetrievalMode): Retrieval mode for query execution.
        top_k (int): Number of candidates to return.
        indexing_config_id (Union[None, Unset, str]): Optional indexing configuration selection (defaults to the
            project's default config).
        query_embedding (Union[None, Unset, list[float]]): Optional query embedding (required for `vector` and `hybrid`
            retrieval).
        query_text (Union[None, Unset, str]): Optional user query text (required for `fts` and `hybrid` retrieval).
        reranker (Union['RerankerConfig', None, Unset]):
    """

    filters: list["PublicFilter"]
    hybrid_alpha: float
    retrieval_mode: RetrievalMode
    top_k: int
    indexing_config_id: Union[None, Unset, str] = UNSET
    query_embedding: Union[None, Unset, list[float]] = UNSET
    query_text: Union[None, Unset, str] = UNSET
    reranker: Union["RerankerConfig", None, Unset] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        from ..models.reranker_config import RerankerConfig

        filters = []
        for filters_item_data in self.filters:
            filters_item = filters_item_data.to_dict()
            filters.append(filters_item)

        hybrid_alpha = self.hybrid_alpha

        retrieval_mode = self.retrieval_mode.value

        top_k = self.top_k

        indexing_config_id: Union[None, Unset, str]
        if isinstance(self.indexing_config_id, Unset):
            indexing_config_id = UNSET
        else:
            indexing_config_id = self.indexing_config_id

        query_embedding: Union[None, Unset, list[float]]
        if isinstance(self.query_embedding, Unset):
            query_embedding = UNSET
        elif isinstance(self.query_embedding, list):
            query_embedding = self.query_embedding

        else:
            query_embedding = self.query_embedding

        query_text: Union[None, Unset, str]
        if isinstance(self.query_text, Unset):
            query_text = UNSET
        else:
            query_text = self.query_text

        reranker: Union[None, Unset, dict[str, Any]]
        if isinstance(self.reranker, Unset):
            reranker = UNSET
        elif isinstance(self.reranker, RerankerConfig):
            reranker = self.reranker.to_dict()
        else:
            reranker = self.reranker

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "filters": filters,
                "hybrid_alpha": hybrid_alpha,
                "retrieval_mode": retrieval_mode,
                "top_k": top_k,
            }
        )
        if indexing_config_id is not UNSET:
            field_dict["indexing_config_id"] = indexing_config_id
        if query_embedding is not UNSET:
            field_dict["query_embedding"] = query_embedding
        if query_text is not UNSET:
            field_dict["query_text"] = query_text
        if reranker is not UNSET:
            field_dict["reranker"] = reranker

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.public_filter import PublicFilter
        from ..models.reranker_config import RerankerConfig

        d = dict(src_dict)
        filters = []
        _filters = d.pop("filters")
        for filters_item_data in _filters:
            filters_item = PublicFilter.from_dict(filters_item_data)

            filters.append(filters_item)

        hybrid_alpha = d.pop("hybrid_alpha")

        retrieval_mode = RetrievalMode(d.pop("retrieval_mode"))

        top_k = d.pop("top_k")

        def _parse_indexing_config_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        indexing_config_id = _parse_indexing_config_id(d.pop("indexing_config_id", UNSET))

        def _parse_query_embedding(data: object) -> Union[None, Unset, list[float]]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, list):
                    raise TypeError()
                query_embedding_type_0 = cast(list[float], data)

                return query_embedding_type_0
            except:  # noqa: E722
                pass
            return cast(Union[None, Unset, list[float]], data)

        query_embedding = _parse_query_embedding(d.pop("query_embedding", UNSET))

        def _parse_query_text(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        query_text = _parse_query_text(d.pop("query_text", UNSET))

        def _parse_reranker(data: object) -> Union["RerankerConfig", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                reranker_type_1 = RerankerConfig.from_dict(data)

                return reranker_type_1
            except:  # noqa: E722
                pass
            return cast(Union["RerankerConfig", None, Unset], data)

        reranker = _parse_reranker(d.pop("reranker", UNSET))

        public_search_intent = cls(
            filters=filters,
            hybrid_alpha=hybrid_alpha,
            retrieval_mode=retrieval_mode,
            top_k=top_k,
            indexing_config_id=indexing_config_id,
            query_embedding=query_embedding,
            query_text=query_text,
            reranker=reranker,
        )

        public_search_intent.additional_properties = d
        return public_search_intent

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
