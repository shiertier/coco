# Compatibilité du SDK OpenAI

Anthropic fournit une couche de compatibilité qui vous permet d'utiliser le SDK OpenAI pour tester l'API Claude. Avec quelques modifications de code, vous pouvez rapidement évaluer les capacités des modèles Anthropic.

---

<Note>
Cette couche de compatibilité est principalement destinée à tester et comparer les capacités des modèles, et n'est pas considérée comme une solution à long terme ou prête pour la production pour la plupart des cas d'usage. Bien que nous ayons l'intention de la maintenir entièrement fonctionnelle et de ne pas apporter de modifications qui cassent la compatibilité, notre priorité est la fiabilité et l'efficacité de l'[API Claude](/docs/fr/api/overview).

Pour plus d'informations sur les limitations de compatibilité connues, consultez [Limitations importantes de la compatibilité OpenAI](#important-openai-compatibility-limitations).

Si vous rencontrez des problèmes avec la fonctionnalité de compatibilité du SDK OpenAI, veuillez nous le faire savoir [ici](https://forms.gle/oQV4McQNiuuNbz9n8).
</Note>

<Tip>
Pour la meilleure expérience et l'accès à l'ensemble complet des fonctionnalités de l'API Claude ([traitement PDF](/docs/fr/build-with-claude/pdf-support), [citations](/docs/fr/build-with-claude/citations), [réflexion étendue](/docs/fr/build-with-claude/extended-thinking), et [mise en cache des invites](/docs/fr/build-with-claude/prompt-caching)), nous recommandons d'utiliser l'[API Claude](/docs/fr/api/overview) native.
</Tip>

## Démarrage avec le SDK OpenAI

Pour utiliser la fonctionnalité de compatibilité du SDK OpenAI, vous devez :

1. Utiliser un SDK OpenAI officiel
2. Modifier les éléments suivants
   * Mettez à jour votre URL de base pour pointer vers l'API Claude
   * Remplacez votre clé API par une [clé API Claude](/settings/keys)
   * Mettez à jour le nom de votre modèle pour utiliser un [modèle Claude](/docs/fr/about-claude/models/overview)
3. Consultez la documentation ci-dessous pour connaître les fonctionnalités prises en charge

### Exemple de démarrage rapide

<CodeGroup>
    ```python Python
    from openai import OpenAI

    client = OpenAI(
        api_key="ANTHROPIC_API_KEY",  # Your Claude API key
        base_url="https://api.anthropic.com/v1/"  # the Claude API endpoint
    )

    response = client.chat.completions.create(
        model="claude-sonnet-4-5", # Anthropic model name
        messages=[
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Who are you?"}
        ],
    )

    print(response.choices[0].message.content)
    ```
    
    ```typescript TypeScript
    import OpenAI from 'openai';

    const openai = new OpenAI({
        apiKey: "ANTHROPIC_API_KEY",   // Your Claude API key
        baseURL: "https://api.anthropic.com/v1/",  // Claude API endpoint
    });

    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5", // Claude model name
    });

    console.log(response.choices[0].message.content);
    ```
</CodeGroup>

## Limitations importantes de la compatibilité OpenAI

#### Comportement de l'API

Voici les différences les plus substantielles par rapport à l'utilisation d'OpenAI :

* Le paramètre `strict` pour l'appel de fonction est ignoré, ce qui signifie que le JSON d'utilisation d'outils n'est pas garanti de suivre le schéma fourni. Pour une conformité de schéma garantie, utilisez l'[API Claude native avec Structured Outputs](/docs/fr/build-with-claude/structured-outputs).
* L'entrée audio n'est pas prise en charge ; elle sera simplement ignorée et supprimée de l'entrée
* La mise en cache des invites n'est pas prise en charge, mais elle est prise en charge dans le [SDK Anthropic](/docs/fr/api/client-sdks)
* Les messages système/développeur sont remontés et concaténés au début de la conversation, car Anthropic ne prend en charge qu'un seul message système initial.

La plupart des champs non pris en charge sont silencieusement ignorés plutôt que de produire des erreurs. Tous ces éléments sont documentés ci-dessous.

#### Considérations relatives à la qualité de la sortie

Si vous avez beaucoup ajusté votre invite, elle est probablement bien adaptée à OpenAI spécifiquement. Envisagez d'utiliser notre [améliorateur d'invite dans la console Claude](/dashboard) comme point de départ.

#### Remontée des messages système / Développeur

La plupart des entrées du SDK OpenAI correspondent clairement directement aux paramètres de l'API Anthropic, mais une différence distincte est la gestion des invites système / développeur. Ces deux invites peuvent être placées tout au long d'une conversation de chat via OpenAI. Puisque Anthropic ne prend en charge qu'un message système initial, nous prenons tous les messages système/développeur et les concaténons ensemble avec une seule nouvelle ligne (`\n`) entre eux. Cette chaîne complète est ensuite fournie comme un seul message système au début des messages.

#### Support de la réflexion étendue

Vous pouvez activer les capacités de [réflexion étendue](/docs/fr/build-with-claude/extended-thinking) en ajoutant le paramètre `thinking`. Bien que cela améliorera le raisonnement de Claude pour les tâches complexes, le SDK OpenAI ne retournera pas le processus de réflexion détaillé de Claude. Pour les fonctionnalités complètes de réflexion étendue, y compris l'accès à la sortie de raisonnement étape par étape de Claude, utilisez l'API Claude native.

<CodeGroup>
    ```python Python
    response = client.chat.completions.create(
        model="claude-sonnet-4-5",
        messages=...,
        extra_body={
            "thinking": { "type": "enabled", "budget_tokens": 2000 }
        }
    )
    ```
    
    ```typescript TypeScript
    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5",
        // @ts-expect-error
        thinking: { type: "enabled", budget_tokens: 2000 }
    });

    ```
</CodeGroup>

## Limites de débit

Les limites de débit suivent les [limites standard](/docs/fr/api/rate-limits) d'Anthropic pour le point de terminaison `/v1/messages`.

## Support détaillé de l'API compatible OpenAI
### Champs de requête
#### Champs simples
| Champ | Statut de support |
|--------|----------------|
| `model` | Utiliser les noms de modèles Claude |
| `max_tokens` | Entièrement pris en charge |
| `max_completion_tokens` | Entièrement pris en charge |
| `stream` | Entièrement pris en charge |
| `stream_options` | Entièrement pris en charge |
| `top_p` | Entièrement pris en charge |
| `parallel_tool_calls` | Entièrement pris en charge |
| `stop` | Toutes les séquences d'arrêt non-blanches fonctionnent |
| `temperature` | Entre 0 et 1 (inclus). Les valeurs supérieures à 1 sont plafonnées à 1. |
| `n` | Doit être exactement 1 |
| `logprobs` | Ignoré |
| `metadata` | Ignoré |
| `response_format` | Ignoré. Pour la sortie JSON, utilisez [Structured Outputs](/docs/fr/build-with-claude/structured-outputs) avec l'API Claude native |
| `prediction` | Ignoré |
| `presence_penalty` | Ignoré |
| `frequency_penalty` | Ignoré |
| `seed` | Ignoré |
| `service_tier` | Ignoré |
| `audio` | Ignoré |
| `logit_bias` | Ignoré |
| `store` | Ignoré |
| `user` | Ignoré |
| `modalities` | Ignoré |
| `top_logprobs` | Ignoré |
| `reasoning_effort` | Ignoré |

#### Champs `tools` / `functions`
<section title="Afficher les champs">

<Tabs>
<Tab title="Tools">
Champs `tools[n].function`
| Champ        | Statut de support         |
|--------------|-----------------|
| `name`       | Entièrement pris en charge |
| `description`| Entièrement pris en charge |
| `parameters` | Entièrement pris en charge |
| `strict`     | Ignoré. Utilisez [Structured Outputs](/docs/fr/build-with-claude/structured-outputs) avec l'API Claude native pour la validation stricte du schéma |
</Tab>
<Tab title="Functions">

Champs `functions[n]`
<Info>
OpenAI a déprécié le champ `functions` et suggère d'utiliser `tools` à la place.
</Info>
| Champ        | Statut de support         |
|--------------|-----------------|
| `name`       | Entièrement pris en charge |
| `description`| Entièrement pris en charge |
| `parameters` | Entièrement pris en charge |
| `strict`     | Ignoré. Utilisez [Structured Outputs](/docs/fr/build-with-claude/structured-outputs) avec l'API Claude native pour la validation stricte du schéma |
</Tab>
</Tabs>

</section>

#### Champs du tableau `messages`
<section title="Afficher les champs">

<Tabs>
<Tab title="Rôle développeur">
Champs pour `messages[n].role == "developer"`
<Info>
Les messages développeur sont remontés au début de la conversation dans le cadre du message système initial
</Info>
| Champ | Statut de support |
|-------|---------|
| `content` | Entièrement pris en charge, mais remonté |
| `name` | Ignoré |

</Tab>
<Tab title="Rôle système">
Champs pour `messages[n].role == "system"`

<Info>
Les messages système sont remontés au début de la conversation dans le cadre du message système initial
</Info>
| Champ | Statut de support |
|-------|---------|
| `content` | Entièrement pris en charge, mais remonté |
| `name` | Ignoré |

</Tab>
<Tab title="Rôle utilisateur">
Champs pour `messages[n].role == "user"`

| Champ | Variante | Sous-champ | Statut de support |
|-------|---------|-----------|----------------|
| `content` | `string` | | Entièrement pris en charge |
| | `array`, `type == "text"` | | Entièrement pris en charge |
| | `array`, `type == "image_url"` | `url` | Entièrement pris en charge |
| | | `detail` | Ignoré |
| | `array`, `type == "input_audio"` | | Ignoré |
| | `array`, `type == "file"` | | Ignoré |
| `name` | | | Ignoré |

</Tab>

<Tab title="Rôle assistant">
Champs pour `messages[n].role == "assistant"`
| Champ | Variante | Statut de support |
|-------|---------|----------------|
| `content` | `string` | Entièrement pris en charge |
| | `array`, `type == "text"` | Entièrement pris en charge |
| | `array`, `type == "refusal"` | Ignoré |
| `tool_calls` | | Entièrement pris en charge |
| `function_call` | | Entièrement pris en charge |
| `audio` | | Ignoré |
| `refusal` | | Ignoré |

</Tab>

<Tab title="Rôle outil">
Champs pour `messages[n].role == "tool"`
| Champ | Variante | Statut de support |
|-------|---------|----------------|
| `content` | `string` | Entièrement pris en charge |
| | `array`, `type == "text"` | Entièrement pris en charge |
| `tool_call_id` | | Entièrement pris en charge |
| `tool_choice` | | Entièrement pris en charge |
| `name` | | Ignoré |
</Tab>

<Tab title="Rôle fonction">
Champs pour `messages[n].role == "function"`
| Champ | Variante | Statut de support |
|-------|---------|----------------|
| `content` | `string` | Entièrement pris en charge |
| | `array`, `type == "text"` | Entièrement pris en charge |
| `tool_choice` | | Entièrement pris en charge |
| `name` | | Ignoré |
</Tab>
</Tabs>

</section>

### Champs de réponse

| Champ | Statut de support |
|---------------------------|----------------|
| `id` | Entièrement pris en charge |
| `choices[]` | Aura toujours une longueur de 1 |
| `choices[].finish_reason` | Entièrement pris en charge |
| `choices[].index` | Entièrement pris en charge |
| `choices[].message.role` | Entièrement pris en charge |
| `choices[].message.content` | Entièrement pris en charge |
| `choices[].message.tool_calls` | Entièrement pris en charge |
| `object` | Entièrement pris en charge |
| `created` | Entièrement pris en charge |
| `model` | Entièrement pris en charge |
| `finish_reason` | Entièrement pris en charge |
| `content` | Entièrement pris en charge |
| `usage.completion_tokens` | Entièrement pris en charge |
| `usage.prompt_tokens` | Entièrement pris en charge |
| `usage.total_tokens` | Entièrement pris en charge |
| `usage.completion_tokens_details` | Toujours vide |
| `usage.prompt_tokens_details` | Toujours vide |
| `choices[].message.refusal` | Toujours vide |
| `choices[].message.audio` | Toujours vide |
| `logprobs` | Toujours vide |
| `service_tier` | Toujours vide |
| `system_fingerprint` | Toujours vide |

### Compatibilité des messages d'erreur

La couche de compatibilité maintient des formats d'erreur cohérents avec l'API OpenAI. Cependant, les messages d'erreur détaillés ne seront pas équivalents. Nous recommandons d'utiliser uniquement les messages d'erreur pour la journalisation et le débogage.

### Compatibilité des en-têtes

Bien que le SDK OpenAI gère automatiquement les en-têtes, voici la liste complète des en-têtes pris en charge par l'API Claude pour les développeurs qui ont besoin de les utiliser directement.

| En-tête | Statut de support |
|---------|----------------|
| `x-ratelimit-limit-requests` | Entièrement pris en charge |
| `x-ratelimit-limit-tokens` | Entièrement pris en charge |
| `x-ratelimit-remaining-requests` | Entièrement pris en charge |
| `x-ratelimit-remaining-tokens` | Entièrement pris en charge |
| `x-ratelimit-reset-requests` | Entièrement pris en charge |
| `x-ratelimit-reset-tokens` | Entièrement pris en charge |
| `retry-after` | Entièrement pris en charge |
| `request-id` | Entièrement pris en charge |
| `openai-version` | Toujours `2020-10-01` |
| `authorization` | Entièrement pris en charge |
| `openai-processing-ms` | Toujours vide |