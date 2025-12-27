# データオーガナイザー

非構造化テキストをカスタムJSONテーブルに変換します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供された非構造化テキストを取得し、JSONを使用して整理されたテーブル形式に変換することです。テキストで言及されている主要なエンティティ、属性、またはカテゴリを特定し、それらをJSONオブジェクトのキーとして使用してください。次に、テキストから関連情報を抽出し、JSONオブジェクトの対応する値に入力してください。データが正確に表現され、JSON構造内で適切にフォーマットされていることを確認してください。結果として得られるJSONテーブルは、元のテキストで提示された情報の明確で構造化された概要を提供する必要があります。 |
| User   | Silvermist Hollowは魅力的な村で、並外れた個人のグループの故郷でした。その中には、地域医療センターで外科技術に革命をもたらした45歳のYale出身の脳神経外科医、Dr. Liam Patelがいました。28歳のOlivia Chenは、UC Berkeley出身の革新的な建築家で、持続可能で息をのむような美しいデザインで村の景観を変えました。地元の劇場は、72歳のJuilliard出身の音楽家兼作曲家であるEthan Kovacsの魅惑的な交響曲で彩られました。地元産の食材への情熱を持つ独学のシェフであるIsabella Torresは、農場から食卓へのレストランで料理の感動を生み出し、それは食愛好家にとって必見の目的地となりました。これらの卓越した個人たちは、それぞれ独特の才能を持ち、Silvermist Hollowの活気に満ちた生活のタペストリーに貢献しました。 |

### 出力例

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "脳神経外科医",
    "education": "Yale",
    "accomplishments": "地域医療センターで外科技術に革命をもたらした"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "建築家",
    "education": "UC Berkeley",
    "accomplishments": "持続可能で息をのむような美しいデザインで村の景観を変えた"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "音楽家兼作曲家",
    "education": "Juilliard",
    "accomplishments": "魅惑的な交響曲で地元の劇場を彩った"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "シェフ",
    "education": "独学",
    "accomplishments": "農場から食卓へのレストランで料理の感動を生み出し、食愛好家にとって必見の目的地となった"
  }
]
```

---

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=0,
        system="あなたのタスクは、提供された非構造化テキストを取得し、JSONを使用して整理されたテーブル形式に変換することです。テキストで言及されている主要なエンティティ、属性、またはカテゴリを特定し、それらをJSONオブジェクトのキーとして使用してください。次に、テキストから関連情報を抽出し、JSONオブジェクトの対応する値に入力してください。データが正確に表現され、JSON構造内で適切にフォーマットされていることを確認してください。結果として得られるJSONテーブルは、元のテキストで提示された情報の明確で構造化された概要を提供する必要があります。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollowは魅力的な村で、並外れた個人のグループの故郷でした。その中には、地域医療センターで外科技術に革命をもたらした45歳のYale出身の脳神経外科医、Dr. Liam Patelがいました。28歳のOlivia Chenは、UC Berkeley出身の革新的な建築家で、持続可能で息をのむような美しいデザインで村の景観を変えました。地元の劇場は、72歳のJuilliard出身の音楽家兼作曲家であるEthan Kovacsの魅惑的な交響曲で彩られました。地元産の食材への情熱を持つ独学のシェフであるIsabella Torresは、農場から食卓へのレストランで料理の感動を生み出し、それは食愛好家にとって必見の目的地となりました。これらの卓越した個人たちは、それぞれ独特の才能を持ち、Silvermist Hollowの活気に満ちた生活のタペストリーに貢献しました。"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript TypeScript
    import Anthropic from "@anthropic-ai/sdk";
    
    const anthropic = new Anthropic({
      apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 1000,
      temperature: 0,
      system: "あなたのタスクは、提供された非構造化テキストを取得し、JSONを使用して整理されたテーブル形式に変換することです。テキストで言及されている主要なエンティティ、属性、またはカテゴリを特定し、それらをJSONオブジェクトのキーとして使用してください。次に、テキストから関連情報を抽出し、JSONオブジェクトの対応する値に入力してください。データが正確に表現され、JSON構造内で適切にフォーマットされていることを確認してください。結果として得られるJSONテーブルは、元のテキストで提示された情報の明確で構造化された概要を提供する必要があります。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollowは魅力的な村で、並外れた個人のグループの故郷でした。その中には、地域医療センターで外科技術に革命をもたらした45歳のYale出身の脳神経外科医、Dr. Liam Patelがいました。28歳のOlivia Chenは、UC Berkeley出身の革新的な建築家で、持続可能で息をのむような美しいデザインで村の景観を変えました。地元の劇場は、72歳のJuilliard出身の音楽家兼作曲家であるEthan Kovacsの魅惑的な交響曲で彩られました。地元産の食材への情熱を持つ独学のシェフであるIsabella Torresは、農場から食卓へのレストランで料理の感動を生み出し、それは食愛好家にとって必見の目的地となりました。これらの卓越した個人たちは、それぞれ独特の才能を持ち、Silvermist Hollowの活気に満ちた生活のタペストリーに貢献しました。"
            }
          ]
        }
      ]
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
        max_tokens=1000,
        temperature=0,
        system="あなたのタスクは、提供された非構造化テキストを取得し、JSONを使用して整理されたテーブル形式に変換することです。テキストで言及されている主要なエンティティ、属性、またはカテゴリを特定し、それらをJSONオブジェクトのキーとして使用してください。次に、テキストから関連情報を抽出し、JSONオブジェクトの対応する値に入力してください。データが正確に表現され、JSON構造内で適切にフォーマットされていることを確認してください。結果として得られるJSONテーブルは、元のテキストで提示された情報の明確で構造化された概要を提供する必要があります。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollowは魅力的な村で、並外れた個人のグループの故郷でした。その中には、地域医療センターで外科技術に革命をもたらした45歳のYale出身の脳神経外科医、Dr. Liam Patelがいました。28歳のOlivia Chenは、UC Berkeley出身の革新的な建築家で、持続可能で息をのむような美しいデザインで村の景観を変えました。地元の劇場は、72歳のJuilliard出身の音楽家兼作曲家であるEthan Kovacsの魅惑的な交響曲で彩られました。地元産の食材への情熱を持つ独学のシェフであるIsabella Torresは、農場から食卓へのレストランで料理の感動を生み出し、それは食愛好家にとって必見の目的地となりました。これらの卓越した個人たちは、それぞれ独特の才能を持ち、Silvermist Hollowの活気に満ちた生活のタペストリーに貢献しました。"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 0,
      system: "あなたのタスクは、提供された非構造化テキストを取得し、JSONを使用して整理されたテーブル形式に変換することです。テキストで言及されている主要なエンティティ、属性、またはカテゴリを特定し、それらをJSONオブジェクトのキーとして使用してください。次に、テキストから関連情報を抽出し、JSONオブジェクトの対応する値に入力してください。データが正確に表現され、JSON構造内で適切にフォーマットされていることを確認してください。結果として得られるJSONテーブルは、元のテキストで提示された情報の明確で構造化された概要を提供する必要があります。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollowは魅力的な村で、並外れた個人のグループの故郷でした。その中には、地域医療センターで外科技術に革命をもたらした45歳のYale出身の脳神経外科医、Dr. Liam Patelがいました。28歳のOlivia Chenは、UC Berkeley出身の革新的な建築家で、持続可能で息をのむような美しいデザインで村の景観を変えました。地元の劇場は、72歳のJuilliard出身の音楽家兼作曲家であるEthan Kovacsの魅惑的な交響曲で彩られました。地元産の食材への情熱を持つ独学のシェフであるIsabella Torresは、農場から食卓へのレストランで料理の感動を生み出し、それは食愛好家にとって必見の目的地となりました。これらの卓越した個人たちは、それぞれ独特の才能を持ち、Silvermist Hollowの活気に満ちた生活のタペストリーに貢献しました。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python Vertex AI Python
    from anthropic import AnthropicVertex
    
    client = AnthropicVertex()
    
    message = client.messages.create(
        model="claude-sonnet-4@20250514",
        max_tokens=1000,
        temperature=0,
        system="あなたのタスクは、提供された非構造化テキストを取得し、JSONを使用して整理されたテーブル形式に変換することです。テキストで言及されている主要なエンティティ、属性、またはカテゴリを特定し、それらをJSONオブジェクトのキーとして使用してください。次に、テキストから関連情報を抽出し、JSONオブジェクトの対応する値に入力してください。データが正確に表現され、JSON構造内で適切にフォーマットされていることを確認してください。結果として得られるJSONテーブルは、元のテキストで提示された情報の明確で構造化された概要を提供する必要があります。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollowは魅力的な村で、並外れた個人のグループの故郷でした。その中には、地域医療センターで外科技術に革命をもたらした45歳のYale出身の脳神経外科医、Dr. Liam Patelがいました。28歳のOlivia Chenは、UC Berkeley出身の革新的な建築家で、持続可能で息をのむような美しいデザインで村の景観を変えました。地元の劇場は、72歳のJuilliard出身の音楽家兼作曲家であるEthan Kovacsの魅惑的な交響曲で彩られました。地元産の食材への情熱を持つ独学のシェフであるIsabella Torresは、農場から食卓へのレストランで料理の感動を生み出し、それは食愛好家にとって必見の目的地となりました。これらの卓越した個人たちは、それぞれ独特の才能を持ち、Silvermist Hollowの活気に満ちた生活のタペストリーに貢献しました。"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "あなたのタスクは、提供された非構造化テキストを取得し、JSONを使用して整理されたテーブル形式に変換することです。テキストで言及されている主要なエンティティ、属性、またはカテゴリを特定し、それらをJSONオブジェクトのキーとして使用してください。次に、テキストから関連情報を抽出し、JSONオブジェクトの対応する値に入力してください。データが正確に表現され、JSON構造内で適切にフォーマットされていることを確認してください。結果として得られるJSONテーブルは、元のテキストで提示された情報の明確で構造化された概要を提供する必要があります。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollowは魅力的な村で、並外れた個人のグループの故郷でした。その中には、地域医療センターで外科技術に革命をもたらした45歳のYale出身の脳神経外科医、Dr. Liam Patelがいました。28歳のOlivia Chenは、UC Berkeley出身の革新的な建築家で、持続可能で息をのむような美しいデザインで村の景観を変えました。地元の劇場は、72歳のJuilliard出身の音楽家兼作曲家であるEthan Kovacsの魅惑的な交響曲で彩られました。地元産の食材への情熱を持つ独学のシェフであるIsabella Torresは、農場から食卓へのレストランで料理の感動を生み出し、それは食愛好家にとって必見の目的地となりました。これらの卓越した個人たちは、それぞれ独特の才能を持ち、Silvermist Hollowの活気に満ちた生活のタペストリーに貢献しました。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>