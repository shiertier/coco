from enum import Enum


class VectorBackendKind(str, Enum):
    PG_VECTOR = "pg_vector"
    QDRANT = "qdrant"

    def __str__(self) -> str:
        return str(self.value)
