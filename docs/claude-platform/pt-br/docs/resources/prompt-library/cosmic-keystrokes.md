# Teclas Cósmicas

Gere um jogo interativo de digitação rápida em um único arquivo HTML, apresentando jogabilidade de rolagem lateral e estilização Tailwind CSS.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Usuário | Escreva-me um aplicativo web totalmente completo como um único arquivo HTML. O aplicativo deve conter um jogo simples de rolagem lateral onde uso WASD para me mover. Ao se mover pelo mundo, ocasionalmente o personagem/sprite encontrará palavras. Quando uma palavra é encontrada, o jogador deve digitar corretamente a palavra o mais rápido possível. Quanto mais rápido a palavra for digitada com sucesso, mais pontos o jogador ganha. Devemos ter um contador no canto superior direito para acompanhar os pontos. As palavras devem ser aleatórias e altamente variáveis para manter o jogo interessante.<br/><br/>Você deve tornar o site muito estético e usar Tailwind. |

## Exemplo de Saída

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <script src="https://cdn.tailwindcss.com"></script>
    <title>Jogo de Digitação com Rolagem Lateral</title>
  </head>
  <body class="bg-gray-900 text-white">
    <div class="fixed top-4 right-4 text-2xl">
      Pontuação: <span id="score">0</span>
    </div>
    <div id="game" class="h-screen w-screen overflow-hidden relative">
      <div
        id="player"
        class="absolute bottom-10 left-10 w-8 h-8 bg-blue-500"
      ></div>
    </div>
    <div
      id="word-input"
      class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-white text-black p-4 rounded hidden"
    >
      <input
        type="text"
        id="typed-word"
        class="border border-gray-300 rounded px-2 py-1 mb-2 w-full"
      />
      <button id="submit-word" class="bg-blue-500 text-white px-4 py-2 rounded">
        Enviar
      </button>
    </div>
    <script>
      const words = [
        'habilidade',
        'capaz',
        'sobre',
        'acima',
        'aceitar',
        'segundo',
        'conta',
        'através',
        'ação',
        'atividade',
        'realmente',
        'endereço',
        'administração',
        'admitir',
        'adulto',
        'afetar',
        'depois',
        'novamente',
        'contra',
        'agência',
        'agente',
        'atrás',
        'concordar',
        'acordo',
        'adiante',
        'ar',
        'todos',
        'permitir',
        'quase',
        'sozinho',
        'junto',
        'já',
        'também',
        'embora',
        'sempre',
        'americano',
        'entre',
        'quantidade',
        'análise',
        'e',
        'animal',
        'outro',
        'resposta',
        'qualquer',
        'alguém',
        'qualquer coisa',
        'aparecer',
        'aplicar',
        'abordagem',
        'área',
        'argumentar',
        'braço',
        'ao redor',
        'chegar',
        'arte',
        'artigo',
        'artista',
        'como',
        'perguntar',
        'assumir',
        'em',
        'atacar',
        'atenção',
        'advogado',
        'audiência',
        'autor',
        'autoridade',
        'disponível',
        'evitar',
        'longe',
        'bebê',
        'voltar',
        'ruim',
        'bolsa',
        'bola',
        'banco',
        'bar',
        'base',
        'ser',
        'bater',
        'bonito',
        'porque',
        'tornar-se',
        'cama',
        'antes',
        'começar',
        'comportamento',
        'atrás',
        'acreditar',
        'benefício',
        'melhor',
        'melhor',
        'entre',
        'além',
        'grande',
        'conta',
        'bilhão',
        'pouco',
        'preto',
        'sangue',
        'azul',
        'conselho',
        'corpo',
        'livro',
        'nascido',
        'ambos',
        'caixa',
        'menino',
        'quebrar',
        'trazer',
        'irmão',
        'orçamento',
        'construir',
        'edifício',
        'negócio',
        'mas',
        'comprar',
        'por',
        'chamar',
        'câmera',
        'campanha',
      ];

      let score = 0;
      let currentWord;
      let startTime;

      const game = document.getElementById('game');
      const player = document.getElementById('player');
      const wordInput = document.getElementById('word-input');
      const typedWord = document.getElementById('typed-word');
      const submitWord = document.getElementById('submit-word');
      const scoreDisplay = document.getElementById('score');

      let playerX = 10;
      let playerY = game.offsetHeight - 50;
      const speed = 5;

      function placeWord() {
        const word = document.createElement('div');
        word.textContent = words[Math.floor(Math.random() * words.length)];
        word.className = 'absolute text-2xl';
        word.style.left = `${game.offsetWidth + 10}px`;
        word.style.top = `${Math.random() * (game.offsetHeight - 50)}px`;
        game.appendChild(word);
        return word;
      }

      function checkCollision(word) {
        const playerRect = player.getBoundingClientRect();
        const wordRect = word.getBoundingClientRect();
        return !(
          playerRect.right < wordRect.left ||
          playerRect.left > wordRect.right ||
          playerRect.bottom < wordRect.top ||
          playerRect.top > wordRect.bottom
        );
      }

      function startInput(word) {
        currentWord = word.textContent;
        wordInput.style.display = 'block';
        typedWord.value = '';
        typedWord.focus();
        startTime = Date.now();
      }

      function endInput() {
        wordInput.style.display = 'none';
        const endTime = Date.now();
        const elapsedTime = endTime - startTime;
        const timeBonus = Math.max(2000 - elapsedTime, 0);
        score += Math.round(1000 + timeBonus);
        scoreDisplay.textContent = score;
      }

      function checkInput() {
        if (typedWord.value === currentWord) {
          endInput();
        }
      }

      function gameLoop() {
        playerY = Math.max(
          0,
          Math.min(playerY, game.offsetHeight - player.offsetHeight)
        );
        playerX = Math.max(
          0,
          Math.min(playerX, game.offsetWidth - player.offsetWidth)
        );
        player.style.top = `${playerY}px`;
        player.style.left = `${playerX}px`;

        const words = Array.from(
          document.querySelectorAll('#game > div:not(#player)')
        );
        words.forEach((word) => {
          const currentLeft = parseInt(word.style.left);
          word.style.left = `${currentLeft - speed}px`;
          if (checkCollision(word)) {
            startInput(word);
          }
          if (currentLeft < -word.offsetWidth) {
            word.remove();
          }
        });

        if (Math.random() < 0.01) {
          placeWord();
        }

        requestAnimationFrame(gameLoop);
      }

      document.addEventListener('keydown', (e) => {
        if (e.key === 'w') playerY -= speed;
        if (e.key === 'a') playerX -= speed;
        if (e.key === 's') playerY += speed;
        if (e.key === 'd') playerX += speed;
      });

      typedWord.addEventListener('input', checkInput);
      submitWord.addEventListener('click', checkInput);

      gameLoop();
    </script>
  </body>
</html>
```

## Solicitação da API

<CodeGroup>

```python Python
import anthropic

client = anthropic.Anthropic(
    # padrão para os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Escreva-me um aplicativo web totalmente completo como um único arquivo HTML. O aplicativo deve conter um jogo simples de rolagem lateral onde uso WASD para me mover. Ao se mover pelo mundo, ocasionalmente o personagem/sprite encontrará palavras. Quando uma palavra é encontrada, o jogador deve digitar corretamente a palavra o mais rápido possível. Quanto mais rápido a palavra for digitada com sucesso, mais pontos o jogador ganha. Devemos ter um contador no canto superior direito para acompanhar os pontos. As palavras devem ser aleatórias e altamente variáveis para manter o jogo interessante.\n\nVocê deve tornar o site muito estético e usar Tailwind."
                }
            ]
        }
    ]
)
print(message.content)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
  apiKey: 'my_api_key', // padrão para process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: 'claude-sonnet-4-5',
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      role: 'user',
      content: [
        {
          type: 'text',
          text: 'Escreva-me um aplicativo web totalmente completo como um único arquivo HTML. O aplicativo deve conter um jogo simples de rolagem lateral onde uso WASD para me mover. Ao se mover pelo mundo, ocasionalmente o personagem/sprite encontrará palavras. Quando uma palavra é encontrada, o jogador deve digitar corretamente a palavra o mais rápido possível. Quanto mais rápido a palavra for digitada com sucesso, mais pontos o jogador ganha. Devemos ter um contador no canto superior direito para acompanhar os pontos. As palavras devem ser aleatórias e altamente variáveis para manter o jogo interessante.\n\nVocê deve tornar o site muito estético e usar Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

```python AWS Bedrock Python
from anthropic import AnthropicBedrock

# Veja https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# para opções de autenticação
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Escreva-me um aplicativo web totalmente completo como um único arquivo HTML. O aplicativo deve conter um jogo simples de rolagem lateral onde uso WASD para me mover. Ao se mover pelo mundo, ocasionalmente o personagem/sprite encontrará palavras. Quando uma palavra é encontrada, o jogador deve digitar corretamente a palavra o mais rápido possível. Quanto mais rápido a palavra for digitada com sucesso, mais pontos o jogador ganha. Devemos ter um contador no canto superior direito para acompanhar os pontos. As palavras devem ser aleatórias e altamente variáveis para manter o jogo interessante.\n\nVocê deve tornar o site muito estético e usar Tailwind."
                }
            ]
        }
    ]
)
print(message.content)
```

```typescript AWS Bedrock TypeScript
import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// Veja https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// para opções de autenticação
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: 'anthropic.claude-sonnet-4-5-20250929-v1:0',
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      role: 'user',
      content: [
        {
          type: 'text',
          text: 'Escreva-me um aplicativo web totalmente completo como um único arquivo HTML. O aplicativo deve conter um jogo simples de rolagem lateral onde uso WASD para me mover. Ao se mover pelo mundo, ocasionalmente o personagem/sprite encontrará palavras. Quando uma palavra é encontrada, o jogador deve digitar corretamente a palavra o mais rápido possível. Quanto mais rápido a palavra for digitada com sucesso, mais pontos o jogador ganha. Devemos ter um contador no canto superior direito para acompanhar os pontos. As palavras devem ser aleatórias e altamente variáveis para manter o jogo interessante.\n\nVocê deve tornar o site muito estético e usar Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

</CodeGroup>