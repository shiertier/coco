from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="IvfPqParams")


@_attrs_define
class IvfPqParams:
    """IVF-PQ index parameters.

    Attributes:
        max_iterations (Union[None, Unset, int]): Max iterations for IVF training.
        num_partitions (Union[None, Unset, int]): Number of IVF partitions.
        num_sub_vectors (Union[None, Unset, int]): Number of PQ sub-vectors.
        sample_rate (Union[None, Unset, int]): Sample rate for IVF training.
    """

    max_iterations: Union[None, Unset, int] = UNSET
    num_partitions: Union[None, Unset, int] = UNSET
    num_sub_vectors: Union[None, Unset, int] = UNSET
    sample_rate: Union[None, Unset, int] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        max_iterations: Union[None, Unset, int]
        if isinstance(self.max_iterations, Unset):
            max_iterations = UNSET
        else:
            max_iterations = self.max_iterations

        num_partitions: Union[None, Unset, int]
        if isinstance(self.num_partitions, Unset):
            num_partitions = UNSET
        else:
            num_partitions = self.num_partitions

        num_sub_vectors: Union[None, Unset, int]
        if isinstance(self.num_sub_vectors, Unset):
            num_sub_vectors = UNSET
        else:
            num_sub_vectors = self.num_sub_vectors

        sample_rate: Union[None, Unset, int]
        if isinstance(self.sample_rate, Unset):
            sample_rate = UNSET
        else:
            sample_rate = self.sample_rate

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if max_iterations is not UNSET:
            field_dict["max_iterations"] = max_iterations
        if num_partitions is not UNSET:
            field_dict["num_partitions"] = num_partitions
        if num_sub_vectors is not UNSET:
            field_dict["num_sub_vectors"] = num_sub_vectors
        if sample_rate is not UNSET:
            field_dict["sample_rate"] = sample_rate

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)

        def _parse_max_iterations(data: object) -> Union[None, Unset, int]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, int], data)

        max_iterations = _parse_max_iterations(d.pop("max_iterations", UNSET))

        def _parse_num_partitions(data: object) -> Union[None, Unset, int]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, int], data)

        num_partitions = _parse_num_partitions(d.pop("num_partitions", UNSET))

        def _parse_num_sub_vectors(data: object) -> Union[None, Unset, int]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, int], data)

        num_sub_vectors = _parse_num_sub_vectors(d.pop("num_sub_vectors", UNSET))

        def _parse_sample_rate(data: object) -> Union[None, Unset, int]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, int], data)

        sample_rate = _parse_sample_rate(d.pop("sample_rate", UNSET))

        ivf_pq_params = cls(
            max_iterations=max_iterations,
            num_partitions=num_partitions,
            num_sub_vectors=num_sub_vectors,
            sample_rate=sample_rate,
        )

        ivf_pq_params.additional_properties = d
        return ivf_pq_params

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
