# Cabeçalhos beta

Documentação para usar cabeçalhos beta com a API Claude

---

Os cabeçalhos beta permitem que você acesse recursos experimentais e novas capacidades de modelo antes que se tornem parte da API padrão.

Esses recursos estão sujeitos a alterações e podem ser modificados ou removidos em versões futuras.

<Info>
Os cabeçalhos beta são frequentemente usados em conjunto com o [namespace beta nos SDKs do cliente](/docs/pt-BR/api/client-sdks#beta-namespace-in-client-sdks)
</Info>

## Como usar cabeçalhos beta

Para acessar recursos beta, inclua o cabeçalho `anthropic-beta` em suas solicitações de API:

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

Ao usar o SDK, você pode especificar cabeçalhos beta nas opções de solicitação:

<CodeGroup>

```python Python
from anthropic import Anthropic

client = Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "Hello, Claude"}
    ],
    betas=["beta-feature-name"]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const msg = await anthropic.beta.messages.create({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  messages: [
    { role: 'user', content: 'Hello, Claude' }
  ],
  betas: ['beta-feature-name']
});
```

```bash cURL
curl https://api.anthropic.com/v1/messages \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: beta-feature-name" \
  -H "content-type: application/json" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello, Claude"}
    ]
  }'
```

</CodeGroup>

<Warning>
Os recursos beta são experimentais e podem:
- Ter alterações disruptivas sem aviso
- Ser descontinuados ou removidos
- Ter limites de taxa ou preços diferentes
- Não estar disponíveis em todas as regiões
</Warning>

### Múltiplos recursos beta

Para usar múltiplos recursos beta em uma única solicitação, inclua todos os nomes de recursos no cabeçalho separados por vírgulas:

```http
anthropic-beta: feature1,feature2,feature3
```

### Convenções de nomenclatura de versão

Os nomes de recursos beta geralmente seguem o padrão: `feature-name-YYYY-MM-DD`, onde a data indica quando a versão beta foi lançada. Sempre use o nome exato do recurso beta conforme documentado.

## Tratamento de erros

Se você usar um cabeçalho beta inválido ou indisponível, receberá uma resposta de erro:

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## Obtendo ajuda

Para perguntas sobre recursos beta:

1. Verifique a documentação para o recurso específico
2. Revise o [changelog da API](/docs/pt-BR/api/versioning) para atualizações
3. Entre em contato com o suporte para assistência com uso em produção

Lembre-se de que os recursos beta são fornecidos "como estão" e podem não ter as mesmas garantias de SLA que os recursos estáveis da API.