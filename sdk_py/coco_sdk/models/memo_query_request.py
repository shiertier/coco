from collections.abc import Mapping
from typing import TYPE_CHECKING, Any, TypeVar, Union, cast

from attrs import define as _attrs_define
from attrs import field as _attrs_field

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.public_search_intent import PublicSearchIntent
    from ..models.retrieval_config import RetrievalConfig


T = TypeVar("T", bound="MemoQueryRequest")


@_attrs_define
class MemoQueryRequest:
    """
    Attributes:
        intent (PublicSearchIntent): Search intent describing how retrieval should run.
        session_token (str):
        retrieval_config (Union['RetrievalConfig', None, Unset]):
    """

    intent: "PublicSearchIntent"
    session_token: str
    retrieval_config: Union["RetrievalConfig", None, Unset] = UNSET
    additional_properties: dict[str, Any] = _attrs_field(init=False, factory=dict)

    def to_dict(self) -> dict[str, Any]:
        from ..models.retrieval_config import RetrievalConfig

        intent = self.intent.to_dict()

        session_token = self.session_token

        retrieval_config: Union[None, Unset, dict[str, Any]]
        if isinstance(self.retrieval_config, Unset):
            retrieval_config = UNSET
        elif isinstance(self.retrieval_config, RetrievalConfig):
            retrieval_config = self.retrieval_config.to_dict()
        else:
            retrieval_config = self.retrieval_config

        field_dict: dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "intent": intent,
                "session_token": session_token,
            }
        )
        if retrieval_config is not UNSET:
            field_dict["retrieval_config"] = retrieval_config

        return field_dict

    @classmethod
    def from_dict(cls: type[T], src_dict: Mapping[str, Any]) -> T:
        from ..models.public_search_intent import PublicSearchIntent
        from ..models.retrieval_config import RetrievalConfig

        d = dict(src_dict)
        intent = PublicSearchIntent.from_dict(d.pop("intent"))

        session_token = d.pop("session_token")

        def _parse_retrieval_config(data: object) -> Union["RetrievalConfig", None, Unset]:
            if data is None:
                return data
            if isinstance(data, Unset):
                return data
            try:
                if not isinstance(data, dict):
                    raise TypeError()
                retrieval_config_type_1 = RetrievalConfig.from_dict(data)

                return retrieval_config_type_1
            except:  # noqa: E722
                pass
            return cast(Union["RetrievalConfig", None, Unset], data)

        retrieval_config = _parse_retrieval_config(d.pop("retrieval_config", UNSET))

        memo_query_request = cls(
            intent=intent,
            session_token=session_token,
            retrieval_config=retrieval_config,
        )

        memo_query_request.additional_properties = d
        return memo_query_request

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
