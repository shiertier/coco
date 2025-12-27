from enum import Enum


class CocoErrorKind(str, Enum):
    COMPUTE = "compute"
    NETWORK = "network"
    STORAGE = "storage"
    SYSTEM = "system"
    USER = "user"

    def __str__(self) -> str:
        return str(self.value)
