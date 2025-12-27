# Goose

> Connect Goose to up-to-date and token-efficient documentation.

[Goose Extensions docs](https://block.github.io/goose/docs/getting-started/using-extensions)

You can find your API key at [ref.tools/keys](https://ref.tools/keys)

## Install Steps

1. Click the `Extensions` button in the left side nav.

2. Click `Add custom extension`

3. Fill in Extension Name as `Ref`

4. Fill in Endpoint as `https://api.ref.tools/mcp`

5. Add a Request Header `x-ref-api-key` with value being your API key.

6. IMPORTANT: make sure you hit "+ Add" next to the request header to ensure it is actually added.

7. Click `Add extension`

<img src="https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-settings.png?fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=1d6e11ee54ac1ec522378add7c970be2" data-og-width="1464" width="1464" data-og-height="2098" height="2098" data-path="install/images/goose-settings.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-settings.png?w=280&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=9ba677e664de72ad9bad9dc0c324f4a0 280w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-settings.png?w=560&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=de667f44f859923465fa93d4e9279b84 560w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-settings.png?w=840&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=ccb6084d6af78f00ebede56eeec94a05 840w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-settings.png?w=1100&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=d5bd131df19cc65056ead5127b55a017 1100w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-settings.png?w=1650&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=c6cdd77ca34d869d88ec19866a14a9db 1650w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-settings.png?w=2500&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=b7448569aed512896959d350a90da4f0 2500w" />

## Verify

Open a new chat and issue the prompt.

```
what is ref tools mcp, search the docs with ref
```

This should results in a `ref_search_documentation` tool call.

<img src="https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-success.png?fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=5a3473ad38b19992bc7573ed52432024" data-og-width="1430" width="1430" data-og-height="316" height="316" data-path="install/images/goose-success.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-success.png?w=280&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=4e49c47341ca21335549ef5a0ebbdabc 280w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-success.png?w=560&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=4777befb144b0a42bed331dc9957b19f 560w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-success.png?w=840&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=20e719d5e10a770de70e2795403e146a 840w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-success.png?w=1100&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=b9acfd11811d13f7a09dbc8f7b385815 1100w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-success.png?w=1650&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=465ae2f962e47d628cdf81c2ba54315a 1650w, https://mintcdn.com/ref/ceuHC_M68gmI3-kZ/install/images/goose-success.png?w=2500&fit=max&auto=format&n=ceuHC_M68gmI3-kZ&q=85&s=27aaa3290236da33319cecfbed35cf07 2500w" />

## Get help

If you have any issues, please reach out to `help@ref.tools`. We're happy to help you get started!


---

> To find navigation and other pages in this documentation, fetch the llms.txt file at: https://docs.ref.tools/llms.txt