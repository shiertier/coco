from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar

from attrs import define as _attrs_define
from attrs import field as _attrs_field

if TYPE_CHECKING:
    from ..models.queue_status_response import QueueStatusResponse
    from ..models.vector_backend_status import VectorBackendStatus
    from ..models.worker_status_response import WorkerStatusResponse


T = TypeVar("T", bound="HealthResponse")


@_attrs_define
class HealthResponse:
    """
    Attributes:
        queue (QueueStatusResponse):
        service (str):
        status (str):
        vector_backend (VectorBackendStatus):
        version (str):
        worker (WorkerStatusResponse):
    """

    queue: "QueueStatusResponse"
    service: str
    status: str
    vector_backend: "VectorBackendStatus"
    version: str
    worker: "WorkerStatusResponse"
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        queue = self.queue.to_dict()

        service = self.service

        status = self.status

        vector_backend = self.vector_backend.to_dict()

        version = self.version

        worker = self.worker.to_dict()

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "queue": queue,
                "service": service,
                "status": status,
                "vector_backend": vector_backend,
                "version": version,
                "worker": worker,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.queue_status_response import QueueStatusResponse
        from ..models.vector_backend_status import VectorBackendStatus
        from ..models.worker_status_response import WorkerStatusResponse

        d = dict(src_dict)
        queue = QueueStatusResponse.from_dict(d.pop("queue"))

        service = d.pop("service")

        status = d.pop("status")

        vector_backend = VectorBackendStatus.from_dict(d.pop("vector_backend"))

        version = d.pop("version")

        worker = WorkerStatusResponse.from_dict(d.pop("worker"))

        health_response = cls(
            queue=queue,
            service=service,
            status=status,
            vector_backend=vector_backend,
            version=version,
            worker=worker,
        )

        health_response.additional_properties = d
        return health_response

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
