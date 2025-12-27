# Les diffusions de Babel

Créez des tweets d'annonce de produit convaincants dans les 10 langues les plus parlées au monde.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|      | Contenu |
| ---- | --- |
| Utilisateur | Écrivez-moi une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées. Le produit est une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux vus à travers les jumelles. Les tweets doivent être passionnants, à la pointe de la technologie et susciter l'intérêt des consommateurs. |
| Utilisateur | Un futur où les humains peuvent télécharger leur conscience dans un royaume numérique. |

### Exemple de sortie

> Voici une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées pour une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux :
>
> Anglais :  
> Introducing the future of bird/wildlife watching! Our new AI binoculars use cutting-edge vision tech to instantly identify any animal you see. Bring the great outdoors to life like never before! #NatureTech #AIBinoculars
>
> Chinois mandarin :  
> 令人兴奋的新品上市!我们的 AI 双筒望远镜融合了尖端视觉技术,可即时识别你所见的任何动物。让大自然的奇迹在你眼前展现,体验前所未有的观赏体验!#智能科技 #AIBinoculars
>
> Hindi :  
> बर्ड/वन्यजीव वॉचिंग के लिए भविष्य का अविष्कार! हमारी नई एआई दूरबीनें अत्याधुनिक दृष्टि प्रौद्योगिकी का उपयोग करती हैं, जो आपको दिखाई देने वाले किसी भी जानवर की पहचान करने में सक्षम हैं। पहले कभी न देखी प्रकृति के अद्भुत दृश्यों का अनुभव करें! #NatureWonders #AIBinoculars
>
> Espagnol :  
> ¡Presentamos el futuro de la observación de aves/vida silvestre! Nuestros nuevos binoculares con IA utilizan técnicas avanzadas de visión para identificar instantáneamente cualquier animal que veas. ¡Experimenta la naturaleza como nunca antes! #TecnologíaNatural #PrismáticosIA
>
> Arabe :  
> نقدم لكم مستقبل مراقبة الطيور/الحياة البرية! توفر نظارات المجهر الجديدة لدينا المزودة بالذكاء الاصطناعي تقنيات رؤية متطورة لتحديد هوية أي حيوان تراه على الفور. شاهد عجائب الطبيعة كما لم تفعل من قبل! #تكنولوجيا*الطبيعة #منظار*ذكي
>
> Bengali :  
> পাখি/বন্য প্রাণী পর্যবেক্ষণের ভবিষ্যতের উন্মোচন! আমাদের নতুন এআই দুর্বীনগুলি অত্যাধুনিক ভিশন প্রযুক্তি ব্যবহার করে যেকোনো প্রাণীকে দ্রুত চিহ্নিত করতে সক্ষম। প্রকৃতির অপরূপ দৃশ্য আগেকার চেয়ে ভিন্নভাবে উপভোগ করুন! #প্রাকৃতিকপ্রযুক্তি #AIBinoculars
>
> Russe :  
> Представляем будущее наблюдения за птицами/дикой природой! Наши новые бинокли с ИИ используют передовые технологии компьютерного зрения для мгновенной идентификации любого животного. Откройте для себя природу по-новому! #ПриродныеТехнологии #БинокльСИИ
>
> Portugais :  
> Apresentamos o futuro da observação de aves/vida selvagem! Os nossos novos binóculos com IA usam técnicas avançadas de visão para identificar instantaneamente qualquer animal que veja. Experiencie a natureza como nunca antes! #TecnologiaNatural #BinóculosIA
>
> Japonais :  
> 鳥/野生動物観察の未来をご紹介!新しい AI バイノキュラーは、最先端のビジョン技術を使用して、あなたが見たあらゆる動物をすばやく特定します。自然をこれまでにない方法で体験してください!#ネイチャーテック #AI バイノキュラー
>
> Punjabi :  
> ਪੰਛੀਆਂ/ਜੰਗਲੀ ਜੀਵਨ ਦੇਖਣ ਦੇ ਭਵਿੱਖ ਦਾ ਐਲਾਨ! ਸਾਡੀਆਂ ਨਵੀਆਂ ਏਆਈ ਦੂਰਬੀਨਾਂ ਅਤਿ ਆਧੁਨਿਕ ਦ੍ਰਿਸ਼ਟੀ ਤਕਨਾਲੋਜੀ ਦੀ ਵਰਤੋਂ ਕਰਦੀਆਂ ਹਨ ਜੋ ਤੁਸੀਂ ਦੇਖਦੇ ਹੋ ਕਿਸੇ ਵੀ ਜਾਨਵਰ ਦੀ ਸਿੱਧੀ ਪਛਾਣ ਕਰਨ ਦੇ ਸਮਰੱਥ ਹਨ। ਪ੍ਰਕਿਰਤੀ ਦਾ ਅਨੁਭਵ ਪਹਿਲਾਂ ਨਾਲੋਂ ਵੱਖਰੇ ਢੰਗ ਨਾਲ ਕਰੋ! #NeighborhoodTech #AIBinoculars

---

### Requête API

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
                        "text": "Écrivez-moi une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées. Le produit est une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux vus à travers les jumelles. Les tweets doivent être passionnants, à la pointe de la technologie et susciter l'intérêt des consommateurs."
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
      apiKey: "my_api_key", // par défaut process.env["ANTHROPIC_API_KEY"]
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
              "text": "Écrivez-moi une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées. Le produit est une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux vus à travers les jumelles. Les tweets doivent être passionnants, à la pointe de la technologie et susciter l'intérêt des consommateurs."
            }
          ]
        }
      ]
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
                        "text": "Écrivez-moi une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées. Le produit est une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux vus à travers les jumelles. Les tweets doivent être passionnants, à la pointe de la technologie et susciter l'intérêt des consommateurs."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // Voir https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // pour les options d'authentification
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
              "text": "Écrivez-moi une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées. Le produit est une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux vus à travers les jumelles. Les tweets doivent être passionnants, à la pointe de la technologie et susciter l'intérêt des consommateurs."
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
                        "text": "Écrivez-moi une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées. Le produit est une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux vus à travers les jumelles. Les tweets doivent être passionnants, à la pointe de la technologie et susciter l'intérêt des consommateurs."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Lit à partir des variables d'environnement `CLOUD_ML_REGION` et `ANTHROPIC_VERTEX_PROJECT_ID`.
    // Passe également par le flux standard `google-auth-library`.
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
              "text": "Écrivez-moi une série de tweets d'annonce de produit dans les 10 langues les plus couramment parlées. Le produit est une nouvelle paire de jumelles de pointe avec des systèmes d'IA intégrés pour identifier les animaux vus à travers les jumelles. Les tweets doivent être passionnants, à la pointe de la technologie et susciter l'intérêt des consommateurs."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>