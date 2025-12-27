# Babel的廣播

用世界上10種最常用語言創建引人注目的產品發布推文。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|      | 內容 |
| ---- | --- |
| User | 用10種最常用的語言為我寫一系列產品發布推文。該產品是一款全新的最先進雙筒望遠鏡，內建AI系統來識別通過望遠鏡觀察到的動物。推文應該令人興奮、前沿，並激發消費者興趣。 |
| User | 一個人類可以將意識上傳到數位領域的未來。 |

### 範例輸出

> 以下是用10種最常用語言為一款內建AI系統識別動物的全新最先進雙筒望遠鏡撰寫的產品發布推文系列：
>
> English:  
> Introducing the future of bird/wildlife watching! Our new AI binoculars use cutting-edge vision tech to instantly identify any animal you see. Bring the great outdoors to life like never before! #NatureTech #AIBinoculars
>
> Mandarin Chinese:  
> 令人兴奋的新品上市!我们的 AI 双筒望远镜融合了尖端视觉技术,可即时识别你所见的任何动物。让大自然的奇迹在你眼前展现,体验前所未有的观赏体验!#智能科技 #AIBinoculars
>
> Hindi:  
> बर्ड/वन्यजीव वॉचिंग के लिए भविष्य का अविष्कार! हमारी नई एआई दूरबीनें अत्याधुनिक दृष्टि प्रौद्योगिकी का उपयोग करती हैं, जो आपको दिखाई देने वाले किसी भी जानवर की पहचान करने में सक्षम हैं। पहले कभी न देखी प्रकृति के अद्भुत दृश्यों का अनुभव करें! #NatureWonders #AIBinoculars
>
> Spanish:  
> ¡Presentamos el futuro de la observación de aves/vida silvestre! Nuestros nuevos binoculares con IA utilizan técnicas avanzadas de visión para identificar instantáneamente cualquier animal que veas. ¡Experimenta la naturaleza como nunca antes! #TecnologíaNatural #PrismáticosIA
>
> Arabic:  
> نقدم لكم مستقبل مراقبة الطيور/الحياة البرية! توفر نظارات المجهر الجديدة لدينا المزودة بالذكاء الاصطناعي تقنيات رؤية متطورة لتحديد هوية أي حيوان تراه على الفور. شاهد عجائب الطبيعة كما لم تفعل من قبل! #تكنولوجيا*الطبيعة #منظار*ذكي
>
> Bengali:  
> পাখি/বন্য প্রাণী পর্যবেক্ষণের ভবিষ্যতের উন্মোচন! আমাদের নতুন এআই দুর্বীনগুলি অত্যাধুনিক ভিশন প্রযুক্তি ব্যবহার করে যেকোনো প্রাণীকে দ্রুত চিহ্নিত করতে সক্ষম। প্রকৃতির অপরূপ দৃশ্য আগেকার চেয়ে ভিন্নভাবে উপভোগ করুন! #প্রাকৃতিকপ্রযুক্তি #AIBinoculars
>
> Russian:  
> Представляем будущее наблюдения за птицами/дикой природой! Наши новые бинокли с ИИ используют передовые технологии компьютерного зрения для мгновенной идентификации любого животного. Откройте для себя природу по-новому! #ПриродныеТехнологии #БинокльСИИ
>
> Portuguese:  
> Apresentamos o futuro da observação de aves/vida selvagem! Os nossos novos binóculos com IA usam técnicas avançadas de visão para identificar instantaneamente qualquer animal que veja. Experiencie a natureza como nunca antes! #TecnologiaNatural #BinóculosIA
>
> Japanese:  
> 鳥/野生動物観察の未来をご紹介!新しい AI バイノキュラーは、最先端のビジョン技術を使用して、あなたが見たあらゆる動物をすばやく特定します。自然をこれまでにない方法で体験してください!#ネイチャーテック #AI バイノキュラー
>
> Punjabi:  
> ਪੰਛੀਆਂ/ਜੰਗਲੀ ਜੀਵਨ ਦੇਖਣ ਦੇ ਭਵਿੱਖ ਦਾ ਐਲਾਨ! ਸਾਡੀਆਂ ਨਵੀਆਂ ਏਆਈ ਦੂਰਬੀਨਾਂ ਅਤਿ ਆਧੁਨਿਕ ਦ੍ਰਿਸ਼ਟੀ ਤਕਨਾਲੋਜੀ ਦੀ ਵਰਤੋਂ ਕਰਦੀਆਂ ਹਨ ਜੋ ਤੁਸੀਂ ਦੇਖਦੇ ਹੋ ਕਿਸੇ ਵੀ ਜਾਨਵਰ ਦੀ ਸਿੱਧੀ ਪਛਾਣ ਕਰਨ ਦੇ ਸਮਰੱਥ ਹਨ। ਪ੍ਰਕਿਰਤੀ ਦਾ ਅਨੁਭਵ ਪਹਿਲਾਂ ਨਾਲੋਂ ਵੱਖਰੇ ਢੰਗ ਨਾਲ ਕਰੋ! #NeighborhoodTech #AIBinoculars

---

### API請求

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
                        "text": "用10種最常用的語言為我寫一系列產品發布推文。該產品是一款全新的最先進雙筒望遠鏡，內建AI系統來識別通過望遠鏡觀察到的動物。推文應該令人興奮、前沿，並激發消費者興趣。"
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
      max_tokens: 2000,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "用10種最常用的語言為我寫一系列產品發布推文。該產品是一款全新的最先進雙筒望遠鏡，內建AI系統來識別通過望遠鏡觀察到的動物。推文應該令人興奮、前沿，並激發消費者興趣。"
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
        max_tokens=2000,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "用10種最常用的語言為我寫一系列產品發布推文。該產品是一款全新的最先進雙筒望遠鏡，內建AI系統來識別通過望遠鏡觀察到的動物。推文應該令人興奮、前沿，並激發消費者興趣。"
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
      max_tokens: 2000,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "用10種最常用的語言為我寫一系列產品發布推文。該產品是一款全新的最先進雙筒望遠鏡，內建AI系統來識別通過望遠鏡觀察到的動物。推文應該令人興奮、前沿，並激發消費者興趣。"
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
        max_tokens=2000,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "用10種最常用的語言為我寫一系列產品發布推文。該產品是一款全新的最先進雙筒望遠鏡，內建AI系統來識別通過望遠鏡觀察到的動物。推文應該令人興奮、前沿，並激發消費者興趣。"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 2000,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "用10種最常用的語言為我寫一系列產品發布推文。該產品是一款全新的最先進雙筒望遠鏡，內建AI系統來識別通過望遠鏡觀察到的動物。推文應該令人興奮、前沿，並激發消費者興趣。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>