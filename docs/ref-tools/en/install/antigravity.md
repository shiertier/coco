# Antigravity

> Connect Antigravity to up-to-date and token-efficient documentation.

[Antigravity MCP docs](https://antigravity.google/docs/mcp)

You can find your Ref API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Open Agent Chat.
2. Click the `...` Additional options button and select `MCP Servers`
3. Click `Manage MCP Servers`. This should open an editor tab called `Managed MCPS`
4. Update `mcp_config.json`

```mcp_config.json  theme={null}
{
  "mcpServers": {
    "ref": {
      "serverUrl": "https://api.ref.tools/mcp",
      "headers": {
        "x-ref-api-key": "<your-api-key>"
      }
    }
  }
}
```

## Verify

Go back to the `Manage MCP Servers` page and hit `Refresh`

You should see Ref with a green dot. It should say `2 tools, 2 prompts enabled`.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt