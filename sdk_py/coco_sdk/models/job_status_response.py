from collections.abc import Mapping
from typing import Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

T = TypeVar("T", bound="JobStatusResponse")


@_attrs_define
class JobStatusResponse:
    """
    Attributes:
        attempts (int):
        created_at (str):
        job_id (str):
        status (str):
        updated_at (str):
        error (Union[None, Unset, str]):
        version_id (Union[None, Unset, str]):
    """

    attempts: int
    created_at: str
    job_id: str
    status: str
    updated_at: str
    error: Union[None, Unset, str] = UNSET
    version_id: Union[None, Unset, str] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        attempts = self.attempts

        created_at = self.created_at

        job_id = self.job_id

        status = self.status

        updated_at = self.updated_at

        error: Union[None, Unset, str]
        if isinstance(self.error, Unset):
            error = UNSET
        else:
            error = self.error

        version_id: Union[None, Unset, str]
        if isinstance(self.version_id, Unset):
            version_id = UNSET
        else:
            version_id = self.version_id

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "attempts": attempts,
                "created_at": created_at,
                "job_id": job_id,
                "status": status,
                "updated_at": updated_at,
            }
        )
        if error is not UNSET:
            field_dict["error"] = error
        if version_id is not UNSET:
            field_dict["version_id"] = version_id

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        d = dict(src_dict)
        attempts = d.pop("attempts")

        created_at = d.pop("created_at")

        job_id = d.pop("job_id")

        status = d.pop("status")

        updated_at = d.pop("updated_at")

        def _parse_error(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        error = _parse_error(d.pop("error", UNSET))

        def _parse_version_id(data: object) -> Union[None, Unset, str]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            return cast(Union[None, Unset, str], data)

        version_id = _parse_version_id(d.pop("version_id", UNSET))

        job_status_response = cls(
            attempts=attempts,
            created_at=created_at,
            job_id=job_id,
            status=status,
            updated_at=updated_at,
            error=error,
            version_id=version_id,
        )

        job_status_response.additional_properties = d
        return job_status_response

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
