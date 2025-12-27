# Versões

Ao fazer solicitações de API, você deve enviar um cabeçalho de solicitação `anthropic-version`. Por exemplo, `anthropic-version: 2023-06-01`. Se você estiver usando nossos [SDKs de cliente](/docs/pt-BR/api/client-sdks), isso é tratado automaticamente para você.

---

Para qualquer versão de API específica, preservaremos:

* Parâmetros de entrada existentes
* Parâmetros de saída existentes

No entanto, podemos fazer o seguinte:

* Adicionar entradas opcionais adicionais
* Adicionar valores adicionais à saída
* Alterar condições para tipos de erro específicos
* Adicionar novas variantes a valores de saída semelhantes a enum (por exemplo, tipos de eventos de streaming)

Geralmente, se você estiver usando a API conforme documentado nesta referência, não quebraremos seu uso.

## Histórico de versões

Sempre recomendamos usar a versão mais recente da API sempre que possível. Versões anteriores são consideradas obsoletas e podem estar indisponíveis para novos usuários.

* `2023-06-01`  
   * Novo formato para eventos server-sent (SSE) de [streaming](/docs/pt-BR/api/streaming):  
         * Completions são incrementais. Por exemplo, `" Olá"`, `" meu"`, `" nome"`, `" é"`, `" Claude." ` em vez de `" Olá"`, `" Olá meu"`, `" Olá meu nome"`, `" Olá meu nome é"`, `" Olá meu nome é Claude."`.  
         * Todos os eventos são [eventos nomeados](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents), em vez de [eventos apenas de dados](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages).  
         * Removido evento desnecessário `data: [DONE]`.  
   * Removidos valores legados `exception` e `truncated` nas respostas.
* `2023-01-01`: Lançamento inicial.