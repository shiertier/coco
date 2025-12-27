from http import HTTPStatus
from typing import Any, Optional, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.error_response import ErrorResponse
from ...models.query_request import QueryRequest
from ...models.query_response_envelope import QueryResponseEnvelope
from ...types import Response


def _get_kwargs(
    *,
    body: QueryRequest,
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: str,
) -> dict[str, Any]:
    headers: dict[str, Any] = {}
    headers["x-coco-org-id"] = x_coco_org_id

    headers["x-coco-user-id"] = x_coco_user_id

    headers["x-coco-project-id"] = x_coco_project_id

    _kwargs: dict[str, Any] = {
        "method": "post",
        "url": "/v1/docs/query",
    }

    _kwargs["json"] = body.to_dict()

    headers["Content-Type"] = "application/json"

    _kwargs["headers"] = headers
    return _kwargs


def _parse_response(
    *, client: Union[AuthenticatedClient, Client], response: httpx.Response
) -> Optional[Union[ErrorResponse, QueryResponseEnvelope]]:
    if response.status_code == 200:
        response_200 = QueryResponseEnvelope.from_dict(response.json())

        return response_200

    if response.status_code == 400:
        response_400 = ErrorResponse.from_dict(response.json())

        return response_400

    if response.status_code == 401:
        response_401 = ErrorResponse.from_dict(response.json())

        return response_401

    if response.status_code == 429:
        response_429 = ErrorResponse.from_dict(response.json())

        return response_429

    if client.raise_on_unexpected_status:
        raise errors.UnexpectedStatus(response.status_code, response.content)
    else:
        return None


def _build_response(
    *, client: Union[AuthenticatedClient, Client], response: httpx.Response
) -> Response[Union[ErrorResponse, QueryResponseEnvelope]]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def sync_detailed(
    *,
    client: Union[AuthenticatedClient, Client],
    body: QueryRequest,
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: str,
) -> Response[Union[ErrorResponse, QueryResponseEnvelope]]:
    """
    Args:
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (str):
        body (QueryRequest):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[ErrorResponse, QueryResponseEnvelope]]
    """

    kwargs = _get_kwargs(
        body=body,
        x_coco_org_id=x_coco_org_id,
        x_coco_user_id=x_coco_user_id,
        x_coco_project_id=x_coco_project_id,
    )

    response = client.get_httpx_client().request(
        **kwargs,
    )

    return _build_response(client=client, response=response)


def sync(
    *,
    client: Union[AuthenticatedClient, Client],
    body: QueryRequest,
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: str,
) -> Optional[Union[ErrorResponse, QueryResponseEnvelope]]:
    """
    Args:
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (str):
        body (QueryRequest):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Union[ErrorResponse, QueryResponseEnvelope]
    """

    return sync_detailed(
        client=client,
        body=body,
        x_coco_org_id=x_coco_org_id,
        x_coco_user_id=x_coco_user_id,
        x_coco_project_id=x_coco_project_id,
    ).parsed


async def asyncio_detailed(
    *,
    client: Union[AuthenticatedClient, Client],
    body: QueryRequest,
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: str,
) -> Response[Union[ErrorResponse, QueryResponseEnvelope]]:
    """
    Args:
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (str):
        body (QueryRequest):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[ErrorResponse, QueryResponseEnvelope]]
    """

    kwargs = _get_kwargs(
        body=body,
        x_coco_org_id=x_coco_org_id,
        x_coco_user_id=x_coco_user_id,
        x_coco_project_id=x_coco_project_id,
    )

    response = await client.get_async_httpx_client().request(**kwargs)

    return _build_response(client=client, response=response)


async def asyncio(
    *,
    client: Union[AuthenticatedClient, Client],
    body: QueryRequest,
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: str,
) -> Optional[Union[ErrorResponse, QueryResponseEnvelope]]:
    """
    Args:
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (str):
        body (QueryRequest):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Union[ErrorResponse, QueryResponseEnvelope]
    """

    return (
        await asyncio_detailed(
            client=client,
            body=body,
            x_coco_org_id=x_coco_org_id,
            x_coco_user_id=x_coco_user_id,
            x_coco_project_id=x_coco_project_id,
        )
    ).parsed
