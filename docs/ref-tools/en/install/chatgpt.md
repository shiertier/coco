# ChatGPT

> Connect ChatGPT to up-to-date and token-efficient documentation.

[ChatGPT MCP docs](https://platform.openai.com/docs/mcp)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Open `Settings`

2. Click `Apps & Connectors`

3. If you are not already in developer mode, scroll to the bottom and click `Advanced settings` and toggle on Developer mode.

4. Click the `Create` button in the top right of the settings modal.

5. Fill in Name as `Ref`

6. Fill in MCP Server URL `https://api.ref.tools/mcp?apiKey=<YOUR_API_KEY>`

7. Under Authentication select `No authentication`. Your API key is set in the server url already.

8. Check the `I understand` box and click Create.

## Verify

1. To use Ref, click the `+` button in the left of the prompt input, go to `More` and select `Ref`.

<img src="https://mintcdn.com/ref/doz1VvgRxLUvu6fi/install/images/chatgpt-use.png?fit=max&auto=format&n=doz1VvgRxLUvu6fi&q=85&s=dbf2c1a0408b7cf3436aae9444785283" data-og-width="1706" width="1706" data-og-height="1364" height="1364" data-path="install/images/chatgpt-use.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/ref/doz1VvgRxLUvu6fi/install/images/chatgpt-use.png?w=280&fit=max&auto=format&n=doz1VvgRxLUvu6fi&q=85&s=73d0f0729b206dedc98f76c39c6b3dd2 280w, https://mintcdn.com/ref/doz1VvgRxLUvu6fi/install/images/chatgpt-use.png?w=560&fit=max&auto=format&n=doz1VvgRxLUvu6fi&q=85&s=bd9325ddc3fd8d3dccd55e934d052c3f 560w, https://mintcdn.com/ref/doz1VvgRxLUvu6fi/install/images/chatgpt-use.png?w=840&fit=max&auto=format&n=doz1VvgRxLUvu6fi&q=85&s=c2c5fea0fe2904d37cbb6289de78438a 840w, https://mintcdn.com/ref/doz1VvgRxLUvu6fi/install/images/chatgpt-use.png?w=1100&fit=max&auto=format&n=doz1VvgRxLUvu6fi&q=85&s=53d7ac965cc0f47a7304de57b481693a 1100w, https://mintcdn.com/ref/doz1VvgRxLUvu6fi/install/images/chatgpt-use.png?w=1650&fit=max&auto=format&n=doz1VvgRxLUvu6fi&q=85&s=8cef1cd1740378996538ad27d901dd47 1650w, https://mintcdn.com/ref/doz1VvgRxLUvu6fi/install/images/chatgpt-use.png?w=2500&fit=max&auto=format&n=doz1VvgRxLUvu6fi&q=85&s=f882aede963b9d527cfa16a7a36a65f7 2500w" />

2. Issue the following prompt.

```
what is ref tools mcp, search the docs with ref
```

This should results in a `ref_search_documentation` tool call.

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt