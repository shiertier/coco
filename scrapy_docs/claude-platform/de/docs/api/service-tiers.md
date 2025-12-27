# Service-Tiers

Verschiedene Service-Tiers ermöglichen es Ihnen, Verfügbarkeit, Leistung und vorhersehbare Kosten basierend auf den Anforderungen Ihrer Anwendung auszugleichen.

---

Wir bieten drei Service-Tiers an:
- **Priority Tier:** Ideal für Workflows, die in der Produktion bereitgestellt werden, wo Zeit, Verfügbarkeit und vorhersehbare Preisgestaltung wichtig sind
- **Standard:** Standard-Tier für Pilotprojekte und Skalierung alltäglicher Anwendungsfälle
- **Batch:** Ideal für asynchrone Workflows, die warten können oder von der Nutzung außerhalb Ihrer normalen Kapazität profitieren

## Standard Tier

Das Standard-Tier ist das Standard-Service-Tier für alle API-Anfragen. Anfragen in diesem Tier werden zusammen mit allen anderen Anfragen priorisiert und unterliegen einer Best-Effort-Verfügbarkeit.

## Priority Tier

Anfragen in diesem Tier werden gegenüber allen anderen Anfragen an Anthropic priorisiert. Diese Priorisierung hilft, ["Server überlastet"-Fehler](/docs/de/api/errors#http-errors) zu minimieren, auch während Spitzenzeiten.

Weitere Informationen finden Sie unter [Erste Schritte mit Priority Tier](#get-started-with-priority-tier)

## Wie Anfragen Tiers zugewiesen werden

Bei der Verarbeitung einer Anfrage weist Anthropic eine Anfrage dem Priority Tier in den folgenden Szenarien zu:
- Ihre Organisation hat ausreichende Priority Tier-Kapazität **input** Token pro Minute
- Ihre Organisation hat ausreichende Priority Tier-Kapazität **output** Token pro Minute

Anthropic zählt die Nutzung gegen Priority Tier-Kapazität wie folgt:

**Input Tokens**
- Cache-Lesevorgänge als 0,1 Token pro Token, der aus dem Cache gelesen wird
- Cache-Schreibvorgänge als 1,25 Token pro Token, der in den Cache mit einer TTL von 5 Minuten geschrieben wird
- Cache-Schreibvorgänge als 2,00 Token pro Token, der in den Cache mit einer TTL von 1 Stunde geschrieben wird
- Für [Long-Context](/docs/de/build-with-claude/context-windows) (>200k Input-Token) Anfragen sind Input-Token 2 Token pro Token
- Alle anderen Input-Token sind 1 Token pro Token

**Output Tokens**
- Für [Long-Context](/docs/de/build-with-claude/context-windows) (>200k Input-Token) Anfragen sind Output-Token 1,5 Token pro Token
- Alle anderen Output-Token sind 1 Token pro Token

Andernfalls werden Anfragen im Standard-Tier verarbeitet.

<Note>
Anfragen, die dem Priority Tier zugewiesen sind, nutzen sowohl die Priority Tier-Kapazität als auch die regulären Rate Limits.
Wenn die Verarbeitung der Anfrage die Rate Limits überschreiten würde, wird die Anfrage abgelehnt.
</Note>

## Verwendung von Service-Tiers

Sie können steuern, welche Service-Tiers für eine Anfrage verwendet werden können, indem Sie den Parameter `service_tier` setzen:

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # Automatically use Priority Tier when available, fallback to standard
)
```

Der Parameter `service_tier` akzeptiert die folgenden Werte:

- `"auto"` (Standard) - Verwendet die Priority Tier-Kapazität, falls verfügbar, und fällt auf Ihre andere Kapazität zurück, falls nicht
- `"standard_only"` - Verwenden Sie nur Standard-Tier-Kapazität, nützlich, wenn Sie Ihre Priority Tier-Kapazität nicht nutzen möchten

Das Antwortobjekt `usage` enthält auch das Service-Tier, das der Anfrage zugewiesen wurde:

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
Dies ermöglicht es Ihnen, zu bestimmen, welches Service-Tier der Anfrage zugewiesen wurde.

Beim Anfordern von `service_tier="auto"` mit einem Modell mit Priority Tier-Verpflichtung bieten diese Antwortheader Einblicke:
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
Sie können das Vorhandensein dieser Header verwenden, um zu erkennen, ob Ihre Anfrage für Priority Tier berechtigt war, auch wenn sie über dem Limit lag.

## Erste Schritte mit Priority Tier

Sie möchten möglicherweise Priority Tier-Kapazität in Anspruch nehmen, wenn Sie interessiert sind an:
- **Höhere Verfügbarkeit**: Ziel von 99,5% Verfügbarkeit mit priorisierten Rechenressourcen
- **Kostenkontrolle**: Vorhersehbare Ausgaben und Rabatte für längere Verpflichtungen
- **Flexible Überlauf**: Automatisches Fallback auf Standard-Tier, wenn Sie Ihre zugesagte Kapazität überschreiten

Die Verpflichtung zu Priority Tier beinhaltet die Entscheidung über:
- Eine Anzahl von Input-Token pro Minute
- Eine Anzahl von Output-Token pro Minute
- Eine Verpflichtungsdauer (1, 3, 6 oder 12 Monate)
- Eine spezifische Modellversion

<Note>
Das Verhältnis von Input- zu Output-Token, die Sie kaufen, ist wichtig. Die Dimensionierung Ihrer Priority Tier-Kapazität, um sie an Ihre tatsächlichen Verkehrsmuster auszurichten, hilft Ihnen, die Auslastung Ihrer gekauften Token zu maximieren.
</Note>

### Unterstützte Modelle

Priority Tier wird unterstützt von:

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7 ([veraltet](/docs/de/about-claude/model-deprecations))
- Claude Haiku 3.5 ([veraltet](/docs/de/about-claude/model-deprecations))

Weitere Informationen zu unseren Modellen finden Sie auf der [Modellübersichtsseite](/docs/de/about-claude/models/overview).

### Wie Sie auf Priority Tier zugreifen

Um Priority Tier zu nutzen:

1. [Kontaktieren Sie den Vertrieb](https://claude.com/contact-sales/priority-tier), um die Bereitstellung abzuschließen
2. (Optional) Aktualisieren Sie Ihre API-Anfragen, um optional den Parameter `service_tier` auf `auto` zu setzen
3. Überwachen Sie Ihre Nutzung über Antwortheader und die Claude Console