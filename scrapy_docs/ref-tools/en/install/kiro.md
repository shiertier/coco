# Kiro

> Connect Kiro to up-to-date and token-efficient documentation.

[Kiro MCP configuration docs](https://kiro.dev/docs/mcp/configuration/)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

Option 1: Click this link.

<a href="kiro://kiro.mcp/add?name=Ref&config=%7B%22url%22%3A%22https%3A%2F%2Fapi.ref.tools%2Fmcp%22%2C%22headers%22%3A%7B%22x-ref-api-key%22%3A%22%3CYOUR_API_KEY%3E%22%7D%2C%22autoApprove%22%3A%5B%22ref_search_documentation%22%2C%22ref_read_url%22%5D%7D">
  <img noZoom src="https://kiro.dev/images/add-to-kiro.svg" alt="Add Ref to Kiro" />
</a>

Make sure you update `disabled: false` and fill in your API key.

Option 2: Manual install.

1. Open the Kiro left side panel via the Kiro ghost icon.
2. At the bottom, see the `MCP SERVERS` sections and mouse over it to see the `Open MCP Config` button. This will open `mcp.json`
3. Update `mcp.json` as follows:

```json  theme={null}
{
  "mcpServers": {
    "Ref": {
      "url": "https://api.ref.tools/mcp",
      "headers": {
        "x-ref-api-key": "<YOUR_API_KEY>"
      },
      "disabled": false,
      "autoApprove": ["ref_search_documentation", "ref_read_url"]
    }
  }
}
```

## Verify

1. Open the MCP Server view in the Kiro feature panel

2. You should see `Ref` listed with a green status indicator

3. Start a new chat and issue the following prompt:

```
what is ref tools mcp, search the docs with ref
```

This should result in a `ref_search_documentation` tool call being executed.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt