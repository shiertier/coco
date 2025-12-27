# Windsurf

> Connect Windsurf to up-to-date and token-efficient documentation.

[Windsurf Cascade MCP docs](https://docs.windsurf.com/windsurf/cascade/mcp)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Open the `mcp_config.json` file. This can be found at a path like `~/.codium/windsurf/mcp_config.json` or by opening Cascade Chat and clicking the `Customizations` icon (looks like a packing box) in the top right then clicking the settings button.

2. Update `mcp_config.json` as follows:

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

Click the Cascade Chat Customizations button and see that Ref is active with 2 tools.

<img src="https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/windsurf-success.png?fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=4d36867a902c37f0d68c052245153a2a" data-og-width="566" width="566" data-og-height="400" height="400" data-path="install/images/windsurf-success.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/windsurf-success.png?w=280&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=8ce4c00cfe903271426369fe6235108d 280w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/windsurf-success.png?w=560&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=5f0e3cb6e54d24ac4f95485945434f63 560w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/windsurf-success.png?w=840&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=71692d838569ad41e80c673cb166bc99 840w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/windsurf-success.png?w=1100&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=162c386877c2a292080b72f117f03492 1100w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/windsurf-success.png?w=1650&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=77f9b5f2bff765c9ab47140b0b3f9e43 1650w, https://mintcdn.com/ref/gYjO0XIdfTw7sBNf/install/images/windsurf-success.png?w=2500&fit=max&auto=format&n=gYjO0XIdfTw7sBNf&q=85&s=95e2d03727a54186bc8e4c26bccf3f22 2500w" />

If it is not, you can click on `Ref` and click the `Refresh` button on the Manage MCPs page to refresh MCPs.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt