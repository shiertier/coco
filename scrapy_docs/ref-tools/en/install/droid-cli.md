# Droid CLI

> Connect Factory Droid CLI to up-to-date and token-efficient documentation.

*Updated Oct 20, 2025*

[Droid CLI MCP install docs](https://docs.factory.ai/cli/configuration/mcp)

You can find your Ref API key at [ref.tools/keys](https://ref.tools/keys)

## Install

Start Droid

```
droid
```

Run the `/mcp` command

```
/mcp add --type http Ref "https://api.ref.tools/mcp?apiKey=<your-api-key>"
```

## Verify

Start Droid:

```
droid
```

Run the `/mcp` command in the Droid TUI

```
/mcp list
```

You should see:

```
●  Configured MCP servers:

   Ref
     Type: http
     URL: https://api.ref.tools/mcp?apiKey=ref-3be3e42726eaebd3b8b7
```

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt