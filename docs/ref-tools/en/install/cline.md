# Cline

> Connect Cline to up-to-date and token-efficient documentation.

[Cline MCP docs](https://docs.cline.bot/mcp/configuring-mcp-servers)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Open Cline plugin in VSCode and click the `MCP Servers` button at the top.

<img src="https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/cline-mcp-servers-button.png?fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=492c1898895815488bada0e8ddba0531" data-og-width="896" width="896" data-og-height="142" height="142" data-path="install/images/cline-mcp-servers-button.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/cline-mcp-servers-button.png?w=280&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=a2f9b8d93ce1f9a393304005df0743f2 280w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/cline-mcp-servers-button.png?w=560&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=bb04e92955130167defb152abab0ee7d 560w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/cline-mcp-servers-button.png?w=840&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=84c44ace73bae6e0e21ae6d7947b6ee4 840w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/cline-mcp-servers-button.png?w=1100&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=7cf3d7a64cfc6a959e69bea462706718 1100w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/cline-mcp-servers-button.png?w=1650&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=948eb33fa3e6f973b463b67a3d91d521 1650w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/cline-mcp-servers-button.png?w=2500&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=7aa2d5c9b6a7bbe934f81ae18a7cbd83 2500w" />

2. Select the `Configure` tab and click `Configure MCP Servers`. This should open `cline_mcp_settings.json`.

3. Update `cline_mcp_settings.json`

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

Open a new chat with Cline and run the following prompt.

```
what is ref tools mcp, search the docs with ref
```

This should results in a `ref_search_documentation` tool call.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt