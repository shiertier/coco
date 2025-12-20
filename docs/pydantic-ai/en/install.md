# Installation

Pydantic AI is available on PyPI as [`pydantic-ai`](https://pypi.org/project/pydantic-ai/) so installation is as simple as:

```bash
pip install pydantic-ai

```

```bash
uv add pydantic-ai

```

(Requires Python 3.10+)

This installs the `pydantic_ai` package, core dependencies, and libraries required to use all the models included in Pydantic AI. If you want to install only those dependencies required to use a specific model, you can install the ["slim"](#slim-install) version of Pydantic AI.

## Use with Pydantic Logfire

Pydantic AI has an excellent (but completely optional) integration with [Pydantic Logfire](https://pydantic.dev/logfire) to help you view and understand agent runs.

Logfire comes included with `pydantic-ai` (but not the ["slim" version](#slim-install)), so you can typically start using it immediately by following the [Logfire setup docs](../logfire/#using-logfire).

## Running Examples

We distribute the [`pydantic_ai_examples`](https://github.com/pydantic/pydantic-ai/tree/main/examples/pydantic_ai_examples) directory as a separate PyPI package ([`pydantic-ai-examples`](https://pypi.org/project/pydantic-ai-examples/)) to make examples extremely easy to customize and run.

To install examples, use the `examples` optional group:

```bash
pip install "pydantic-ai[examples]"

```

```bash
uv add "pydantic-ai[examples]"

```

To run the examples, follow instructions in the [examples docs](../examples/setup/).

## Slim Install

If you know which model you're going to use and want to avoid installing superfluous packages, you can use the [`pydantic-ai-slim`](https://pypi.org/project/pydantic-ai-slim/) package. For example, if you're using just OpenAIChatModel, you would run:

```bash
pip install "pydantic-ai-slim[openai]"

```

```bash
uv add "pydantic-ai-slim[openai]"

```

`pydantic-ai-slim` has the following optional groups:

- `logfire` — installs [Pydantic Logfire](../logfire/) dependency `logfire` [PyPI ↗](https://pypi.org/project/logfire)
- `evals` — installs [Pydantic Evals](../evals/) dependency `pydantic-evals` [PyPI ↗](https://pypi.org/project/pydantic-evals)
- `openai` — installs [OpenAI Model](../models/openai/) dependency `openai` [PyPI ↗](https://pypi.org/project/openai)
- `vertexai` — installs `GoogleVertexProvider` dependencies `google-auth` [PyPI ↗](https://pypi.org/project/google-auth) and `requests` [PyPI ↗](https://pypi.org/project/requests)
- `google` — installs [Google Model](../models/google/) dependency `google-genai` [PyPI ↗](https://pypi.org/project/google-genai)
- `anthropic` — installs [Anthropic Model](../models/anthropic/) dependency `anthropic` [PyPI ↗](https://pypi.org/project/anthropic)
- `groq` — installs [Groq Model](../models/groq/) dependency `groq` [PyPI ↗](https://pypi.org/project/groq)
- `mistral` — installs [Mistral Model](../models/mistral/) dependency `mistralai` [PyPI ↗](https://pypi.org/project/mistralai)
- `cohere` - installs [Cohere Model](../models/cohere/) dependency `cohere` [PyPI ↗](https://pypi.org/project/cohere)
- `bedrock` - installs [Bedrock Model](../models/bedrock/) dependency `boto3` [PyPI ↗](https://pypi.org/project/boto3)
- `huggingface` - installs [Hugging Face Model](../models/huggingface/) dependency `huggingface-hub[inference]` [PyPI ↗](https://pypi.org/project/huggingface-hub)
- `outlines-transformers` - installs [Outlines Model](../models/outlines/) dependency `outlines[transformers]` [PyPI ↗](https://pypi.org/project/outlines)
- `outlines-llamacpp` - installs [Outlines Model](../models/outlines/) dependency `outlines[llamacpp]` [PyPI ↗](https://pypi.org/project/outlines)
- `outlines-mlxlm` - installs [Outlines Model](../models/outlines/) dependency `outlines[mlxlm]` [PyPI ↗](https://pypi.org/project/outlines)
- `outlines-sglang` - installs [Outlines Model](../models/outlines/) dependency `outlines[sglang]` [PyPI ↗](https://pypi.org/project/outlines)
- `outlines-vllm-offline` - installs [Outlines Model](../models/outlines/) dependencies `outlines` [PyPI ↗](https://pypi.org/project/outlines) and `vllm` [PyPI ↗](https://pypi.org/project/vllm)
- `duckduckgo` - installs [DuckDuckGo Search Tool](../common-tools/#duckduckgo-search-tool) dependency `ddgs` [PyPI ↗](https://pypi.org/project/ddgs)
- `tavily` - installs [Tavily Search Tool](../common-tools/#tavily-search-tool) dependency `tavily-python` [PyPI ↗](https://pypi.org/project/tavily-python)
- `cli` - installs [CLI](../cli/) dependencies `rich` [PyPI ↗](https://pypi.org/project/rich), `prompt-toolkit` [PyPI ↗](https://pypi.org/project/prompt-toolkit), and `argcomplete` [PyPI ↗](https://pypi.org/project/argcomplete)
- `mcp` - installs [MCP](../mcp/client/) dependency `mcp` [PyPI ↗](https://pypi.org/project/mcp)
- `fastmcp` - installs [FastMCP](../mcp/fastmcp-client/) dependency `fastmcp` [PyPI ↗](https://pypi.org/project/fastmcp)
- `a2a` - installs [A2A](../a2a/) dependency `fasta2a` [PyPI ↗](https://pypi.org/project/fasta2a)
- `ui` - installs [UI Event Streams](../ui/overview/) dependency `starlette` [PyPI ↗](https://pypi.org/project/starlette)
- `ag-ui` - installs [AG-UI Event Stream Protocol](../ui/ag-ui/) dependencies `ag-ui-protocol` [PyPI ↗](https://pypi.org/project/ag-ui-protocol) and `starlette` [PyPI ↗](https://pypi.org/project/starlette)
- `dbos` - installs [DBOS Durable Execution](../durable_execution/dbos/) dependency `dbos` [PyPI ↗](https://pypi.org/project/dbos)
- `prefect` - installs [Prefect Durable Execution](../durable_execution/prefect/) dependency `prefect` [PyPI ↗](https://pypi.org/project/prefect)

You can also install dependencies for multiple models and use cases, for example:

```bash
pip install "pydantic-ai-slim[openai,google,logfire]"

```

```bash
uv add "pydantic-ai-slim[openai,google,logfire]"

```
