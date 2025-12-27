# OpenCode

> Connect OpenCode to up-to-date and token-efficient documentation.

[Opencode MCP docs](https://opencode.ai/docs/mcp-servers/)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Create an `opencode.jsonc` file in your project that matches the following:

```
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "Ref": {
      "type": "remote",
      "url": "https://api.ref.tools/mcp",
      "enabled": true,
      "headers": {
        "x-ref-api-key": "YOUR_API_KEY"
      }
    }
  }
}
```

## Verify

Open a new chat and issue the prompt.

```
what is ref tools mcp, search the docs with ref
```

This should results in a `ref_search_documentation` tool call.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt