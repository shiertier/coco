from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..models.retrieval_mode import RetrievalMode
from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.reranker_config import RerankerConfig
    from ..models.vector_backend_config import VectorBackendConfig


T = TypeVar("T", bound="RetrievalConfig")


@_attrs_define
class RetrievalConfig:
    """Query-time retrieval configuration.

    Attributes:
        hybrid_alpha (float): Hybrid weight for vector vs. keyword scoring.
        retrieval_mode (RetrievalMode): Retrieval mode for query execution.
        top_k (int): Number of candidates to return.
        reranker (Union['RerankerConfig', None, Unset]):
        vector_backend (Union['VectorBackendConfig', None, Unset]):
    """

    hybrid_alpha: float
    retrieval_mode: RetrievalMode
    top_k: int
    reranker: Union["RerankerConfig", None, Unset] = UNSET
    vector_backend: Union["VectorBackendConfig", None, Unset] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        from ..models.reranker_config import RerankerConfig
        from ..models.vector_backend_config import VectorBackendConfig

        hybrid_alpha = self.hybrid_alpha

        retrieval_mode = self.retrieval_mode.value

        top_k = self.top_k

        reranker: Union[None, Unset, dict[str, Any]]
        if isinstance(self.reranker, Unset):
            reranker = UNSET
        elif isinstance(self.reranker, RerankerConfig):
            reranker = self.reranker.to_dict()
        else:
            reranker = self.reranker

        vector_backend: Union[None, Unset, dict[str, Any]]
        if isinstance(self.vector_backend, Unset):
            vector_backend = UNSET
        elif isinstance(self.vector_backend, VectorBackendConfig):
            vector_backend = self.vector_backend.to_dict()
        else:
            vector_backend = self.vector_backend

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "hybrid_alpha": hybrid_alpha,
                "retrieval_mode": retrieval_mode,
                "top_k": top_k,
            }
        )
        if reranker is not UNSET:
            field_dict["reranker"] = reranker
        if vector_backend is not UNSET:
            field_dict["vector_backend"] = vector_backend

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.reranker_config import RerankerConfig
        from ..models.vector_backend_config import VectorBackendConfig

        d = dict(src_dict)
        hybrid_alpha = d.pop("hybrid_alpha")

        retrieval_mode = RetrievalMode(d.pop("retrieval_mode"))

        top_k = d.pop("top_k")

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

        def _parse_vector_backend(data: object) -> Union["VectorBackendConfig", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                vector_backend_type_1 = VectorBackendConfig.from_dict(data)

                return vector_backend_type_1
            except:  # noqa: E722
                pass
            return cast(Union["VectorBackendConfig", None, Unset], data)

        vector_backend = _parse_vector_backend(d.pop("vector_backend", UNSET))

        retrieval_config = cls(
            hybrid_alpha=hybrid_alpha,
            retrieval_mode=retrieval_mode,
            top_k=top_k,
            reranker=reranker,
            vector_backend=vector_backend,
        )

        retrieval_config.additional_properties = d
        return retrieval_config

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
