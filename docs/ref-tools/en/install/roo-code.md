# Roo Code

> Connect Roo Code to up-to-date and token-efficient documentation.

[Roo Code MCP docs](https://docs.roocode.com/features/mcp/using-mcp-in-roo)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Click the `...` icon in the top right to expand additional menu items and selecte `MCP Servers`

2. Click either `Edit Global MCP` or `Edit Project MCP`.

3. Update the json file that is opened to match the following.

```
{
  "mcpServers": {
    "Ref": {
      "command": "npx",
      "args": ["ref-tools-mcp@latest"],
      "env": {
        "REF_API_KEY": "<YOUR_API_KEY>"
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