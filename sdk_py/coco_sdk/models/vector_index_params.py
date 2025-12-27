from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.hnsw_params import HnswParams
    from ..models.ivf_pq_params import IvfPqParams


T = TypeVar("T", bound="VectorIndexParams")


@_attrs_define
class VectorIndexParams:
    """Backend-specific index parameter overrides.

    Attributes:
        hnsw (Union['HnswParams', None, Unset]):
        ivf_pq (Union['IvfPqParams', None, Unset]):
    """

    hnsw: Union["HnswParams", None, Unset] = UNSET
    ivf_pq: Union["IvfPqParams", None, Unset] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        from ..models.hnsw_params import HnswParams
        from ..models.ivf_pq_params import IvfPqParams

        hnsw: Union[None, Unset, dict[str, Any]]
        if isinstance(self.hnsw, Unset):
            hnsw = UNSET
        elif isinstance(self.hnsw, HnswParams):
            hnsw = self.hnsw.to_dict()
        else:
            hnsw = self.hnsw

        ivf_pq: Union[None, Unset, dict[str, Any]]
        if isinstance(self.ivf_pq, Unset):
            ivf_pq = UNSET
        elif isinstance(self.ivf_pq, IvfPqParams):
            ivf_pq = self.ivf_pq.to_dict()
        else:
            ivf_pq = self.ivf_pq

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if hnsw is not UNSET:
            field_dict["hnsw"] = hnsw
        if ivf_pq is not UNSET:
            field_dict["ivf_pq"] = ivf_pq

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.hnsw_params import HnswParams
        from ..models.ivf_pq_params import IvfPqParams

        d = dict(src_dict)

        def _parse_hnsw(data: object) -> Union["HnswParams", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                hnsw_type_1 = HnswParams.from_dict(data)

                return hnsw_type_1
            except:  # noqa: E722
                pass
            return cast(Union["HnswParams", None, Unset], data)

        hnsw = _parse_hnsw(d.pop("hnsw", UNSET))

        def _parse_ivf_pq(data: object) -> Union["IvfPqParams", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                ivf_pq_type_1 = IvfPqParams.from_dict(data)

                return ivf_pq_type_1
            except:  # noqa: E722
                pass
            return cast(Union["IvfPqParams", None, Unset], data)

        ivf_pq = _parse_ivf_pq(d.pop("ivf_pq", UNSET))

        vector_index_params = cls(
            hnsw=hnsw,
            ivf_pq=ivf_pq,
        )

        vector_index_params.additional_properties = d
        return vector_index_params

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
