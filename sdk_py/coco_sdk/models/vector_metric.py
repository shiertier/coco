from enum import Enum


class VectorMetric(str, Enum):
    COSINE = "cosine"
    DOT = "dot"
    L2 = "l2"

    def __str__(self) -> str:
        return str(self.value)
