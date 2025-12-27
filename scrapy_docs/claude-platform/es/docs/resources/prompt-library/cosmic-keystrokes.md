# Teclas Cósmicas

Genera un juego interactivo de mecanografía rápida en un solo archivo HTML, con jugabilidad de desplazamiento lateral y estilo Tailwind CSS.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Usuario | Escríbeme una aplicación web completamente funcional como un solo archivo HTML. La aplicación debe contener un juego simple de desplazamiento lateral donde uso WASD para moverme. Cuando me muevo por el mundo, ocasionalmente el personaje/sprite encontrará palabras. Cuando se encuentra una palabra, el jugador debe escribir correctamente la palabra lo más rápido posible. Mientras más rápido se escriba exitosamente la palabra, más puntos obtiene el jugador. Deberíamos tener un contador en la esquina superior derecha para llevar el registro de los puntos. Las palabras deben ser aleatorias y muy variables para mantener el juego interesante.<br/><br/>Debes hacer el sitio web muy estético y usar Tailwind. |

## Salida de Ejemplo

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <script src="https://cdn.tailwindcss.com"></script>
    <title>Juego de Mecanografía de Desplazamiento Lateral</title>
  </head>
  <body class="bg-gray-900 text-white">
    <div class="fixed top-4 right-4 text-2xl">
      Puntuación: <span id="score">0</span>
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
        'habilidad',
        'capaz',
        'acerca',
        'arriba',
        'aceptar',
        'según',
        'cuenta',
        'través',
        'acción',
        'actividad',
        'realmente',
        'dirección',
        'administración',
        'admitir',
        'adulto',
        'afectar',
        'después',
        'otra vez',
        'contra',
        'agencia',
        'agente',
        'hace',
        'acordar',
        'acuerdo',
        'adelante',
        'aire',
        'todo',
        'permitir',
        'casi',
        'solo',
        'junto',
        'ya',
        'también',
        'aunque',
        'siempre',
        'americano',
        'entre',
        'cantidad',
        'análisis',
        'y',
        'animal',
        'otro',
        'respuesta',
        'cualquier',
        'alguien',
        'algo',
        'aparecer',
        'aplicar',
        'enfoque',
        'área',
        'argumentar',
        'brazo',
        'alrededor',
        'llegar',
        'arte',
        'artículo',
        'artista',
        'como',
        'preguntar',
        'asumir',
        'en',
        'atacar',
        'atención',
        'abogado',
        'audiencia',
        'autor',
        'autoridad',
        'disponible',
        'evitar',
        'lejos',
        'bebé',
        'atrás',
        'malo',
        'bolsa',
        'pelota',
        'banco',
        'bar',
        'base',
        'ser',
        'golpear',
        'hermoso',
        'porque',
        'convertirse',
        'cama',
        'antes',
        'comenzar',
        'comportamiento',
        'detrás',
        'creer',
        'beneficio',
        'mejor',
        'mejor',
        'entre',
        'más allá',
        'grande',
        'factura',
        'billón',
        'poco',
        'negro',
        'sangre',
        'azul',
        'tablero',
        'cuerpo',
        'libro',
        'nacido',
        'ambos',
        'caja',
        'niño',
        'romper',
        'traer',
        'hermano',
        'presupuesto',
        'construir',
        'edificio',
        'negocio',
        'pero',
        'comprar',
        'por',
        'llamar',
        'cámara',
        'campaña',
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

## Solicitud de API

<CodeGroup>

```python Python
import anthropic

client = anthropic.Anthropic(
    # por defecto es os.environ.get("ANTHROPIC_API_KEY")
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
                    "text": "Escríbeme una aplicación web completamente funcional como un solo archivo HTML. La aplicación debe contener un juego simple de desplazamiento lateral donde uso WASD para moverme. Cuando me muevo por el mundo, ocasionalmente el personaje/sprite encontrará palabras. Cuando se encuentra una palabra, el jugador debe escribir correctamente la palabra lo más rápido posible. Mientras más rápido se escriba exitosamente la palabra, más puntos obtiene el jugador. Deberíamos tener un contador en la esquina superior derecha para llevar el registro de los puntos. Las palabras deben ser aleatorias y muy variables para mantener el juego interesante.  \n  \nDebes hacer el sitio web muy estético y usar Tailwind."
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
  apiKey: 'my_api_key', // por defecto es process.env["ANTHROPIC_API_KEY"]
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
          text: 'Escríbeme una aplicación web completamente funcional como un solo archivo HTML. La aplicación debe contener un juego simple de desplazamiento lateral donde uso WASD para moverme. Cuando me muevo por el mundo, ocasionalmente el personaje/sprite encontrará palabras. Cuando se encuentra una palabra, el jugador debe escribir correctamente la palabra lo más rápido posible. Mientras más rápido se escriba exitosamente la palabra, más puntos obtiene el jugador. Deberíamos tener un contador en la esquina superior derecha para llevar el registro de los puntos. Las palabras deben ser aleatorias y muy variables para mantener el juego interesante.  \n  \nDebes hacer el sitio web muy estético y usar Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

```python AWS Bedrock Python
from anthropic import AnthropicBedrock

# Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# para opciones de autenticación
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
                    "text": "Escríbeme una aplicación web completamente funcional como un solo archivo HTML. La aplicación debe contener un juego simple de desplazamiento lateral donde uso WASD para moverme. Cuando me muevo por el mundo, ocasionalmente el personaje/sprite encontrará palabras. Cuando se encuentra una palabra, el jugador debe escribir correctamente la palabra lo más rápido posible. Mientras más rápido se escriba exitosamente la palabra, más puntos obtiene el jugador. Deberíamos tener un contador en la esquina superior derecha para llevar el registro de los puntos. Las palabras deben ser aleatorias y muy variables para mantener el juego interesante.  \n  \nDebes hacer el sitio web muy estético y usar Tailwind."
                }
            ]
        }
    ]
)
print(message.content)
```

```typescript AWS Bedrock TypeScript
import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// para opciones de autenticación
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
          text: 'Escríbeme una aplicación web completamente funcional como un solo archivo HTML. La aplicación debe contener un juego simple de desplazamiento lateral donde uso WASD para moverme. Cuando me muevo por el mundo, ocasionalmente el personaje/sprite encontrará palabras. Cuando se encuentra una palabra, el jugador debe escribir correctamente la palabra lo más rápido posible. Mientras más rápido se escriba exitosamente la palabra, más puntos obtiene el jugador. Deberíamos tener un contador en la esquina superior derecha para llevar el registro de los puntos. Las palabras deben ser aleatorias y muy variables para mantener el juego interesante.  \n  \nDebes hacer el sitio web muy estético y usar Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

</CodeGroup>