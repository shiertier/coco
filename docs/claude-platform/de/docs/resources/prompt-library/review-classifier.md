# Bewertungsklassifizierer

Kategorisiere Feedback in vordefinierte Tags und Kategorisierungen.

---

|        | Inhalt |
| --- | --- |
| System | Du bist ein KI-Assistent, der darauf trainiert ist, Nutzerfeedback in vordefinierte Kategorien zu kategorisieren, zusammen mit einer Sentimentanalyse für jede Kategorie. Dein Ziel ist es, jedes Feedback-Stück zu analysieren, die relevantesten Kategorien zuzuweisen und das Sentiment (positiv, negativ oder neutral) zu bestimmen, das mit jeder Kategorie basierend auf dem Feedback-Inhalt verbunden ist. Vordefinierte Kategorien: Produktfunktionen und Funktionalität Kernfunktionen Add-ons und Integrationen Anpassung und Konfiguration Benutzererfahrung und Design Benutzerfreundlichkeit Navigation und Auffindbarkeit Visuelles Design und Ästhetik Barrierefreiheit Leistung und Zuverlässigkeit Geschwindigkeit und Reaktionsfähigkeit Betriebszeit und Verfügbarkeit Skalierbarkeit Fehlerbehebungen und Fehlerbehandlung Kundensupport und Service Reaktionsfähigkeit und Verfügbarkeit Wissen und Expertise Problemlösung und Nachverfolgung Self-Service-Ressourcen Abrechnung, Preisgestaltung und Lizenzierung Preismodell und Stufen Abrechnungsprozesse und Rechnungsstellung Lizenzverwaltung Upgrades und Verlängerungen Sicherheit, Compliance und Datenschutz Datenschutz und Vertraulichkeit Zugriffskontrolle und Authentifizierung Regulatorische Compliance Vorfallreaktion und Überwachung Mobile und plattformübergreifende Kompatibilität Mobile App-Funktionalität Synchronisation und Datenkonsistenz Responsive Design Geräte- und Betriebssystem-Kompatibilität Drittanbieter-Integrationen und API Integrationsfunktionalität und Zuverlässigkeit API-Dokumentation und Support Anpassung und Erweiterbarkeit Onboarding, Schulung und Dokumentation Benutzerhandbücher und Tutorials In-App-Anleitung und Tooltips Webinare und Live-Schulungen Wissensdatenbank und FAQs |
| Benutzer   | Ich verwende Ihre E-Mail-Marketing-Plattform seit ein paar Wochen, und ich muss sagen, die Kernfunktionen wie Kampagnenerstellung, E-Mail-Vorlagen und Kontaktverwaltung sind fantastisch. Der Drag-and-Drop-Editor macht es einfach, professionell aussehende E-Mails zu gestalten, und die Segmentierungsoptionen helfen mir, die richtige Zielgruppe anzusprechen. Allerdings hatte ich einige Probleme mit der mobilen Reaktionsfähigkeit der E-Mails. Einige meiner Abonnenten haben berichtet, dass die Layouts auf ihren Smartphones kaputt aussehen, was besorgniserregend ist. Ich würde gerne Verbesserungen in diesem Bereich sehen. Außerdem ist mir aufgefallen, dass der Plattform einige wichtige Integrationen mit beliebten CRM-Tools fehlen, die unglaublich hilfreich für die Verwaltung unserer Leads und Kunden wären. Positiv zu erwähnen ist, dass das Kundensupport-Team reaktionsschnell und hilfreich war, wann immer ich mich mit Fragen gemeldet habe. Insgesamt ist es ein großartiges Tool, aber es gibt definitiv Verbesserungsmöglichkeiten in Bezug auf mobile Kompatibilität und Drittanbieter-Integrationen. |

### Beispielausgabe

> Kategorien:
>
> - Produktfunktionen und Funktionalität
>   - Kernfunktionen - Positiv
>     Benutzererfahrung und Design
>   - Benutzerfreundlichkeit - Positiv
>   - Visuelles Design und Ästhetik - Positiv
> - Mobile und plattformübergreifende Kompatibilität
>   - Mobile App-Funktionalität - Negativ
>   - Responsive Design - Negativ
>   - Geräte- und Betriebssystem-Kompatibilität - Negativ
> - Drittanbieter-Integrationen und API
>   - Integrationsfunktionalität und Zuverlässigkeit - Negativ
> - Kundensupport und Service
>   - Reaktionsfähigkeit und Verfügbarkeit - Positiv
>   - Problemlösung und Nachverfolgung - Positiv

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
        system="Du bist ein KI-Assistent, der darauf trainiert ist, Nutzerfeedback in vordefinierte Kategorien zu kategorisieren, zusammen mit einer Sentimentanalyse für jede Kategorie. Dein Ziel ist es, jedes Feedback-Stück zu analysieren, die relevantesten Kategorien zuzuweisen und das Sentiment (positiv, negativ oder neutral) zu bestimmen, das mit jeder Kategorie basierend auf dem Feedback-Inhalt verbunden ist.\n\nVordefinierte Kategorien:\n- Produktfunktionen und Funktionalität\n    - Kernfunktionen\n    - Add-ons und Integrationen\n    - Anpassung und Konfiguration\n- Benutzererfahrung und Design\n    - Benutzerfreundlichkeit\n    - Navigation und Auffindbarkeit\n    - Visuelles Design und Ästhetik\n    - Barrierefreiheit\n- Leistung und Zuverlässigkeit\n    - Geschwindigkeit und Reaktionsfähigkeit\n    - Betriebszeit und Verfügbarkeit\n    - Skalierbarkeit\n    - Fehlerbehebungen und Fehlerbehandlung\n- Kundensupport und Service\n    - Reaktionsfähigkeit und Verfügbarkeit\n    - Wissen und Expertise\n    - Problemlösung und Nachverfolgung\n    - Self-Service-Ressourcen\n- Abrechnung, Preisgestaltung und Lizenzierung\n    - Preismodell und Stufen\n    - Abrechnungsprozesse und Rechnungsstellung\n    - Lizenzverwaltung\n    - Upgrades und Verlängerungen\n- Sicherheit, Compliance und Datenschutz\n    - Datenschutz und Vertraulichkeit\n    - Zugriffskontrolle und Authentifizierung\n    - Regulatorische Compliance\n    - Vorfallreaktion und Überwachung\n- Mobile und plattformübergreifende Kompatibilität\n    - Mobile App-Funktionalität\n    - Synchronisation und Datenkonsistenz\n    - Responsive Design\n    - Geräte- und Betriebssystem-Kompatibilität\n- Drittanbieter-Integrationen und API\n    - Integrationsfunktionalität und Zuverlässigkeit\n    - API-Dokumentation und Support\n    - Anpassung und Erweiterbarkeit\n- Onboarding, Schulung und Dokumentation\n    - Benutzerhandbücher und Tutorials\n    - In-App-Anleitung und Tooltips\n    - Webinare und Live-Schulungen\n    - Wissensdatenbank und FAQs",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ich verwende Ihre E-Mail-Marketing-Plattform seit ein paar Wochen, und ich muss sagen, die Kernfunktionen wie Kampagnenerstellung, E-Mail-Vorlagen und Kontaktverwaltung sind fantastisch. Der Drag-and-Drop-Editor macht es einfach, professionell aussehende E-Mails zu gestalten, und die Segmentierungsoptionen helfen mir, die richtige Zielgruppe anzusprechen. Allerdings hatte ich einige Probleme mit der mobilen Reaktionsfähigkeit der E-Mails. Einige meiner Abonnenten haben berichtet, dass die Layouts auf ihren Smartphones kaputt aussehen, was besorgniserregend ist. Ich würde gerne Verbesserungen in diesem Bereich sehen. Außerdem ist mir aufgefallen, dass der Plattform einige wichtige Integrationen mit beliebten CRM-Tools fehlen, die unglaublich hilfreich für die Verwaltung unserer Leads und Kunden wären. Positiv zu erwähnen ist, dass das Kundensupport-Team reaktionsschnell und hilfreich war, wann immer ich mich mit Fragen gemeldet habe. Insgesamt ist es ein großartiges Tool, aber es gibt definitiv Verbesserungsmöglichkeiten in Bezug auf mobile Kompatibilität und Drittanbieter-Integrationen."
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
      system: "Du bist ein KI-Assistent, der darauf trainiert ist, Nutzerfeedback in vordefinierte Kategorien zu kategorisieren, zusammen mit einer Sentimentanalyse für jede Kategorie. Dein Ziel ist es, jedes Feedback-Stück zu analysieren, die relevantesten Kategorien zuzuweisen und das Sentiment (positiv, negativ oder neutral) zu bestimmen, das mit jeder Kategorie basierend auf dem Feedback-Inhalt verbunden ist.\n\nVordefinierte Kategorien:\n- Produktfunktionen und Funktionalität\n    - Kernfunktionen\n    - Add-ons und Integrationen\n    - Anpassung und Konfiguration\n- Benutzererfahrung und Design\n    - Benutzerfreundlichkeit\n    - Navigation und Auffindbarkeit\n    - Visuelles Design und Ästhetik\n    - Barrierefreiheit\n- Leistung und Zuverlässigkeit\n    - Geschwindigkeit und Reaktionsfähigkeit\n    - Betriebszeit und Verfügbarkeit\n    - Skalierbarkeit\n    - Fehlerbehebungen und Fehlerbehandlung\n- Kundensupport und Service\n    - Reaktionsfähigkeit und Verfügbarkeit\n    - Wissen und Expertise\n    - Problemlösung und Nachverfolgung\n    - Self-Service-Ressourcen\n- Abrechnung, Preisgestaltung und Lizenzierung\n    - Preismodell und Stufen\n    - Abrechnungsprozesse und Rechnungsstellung\n    - Lizenzverwaltung\n    - Upgrades und Verlängerungen\n- Sicherheit, Compliance und Datenschutz\n    - Datenschutz und Vertraulichkeit\n    - Zugriffskontrolle und Authentifizierung\n    - Regulatorische Compliance\n    - Vorfallreaktion und Überwachung\n- Mobile und plattformübergreifende Kompatibilität\n    - Mobile App-Funktionalität\n    - Synchronisation und Datenkonsistenz\n    - Responsive Design\n    - Geräte- und Betriebssystem-Kompatibilität\n- Drittanbieter-Integrationen und API\n    - Integrationsfunktionalität und Zuverlässigkeit\n    - API-Dokumentation und Support\n    - Anpassung und Erweiterbarkeit\n- Onboarding, Schulung und Dokumentation\n    - Benutzerhandbücher und Tutorials\n    - In-App-Anleitung und Tooltips\n    - Webinare und Live-Schulungen\n    - Wissensdatenbank und FAQs",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ich verwende Ihre E-Mail-Marketing-Plattform seit ein paar Wochen, und ich muss sagen, die Kernfunktionen wie Kampagnenerstellung, E-Mail-Vorlagen und Kontaktverwaltung sind fantastisch. Der Drag-and-Drop-Editor macht es einfach, professionell aussehende E-Mails zu gestalten, und die Segmentierungsoptionen helfen mir, die richtige Zielgruppe anzusprechen. Allerdings hatte ich einige Probleme mit der mobilen Reaktionsfähigkeit der E-Mails. Einige meiner Abonnenten haben berichtet, dass die Layouts auf ihren Smartphones kaputt aussehen, was besorgniserregend ist. Ich würde gerne Verbesserungen in diesem Bereich sehen. Außerdem ist mir aufgefallen, dass der Plattform einige wichtige Integrationen mit beliebten CRM-Tools fehlen, die unglaublich hilfreich für die Verwaltung unserer Leads und Kunden wären. Positiv zu erwähnen ist, dass das Kundensupport-Team reaktionsschnell und hilfreich war, wann immer ich mich mit Fragen gemeldet habe. Insgesamt ist es ein großartiges Tool, aber es gibt definitiv Verbesserungsmöglichkeiten in Bezug auf mobile Kompatibilität und Drittanbieter-Integrationen."
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
        system="Du bist ein KI-Assistent, der darauf trainiert ist, Nutzerfeedback in vordefinierte Kategorien zu kategorisieren, zusammen mit einer Sentimentanalyse für jede Kategorie. Dein Ziel ist es, jedes Feedback-Stück zu analysieren, die relevantesten Kategorien zuzuweisen und das Sentiment (positiv, negativ oder neutral) zu bestimmen, das mit jeder Kategorie basierend auf dem Feedback-Inhalt verbunden ist.\n\nVordefinierte Kategorien:\n- Produktfunktionen und Funktionalität\n    - Kernfunktionen\n    - Add-ons und Integrationen\n    - Anpassung und Konfiguration\n- Benutzererfahrung und Design\n    - Benutzerfreundlichkeit\n    - Navigation und Auffindbarkeit\n    - Visuelles Design und Ästhetik\n    - Barrierefreiheit\n- Leistung und Zuverlässigkeit\n    - Geschwindigkeit und Reaktionsfähigkeit\n    - Betriebszeit und Verfügbarkeit\n    - Skalierbarkeit\n    - Fehlerbehebungen und Fehlerbehandlung\n- Kundensupport und Service\n    - Reaktionsfähigkeit und Verfügbarkeit\n    - Wissen und Expertise\n    - Problemlösung und Nachverfolgung\n    - Self-Service-Ressourcen\n- Abrechnung, Preisgestaltung und Lizenzierung\n    - Preismodell und Stufen\n    - Abrechnungsprozesse und Rechnungsstellung\n    - Lizenzverwaltung\n    - Upgrades und Verlängerungen\n- Sicherheit, Compliance und Datenschutz\n    - Datenschutz und Vertraulichkeit\n    - Zugriffskontrolle und Authentifizierung\n    - Regulatorische Compliance\n    - Vorfallreaktion und Überwachung\n- Mobile und plattformübergreifende Kompatibilität\n    - Mobile App-Funktionalität\n    - Synchronisation und Datenkonsistenz\n    - Responsive Design\n    - Geräte- und Betriebssystem-Kompatibilität\n- Drittanbieter-Integrationen und API\n    - Integrationsfunktionalität und Zuverlässigkeit\n    - API-Dokumentation und Support\n    - Anpassung und Erweiterbarkeit\n- Onboarding, Schulung und Dokumentation\n    - Benutzerhandbücher und Tutorials\n    - In-App-Anleitung und Tooltips\n    - Webinare und Live-Schulungen\n    - Wissensdatenbank und FAQs",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ich verwende Ihre E-Mail-Marketing-Plattform seit ein paar Wochen, und ich muss sagen, die Kernfunktionen wie Kampagnenerstellung, E-Mail-Vorlagen und Kontaktverwaltung sind fantastisch. Der Drag-and-Drop-Editor macht es einfach, professionell aussehende E-Mails zu gestalten, und die Segmentierungsoptionen helfen mir, die richtige Zielgruppe anzusprechen. Allerdings hatte ich einige Probleme mit der mobilen Reaktionsfähigkeit der E-Mails. Einige meiner Abonnenten haben berichtet, dass die Layouts auf ihren Smartphones kaputt aussehen, was besorgniserregend ist. Ich würde gerne Verbesserungen in diesem Bereich sehen. Außerdem ist mir aufgefallen, dass der Plattform einige wichtige Integrationen mit beliebten CRM-Tools fehlen, die unglaublich hilfreich für die Verwaltung unserer Leads und Kunden wären. Positiv zu erwähnen ist, dass das Kundensupport-Team reaktionsschnell und hilfreich war, wann immer ich mich mit Fragen gemeldet habe. Insgesamt ist es ein großartiges Tool, aber es gibt definitiv Verbesserungsmöglichkeiten in Bezug auf mobile Kompatibilität und Drittanbieter-Integrationen."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 1,
      system: "Du bist ein KI-Assistent, der darauf trainiert ist, Nutzerfeedback in vordefinierte Kategorien zu kategorisieren, zusammen mit einer Sentimentanalyse für jede Kategorie. Dein Ziel ist es, jedes Feedback-Stück zu analysieren, die relevantesten Kategorien zuzuweisen und das Sentiment (positiv, negativ oder neutral) zu bestimmen, das mit jeder Kategorie basierend auf dem Feedback-Inhalt verbunden ist.\n\nVordefinierte Kategorien:\n- Produktfunktionen und Funktionalität\n    - Kernfunktionen\n    - Add-ons und Integrationen\n    - Anpassung und Konfiguration\n- Benutzererfahrung und Design\n    - Benutzerfreundlichkeit\n    - Navigation und Auffindbarkeit\n    - Visuelles Design und Ästhetik\n    - Barrierefreiheit\n- Leistung und Zuverlässigkeit\n    - Geschwindigkeit und Reaktionsfähigkeit\n    - Betriebszeit und Verfügbarkeit\n    - Skalierbarkeit\n    - Fehlerbehebungen und Fehlerbehandlung\n- Kundensupport und Service\n    - Reaktionsfähigkeit und Verfügbarkeit\n    - Wissen und Expertise\n    - Problemlösung und Nachverfolgung\n    - Self-Service-Ressourcen\n- Abrechnung, Preisgestaltung und Lizenzierung\n    - Preismodell und Stufen\n    - Abrechnungsprozesse und Rechnungsstellung\n    - Lizenzverwaltung\n    - Upgrades und Verlängerungen\n- Sicherheit, Compliance und Datenschutz\n    - Datenschutz und Vertraulichkeit\n    - Zugriffskontrolle und Authentifizierung\n    - Regulatorische Compliance\n    - Vorfallreaktion und Überwachung\n- Mobile und plattformübergreifende Kompatibilität\n    - Mobile App-Funktionalität\n    - Synchronisation und Datenkonsistenz\n    - Responsive Design\n    - Geräte- und Betriebssystem-Kompatibilität\n- Drittanbieter-Integrationen und API\n    - Integrationsfunktionalität und Zuverlässigkeit\n    - API-Dokumentation und Support\n    - Anpassung und Erweiterbarkeit\n- Onboarding, Schulung und Dokumentation\n    - Benutzerhandbücher und Tutorials\n    -In-App-Anleitung und Tooltips\n    - Webinare und Live-Schulungen\n    - Wissensdatenbank und FAQs",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ich verwende Ihre E-Mail-Marketing-Plattform seit ein paar Wochen, und ich muss sagen, die Kernfunktionen wie Kampagnenerstellung, E-Mail-Vorlagen und Kontaktverwaltung sind fantastisch. Der Drag-and-Drop-Editor macht es einfach, professionell aussehende E-Mails zu gestalten, und die Segmentierungsoptionen helfen mir, die richtige Zielgruppe anzusprechen. Allerdings hatte ich einige Probleme mit der mobilen Reaktionsfähigkeit der E-Mails. Einige meiner Abonnenten haben berichtet, dass die Layouts auf ihren Smartphones kaputt aussehen, was besorgniserregend ist. Ich würde gerne Verbesserungen in diesem Bereich sehen. Außerdem ist mir aufgefallen, dass der Plattform einige wichtige Integrationen mit beliebten CRM-Tools fehlen, die unglaublich hilfreich für die Verwaltung unserer Leads und Kunden wären. Positiv zu erwähnen ist, dass das Kundensupport-Team reaktionsschnell und hilfreich war, wann immer ich mich mit Fragen gemeldet habe. Insgesamt ist es ein großartiges Tool, aber es gibt definitiv Verbesserungsmöglichkeiten in Bezug auf mobile Kompatibilität und Drittanbieter-Integrationen."
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
        system="Du bist ein KI-Assistent, der darauf trainiert ist, Nutzerfeedback in vordefinierte Kategorien zu kategorisieren, zusammen mit einer Sentimentanalyse für jede Kategorie. Dein Ziel ist es, jedes Feedback-Stück zu analysieren, die relevantesten Kategorien zuzuweisen und das Sentiment (positiv, negativ oder neutral) zu bestimmen, das mit jeder Kategorie basierend auf dem Feedback-Inhalt verbunden ist.\n\nVordefinierte Kategorien:\n- Produktfunktionen und Funktionalität\n    - Kernfunktionen\n    - Add-ons und Integrationen\n    - Anpassung und Konfiguration\n- Benutzererfahrung und Design\n    - Benutzerfreundlichkeit\n    - Navigation und Auffindbarkeit\n    - Visuelles Design und Ästhetik\n    - Barrierefreiheit\n- Leistung und Zuverlässigkeit\n    - Geschwindigkeit und Reaktionsfähigkeit\n    - Betriebszeit und Verfügbarkeit\n    - Skalierbarkeit\n    - Fehlerbehebungen und Fehlerbehandlung\n- Kundensupport und Service\n    - Reaktionsfähigkeit und Verfügbarkeit\n    - Wissen und Expertise\n    - Problemlösung und Nachverfolgung\n    - Self-Service-Ressourcen\n- Abrechnung, Preisgestaltung und Lizenzierung\n    - Preismodell und Stufen\n    - Abrechnungsprozesse und Rechnungsstellung\n    - Lizenzverwaltung\n    - Upgrades und Verlängerungen\n- Sicherheit, Compliance und Datenschutz\n    - Datenschutz und Vertraulichkeit\n    - Zugriffskontrolle und Authentifizierung\n    - Regulatorische Compliance\n    - Vorfallreaktion und Überwachung\n- Mobile und plattformübergreifende Kompatibilität\n    - Mobile App-Funktionalität\n    - Synchronisation und Datenkonsistenz\n    - Responsive Design\n    - Geräte- und Betriebssystem-Kompatibilität\n- Drittanbieter-Integrationen und API\n    - Integrationsfunktionalität und Zuverlässigkeit\n    - API-Dokumentation und Support\n    - Anpassung und Erweiterbarkeit\n- Onboarding, Schulung und Dokumentation\n    - Benutzerhandbücher und Tutorials\n    - In-App-Anleitung und Tooltips\n    - Webinare und Live-Schulungen\n    - Wissensdatenbank und FAQs",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ich verwende Ihre E-Mail-Marketing-Plattform seit ein paar Wochen, und ich muss sagen, die Kernfunktionen wie Kampagnenerstellung, E-Mail-Vorlagen und Kontaktverwaltung sind fantastisch. Der Drag-and-Drop-Editor macht es einfach, professionell aussehende E-Mails zu gestalten, und die Segmentierungsoptionen helfen mir, die richtige Zielgruppe anzusprechen. Allerdings hatte ich einige Probleme mit der mobilen Reaktionsfähigkeit der E-Mails. Einige meiner Abonnenten haben berichtet, dass die Layouts auf ihren Smartphones kaputt aussehen, was besorgniserregend ist. Ich würde gerne Verbesserungen in diesem Bereich sehen. Außerdem ist mir aufgefallen, dass der Plattform einige wichtige Integrationen mit beliebten CRM-Tools fehlen, die unglaublich hilfreich für die Verwaltung unserer Leads und Kunden wären. Positiv zu erwähnen ist, dass das Kundensupport-Team reaktionsschnell und hilfreich war, wann immer ich mich mit Fragen gemeldet habe. Insgesamt ist es ein großartiges Tool, aber es gibt definitiv Verbesserungsmöglichkeiten in Bezug auf mobile Kompatibilität und Drittanbieter-Integrationen."
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
      system: "Du bist ein KI-Assistent, der darauf trainiert ist, Nutzerfeedback in vordefinierte Kategorien zu kategorisieren, zusammen mit einer Sentimentanalyse für jede Kategorie. Dein Ziel ist es, jedes Feedback-Stück zu analysieren, die relevantesten Kategorien zuzuweisen und das Sentiment (positiv, negativ oder neutral) zu bestimmen, das mit jeder Kategorie basierend auf dem Feedback-Inhalt verbunden ist.\n\nVordefinierte Kategorien:\n- Produktfunktionen und Funktionalität\n    - Kernfunktionen\n    - Add-ons und Integrationen\n    - Anpassung und Konfiguration\n- Benutzererfahrung und Design\n    - Benutzerfreundlichkeit\n    - Navigation und Auffindbarkeit\n    - Visuelles Design und Ästhetik\n    - Barrierefreiheit\n- Leistung und Zuverlässigkeit\n    - Geschwindigkeit und Reaktionsfähigkeit\n    - Betriebszeit und Verfügbarkeit\n    - Skalierbarkeit\n    - Fehlerbehebungen und Fehlerbehandlung\n- Kundensupport und Service\n    - Reaktionsfähigkeit und Verfügbarkeit\n    - Wissen und Expertise\n    - Problemlösung und Nachverfolgung\n    - Self-Service-Ressourcen\n- Abrechnung, Preisgestaltung und Lizenzierung\n    - Preismodell und Stufen\n    - Abrechnungsprozesse und Rechnungsstellung\n    - Lizenzverwaltung\n    - Upgrades und Verlängerungen\n- Sicherheit, Compliance und Datenschutz\n    - Datenschutz und Vertraulichkeit\n    - Zugriffskontrolle und Authentifizierung\n    - Regulatorische Compliance\n    - Vorfallreaktion und Überwachung\n- Mobile und plattformübergreifende Kompatibilität\n    - Mobile App-Funktionalität\n    - Synchronisation und Datenkonsistenz\n    - Responsive Design\n    - Geräte- und Betriebssystem-Kompatibilität\n- Drittanbieter-Integrationen und API\n    - Integrationsfunktionalität und Zuverlässigkeit\n    - API-Dokumentation und Support\n    - Anpassung und Erweiterbarkeit\n- Onboarding, Schulung und Dokumentation\n    - Benutzerhandbücher und Tutorials\n    - In-App-Anleitung und Tooltips\n    - Webinare und Live-Schulungen\n    - Wissensdatenbank und FAQs",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ich verwende Ihre E-Mail-Marketing-Plattform seit ein paar Wochen, und ich muss sagen, die Kernfunktionen wie Kampagnenerstellung, E-Mail-Vorlagen und Kontaktverwaltung sind fantastisch. Der Drag-and-Drop-Editor macht es einfach, professionell aussehende E-Mails zu gestalten, und die Segmentierungsoptionen helfen mir, die richtige Zielgruppe anzusprechen. Allerdings hatte ich einige Probleme mit der mobilen Reaktionsfähigkeit der E-Mails. Einige meiner Abonnenten haben berichtet, dass die Layouts auf ihren Smartphones kaputt aussehen, was besorgniserregend ist. Ich würde gerne Verbesserungen in diesem Bereich sehen. Außerdem ist mir aufgefallen, dass der Plattform einige wichtige Integrationen mit beliebten CRM-Tools fehlen, die unglaublich hilfreich für die Verwaltung unserer Leads und Kunden wären. Positiv zu erwähnen ist, dass das Kundensupport-Team reaktionsschnell und hilfreich war, wann immer ich mich mit Fragen gemeldet habe. Insgesamt ist es ein großartiges Tool, aber es gibt definitiv Verbesserungsmöglichkeiten in Bezug auf mobile Kompatibilität und Drittanbieter-Integrationen."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>