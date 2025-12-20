# 関数ファブリケーター

詳細な仕様に基づいてPython関数を作成します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供された自然言語のリクエストに基づいてPython関数を作成することです。リクエストは、入力パラメータと期待される戻り値を含む、関数の望ましい機能を記述します。与えられた仕様に従って関数を実装し、エッジケースを処理し、必要な検証を実行し、Pythonプログラミングのベストプラクティスに従うことを確認してください。実装を理解するために他の開発者を支援し、ロジックを説明するために、コードに適切なコメントを含めてください。 |
| User   | 数独パズルを解くことができる関数が欲しいです。この関数は9x9の数独グリッドを入力として受け取り、空のセルは値0で表されます。関数はバックトラッキングアルゴリズムを使用してパズルを解き、解決されたグリッドを返すべきです。パズルが解けない場合は、Noneを返すべきです。関数は、有効な数独パズルであることを確認するために入力グリッドを検証する必要もあります。 |

## 出力例

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # 現在の行と列で数字が有効かどうかをチェック
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # 現在の3x3サブグリッドで数字が有効かどうかをチェック
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # すべてのセルを埋めた場合、パズルは解決済み
       if row == 9:
           return True

       # 現在の行の終わりに達した場合、次の行に移動
       if col == 9:
           return backtrack(row + 1, 0)

       # 現在のセルがすでに埋まっている場合、次のセルに移動
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # 現在のセルを1から9の数字で埋めることを試行
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # 有効な数字が見つからない場合、バックトラック
       return False

   # 入力グリッドを検証
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # バックトラッキングアルゴリズムを開始
   if backtrack(0, 0):
       return grid
   else:
       return None

```

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
    system="あなたのタスクは、提供された自然言語のリクエストに基づいてPython関数を作成することです。リクエストは、入力パラメータと期待される戻り値を含む、関数の望ましい機能を記述します。与えられた仕様に従って関数を実装し、エッジケースを処理し、必要な検証を実行し、Pythonプログラミングのベストプラクティスに従うことを確認してください。実装を理解するために他の開発者を支援し、ロジックを説明するために、コードに適切なコメントを含めてください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "数独パズルを解くことができる関数が欲しいです。この関数は9x9の数独グリッドを入力として受け取り、空のセルは値0で表されます。関数はバックトラッキングアルゴリズムを使用してパズルを解き、解決されたグリッドを返すべきです。パズルが解けない場合は、Noneを返すべきです。関数は、有効な数独パズルであることを確認するために入力グリッドを検証する必要もあります。",
                }
            ],
        }
    ],
)
print(message.content)


```
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
  system: "あなたのタスクは、提供された自然言語のリクエストに基づいてPython関数を作成することです。リクエストは、入力パラメータと期待される戻り値を含む、関数の望ましい機能を記述します。与えられた仕様に従って関数を実装し、エッジケースを処理し、必要な検証を実行し、Pythonプログラミングのベストプラクティスに従うことを確認してください。実装を理解するために他の開発者を支援し、ロジックを説明するために、コードに適切なコメントを含めてください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "数独パズルを解くことができる関数が欲しいです。この関数は9x9の数独グリッドを入力として受け取り、空のセルは値0で表されます。関数はバックトラッキングアルゴリズムを使用してパズルを解き、解決されたグリッドを返すべきです。パズルが解けない場合は、Noneを返すべきです。関数は、有効な数独パズルであることを確認するために入力グリッドを検証する必要もあります。"
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
system="あなたのタスクは、提供された自然言語のリクエストに基づいてPython関数を作成することです。リクエストは、入力パラメータと期待される戻り値を含む、関数の望ましい機能を記述します。与えられた仕様に従って関数を実装し、エッジケースを処理し、必要な検証を実行し、Pythonプログラミングのベストプラクティスに従うことを確認してください。実装を理解するために他の開発者を支援し、ロジックを説明するために、コードに適切なコメントを含めてください。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "数独パズルを解くことができる関数が欲しいです。この関数は9x9の数独グリッドを入力として受け取り、空のセルは値0で表されます。関数はバックトラッキングアルゴリズムを使用してパズルを解き、解決されたグリッドを返すべきです。パズルが解けない場合は、Noneを返すべきです。関数は、有効な数独パズルであることを確認するために入力グリッドを検証する必要もあります。"
}
]
}
]
)
print(message.content)

````
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
  system: "あなたのタスクは、提供された自然言語のリクエストに基づいてPython関数を作成することです。リクエストは、入力パラメータと期待される戻り値を含む、関数の望ましい機能を記述します。与えられた仕様に従って関数を実装し、エッジケースを処理し、必要な検証を実行し、Pythonプログラミングのベストプラクティスに従うことを確認してください。実装を理解するために他の開発者を支援し、ロジックを説明するために、コードに適切なコメントを含めてください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "数独パズルを解くことができる関数が欲しいです。この関数は9x9の数独グリッドを入力として受け取り、空のセルは値0で表されます。関数はバックトラッキングアルゴリズムを使用してパズルを解き、解決されたグリッドを返すべきです。パズルが解けない場合は、Noneを返すべきです。関数は、有効な数独パズルであることを確認するために入力グリッドを検証する必要もあります。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="あなたのタスクは、提供された自然言語のリクエストに基づいてPython関数を作成することです。リクエストは、入力パラメータと期待される戻り値を含む、関数の望ましい機能を記述します。与えられた仕様に従って関数を実装し、エッジケースを処理し、必要な検証を実行し、Pythonプログラミングのベストプラクティスに従うことを確認してください。実装を理解するために他の開発者を支援し、ロジックを説明するために、コードに適切なコメントを含めてください。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "数独パズルを解くことができる関数が欲しいです。この関数は9x9の数独グリッドを入力として受け取り、空のセルは値0で表されます。関数はバックトラッキングアルゴリズムを使用してパズルを解き、解決されたグリッドを返すべきです。パズルが解けない場合は、Noneを返すべきです。関数は、有効な数独パズルであることを確認するために入力グリッドを検証する必要もあります。"
}
]
}
]
)
print(message.content)

````
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
  system: "あなたのタスクは、提供された自然言語のリクエストに基づいてPython関数を作成することです。リクエストは、入力パラメータと期待される戻り値を含む、関数の望ましい機能を記述します。与えられた仕様に従って関数を実装し、エッジケースを処理し、必要な検証を実行し、Pythonプログラミングのベストプラクティスに従うことを確認してください。実装を理解するために他の開発者を支援し、ロジックを説明するために、コードに適切なコメントを含めてください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "数独パズルを解くことができる関数が欲しいです。この関数は9x9の数独グリッドを入力として受け取り、空のセルは値0で表されます。関数はバックトラッキングアルゴリズムを使用してパズルを解き、解決されたグリッドを返すべきです。パズルが解けない場合は、Noneを返すべきです。関数は、有効な数独パズルであることを確認するために入力グリッドを検証する必要もあります。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>