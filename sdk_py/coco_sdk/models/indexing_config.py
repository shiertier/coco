from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..models.vector_metric import VectorMetric
from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.chunking_strategy import ChunkingStrategy
    from ..models.embedding_config import EmbeddingConfig
    from ..models.vector_backend_config import VectorBackendConfig
    from ..models.vector_index_params import VectorIndexParams


T = TypeVar("T", bound="IndexingConfig")


@_attrs_define
class IndexingConfig:
    """Indexing-time configuration for a project.

    Attributes:
        chunking (ChunkingStrategy): Chunking strategy parameters.
        config_id (str): Stable identifier of this indexing strategy.
            Note: a config_id must map to a compatible embedding dimension/metric.
        embedding (EmbeddingConfig): Embedding model configuration.
        vector_metric (VectorMetric): Supported vector similarity metrics.
        index_params (Union['VectorIndexParams', None, Unset]):
        vector_backend (Union['VectorBackendConfig', None, Unset]):
    """

    chunking: "ChunkingStrategy"
    config_id: str
    embedding: "EmbeddingConfig"
    vector_metric: VectorMetric
    index_params: Union["VectorIndexParams", None, Unset] = UNSET
    vector_backend: Union["VectorBackendConfig", None, Unset] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        from ..models.vector_backend_config import VectorBackendConfig
        from ..models.vector_index_params import VectorIndexParams

        chunking = self.chunking.to_dict()

        config_id = self.config_id

        embedding = self.embedding.to_dict()

        vector_metric = self.vector_metric.value

        index_params: Union[None, Unset, dict[str, Any]]
        if isinstance(self.index_params, Unset):
            index_params = UNSET
        elif isinstance(self.index_params, VectorIndexParams):
            index_params = self.index_params.to_dict()
        else:
            index_params = self.index_params

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
                "chunking": chunking,
                "config_id": config_id,
                "embedding": embedding,
                "vector_metric": vector_metric,
            }
        )
        if index_params is not UNSET:
            field_dict["index_params"] = index_params
        if vector_backend is not UNSET:
            field_dict["vector_backend"] = vector_backend

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.chunking_strategy import ChunkingStrategy
        from ..models.embedding_config import EmbeddingConfig
        from ..models.vector_backend_config import VectorBackendConfig
        from ..models.vector_index_params import VectorIndexParams

        d = dict(src_dict)
        chunking = ChunkingStrategy.from_dict(d.pop("chunking"))

        config_id = d.pop("config_id")

        embedding = EmbeddingConfig.from_dict(d.pop("embedding"))

        vector_metric = VectorMetric(d.pop("vector_metric"))

        def _parse_index_params(data: object) -> Union["VectorIndexParams", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                index_params_type_1 = VectorIndexParams.from_dict(data)

                return index_params_type_1
            except:  # noqa: E722
                pass
            return cast(Union["VectorIndexParams", None, Unset], data)

        index_params = _parse_index_params(d.pop("index_params", UNSET))

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

        indexing_config = cls(
            chunking=chunking,
            config_id=config_id,
            embedding=embedding,
            vector_metric=vector_metric,
            index_params=index_params,
            vector_backend=vector_backend,
        )

        indexing_config.additional_properties = d
        return indexing_config

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
