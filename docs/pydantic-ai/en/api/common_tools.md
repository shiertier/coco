# `pydantic_ai.common_tools`

### DuckDuckGoResult

Bases: `TypedDict`

A DuckDuckGo search result.

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/duckduckgo.py`

```python
class DuckDuckGoResult(TypedDict):
    """A DuckDuckGo search result."""

    title: str
    """The title of the search result."""
    href: str
    """The URL of the search result."""
    body: str
    """The body of the search result."""

```

#### title

```python
title: str

```

The title of the search result.

#### href

```python
href: str

```

The URL of the search result.

#### body

```python
body: str

```

The body of the search result.

### DuckDuckGoSearchTool

The DuckDuckGo search tool.

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/duckduckgo.py`

```python
@dataclass
class DuckDuckGoSearchTool:
    """The DuckDuckGo search tool."""

    client: DDGS
    """The DuckDuckGo search client."""

    _: KW_ONLY

    max_results: int | None
    """The maximum number of results. If None, returns results only from the first response."""

    async def __call__(self, query: str) -> list[DuckDuckGoResult]:
        """Searches DuckDuckGo for the given query and returns the results.

        Args:
            query: The query to search for.

        Returns:
            The search results.
        """
        search = functools.partial(self.client.text, max_results=self.max_results)
        results = await anyio.to_thread.run_sync(search, query)
        return duckduckgo_ta.validate_python(results)

```

#### client

```python
client: DDGS

```

The DuckDuckGo search client.

#### max_results

```python
max_results: int | None

```

The maximum number of results. If None, returns results only from the first response.

#### __call__

```python
__call__(query: str) -> list[DuckDuckGoResult]

```

Searches DuckDuckGo for the given query and returns the results.

Parameters:

| Name | Type | Description | Default | | --- | --- | --- | --- | | `query` | `str` | The query to search for. | *required* |

Returns:

| Type | Description | | --- | --- | | `list[DuckDuckGoResult]` | The search results. |

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/duckduckgo.py`

```python
async def __call__(self, query: str) -> list[DuckDuckGoResult]:
    """Searches DuckDuckGo for the given query and returns the results.

    Args:
        query: The query to search for.

    Returns:
        The search results.
    """
    search = functools.partial(self.client.text, max_results=self.max_results)
    results = await anyio.to_thread.run_sync(search, query)
    return duckduckgo_ta.validate_python(results)

```

### duckduckgo_search_tool

```python
duckduckgo_search_tool(
    duckduckgo_client: DDGS | None = None,
    max_results: int | None = None,
)

```

Creates a DuckDuckGo search tool.

Parameters:

| Name | Type | Description | Default | | --- | --- | --- | --- | | `duckduckgo_client` | `DDGS | None` | The DuckDuckGo search client. | `None` | | `max_results` | `int | None` | The maximum number of results. If None, returns results only from the first response. | `None` |

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/duckduckgo.py`

```python
def duckduckgo_search_tool(duckduckgo_client: DDGS | None = None, max_results: int | None = None):
    """Creates a DuckDuckGo search tool.

    Args:
        duckduckgo_client: The DuckDuckGo search client.
        max_results: The maximum number of results. If None, returns results only from the first response.
    """
    return Tool[Any](
        DuckDuckGoSearchTool(client=duckduckgo_client or DDGS(), max_results=max_results).__call__,
        name='duckduckgo_search',
        description='Searches DuckDuckGo for the given query and returns the results.',
    )

```

### TavilySearchResult

Bases: `TypedDict`

A Tavily search result.

See [Tavily Search Endpoint documentation](https://docs.tavily.com/api-reference/endpoint/search) for more information.

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/tavily.py`

```python
class TavilySearchResult(TypedDict):
    """A Tavily search result.

    See [Tavily Search Endpoint documentation](https://docs.tavily.com/api-reference/endpoint/search)
    for more information.
    """

    title: str
    """The title of the search result."""
    url: str
    """The URL of the search result.."""
    content: str
    """A short description of the search result."""
    score: float
    """The relevance score of the search result."""

```

#### title

```python
title: str

```

The title of the search result.

#### url

```python
url: str

```

The URL of the search result..

#### content

```python
content: str

```

A short description of the search result.

#### score

```python
score: float

```

The relevance score of the search result.

### TavilySearchTool

The Tavily search tool.

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/tavily.py`

```python
@dataclass
class TavilySearchTool:
    """The Tavily search tool."""

    client: AsyncTavilyClient
    """The Tavily search client."""

    async def __call__(
        self,
        query: str,
        search_deep: Literal['basic', 'advanced'] = 'basic',
        topic: Literal['general', 'news'] = 'general',
        time_range: Literal['day', 'week', 'month', 'year', 'd', 'w', 'm', 'y'] | None = None,
    ) -> list[TavilySearchResult]:
        """Searches Tavily for the given query and returns the results.

        Args:
            query: The search query to execute with Tavily.
            search_deep: The depth of the search.
            topic: The category of the search.
            time_range: The time range back from the current date to filter results.

        Returns:
            A list of search results from Tavily.
        """
        results = await self.client.search(query, search_depth=search_deep, topic=topic, time_range=time_range)  # type: ignore[reportUnknownMemberType]
        return tavily_search_ta.validate_python(results['results'])

```

#### client

```python
client: AsyncTavilyClient

```

The Tavily search client.

#### __call__

```python
__call__(
    query: str,
    search_deep: Literal["basic", "advanced"] = "basic",
    topic: Literal["general", "news"] = "general",
    time_range: (
        Literal[
            "day",
            "week",
            "month",
            "year",
            "d",
            "w",
            "m",
            "y",
        ]
        | None
    ) = None,
) -> list[TavilySearchResult]

```

Searches Tavily for the given query and returns the results.

Parameters:

| Name | Type | Description | Default | | --- | --- | --- | --- | | `query` | `str` | The search query to execute with Tavily. | *required* | | `search_deep` | `Literal['basic', 'advanced']` | The depth of the search. | `'basic'` | | `topic` | `Literal['general', 'news']` | The category of the search. | `'general'` | | `time_range` | `Literal['day', 'week', 'month', 'year', 'd', 'w', 'm', 'y'] | None` | The time range back from the current date to filter results. | `None` |

Returns:

| Type | Description | | --- | --- | | `list[TavilySearchResult]` | A list of search results from Tavily. |

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/tavily.py`

```python
async def __call__(
    self,
    query: str,
    search_deep: Literal['basic', 'advanced'] = 'basic',
    topic: Literal['general', 'news'] = 'general',
    time_range: Literal['day', 'week', 'month', 'year', 'd', 'w', 'm', 'y'] | None = None,
) -> list[TavilySearchResult]:
    """Searches Tavily for the given query and returns the results.

    Args:
        query: The search query to execute with Tavily.
        search_deep: The depth of the search.
        topic: The category of the search.
        time_range: The time range back from the current date to filter results.

    Returns:
        A list of search results from Tavily.
    """
    results = await self.client.search(query, search_depth=search_deep, topic=topic, time_range=time_range)  # type: ignore[reportUnknownMemberType]
    return tavily_search_ta.validate_python(results['results'])

```

### tavily_search_tool

```python
tavily_search_tool(api_key: str)

```

Creates a Tavily search tool.

Parameters:

| Name | Type | Description | Default | | --- | --- | --- | --- | | `api_key` | `str` | The Tavily API key. You can get one by signing up at https://app.tavily.com/home. | *required* |

Source code in `pydantic_ai_slim/pydantic_ai/common_tools/tavily.py`

```python
def tavily_search_tool(api_key: str):
    """Creates a Tavily search tool.

    Args:
        api_key: The Tavily API key.

            You can get one by signing up at [https://app.tavily.com/home](https://app.tavily.com/home).
    """
    return Tool[Any](
        TavilySearchTool(client=AsyncTavilyClient(api_key)).__call__,
        name='tavily_search',
        description='Searches Tavily for the given query and returns the results.',
    )

```
