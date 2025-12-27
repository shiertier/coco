from http import HTTPStatus
from typing import Any, Optional, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.error_response import ErrorResponse
from ...models.list_configs_response import ListConfigsResponse
from ...types import UNSET, Response, Unset


def _get_kwargs(
    *,
    org_id: str,
    user_id: Union[None, Unset, str] = UNSET,
    project_id: Union[None, Unset, str] = UNSET,
) -> dict[str, Any]:
    params: dict[str, Any] = {}

    params["org_id"] = org_id

    json_user_id: Union[None, Unset, str]
    if isinstance(user_id, Unset):
        json_user_id = UNSET
    else:
        json_user_id = user_id
    params["user_id"] = json_user_id

    json_project_id: Union[None, Unset, str]
    if isinstance(project_id, Unset):
        json_project_id = UNSET
    else:
        json_project_id = project_id
    params["project_id"] = json_project_id

    params = {k: v for k, v in params.items() if v is not UNSET and v is not None}

    _kwargs: dict[str, Any] = {
        "method": "get",
        "url": "/v1/sys/configs",
        "params": params,
    }

    return _kwargs


def _parse_response(
    *, client: Union[AuthenticatedClient, Client], response: httpx.Response
) -> Optional[Union[ErrorResponse, ListConfigsResponse]]:
    if response.status_code == 200:
        response_200 = ListConfigsResponse.from_dict(response.json())

        return response_200

    if response.status_code == 400:
        response_400 = ErrorResponse.from_dict(response.json())

        return response_400

    if response.status_code == 401:
        response_401 = ErrorResponse.from_dict(response.json())

        return response_401

    if client.raise_on_unexpected_status:
        raise errors.UnexpectedStatus(response.status_code, response.content)
    else:
        return None


def _build_response(
    *, client: Union[AuthenticatedClient, Client], response: httpx.Response
) -> Response[Union[ErrorResponse, ListConfigsResponse]]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def sync_detailed(
    *,
    client: Union[AuthenticatedClient, Client],
    org_id: str,
    user_id: Union[None, Unset, str] = UNSET,
    project_id: Union[None, Unset, str] = UNSET,
) -> Response[Union[ErrorResponse, ListConfigsResponse]]:
    """
    Args:
        org_id (str):
        user_id (Union[None, Unset, str]):
        project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[ErrorResponse, ListConfigsResponse]]
    """

    kwargs = _get_kwargs(
        org_id=org_id,
        user_id=user_id,
        project_id=project_id,
    )

    response = client.get_httpx_client().request(
        **kwargs,
    )

    return _build_response(client=client, response=response)


def sync(
    *,
    client: Union[AuthenticatedClient, Client],
    org_id: str,
    user_id: Union[None, Unset, str] = UNSET,
    project_id: Union[None, Unset, str] = UNSET,
) -> Optional[Union[ErrorResponse, ListConfigsResponse]]:
    """
    Args:
        org_id (str):
        user_id (Union[None, Unset, str]):
        project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Union[ErrorResponse, ListConfigsResponse]
    """

    return sync_detailed(
        client=client,
        org_id=org_id,
        user_id=user_id,
        project_id=project_id,
    ).parsed


async def asyncio_detailed(
    *,
    client: Union[AuthenticatedClient, Client],
    org_id: str,
    user_id: Union[None, Unset, str] = UNSET,
    project_id: Union[None, Unset, str] = UNSET,
) -> Response[Union[ErrorResponse, ListConfigsResponse]]:
    """
    Args:
        org_id (str):
        user_id (Union[None, Unset, str]):
        project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[ErrorResponse, ListConfigsResponse]]
    """

    kwargs = _get_kwargs(
        org_id=org_id,
        user_id=user_id,
        project_id=project_id,
    )

    response = await client.get_async_httpx_client().request(**kwargs)

    return _build_response(client=client, response=response)


async def asyncio(
    *,
    client: Union[AuthenticatedClient, Client],
    org_id: str,
    user_id: Union[None, Unset, str] = UNSET,
    project_id: Union[None, Unset, str] = UNSET,
) -> Optional[Union[ErrorResponse, ListConfigsResponse]]:
    """
    Args:
        org_id (str):
        user_id (Union[None, Unset, str]):
        project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Union[ErrorResponse, ListConfigsResponse]
    """

    return (
        await asyncio_detailed(
            client=client,
            org_id=org_id,
            user_id=user_id,
            project_id=project_id,
        )
    ).parsed
