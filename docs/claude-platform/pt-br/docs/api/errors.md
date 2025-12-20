# Erros

---

## Erros HTTP

Nossa API segue um formato previsível de código de erro HTTP:

* 400 - `invalid_request_error`: Houve um problema com o formato ou conteúdo da sua solicitação. Também podemos usar este tipo de erro para outros códigos de status 4XX não listados abaixo.
* 401 - `authentication_error`: Há um problema com sua chave de API.
* 403 - `permission_error`: Sua chave de API não tem permissão para usar o recurso especificado.
* 404 - `not_found_error`: O recurso solicitado não foi encontrado.
* 413 - `request_too_large`: A solicitação excede o número máximo permitido de bytes. O tamanho máximo da solicitação é 32 MB para endpoints de API padrão.
* 429 - `rate_limit_error`: Sua conta atingiu um limite de taxa.
* 500 - `api_error`: Ocorreu um erro inesperado interno aos sistemas da Anthropic.
* 529 - `overloaded_error`: A API está temporariamente sobrecarregada.

  <Warning>
  Erros 529 podem ocorrer quando as APIs experimentam alto tráfego em todos os usuários.
  
  Em casos raros, se sua organização tiver um aumento acentuado no uso, você pode ver erros 429 devido aos limites de aceleração na API. Para evitar atingir os limites de aceleração, aumente seu tráfego gradualmente e mantenha padrões de uso consistentes.
  </Warning>

Ao receber uma resposta de [streaming](/docs/pt-BR/build-with-claude/streaming) via SSE, é possível que um erro possa ocorrer após retornar uma resposta 200, caso em que o tratamento de erros não seguiria esses mecanismos padrão.

## Limites de tamanho de solicitação

A API impõe limites de tamanho de solicitação para garantir desempenho ideal:

| Tipo de Endpoint | Tamanho Máximo da Solicitação |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/pt-BR/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/pt-BR/build-with-claude/files) | 500 MB |

Se você exceder esses limites, receberá um erro 413 `request_too_large`. O erro é retornado do Cloudflare antes que a solicitação chegue aos nossos servidores de API.

## Formatos de erro

Os erros são sempre retornados como JSON, com um objeto `error` de nível superior que sempre inclui um valor `type` e `message`. A resposta também inclui um campo `request_id` para facilitar o rastreamento e depuração. Por exemplo:

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

De acordo com nossa política de [versionamento](/docs/pt-BR/api/versioning), podemos expandir os valores dentro desses objetos, e é possível que os valores `type` cresçam ao longo do tempo.

## ID da solicitação

Toda resposta da API inclui um cabeçalho único `request-id`. Este cabeçalho contém um valor como `req_018EeWyXxfu5pfWkrYcMdjWG`. Ao entrar em contato com o suporte sobre uma solicitação específica, inclua este ID para nos ajudar a resolver rapidamente seu problema.

Nossos SDKs oficiais fornecem este valor como uma propriedade em objetos de resposta de nível superior, contendo o valor do cabeçalho `request-id`:

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## Solicitações longas

<Warning>
 Recomendamos fortemente o uso da [streaming Messages API](/docs/pt-BR/build-with-claude/streaming) ou [Message Batches API](/docs/pt-BR/api/creating-message-batches) para solicitações de longa duração, especialmente aquelas com mais de 10 minutos.
</Warning>

Não recomendamos definir valores grandes de `max_tokens` sem usar nossa [streaming Messages API](/docs/pt-BR/build-with-claude/streaming)
ou [Message Batches API](/docs/pt-BR/api/creating-message-batches):

- Algumas redes podem descartar conexões inativas após um período variável de tempo, o que
pode fazer com que a solicitação falhe ou expire sem receber uma resposta da Anthropic.
- As redes diferem em confiabilidade; nossa [Message Batches API](/docs/pt-BR/api/creating-message-batches) pode ajudá-lo a
gerenciar o risco de problemas de rede permitindo que você consulte os resultados em vez de exigir uma conexão de rede ininterrupta.

Se você está construindo uma integração direta da API, deve estar ciente de que definir um [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) pode reduzir o impacto de timeouts de conexão inativa em algumas redes.

Nossos [SDKs](/docs/pt-BR/api/client-sdks) validarão que suas solicitações da Messages API não-streaming não devem exceder um timeout de 10 minutos e
também definirão uma opção de socket para TCP keep-alive.