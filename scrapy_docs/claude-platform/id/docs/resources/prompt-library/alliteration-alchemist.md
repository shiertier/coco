# Alkemis aliterasi

Menghasilkan frasa dan kalimat aliteratif untuk subjek yang diberikan.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah membuat frasa dan kalimat aliteratif untuk subjek yang diberikan. Pastikan bahwa aliterasi tidak hanya terdengar menyenangkan tetapi juga menyampaikan informasi yang relevan atau membangkitkan emosi yang sesuai terkait dengan subjek tersebut. |
| User   | Samudra |

### Contoh output

> 1.  Ombak berbisik membasahi pantai.
> 2.  Percikan air asin menyembur di pasir.
> 3.  Arus yang menghantam mengukir tebing pantai.
> 4.  Kedalaman yang dalam dan gelap menyembunyikan bahaya tersembunyi.
> 5.  Sungai kecil yang bergelembung membawa kehidupan ke pantai.
> 6.  Mamalia yang perkasa dan megah bermigrasi melalui perairan.
> 7.  Pasang surut yang menggelegar melempar kapal ke sana kemari.
> 8.  Kawanan ikan yang berkilauan melindungi makhluk laut yang misterius.
> 9.  Pantulan yang beriak mengungkapkan dunia di bawah permukaan.
> 10. Laut yang berputar dan bergelombang membentuk garis pantai.
> 11. Busa yang berbusa dan berbuih mengalir dengan bebas.
> 12. Camar yang anggun meluncur di atas samudra yang berkilauan.
> 13. Lumba-lumba yang bermain menyelam melalui perairan yang murni.
> 14. Angin asin bertiup melintasi biru yang tak terbatas.
> 15. Riak yang berirama bergulung menuju pantai berbatu.
> 16. Gua karang menyembunyikan komunitas akuatik yang berwarna-warni.
> 17. Langit badai dan laut yang membengkak menandakan kekuatan alam.
> 18. Pasang surut yang bergulir mengubah teluk yang tenang.
> 19. Angin yang berputar dan berderu mengaduk ombak.
> 20. Perairan safir yang tenang membentang ke cakrawala.

---

### Permintaan API

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
        temperature=1,
        system="Tugas Anda adalah membuat frasa dan kalimat aliteratif untuk subjek yang diberikan. Pastikan bahwa aliterasi tidak hanya terdengar menyenangkan tetapi juga menyampaikan informasi yang relevan atau membangkitkan emosi yang sesuai terkait dengan subjek tersebut.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Samudra"
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
      temperature: 1,
      system: "Tugas Anda adalah membuat frasa dan kalimat aliteratif untuk subjek yang diberikan. Pastikan bahwa aliterasi tidak hanya terdengar menyenangkan tetapi juga menyampaikan informasi yang relevan atau membangkitkan emosi yang sesuai terkait dengan subjek tersebut.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Samudra"
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
        temperature=1,
        system="Tugas Anda adalah membuat frasa dan kalimat aliteratif untuk subjek yang diberikan. Pastikan bahwa aliterasi tidak hanya terdengar menyenangkan tetapi juga menyampaikan informasi yang relevan atau membangkitkan emosi yang sesuai terkait dengan subjek tersebut.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Samudra"
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
      temperature: 1,
      system: "Tugas Anda adalah membuat frasa dan kalimat aliteratif untuk subjek yang diberikan. Pastikan bahwa aliterasi tidak hanya terdengar menyenangkan tetapi juga menyampaikan informasi yang relevan atau membangkitkan emosi yang sesuai terkait dengan subjek tersebut.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Samudra"
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
        temperature=1,
        system="Tugas Anda adalah membuat frasa dan kalimat aliteratif untuk subjek yang diberikan. Pastikan bahwa aliterasi tidak hanya terdengar menyenangkan tetapi juga menyampaikan informasi yang relevan atau membangkitkan emosi yang sesuai terkait dengan subjek tersebut.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Samudra"
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
      max_tokens: 1000,
      temperature: 1,
      system: "Tugas Anda adalah membuat frasa dan kalimat aliteratif untuk subjek yang diberikan. Pastikan bahwa aliterasi tidak hanya terdengar menyenangkan tetapi juga menyampaikan informasi yang relevan atau membangkitkan emosi yang sesuai terkait dengan subjek tersebut.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Samudra"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>