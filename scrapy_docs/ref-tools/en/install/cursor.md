# Cursor

> Connect Cursor to up-to-date and token-efficient documentation.

[Cursor MCP docs](https://cursor.com/docs/context/mcp)

You can find your Ref API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

Installing in Cursor is as easy as 1-click. Visit [ref.tools/install]()

You can also manually install via Cursor's settings.

1. Open `Cursor Settings`
2. Click `Tools & MCP`
3. Click `New MCP Server`. This will open an `mcp.json` file.
4. Update `mcp.json`

```mcp.json  theme={null}
{
  "mcpServers": {
    "Ref": {
      "type": "http",
      "url": "https://api.ref.tools/mcp",
      "headers": {
        "x-ref-api-key": "your-api-key"
      }
    }
  }
}
```

## Verify

In `Cursor Settings` click `Tools & MCP`.

You should see Ref with a green dot. It should say `2 tools, 2 prompts enabled`.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt