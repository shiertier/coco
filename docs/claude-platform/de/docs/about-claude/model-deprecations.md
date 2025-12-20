# Modell-Abschreibungen

Informationen über veraltete Anthropic-Modelle, deren Status und empfohlene Ersatzmodelle.

---

Während wir sicherere und leistungsfähigere Modelle einführen, ziehen wir regelmäßig ältere Modelle zurück. Anwendungen, die auf Anthropic-Modelle angewiesen sind, müssen möglicherweise gelegentlich aktualisiert werden, um weiterhin zu funktionieren. Betroffene Kunden werden immer per E-Mail und in unserer Dokumentation benachrichtigt.

Diese Seite listet alle API-Abschreibungen zusammen mit empfohlenen Ersatzmodellen auf.

## Übersicht

Anthropic verwendet die folgenden Begriffe, um den Lebenszyklus unserer Modelle zu beschreiben:
- **Aktiv**: Das Modell wird vollständig unterstützt und zur Verwendung empfohlen.
- **Legacy**: Das Modell erhält keine Updates mehr und kann in Zukunft abgeschrieben werden.
- **Veraltet**: Das Modell ist nicht mehr für neue Kunden verfügbar, bleibt aber für bestehende Benutzer bis zur Einstellung verfügbar. Wir weisen an diesem Punkt ein Einstellungsdatum zu.
- **Eingestellt**: Das Modell ist nicht mehr verfügbar. Anfragen an eingestellte Modelle schlagen fehl.

<Warning>
Bitte beachten Sie, dass veraltete Modelle wahrscheinlich weniger zuverlässig sind als aktive Modelle. Wir empfehlen Ihnen dringend, Workloads auf aktive Modelle zu verschieben, um das höchste Maß an Unterstützung und Zuverlässigkeit zu gewährleisten.
</Warning>

## Migration zu Ersatzmodellen

Sobald ein Modell veraltet ist, migrieren Sie bitte alle Verwendungen zu einem geeigneten Ersatzmodell vor dem Einstellungsdatum. Anfragen an Modelle nach dem Einstellungsdatum schlagen fehl.

Um die Leistung von Ersatzmodellen bei Ihren Aufgaben zu messen, empfehlen wir gründliche Tests Ihrer Anwendungen mit den neuen Modellen lange vor dem Einstellungsdatum.

Spezifische Anweisungen zur Migration von Claude 3.7 zu Claude 4.5-Modellen finden Sie unter [Migration zu Claude 4.5](/docs/de/about-claude/models/migrating-to-claude-4).

## Benachrichtigungen

Anthropic benachrichtigt Kunden mit aktiven Bereitstellungen für Modelle mit bevorstehenden Einstellungen. Wir geben mindestens 60 Tage Vorankündigung vor der Modelleinstellung für öffentlich freigegebene Modelle.

## Audit der Modellnutzung

Um die Verwendung veralteter Modelle zu identifizieren, können Kunden auf ein Audit ihrer API-Nutzung zugreifen. Führen Sie diese Schritte aus:

1. Gehen Sie zur Seite [Nutzung](/settings/usage) in der Konsole
2. Klicken Sie auf die Schaltfläche "Exportieren"
3. Überprüfen Sie die heruntergeladene CSV-Datei, um die Nutzung aufgeschlüsselt nach API-Schlüssel und Modell zu sehen

Dieses Audit hilft Ihnen, alle Instanzen zu lokalisieren, in denen Ihre Anwendung noch veraltete Modelle verwendet, sodass Sie Updates auf neuere Modelle vor dem Einstellungsdatum priorisieren können.

## Best Practices

1. Überprüfen Sie regelmäßig unsere Dokumentation auf Updates zu Modellabschreibungen.
2. Testen Sie Ihre Anwendungen mit neueren Modellen lange vor dem Einstellungsdatum Ihres aktuellen Modells.
3. Aktualisieren Sie Ihren Code, um das empfohlene Ersatzmodell so bald wie möglich zu verwenden.
4. Kontaktieren Sie unser Support-Team, wenn Sie Hilfe bei der Migration benötigen oder Fragen haben.

## Nachteile der Abschreibung und Minderungsmaßnahmen

Wir schreiben derzeit Modelle ab und stellen sie ein, um Kapazität für neue Modellveröffentlichungen zu gewährleisten. Wir erkennen an, dass dies mit Nachteilen verbunden ist:
- Benutzer, die bestimmte Modelle schätzen, müssen zu neuen Versionen migrieren
- Forscher verlieren Zugriff auf Modelle für laufende und vergleichende Studien
- Die Modelleinstellung führt zu Sicherheits- und Modellwohlfahrtsrisiken

Irgendwann hoffen wir, vergangene Modelle wieder öffentlich verfügbar zu machen. In der Zwischenzeit haben wir uns zur langfristigen Bewahrung von Modellgewichten und anderen Maßnahmen verpflichtet, um diese Auswirkungen zu mindern. Weitere Details finden Sie unter [Zusagen zur Modellabschreibung und Bewahrung](https://www.anthropic.com/research/deprecation-commitments).

## Modellstatus

Alle öffentlich freigegebenen Modelle sind unten mit ihrem Status aufgelistet:

| API-Modellname              | Aktueller Status    | Veraltet          | Vorläufiges Einstellungsdatum |
|:----------------------------|:--------------------|:------------------|:-------------------------|
| `claude-3-opus-20240229`    | Veraltet            | 30. Juni 2025     | 5. Januar 2026          |
| `claude-3-haiku-20240307`   | Aktiv               | N/A               | Nicht vor dem 7. März 2025 |
| `claude-3-5-haiku-20241022` | Veraltet            | 19. Dezember 2025 | 19. Februar 2026          |
| `claude-3-7-sonnet-20250219`| Veraltet            | 28. Oktober 2025  | 19. Februar 2026          |
| `claude-sonnet-4-20250514`  | Aktiv               | N/A               | Nicht vor dem 14. Mai 2026 |
| `claude-opus-4-20250514`    | Aktiv               | N/A               | Nicht vor dem 14. Mai 2026 |
| `claude-opus-4-1-20250805`  | Aktiv               | N/A               | Nicht vor dem 5. August 2026 |
| `claude-sonnet-4-5-20250929`| Aktiv               | N/A               | Nicht vor dem 29. September 2026 |
| `claude-haiku-4-5-20251001` | Aktiv               | N/A               | Nicht vor dem 15. Oktober 2026 |
| `claude-opus-4-5-20251101`  | Aktiv               | N/A               | Nicht vor dem 24. November 2026 |

## Abschreibungsverlauf

Alle Abschreibungen sind unten aufgelistet, mit den neuesten Ankündigungen oben.

### 2025-12-19: Claude Haiku 3.5-Modell

Am 19. Dezember 2025 benachrichtigten wir Entwickler, die das Claude Haiku 3.5-Modell verwenden, über die bevorstehende Einstellung in der Claude API.

| Einstellungsdatum           | Veraltetes Modell           | Empfohlener Ersatz              |
|:----------------------------|:----------------------------|:--------------------------------|
| 19. Februar 2026            | `claude-3-5-haiku-20241022` | `claude-haiku-4-5-20251001`     |

### 2025-10-28: Claude Sonnet 3.7-Modell

Am 28. Oktober 2025 benachrichtigten wir Entwickler, die das Claude Sonnet 3.7-Modell verwenden, über die bevorstehende Einstellung in der Claude API.

| Einstellungsdatum           | Veraltetes Modell           | Empfohlener Ersatz              |
|:----------------------------|:----------------------------|:--------------------------------|
| 19. Februar 2026            | `claude-3-7-sonnet-20250219`| `claude-sonnet-4-5-20250929`     |

### 2025-08-13: Claude Sonnet 3.5-Modelle

<Note>
Diese Modelle wurden am 28. Oktober 2025 eingestellt.
</Note>

Am 13. August 2025 benachrichtigten wir Entwickler, die Claude Sonnet 3.5-Modelle verwenden, über die bevorstehende Einstellung.

| Einstellungsdatum           | Veraltetes Modell           | Empfohlener Ersatz              |
|:----------------------------|:----------------------------|:--------------------------------|
| 28. Oktober 2025            | `claude-3-5-sonnet-20240620`| `claude-sonnet-4-5-20250929`     |
| 28. Oktober 2025            | `claude-3-5-sonnet-20241022`| `claude-sonnet-4-5-20250929`     |

### 2025-06-30: Claude Opus 3-Modell

Am 30. Juni 2025 benachrichtigten wir Entwickler, die das Claude Opus 3-Modell verwenden, über die bevorstehende Einstellung.

| Einstellungsdatum           | Veraltetes Modell           | Empfohlener Ersatz              |
|:----------------------------|:----------------------------|:--------------------------------|
| 5. Januar 2026              | `claude-3-opus-20240229`    | `claude-opus-4-1-20250805`      |

### 2025-01-21: Claude 2, Claude 2.1 und Claude Sonnet 3-Modelle

<Note>
Diese Modelle wurden am 21. Juli 2025 eingestellt.
</Note>

Am 21. Januar 2025 benachrichtigten wir Entwickler, die Claude 2, Claude 2.1 und Claude Sonnet 3-Modelle verwenden, über die bevorstehenden Einstellungen.

| Einstellungsdatum           | Veraltetes Modell           | Empfohlener Ersatz              |
|:----------------------------|:----------------------------|:--------------------------------|
| 21. Juli 2025               | `claude-2.0`                | `claude-sonnet-4-5-20250929`      |
| 21. Juli 2025               | `claude-2.1`                | `claude-sonnet-4-5-20250929`      |
| 21. Juli 2025               | `claude-3-sonnet-20240229`  | `claude-sonnet-4-5-20250929`      |

### 2024-09-04: Claude 1 und Instant-Modelle

<Note>
Diese Modelle wurden am 6. November 2024 eingestellt.
</Note>

Am 4. September 2024 benachrichtigten wir Entwickler, die Claude 1 und Instant-Modelle verwenden, über die bevorstehenden Einstellungen.

| Einstellungsdatum           | Veraltetes Modell         | Empfohlener Ersatz         |
|:----------------------------|:--------------------------|:---------------------------|
| 6. November 2024            | `claude-1.0`              | `claude-haiku-4-5-20251001`|
| 6. November 2024            | `claude-1.1`              | `claude-haiku-4-5-20251001`|
| 6. November 2024            | `claude-1.2`              | `claude-haiku-4-5-20251001`|
| 6. November 2024            | `claude-1.3`              | `claude-haiku-4-5-20251001`|
| 6. November 2024            | `claude-instant-1.0`      | `claude-haiku-4-5-20251001`|
| 6. November 2024            | `claude-instant-1.1`      | `claude-haiku-4-5-20251001`|
| 6. November 2024            | `claude-instant-1.2`      | `claude-haiku-4-5-20251001`|