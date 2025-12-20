# Model Providers

Pydantic AI is model-agnostic and has built-in support for multiple model providers:

- [OpenAI](../openai/)
- [Anthropic](../anthropic/)
- [Gemini](../google/) (via two different APIs: Generative Language API and VertexAI API)
- [Bedrock](../bedrock/)
- [Cerebras](../cerebras/)
- [Cohere](../cohere/)
- [Groq](../groq/)
- [Hugging Face](../huggingface/)
- [Mistral](../mistral/)
- [OpenRouter](../openrouter/)
- [Outlines](../outlines/)

## OpenAI-compatible Providers

In addition, many providers are compatible with the OpenAI API, and can be used with `OpenAIChatModel` in Pydantic AI:

- [Alibaba Cloud Model Studio (DashScope)](../openai/#alibaba-cloud-model-studio-dashscope)
- [Azure AI Foundry](../openai/#azure-ai-foundry)
- [DeepSeek](../openai/#deepseek)
- [Fireworks AI](../openai/#fireworks-ai)
- [GitHub Models](../openai/#github-models)
- [Grok (xAI)](../openai/#grok-xai)
- [Heroku](../openai/#heroku-ai)
- [LiteLLM](../openai/#litellm)
- [Nebius AI Studio](../openai/#nebius-ai-studio)
- [Ollama](../openai/#ollama)
- [OVHcloud AI Endpoints](../openai/#ovhcloud-ai-endpoints)
- [Perplexity](../openai/#perplexity)
- [Together AI](../openai/#together-ai)
- [Vercel AI Gateway](../openai/#vercel-ai-gateway)

Pydantic AI also comes with [`TestModel`](../../api/models/test/) and [`FunctionModel`](../../api/models/function/) for testing and development.

To use each model provider, you need to configure your local environment and make sure you have the right packages installed. If you try to use the model without having done so, you'll be told what to install.

## Models and Providers

Pydantic AI uses a few key terms to describe how it interacts with different LLMs:

- **Model**: This refers to the Pydantic AI class used to make requests following a specific LLM API (generally by wrapping a vendor-provided SDK, like the `openai` python SDK). These classes implement a vendor-SDK-agnostic API, ensuring a single Pydantic AI agent is portable to different LLM vendors without any other code changes just by swapping out the Model it uses. Model classes are named roughly in the format `<VendorSdk>Model`, for example, we have `OpenAIChatModel`, `AnthropicModel`, `GoogleModel`, etc. When using a Model class, you specify the actual LLM model name (e.g., `gpt-5`, `claude-sonnet-4-5`, `gemini-2.5-flash`) as a parameter.
- **Provider**: This refers to provider-specific classes which handle the authentication and connections to an LLM vendor. Passing a non-default *Provider* as a parameter to a Model is how you can ensure that your agent will make requests to a specific endpoint, or make use of a specific approach to authentication (e.g., you can use Azure auth with the `OpenAIChatModel` by way of the `AzureProvider`). In particular, this is how you can make use of an AI gateway, or an LLM vendor that offers API compatibility with the vendor SDK used by an existing Model (such as `OpenAIChatModel`).
- **Profile**: This refers to a description of how requests to a specific model or family of models need to be constructed to get the best results, independent of the model and provider classes used. For example, different models have different restrictions on the JSON schemas that can be used for tools, and the same schema transformer needs to be used for Gemini models whether you're using `GoogleModel` with model name `gemini-2.5-pro-preview`, or `OpenAIChatModel` with `OpenRouterProvider` and model name `google/gemini-2.5-pro-preview`.

When you instantiate an Agent with just a name formatted as `<provider>:<model>`, e.g. `openai:gpt-5` or `openrouter:google/gemini-2.5-pro-preview`, Pydantic AI will automatically select the appropriate model class, provider, and profile. If you want to use a different provider or profile, you can instantiate a model class directly and pass in `provider` and/or `profile` arguments.

## Custom Models

Note

If a model API is compatible with the OpenAI API, you do not need a custom model class and can provide your own [custom provider](../openai/#openai-compatible-models) instead.

To implement support for a model API that's not already supported, you will need to subclass the Model abstract base class. For streaming, you'll also need to implement the StreamedResponse abstract base class.

The best place to start is to review the source code for existing implementations, e.g. [`OpenAIChatModel`](https://github.com/pydantic/pydantic-ai/blob/main/pydantic_ai_slim/pydantic_ai/models/openai.py).

For details on when we'll accept contributions adding new models to Pydantic AI, see the [contributing guidelines](../../contributing/#new-model-rules).

## Fallback Model

You can use FallbackModel to attempt multiple models in sequence until one successfully returns a result. Under the hood, Pydantic AI automatically switches from one model to the next if the current model returns a 4xx or 5xx status code.

Note

The provider SDKs on which Models are based (like OpenAI, Anthropic, etc.) often have built-in retry logic that can delay the `FallbackModel` from activating.

When using `FallbackModel`, it's recommended to disable provider SDK retries to ensure immediate fallback, for example by setting `max_retries=0` on a [custom OpenAI client](../openai/#custom-openai-client).

In the following example, the agent first makes a request to the OpenAI model (which fails due to an invalid API key), and then falls back to the Anthropic model.

fallback_model.py

```python
from pydantic_ai import Agent
from pydantic_ai.models.anthropic import AnthropicModel
from pydantic_ai.models.fallback import FallbackModel
from pydantic_ai.models.openai import OpenAIChatModel

openai_model = OpenAIChatModel('gpt-5')
anthropic_model = AnthropicModel('claude-sonnet-4-5')
fallback_model = FallbackModel(openai_model, anthropic_model)

agent = Agent(fallback_model)
response = agent.run_sync('What is the capital of France?')
print(response.data)
#> Paris

print(response.all_messages())
"""
[
    ModelRequest(
        parts=[
            UserPromptPart(
                content='What is the capital of France?',
                timestamp=datetime.datetime(...),
                part_kind='user-prompt',
            )
        ],
        kind='request',
    ),
    ModelResponse(
        parts=[TextPart(content='Paris', part_kind='text')],
        model_name='claude-sonnet-4-5',
        timestamp=datetime.datetime(...),
        kind='response',
        provider_response_id=None,
    ),
]
"""

```

The `ModelResponse` message above indicates in the `model_name` field that the output was returned by the Anthropic model, which is the second model specified in the `FallbackModel`.

Note

Each model's options should be configured individually. For example, `base_url`, `api_key`, and custom clients should be set on each model itself, not on the `FallbackModel`.

### Per-Model Settings

You can configure different ModelSettings for each model in a fallback chain by passing the `settings` parameter when creating each model. This is particularly useful when different providers have different optimal configurations:

fallback_model_per_settings.py

```python
from pydantic_ai import Agent, ModelSettings
from pydantic_ai.models.anthropic import AnthropicModel
from pydantic_ai.models.fallback import FallbackModel
from pydantic_ai.models.openai import OpenAIChatModel

# Configure each model with provider-specific optimal settings
openai_model = OpenAIChatModel(
    'gpt-5',
    settings=ModelSettings(temperature=0.7, max_tokens=1000)  # Higher creativity for OpenAI
)
anthropic_model = AnthropicModel(
    'claude-sonnet-4-5',
    settings=ModelSettings(temperature=0.2, max_tokens=1000)  # Lower temperature for consistency
)

fallback_model = FallbackModel(openai_model, anthropic_model)
agent = Agent(fallback_model)

result = agent.run_sync('Write a creative story about space exploration')
print(result.output)
"""
In the year 2157, Captain Maya Chen piloted her spacecraft through the vast expanse of the Andromeda Galaxy. As she discovered a planet with crystalline mountains that sang in harmony with the cosmic winds, she realized that space exploration was not just about finding new worlds, but about finding new ways to understand the universe and our place within it.
"""

```

In this example, if the OpenAI model fails, the agent will automatically fall back to the Anthropic model with its own configured settings. The `FallbackModel` itself doesn't have settings - it uses the individual settings of whichever model successfully handles the request.

### Exception Handling

The next example demonstrates the exception-handling capabilities of `FallbackModel`. If all models fail, a FallbackExceptionGroup is raised, which contains all the exceptions encountered during the `run` execution.

fallback_model_failure.py

```python
from pydantic_ai import Agent, ModelAPIError
from pydantic_ai.models.anthropic import AnthropicModel
from pydantic_ai.models.fallback import FallbackModel
from pydantic_ai.models.openai import OpenAIChatModel

openai_model = OpenAIChatModel('gpt-5')
anthropic_model = AnthropicModel('claude-sonnet-4-5')
fallback_model = FallbackModel(openai_model, anthropic_model)

agent = Agent(fallback_model)
try:
    response = agent.run_sync('What is the capital of France?')
except* ModelAPIError as exc_group:
    for exc in exc_group.exceptions:
        print(exc)

```

Since [`except*`](https://docs.python.org/3/reference/compound_stmts.html#except-star) is only supported in Python 3.11+, we use the [`exceptiongroup`](https://github.com/agronholm/exceptiongroup) backport package for earlier Python versions:

fallback_model_failure.py

```python
from exceptiongroup import catch

from pydantic_ai import Agent, ModelAPIError
from pydantic_ai.models.anthropic import AnthropicModel
from pydantic_ai.models.fallback import FallbackModel
from pydantic_ai.models.openai import OpenAIChatModel


def model_status_error_handler(exc_group: BaseExceptionGroup) -> None:
    for exc in exc_group.exceptions:
        print(exc)


openai_model = OpenAIChatModel('gpt-5')
anthropic_model = AnthropicModel('claude-sonnet-4-5')
fallback_model = FallbackModel(openai_model, anthropic_model)

agent = Agent(fallback_model)
with catch({ModelAPIError: model_status_error_handler}):
    response = agent.run_sync('What is the capital of France?')

```

By default, the `FallbackModel` only moves on to the next model if the current model raises a ModelAPIError, which includes ModelHTTPError. You can customize this behavior by passing a custom `fallback_on` argument to the `FallbackModel` constructor.

Note

Validation errors (from [structured output](../../output/#structured-output) or [tool parameters](../../tools/)) do **not** trigger fallback. These errors use the [retry mechanism](../../agents/#reflection-and-self-correction) instead, which re-prompts the same model to try again. This is intentional: validation errors stem from the non-deterministic nature of LLMs and may succeed on retry, whereas API errors (4xx/5xx) generally indicate issues that won't resolve by retrying the same request.
