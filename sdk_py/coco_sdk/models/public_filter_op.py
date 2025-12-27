from enum import Enum


class PublicFilterOp(str, Enum):
    CONTAINS = "contains"
    EQ = "eq"

    def __str__(self) -> str:
        return str(self.value)
