# Frappes Cosmiques

Générez un jeu de frappe de vitesse interactif dans un seul fichier HTML, avec un gameplay à défilement latéral et un style Tailwind CSS.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| Utilisateur | Écrivez-moi une application web complètement fonctionnelle dans un seul fichier HTML. L'application devrait contenir un jeu simple à défilement latéral où j'utilise WASD pour me déplacer. En me déplaçant dans le monde, occasionnellement le personnage/sprite rencontrera des mots. Quand un mot est rencontré, le joueur doit taper correctement le mot aussi vite que possible. Plus le mot est tapé avec succès rapidement, plus le joueur obtient de points. Nous devrions avoir un compteur en haut à droite pour suivre les points. Les mots devraient être aléatoires et très variés pour garder le jeu intéressant.<br/><br/>Vous devriez rendre le site web très esthétique et utiliser Tailwind. |

## Exemple de Sortie

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <script src="https://cdn.tailwindcss.com"></script>
    <title>Jeu de Frappe à Défilement Latéral</title>
  </head>
  <body class="bg-gray-900 text-white">
    <div class="fixed top-4 right-4 text-2xl">
      Score: <span id="score">0</span>
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
        Soumettre
      </button>
    </div>
    <script>
      const words = [
        'capacité',
        'capable',
        'à propos',
        'au-dessus',
        'accepter',
        'selon',
        'compte',
        'à travers',
        'action',
        'activité',
        'réellement',
        'adresse',
        'administration',
        'admettre',
        'adulte',
        'affecter',
        'après',
        'encore',
        'contre',
        'agence',
        'agent',
        'il y a',
        'accepter',
        'accord',
        'devant',
        'air',
        'tout',
        'permettre',
        'presque',
        'seul',
        'le long',
        'déjà',
        'aussi',
        'bien que',
        'toujours',
        'Américain',
        'parmi',
        'montant',
        'analyse',
        'et',
        'animal',
        'autre',
        'réponse',
        'tout',
        'quelqu\'un',
        'quelque chose',
        'apparaître',
        'appliquer',
        'approche',
        'zone',
        'argumenter',
        'bras',
        'autour',
        'arriver',
        'art',
        'article',
        'artiste',
        'comme',
        'demander',
        'supposer',
        'à',
        'attaque',
        'attention',
        'avocat',
        'audience',
        'auteur',
        'autorité',
        'disponible',
        'éviter',
        'loin',
        'bébé',
        'retour',
        'mauvais',
        'sac',
        'balle',
        'banque',
        'bar',
        'base',
        'être',
        'battre',
        'beau',
        'parce que',
        'devenir',
        'lit',
        'avant',
        'commencer',
        'comportement',
        'derrière',
        'croire',
        'bénéfice',
        'meilleur',
        'mieux',
        'entre',
        'au-delà',
        'grand',
        'facture',
        'milliard',
        'peu',
        'noir',
        'sang',
        'bleu',
        'conseil',
        'corps',
        'livre',
        'né',
        'les deux',
        'boîte',
        'garçon',
        'casser',
        'apporter',
        'frère',
        'budget',
        'construire',
        'bâtiment',
        'affaires',
        'mais',
        'acheter',
        'par',
        'appeler',
        'caméra',
        'campagne',
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

## Requête API

<CodeGroup>

```python Python
import anthropic

client = anthropic.Anthropic(
    # par défaut os.environ.get("ANTHROPIC_API_KEY")
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
                    "text": "Écrivez-moi une application web complètement fonctionnelle dans un seul fichier HTML. L'application devrait contenir un jeu simple à défilement latéral où j'utilise WASD pour me déplacer. En me déplaçant dans le monde, occasionnellement le personnage/sprite rencontrera des mots. Quand un mot est rencontré, le joueur doit taper correctement le mot aussi vite que possible. Plus le mot est tapé avec succès rapidement, plus le joueur obtient de points. Nous devrions avoir un compteur en haut à droite pour suivre les points. Les mots devraient être aléatoires et très variés pour garder le jeu intéressant.\n\nVous devriez rendre le site web très esthétique et utiliser Tailwind."
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
  apiKey: 'my_api_key', // par défaut process.env["ANTHROPIC_API_KEY"]
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
          text: 'Écrivez-moi une application web complètement fonctionnelle dans un seul fichier HTML. L\'application devrait contenir un jeu simple à défilement latéral où j\'utilise WASD pour me déplacer. En me déplaçant dans le monde, occasionnellement le personnage/sprite rencontrera des mots. Quand un mot est rencontré, le joueur doit taper correctement le mot aussi vite que possible. Plus le mot est tapé avec succès rapidement, plus le joueur obtient de points. Nous devrions avoir un compteur en haut à droite pour suivre les points. Les mots devraient être aléatoires et très variés pour garder le jeu intéressant.\n\nVous devriez rendre le site web très esthétique et utiliser Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

```python AWS Bedrock Python
from anthropic import AnthropicBedrock

# Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# pour les options d'authentification
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
                    "text": "Écrivez-moi une application web complètement fonctionnelle dans un seul fichier HTML. L'application devrait contenir un jeu simple à défilement latéral où j'utilise WASD pour me déplacer. En me déplaçant dans le monde, occasionnellement le personnage/sprite rencontrera des mots. Quand un mot est rencontré, le joueur doit taper correctement le mot aussi vite que possible. Plus le mot est tapé avec succès rapidement, plus le joueur obtient de points. Nous devrions avoir un compteur en haut à droite pour suivre les points. Les mots devraient être aléatoires et très variés pour garder le jeu intéressant.\n\nVous devriez rendre le site web très esthétique et utiliser Tailwind."
                }
            ]
        }
    ]
)
print(message.content)
```

```typescript AWS Bedrock TypeScript
import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// pour les options d'authentification
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
          text: 'Écrivez-moi une application web complètement fonctionnelle dans un seul fichier HTML. L\'application devrait contenir un jeu simple à défilement latéral où j\'utilise WASD pour me déplacer. En me déplaçant dans le monde, occasionnellement le personnage/sprite rencontrera des mots. Quand un mot est rencontré, le joueur doit taper correctement le mot aussi vite que possible. Plus le mot est tapé avec succès rapidement, plus le joueur obtient de points. Nous devrions avoir un compteur en haut à droite pour suivre les points. Les mots devraient être aléatoires et très variés pour garder le jeu intéressant.\n\nVous devriez rendre le site web très esthétique et utiliser Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

</CodeGroup>