# Ratenlimits

Um Missbrauch zu verhindern und die Kapazität unserer API zu verwalten, haben wir Limits implementiert, wie viel eine Organisation die Claude API nutzen kann.

---

Wir haben zwei Arten von Limits:

1. **Ausgabenlimits** setzen maximale monatliche Kosten, die eine Organisation für die API-Nutzung anfallen können.
2. **Ratenlimits** setzen die maximale Anzahl von API-Anfragen, die eine Organisation über einen definierten Zeitraum hinweg stellen kann.

Wir erzwingen servicekonfigurierte Limits auf Organisationsebene, aber Sie können auch benutzerkonfigurierbare Limits für die Workspaces Ihrer Organisation festlegen.

Diese Limits gelten sowohl für Standard- als auch für Priority Tier-Nutzung. Weitere Informationen zu Priority Tier, das verbesserte Service-Level gegen zugesicherte Ausgaben bietet, finden Sie unter [Service Tiers](/docs/de/api/service-tiers).

## Über unsere Limits

* Limits sind so konzipiert, dass sie API-Missbrauch verhindern und gleichzeitig die Auswirkungen auf häufige Nutzungsmuster von Kunden minimieren.
* Limits werden nach **Nutzungsstufe** definiert, wobei jede Stufe mit einem anderen Satz von Ausgaben- und Ratenlimits verbunden ist.
* Ihre Organisation wird automatisch in höhere Stufen aufgestuft, wenn Sie bestimmte Schwellenwerte bei der API-Nutzung erreichen.
  Limits werden auf Organisationsebene festgelegt. Sie können die Limits Ihrer Organisation auf der [Limits-Seite](/settings/limits) in der [Claude Console](/) einsehen.
* Sie können Ratenlimits über kürzere Zeitintervalle erreichen. Beispielsweise kann eine Rate von 60 Anfragen pro Minute (RPM) als 1 Anfrage pro Sekunde erzwungen werden. Kurze Bursts von Anfragen mit hohem Volumen können das Ratenlimit überschreiten und zu Ratenlimit-Fehlern führen.
* Die unten beschriebenen Limits sind unsere Standard-Tier-Limits. Wenn Sie höhere, benutzerdefinierte Limits oder Priority Tier für verbesserte Service-Level suchen, kontaktieren Sie den Vertrieb über die [Claude Console](/settings/limits).
* Wir verwenden den [Token-Bucket-Algorithmus](https://en.wikipedia.org/wiki/Token_bucket) für die Ratenbegrenzung. Dies bedeutet, dass Ihre Kapazität kontinuierlich bis zu Ihrem maximalen Limit aufgefüllt wird, anstatt in festen Intervallen zurückgesetzt zu werden.
* Alle hier beschriebenen Limits stellen maximale zulässige Nutzung dar, keine garantierten Mindestwerte. Diese Limits sollen unbeabsichtigte Überausgaben reduzieren und eine faire Ressourcenverteilung unter Benutzern gewährleisten.

## Ausgabenlimits

Jede Nutzungsstufe hat ein Limit, wie viel Sie pro Kalendermonat für die API ausgeben können. Sobald Sie das Ausgabenlimit Ihrer Stufe erreichen, müssen Sie bis zur nächsten Stufe bis zum nächsten Monat warten, um die API erneut nutzen zu können.

Um sich für die nächste Stufe zu qualifizieren, müssen Sie eine Einzahlungsanforderung erfüllen. Um das Risiko einer Überfinanzierung Ihres Kontos zu minimieren, können Sie nicht mehr als Ihr monatliches Ausgabenlimit einzahlen.

### Anforderungen zum Aufstieg in die nächste Stufe
<table>
  <thead>
    <tr>
      <th>Nutzungsstufe</th>
      <th>Gutschein-Kauf</th>
      <th>Maximaler Gutschein-Kauf</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Stufe 1</td>
      <td>\$5</td>
      <td>\$100</td>
    </tr>
    <tr>
      <td>Stufe 2</td>
      <td>\$40</td>
      <td>\$500</td>
    </tr>
    <tr>
      <td>Stufe 3</td>
      <td>\$200</td>
      <td>\$1.000</td>
    </tr>
    <tr>
      <td>Stufe 4</td>
      <td>\$400</td>
      <td>\$5.000</td>
    </tr>
    <tr>
      <td>Monatliche Abrechnung</td>
      <td>N/A</td>
      <td>N/A</td>
    </tr>
  </tbody>
</table>

<Note>
**Gutschein-Kauf** zeigt die kumulativen Gutschein-Käufe (ohne Steuern) an, die erforderlich sind, um diese Stufe zu erreichen. Sie werden sofort aufgestuft, wenn Sie den Schwellenwert erreichen.

**Maximaler Gutschein-Kauf** begrenzt den maximalen Betrag, den Sie in einer einzelnen Transaktion zu Ihrem Konto hinzufügen können, um eine Kontoüberfinanzierung zu verhindern.
</Note>

## Ratenlimits

Unsere Ratenlimits für die Messages API werden in Anfragen pro Minute (RPM), Eingabe-Token pro Minute (ITPM) und Ausgabe-Token pro Minute (OTPM) für jede Modellklasse gemessen.
Wenn Sie eines der Ratenlimits überschreiten, erhalten Sie einen [429-Fehler](/docs/de/api/errors), der beschreibt, welches Ratenlimit überschritten wurde, zusammen mit einem `retry-after`-Header, der angibt, wie lange Sie warten sollten.

<Note>
Sie können auch auf 429-Fehler aufgrund von Beschleunigungslimits auf der API stoßen, wenn Ihre Organisation einen starken Anstieg der Nutzung hat. Um Beschleunigungslimits zu vermeiden, erhöhen Sie Ihren Datenverkehr schrittweise und halten Sie konsistente Nutzungsmuster ein.
</Note>

### Cache-bewusste ITPM

Viele API-Anbieter verwenden ein kombiniertes "Token pro Minute" (TPM) Limit, das alle Token einschließen kann, sowohl gecachte als auch ungecachte, Eingabe und Ausgabe. **Bei den meisten Claude-Modellen zählen nur ungecachte Eingabe-Token zu Ihren ITPM-Ratenlimits.** Dies ist ein großer Vorteil, der unsere Ratenlimits effektiv höher macht, als sie zunächst erscheinen mögen.

ITPM-Ratenlimits werden am Anfang jeder Anfrage geschätzt, und die Schätzung wird während der Anfrage angepasst, um die tatsächliche Anzahl der verwendeten Eingabe-Token widerzuspiegeln.

Hier ist, was zu ITPM zählt:
- `input_tokens` (Token nach dem letzten Cache-Breakpoint) ✓ **Zählen zu ITPM**
- `cache_creation_input_tokens` (Token, die in den Cache geschrieben werden) ✓ **Zählen zu ITPM**
- `cache_read_input_tokens` (Token, die aus dem Cache gelesen werden) ✗ **Zählen NICHT zu ITPM** bei den meisten Modellen

<Note>
Das Feld `input_tokens` stellt nur Token dar, die **nach Ihrem letzten Cache-Breakpoint** erscheinen, nicht alle Eingabe-Token in Ihrer Anfrage. Um die Gesamteingabe-Token zu berechnen:

```
total_input_tokens = cache_read_input_tokens + cache_creation_input_tokens + input_tokens
```

Dies bedeutet, dass wenn Sie gecachten Inhalt haben, `input_tokens` typischerweise viel kleiner ist als Ihre Gesamteingabe. Beispielsweise würden Sie mit einem 200K-Token-gecachten Dokument und einer 50-Token-Benutzerfrage `input_tokens: 50` sehen, obwohl die Gesamteingabe 200.050 Token beträgt.

Für Ratenlimit-Zwecke bei den meisten Modellen zählen nur `input_tokens` + `cache_creation_input_tokens` zu Ihrem ITPM-Limit, was [Prompt-Caching](/docs/de/build-with-claude/prompt-caching) zu einer effektiven Möglichkeit macht, Ihren effektiven Durchsatz zu erhöhen.
</Note>

**Beispiel**: Mit einem 2.000.000 ITPM-Limit und einer 80%-Cache-Hit-Rate könnten Sie effektiv 10.000.000 Gesamteingabe-Token pro Minute verarbeiten (2M ungecacht + 8M gecacht), da gecachte Token nicht zu Ihrem Ratenlimit zählen.

<Note>
Einige ältere Modelle (gekennzeichnet mit † in den Ratenlimit-Tabellen unten) zählen auch `cache_read_input_tokens` zu ITPM-Ratenlimits.

Bei allen Modellen ohne das †-Zeichen zählen gecachte Eingabe-Token nicht zu Ratenlimits und werden zu einem reduzierten Satz abgerechnet (10% des Basis-Eingabe-Token-Preises). Dies bedeutet, dass Sie einen deutlich höheren effektiven Durchsatz erreichen können, indem Sie [Prompt-Caching](/docs/de/build-with-claude/prompt-caching) verwenden.
</Note>

<Tip>
**Maximieren Sie Ihre Ratenlimits mit Prompt-Caching**

Um das Beste aus Ihren Ratenlimits herauszuholen, verwenden Sie [Prompt-Caching](/docs/de/build-with-claude/prompt-caching) für wiederholte Inhalte wie:
- Systeminstruktionen und Prompts
- Große Kontextdokumente
- Tool-Definitionen
- Gesprächsverlauf

Mit effektivem Caching können Sie Ihren tatsächlichen Durchsatz dramatisch erhöhen, ohne Ihre Ratenlimits zu erhöhen. Überwachen Sie Ihre Cache-Hit-Rate auf der [Nutzungsseite](/settings/usage), um Ihre Caching-Strategie zu optimieren.
</Tip>

OTPM-Ratenlimits werden basierend auf `max_tokens` am Anfang jeder Anfrage geschätzt, und die Schätzung wird am Ende der Anfrage angepasst, um die tatsächliche Anzahl der erzeugten Ausgabe-Token widerzuspiegeln.
Wenn Sie OTPM-Limits früher als erwartet erreichen, versuchen Sie, `max_tokens` zu reduzieren, um die Größe Ihrer Vervollständigungen besser zu approximieren.

Ratenlimits werden separat für jedes Modell angewendet; daher können Sie verschiedene Modelle bis zu ihren jeweiligen Limits gleichzeitig verwenden.
Sie können Ihre aktuellen Ratenlimits und das Verhalten in der [Claude Console](/settings/limits) überprüfen.

<Note>
Für Anfragen mit langem Kontext (>200K Token) bei Verwendung des `context-1m-2025-08-07` Beta-Headers mit Claude Sonnet 4.x gelten separate Ratenlimits. Siehe [Ratenlimits für langen Kontext](#long-context-rate-limits) unten.
</Note>

<Tabs>
<Tab title="Stufe 1">
| Modell                                                                                       | Maximale Anfragen pro Minute (RPM) | Maximale Eingabe-Token pro Minute (ITPM) | Maximale Ausgabe-Token pro Minute (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 50                                | 30.000                                 | 8.000                                   |
| Claude Sonnet 3.7 ([veraltet](/docs/de/about-claude/model-deprecations))                     | 50                                | 20.000                                 | 8.000                                   |
| Claude Haiku 4.5                                                                             | 50                                | 50.000                                 | 10.000                                  |
| Claude Haiku 3.5 ([veraltet](/docs/de/about-claude/model-deprecations))                      | 50                                | 50.000<sup>†</sup>                     | 10.000                                  |
| Claude Haiku 3                                                                               | 50                                | 50.000<sup>†</sup>                     | 10.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 50                                | 30.000                                 | 8.000                                   |
| Claude Opus 3 ([veraltet](/docs/de/about-claude/model-deprecations))                        | 50                                | 20.000<sup>†</sup>                     | 4.000                                   |

</Tab>
<Tab title="Stufe 2">
| Modell                                                                                       | Maximale Anfragen pro Minute (RPM) | Maximale Eingabe-Token pro Minute (ITPM) | Maximale Ausgabe-Token pro Minute (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 1.000                             | 450.000                                | 90.000                                  |
| Claude Sonnet 3.7 ([veraltet](/docs/de/about-claude/model-deprecations))                     | 1.000                             | 40.000                                 | 16.000                                  |
| Claude Haiku 4.5                                                                             | 1.000                             | 450.000                                | 90.000                                  |
| Claude Haiku 3.5 ([veraltet](/docs/de/about-claude/model-deprecations))                      | 1.000                             | 100.000<sup>†</sup>                    | 20.000                                  |
| Claude Haiku 3                                                                               | 1.000                             | 100.000<sup>†</sup>                    | 20.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 1.000                             | 450.000                                | 90.000                                  |
| Claude Opus 3 ([veraltet](/docs/de/about-claude/model-deprecations))                        | 1.000                             | 40.000<sup>†</sup>                     | 8.000                                   |

</Tab>
<Tab title="Stufe 3">
| Modell                                                                                       | Maximale Anfragen pro Minute (RPM) | Maximale Eingabe-Token pro Minute (ITPM) | Maximale Ausgabe-Token pro Minute (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 2.000                             | 800.000                                | 160.000                                 |
| Claude Sonnet 3.7 ([veraltet](/docs/de/about-claude/model-deprecations))                     | 2.000                             | 80.000                                 | 32.000                                  |
| Claude Haiku 4.5                                                                             | 2.000                             | 1.000.000                              | 200.000                                 |
| Claude Haiku 3.5 ([veraltet](/docs/de/about-claude/model-deprecations))                      | 2.000                             | 200.000<sup>†</sup>                    | 40.000                                  |
| Claude Haiku 3                                                                               | 2.000                             | 200.000<sup>†</sup>                    | 40.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 2.000                             | 800.000                                | 160.000                                 |
| Claude Opus 3 ([veraltet](/docs/de/about-claude/model-deprecations))                        | 2.000                             | 80.000<sup>†</sup>                     | 16.000                                  |

</Tab>
<Tab title="Stufe 4">
| Modell                                                                                       | Maximale Anfragen pro Minute (RPM) | Maximale Eingabe-Token pro Minute (ITPM) | Maximale Ausgabe-Token pro Minute (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 4.000                             | 2.000.000                              | 400.000                                 |
| Claude Sonnet 3.7 ([veraltet](/docs/de/about-claude/model-deprecations))                     | 4.000                             | 200.000                                | 80.000                                  |
| Claude Haiku 4.5                                                                             | 4.000                             | 4.000.000                              | 800.000                                 |
| Claude Haiku 3.5 ([veraltet](/docs/de/about-claude/model-deprecations))                      | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |
| Claude Haiku 3                                                                               | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 4.000                             | 2.000.000                              | 400.000                                 |
| Claude Opus 3 ([veraltet](/docs/de/about-claude/model-deprecations))                        | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |

</Tab>
<Tab title="Benutzerdefiniert">
Wenn Sie höhere Limits für einen Enterprise-Anwendungsfall suchen, kontaktieren Sie den Vertrieb über die [Claude Console](/settings/limits).
</Tab>
</Tabs>

_<sup>* - Das Opus 4.x-Ratenlimit ist ein Gesamtlimit, das für kombinierten Datenverkehr über Opus 4, Opus 4.1 und Opus 4.5 gilt.</sup>_

_<sup>** - Das Sonnet 4.x-Ratenlimit ist ein Gesamtlimit, das für kombinierten Datenverkehr über Sonnet 4 und Sonnet 4.5 gilt.</sup>_

_<sup>† - Limit zählt `cache_read_input_tokens` zur ITPM-Nutzung.</sup>_

### Message Batches API

Die Message Batches API hat ihre eigenen Ratenlimits, die über alle Modelle hinweg geteilt werden. Diese umfassen ein Limit für Anfragen pro Minute (RPM) für alle API-Endpunkte und ein Limit für die Anzahl der Batch-Anfragen, die sich gleichzeitig in der Verarbeitungswarteschlange befinden können. Eine "Batch-Anfrage" bezieht sich hier auf einen Teil eines Message Batch. Sie können einen Message Batch mit Tausenden von Batch-Anfragen erstellen, von denen jede zu diesem Limit zählt. Eine Batch-Anfrage wird als Teil der Verarbeitungswarteschlange betrachtet, wenn sie noch nicht erfolgreich vom Modell verarbeitet wurde.

<Tabs>
<Tab title="Stufe 1">
| Maximale Anfragen pro Minute (RPM) | Maximale Batch-Anfragen in Verarbeitungswarteschlange | Maximale Batch-Anfragen pro Batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 50                                | 100.000                                    | 100.000                          |
</Tab>
<Tab title="Stufe 2">
| Maximale Anfragen pro Minute (RPM) | Maximale Batch-Anfragen in Verarbeitungswarteschlange | Maximale Batch-Anfragen pro Batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 1.000                             | 200.000                                    | 100.000                          |
</Tab>
<Tab title="Stufe 3">
| Maximale Anfragen pro Minute (RPM) | Maximale Batch-Anfragen in Verarbeitungswarteschlange | Maximale Batch-Anfragen pro Batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 2.000                             | 300.000                                    | 100.000                          |
</Tab>
<Tab title="Stufe 4">
| Maximale Anfragen pro Minute (RPM) | Maximale Batch-Anfragen in Verarbeitungswarteschlange | Maximale Batch-Anfragen pro Batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 4.000                             | 500.000                                    | 100.000                          |
</Tab>
<Tab title="Benutzerdefiniert">
Wenn Sie höhere Limits für einen Enterprise-Anwendungsfall suchen, kontaktieren Sie den Vertrieb über die [Claude Console](/settings/limits).
</Tab>
</Tabs>

### Ratenlimits für langen Kontext

Bei Verwendung von Claude Sonnet 4 und Sonnet 4.5 mit [aktiviertem 1M-Token-Kontextfenster](/docs/de/build-with-claude/context-windows#1m-token-context-window) gelten die folgenden dedizierten Ratenlimits für Anfragen, die 200K Token überschreiten.

<Note>
Das 1M-Token-Kontextfenster befindet sich derzeit in der Beta-Phase für Organisationen in Nutzungsstufe 4 und Organisationen mit benutzerdefinierten Ratenlimits. Das 1M-Token-Kontextfenster ist nur für Claude Sonnet 4 und Sonnet 4.5 verfügbar.
</Note>

<Tabs>
<Tab title="Stufe 4">
| Maximale Eingabe-Token pro Minute (ITPM) | Maximale Ausgabe-Token pro Minute (OTPM) |
| -------------------------------------- | --------------------------------------- |
| 1.000.000                              | 200.000                                 |
</Tab>
<Tab title="Benutzerdefiniert">
Für benutzerdefinierte Ratenlimits für langen Kontext in Enterprise-Anwendungsfällen kontaktieren Sie den Vertrieb über die [Claude Console](/settings/limits).
</Tab>
</Tabs>

<Tip>
Um das Beste aus dem 1M-Token-Kontextfenster mit Ratenlimits herauszuholen, verwenden Sie [Prompt-Caching](/docs/de/build-with-claude/prompt-caching).
</Tip>

### Überwachung Ihrer Ratenlimits in der Console

Sie können Ihre Ratenlimit-Nutzung auf der [Nutzungsseite](/settings/usage) der [Claude Console](/) überwachen.

Zusätzlich zu Token- und Anfrage-Diagrammen bietet die Nutzungsseite zwei separate Ratenlimit-Diagramme. Verwenden Sie diese Diagramme, um zu sehen, wie viel Spielraum Sie zum Wachsen haben, wann Sie möglicherweise Spitzenlast erreichen, um besser zu verstehen, welche Ratenlimits Sie anfordern sollten, oder wie Sie Ihre Caching-Raten verbessern können. Die Diagramme visualisieren eine Reihe von Metriken für ein bestimmtes Ratenlimit (z. B. pro Modell):

- Das Diagramm **Ratenlimit - Eingabe-Token** enthält:
  - Stündliches Maximum ungecachter Eingabe-Token pro Minute
  - Ihr aktuelles Eingabe-Token-pro-Minute-Ratenlimit
  - Die Cache-Rate für Ihre Eingabe-Token (d. h. der Prozentsatz der Eingabe-Token, die aus dem Cache gelesen werden)
- Das Diagramm **Ratenlimit - Ausgabe-Token** enthält:
  - Stündliches Maximum Ausgabe-Token pro Minute
  - Ihr aktuelles Ausgabe-Token-pro-Minute-Ratenlimit

## Festlegen niedrigerer Limits für Workspaces

Um Workspaces in Ihrer Organisation vor möglicher Übernutzung zu schützen, können Sie benutzerdefinierte Ausgaben- und Ratenlimits pro Workspace festlegen.

Beispiel: Wenn das Limit Ihrer Organisation 40.000 Eingabe-Token pro Minute und 8.000 Ausgabe-Token pro Minute beträgt, könnten Sie einen Workspace auf 30.000 Gesamttoken pro Minute begrenzen. Dies schützt andere Workspaces vor möglicher Übernutzung und gewährleistet eine gerechtere Ressourcenverteilung in Ihrer Organisation. Die verbleibenden ungenutzten Token pro Minute (oder mehr, wenn dieser Workspace das Limit nicht nutzt) stehen dann anderen Workspaces zur Verfügung.

Hinweis:
- Sie können keine Limits für den Standard-Workspace festlegen.
- Falls nicht festgelegt, entsprechen Workspace-Limits dem Organisationslimit.
- Organisationsweite Limits gelten immer, auch wenn Workspace-Limits zusammen mehr ausmachen.
- Unterstützung für Eingabe- und Ausgabe-Token-Limits wird in Zukunft zu Workspaces hinzugefügt.

## Response-Header

Die API-Antwort enthält Header, die das erzwungene Ratenlimit, die aktuelle Nutzung und den Zeitpunkt des Limit-Zurücksetzen anzeigen.

Die folgenden Header werden zurückgegeben:

| Header                                        | Beschreibung                                                                                                                                     |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `retry-after`                                 | Die Anzahl der Sekunden, die Sie warten müssen, bevor Sie die Anfrage erneut versuchen können. Frühere Versuche schlagen fehl.                   |
| `anthropic-ratelimit-requests-limit`          | Die maximale Anzahl von Anfragen, die innerhalb eines Ratenlimit-Zeitraums zulässig sind.                                                       |
| `anthropic-ratelimit-requests-remaining`      | Die Anzahl der verbleibenden Anfragen, bevor eine Ratenbegrenzung erfolgt.                                                                      |
| `anthropic-ratelimit-requests-reset`          | Der Zeitpunkt, zu dem das Anfrage-Ratenlimit vollständig aufgefüllt wird, im RFC 3339-Format angegeben.                                        |
| `anthropic-ratelimit-tokens-limit`            | Die maximale Anzahl von Token, die innerhalb eines Ratenlimit-Zeitraums zulässig sind.                                                          |
| `anthropic-ratelimit-tokens-remaining`        | Die Anzahl der verbleibenden Token (auf das nächste Tausend gerundet), bevor eine Ratenbegrenzung erfolgt.                                      |
| `anthropic-ratelimit-tokens-reset`            | Der Zeitpunkt, zu dem das Token-Ratenlimit vollständig aufgefüllt wird, im RFC 3339-Format angegeben.                                          |
| `anthropic-ratelimit-input-tokens-limit`      | Die maximale Anzahl von Eingabe-Token, die innerhalb eines Ratenlimit-Zeitraums zulässig sind.                                                  |
| `anthropic-ratelimit-input-tokens-remaining`  | Die Anzahl der verbleibenden Eingabe-Token (auf das nächste Tausend gerundet), bevor eine Ratenbegrenzung erfolgt.                              |
| `anthropic-ratelimit-input-tokens-reset`      | Der Zeitpunkt, zu dem das Eingabe-Token-Ratenlimit vollständig aufgefüllt wird, im RFC 3339-Format angegeben.                                  |
| `anthropic-ratelimit-output-tokens-limit`     | Die maximale Anzahl von Ausgabe-Token, die innerhalb eines Ratenlimit-Zeitraums zulässig sind.                                                  |
| `anthropic-ratelimit-output-tokens-remaining` | Die Anzahl der verbleibenden Ausgabe-Token (auf das nächste Tausend gerundet), bevor eine Ratenbegrenzung erfolgt.                              |
| `anthropic-ratelimit-output-tokens-reset`     | Der Zeitpunkt, zu dem das Ausgabe-Token-Ratenlimit vollständig aufgefüllt wird, im RFC 3339-Format angegeben.                                  |
| `anthropic-priority-input-tokens-limit`       | Die maximale Anzahl von Priority Tier-Eingabe-Token, die innerhalb eines Ratenlimit-Zeitraums zulässig sind. (Nur Priority Tier)                |
| `anthropic-priority-input-tokens-remaining`   | Die Anzahl der verbleibenden Priority Tier-Eingabe-Token (auf das nächste Tausend gerundet), bevor eine Ratenbegrenzung erfolgt. (Nur Priority Tier) |
| `anthropic-priority-input-tokens-reset`       | Der Zeitpunkt, zu dem das Priority Tier-Eingabe-Token-Ratenlimit vollständig aufgefüllt wird, im RFC 3339-Format angegeben. (Nur Priority Tier) |
| `anthropic-priority-output-tokens-limit`      | Die maximale Anzahl von Priority Tier-Ausgabe-Token, die innerhalb eines Ratenlimit-Zeitraums zulässig sind. (Nur Priority Tier)                |
| `anthropic-priority-output-tokens-remaining`  | Die Anzahl der verbleibenden Priority Tier-Ausgabe-Token (auf das nächste Tausend gerundet), bevor eine Ratenbegrenzung erfolgt. (Nur Priority Tier) |
| `anthropic-priority-output-tokens-reset`      | Der Zeitpunkt, zu dem das Priority Tier-Ausgabe-Token-Ratenlimit vollständig aufgefüllt wird, im RFC 3339-Format angegeben. (Nur Priority Tier) |

Die Header `anthropic-ratelimit-tokens-*` zeigen die Werte für das derzeit gültige restriktivste Limit an. Wenn Sie beispielsweise das Workspace-Pro-Minute-Token-Limit überschritten haben, enthalten die Header die Workspace-Pro-Minute-Token-Ratenlimit-Werte. Wenn Workspace-Limits nicht gelten, geben die Header die Gesamttoken zurück, die verbleibend sind, wobei die Summe aus Eingabe- und Ausgabe-Token besteht. Dieser Ansatz stellt sicher, dass Sie Sichtbarkeit in die relevanteste Einschränkung Ihrer aktuellen API-Nutzung haben.