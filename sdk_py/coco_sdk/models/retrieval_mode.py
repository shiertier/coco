from enum import Enum


class RetrievalMode(str, Enum):
    FTS = "fts"
    HYBRID = "hybrid"
    VECTOR = "vector"

    def __str__(self) -> str:
        return str(self.value)
