# 우주 키스트로크

사이드 스크롤링 게임플레이와 Tailwind CSS 스타일링을 특징으로 하는 인터랙티브 속도 타이핑 게임을 단일 HTML 파일로 생성합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| User | 완전한 웹 앱을 단일 HTML 파일로 작성해주세요. 이 앱은 WASD를 사용하여 이동하는 간단한 사이드 스크롤링 게임을 포함해야 합니다. 세계를 돌아다니다 보면 가끔 캐릭터/스프라이트가 단어를 만나게 됩니다. 단어를 만나면 플레이어는 가능한 한 빠르게 단어를 정확히 타이핑해야 합니다. 단어를 성공적으로 타이핑할수록 더 많은 점수를 얻습니다. 점수를 추적하기 위해 오른쪽 상단에 카운터가 있어야 합니다. 게임을 흥미롭게 유지하기 위해 단어는 무작위이고 매우 다양해야 합니다.<br/><br/>웹사이트를 매우 미적으로 만들고 Tailwind를 사용해야 합니다. |

## 예시 출력

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <script src="https://cdn.tailwindcss.com"></script>
    <title>Side-Scrolling Typing Game</title>
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
        Submit
      </button>
    </div>
    <script>
      const words = [
        'ability',
        'able',
        'about',
        'above',
        'accept',
        'according',
        'account',
        'across',
        'action',
        'activity',
        'actually',
        'address',
        'administration',
        'admit',
        'adult',
        'affect',
        'after',
        'again',
        'against',
        'agency',
        'agent',
        'ago',
        'agree',
        'agreement',
        'ahead',
        'air',
        'all',
        'allow',
        'almost',
        'alone',
        'along',
        'already',
        'also',
        'although',
        'always',
        'American',
        'among',
        'amount',
        'analysis',
        'and',
        'animal',
        'another',
        'answer',
        'any',
        'anyone',
        'anything',
        'appear',
        'apply',
        'approach',
        'area',
        'argue',
        'arm',
        'around',
        'arrive',
        'art',
        'article',
        'artist',
        'as',
        'ask',
        'assume',
        'at',
        'attack',
        'attention',
        'attorney',
        'audience',
        'author',
        'authority',
        'available',
        'avoid',
        'away',
        'baby',
        'back',
        'bad',
        'bag',
        'ball',
        'bank',
        'bar',
        'base',
        'be',
        'beat',
        'beautiful',
        'because',
        'become',
        'bed',
        'before',
        'begin',
        'behavior',
        'behind',
        'believe',
        'benefit',
        'best',
        'better',
        'between',
        'beyond',
        'big',
        'bill',
        'billion',
        'bit',
        'black',
        'blood',
        'blue',
        'board',
        'body',
        'book',
        'born',
        'both',
        'box',
        'boy',
        'break',
        'bring',
        'brother',
        'budget',
        'build',
        'building',
        'business',
        'but',
        'buy',
        'by',
        'call',
        'camera',
        'campaign',
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

## API 요청

<CodeGroup>

```python Python
import anthropic

client = anthropic.Anthropic(
    # defaults to os.environ.get("ANTHROPIC_API_KEY")
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
                    "text": "Write me a fully complete web app as a single HTML file. The app should contain a simple side-scrolling game where I use WASD to move around.  When moving around the world, occasionally the character/sprite will encounter words. When a word is encountered, the player must correctly type the word as fast as possible.The faster the word is successfully typed, the more point the player gets. We should have a counter in the top-right to keep track of points. Words should be random and highly variable to keep the game interesting.  \n  \nYou should make the website very aesthetic and use Tailwind."
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
  apiKey: 'my_api_key', // defaults to process.env["ANTHROPIC_API_KEY"]
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
          text: 'Write me a fully complete web app as a single HTML file. The app should contain a simple side-scrolling game where I use WASD to move around.  When moving around the world, occasionally the character/sprite will encounter words. When a word is encountered, the player must correctly type the word as fast as possible.The faster the word is successfully typed, the more point the player gets. We should have a counter in the top-right to keep track of points. Words should be random and highly variable to keep the game interesting.  \n  \nYou should make the website very aesthetic and use Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

```python AWS Bedrock Python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
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
                    "text": "Write me a fully complete web app as a single HTML file. The app should contain a simple side-scrolling game where I use WASD to move around.  When moving around the world, occasionally the character/sprite will encounter words. When a word is encountered, the player must correctly type the word as fast as possible.The faster the word is successfully typed, the more point the player gets. We should have a counter in the top-right to keep track of points. Words should be random and highly variable to keep the game interesting.  \n  \nYou should make the website very aesthetic and use Tailwind."
                }
            ]
        }
    ]
)
print(message.content)
```

```typescript AWS Bedrock TypeScript
import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
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
          text: 'Write me a fully complete web app as a single HTML file. The app should contain a simple side-scrolling game where I use WASD to move around.  When moving around the world, occasionally the character/sprite will encounter words. When a word is encountered, the player must correctly type the word as fast as possible.The faster the word is successfully typed, the more point the player gets. We should have a counter in the top-right to keep track of points. Words should be random and highly variable to keep the game interesting.  \n  \nYou should make the website very aesthetic and use Tailwind.',
        },
      ],
    },
  ],
});
console.log(msg);
```

</CodeGroup>