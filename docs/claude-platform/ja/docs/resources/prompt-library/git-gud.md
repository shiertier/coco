# Git gud

ユーザーが説明したバージョン管理アクションに基づいて適切なGitコマンドを生成します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

| Role | Content |
| ---- | --- |
| User | ローカルファイルにいくつかの変更を加えました。これらをローカルのGitリポジトリに保存したいと思います。どのGitコマンドを使用すべきでしょうか？ |

## 出力例

ローカルファイルに加えた変更をローカルのGitリポジトリに保存するには、以下のGitコマンドを使用する必要があります：

1.  `git add .` または `git add <file>`

    - このコマンドは、あなたが行った変更をステージングし、コミットの準備をします。`.`は現在のディレクトリとそのサブディレクトリ内のすべての変更を追加します。特定のファイルのみをステージングしたい場合は、`.`をファイル名またはパスに置き換えてください。

2.  `git commit -m "Your commit message"`

    - このコマンドは、ステージングされた変更をローカルのGitリポジトリにコミットします。`-m`フラグを使用すると、行った変更を説明するコミットメッセージを提供できます。`"Your commit message"`を変更の簡潔で説明的な要約に置き換えてください。

例えば、`example.txt`という名前のファイルに変更を加えた場合、以下を実行します：

```
git add example.txt
git commit -m "Update example.txt with new content"

```

これらのコマンドは、提供されたメッセージで新しいコミットを作成し、ローカルのGitリポジトリに変更を保存します。後でこれらの変更をリモートリポジトリにプッシュしたり、ローカルブランチでの作業を続けることができます。

ターミナルまたはコマンドプロンプトで、Gitリポジトリディレクトリ内からこれらのコマンドを実行することを忘れないでください。

---

## APIリクエスト

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?",
                }
            ],
        }
    ],
)
print(message.content)


````
</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=1000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>