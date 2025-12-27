# Gemini CLI

> Connect Gemini CLI to up-to-date and token-efficient documentation.

[Gemini CLI Docs](https://github.com/google-gemini/gemini-cli)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

Gemini CLI is configured through a `settings.json` file, typically found at `~/.gemini/settings.json`.

To enable Ref, update `settings.json`:

```settings.json  theme={null}
{
  "mcpServers": {
    "ref": {
      "httpUrl": "https://api.ref.tools/mcp?apiKey=<your-api-key>"
    }
  }
}
```

## Verify

Run the command:

```
gemini mcp list
```

You should see:

```
➜  gemini mcp list

✓ ref: https://api.ref.tools/mcp?apiKey=<your-api-key> (http) - Connected
```

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt