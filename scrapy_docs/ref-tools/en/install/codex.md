# Codex CLI

> Connect Codex CLI to up-to-date and token-efficient documentation.

*Updated Oct 14, 2025*

[Codex MCP install docs](https://github.com/openai/codex/blob/main/docs/config.md#connecting-to-mcp-servers)

You can find your Ref API key at [ref.tools/keys](https://ref.tools/keys)

## Install

Codex is configured in a `config.toml` file. This typically lives at `~/.codex/config.toml`

To enable Ref update your `config.toml` to include:

```config.toml  theme={null}
[mcp_servers.ref]
url = "https://api.ref.tools/mcp?apiKey=<your-api-key>"
```

If you are on an older version of Codex and you get the error `Error loading configuration: missing field 'command' in 'mcp_servers.ref'` and do not want to update, you can install the `stdio` server:

```config.toml  theme={null}
[mcp_servers.ref]
command = "npx"
args = ["-y", "ref-tools-mcp@latest"]
env = { "REF_API_KEY" = "<your-api-key>" }
```

## Verify

Run the command:

```
codex mcp list
```

You should see:

```
➜  codex mcp list
Name  Url                                                        Bearer Token Env Var  Status   Auth
ref   https://api.ref.tools/mcp?apiKey=<your-api-key>.           -                     enabled  Unsupporte
```

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt