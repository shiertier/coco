from http import HTTPStatus
from typing import Any, Optional, Union

import httpx

from ... import errors
from ...client import AuthenticatedClient, Client
from ...models.job_status_response import JobStatusResponse
from ...types import UNSET, Response, Unset


def _get_kwargs(
    id: str,
    *,
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: Union[None, Unset, str] = UNSET,
) -> dict[str, Any]:
    headers: dict[str, Any] = {}
    headers["x-coco-org-id"] = x_coco_org_id

    headers["x-coco-user-id"] = x_coco_user_id

    if not isinstance(x_coco_project_id, Unset):
        headers["x-coco-project-id"] = x_coco_project_id

    _kwargs: dict[str, Any] = {
        "method": "get",
        "url": f"/v1/jobs/{id}/events",
    }

    _kwargs["headers"] = headers
    return _kwargs


def _parse_response(
    *, client: Union[AuthenticatedClient, Client], response: httpx.Response
) -> Optional[JobStatusResponse]:
    if response.status_code == 200:
        response_200 = JobStatusResponse.from_dict(response.text)

        return response_200

    if client.raise_on_unexpected_status:
        raise errors.UnexpectedStatus(response.status_code, response.content)
    else:
        return None


def _build_response(
    *, client: Union[AuthenticatedClient, Client], response: httpx.Response
) -> Response[JobStatusResponse]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def sync_detailed(
    id: str,
    *,
    client: Union[AuthenticatedClient, Client],
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: Union[None, Unset, str] = UNSET,
) -> Response[JobStatusResponse]:
    """
    Args:
        id (str):
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[JobStatusResponse]
    """

    kwargs = _get_kwargs(
        id=id,
        x_coco_org_id=x_coco_org_id,
        x_coco_user_id=x_coco_user_id,
        x_coco_project_id=x_coco_project_id,
    )

    response = client.get_httpx_client().request(
        **kwargs,
    )

    return _build_response(client=client, response=response)


def sync(
    id: str,
    *,
    client: Union[AuthenticatedClient, Client],
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: Union[None, Unset, str] = UNSET,
) -> Optional[JobStatusResponse]:
    """
    Args:
        id (str):
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        JobStatusResponse
    """

    return sync_detailed(
        id=id,
        client=client,
        x_coco_org_id=x_coco_org_id,
        x_coco_user_id=x_coco_user_id,
        x_coco_project_id=x_coco_project_id,
    ).parsed


async def asyncio_detailed(
    id: str,
    *,
    client: Union[AuthenticatedClient, Client],
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: Union[None, Unset, str] = UNSET,
) -> Response[JobStatusResponse]:
    """
    Args:
        id (str):
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[JobStatusResponse]
    """

    kwargs = _get_kwargs(
        id=id,
        x_coco_org_id=x_coco_org_id,
        x_coco_user_id=x_coco_user_id,
        x_coco_project_id=x_coco_project_id,
    )

    response = await client.get_async_httpx_client().request(**kwargs)

    return _build_response(client=client, response=response)


async def asyncio(
    id: str,
    *,
    client: Union[AuthenticatedClient, Client],
    x_coco_org_id: str,
    x_coco_user_id: str,
    x_coco_project_id: Union[None, Unset, str] = UNSET,
) -> Optional[JobStatusResponse]:
    """
    Args:
        id (str):
        x_coco_org_id (str):
        x_coco_user_id (str):
        x_coco_project_id (Union[None, Unset, str]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        JobStatusResponse
    """

    return (
        await asyncio_detailed(
            id=id,
            client=client,
            x_coco_org_id=x_coco_org_id,
            x_coco_user_id=x_coco_user_id,
            x_coco_project_id=x_coco_project_id,
        )
    ).parsed
