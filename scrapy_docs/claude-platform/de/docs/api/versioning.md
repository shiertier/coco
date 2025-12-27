# Versionen

Beim Stellen von API-Anfragen müssen Sie einen `anthropic-version` Request-Header senden. Zum Beispiel `anthropic-version: 2023-06-01`. Wenn Sie unsere [Client-SDKs](/docs/de/api/client-sdks) verwenden, wird dies automatisch für Sie erledigt.

---

Für jede gegebene API-Version werden wir Folgendes beibehalten:

* Bestehende Eingabeparameter
* Bestehende Ausgabeparameter

Wir können jedoch Folgendes tun:

* Zusätzliche optionale Eingaben hinzufügen
* Zusätzliche Werte zur Ausgabe hinzufügen
* Bedingungen für spezifische Fehlertypen ändern
* Neue Varianten zu enum-ähnlichen Ausgabewerten hinzufügen (zum Beispiel Streaming-Event-Typen)

Generell, wenn Sie die API wie in dieser Referenz dokumentiert verwenden, werden wir Ihre Nutzung nicht beeinträchtigen.

## Versionshistorie

Wir empfehlen immer, wann immer möglich die neueste API-Version zu verwenden. Frühere Versionen gelten als veraltet und sind möglicherweise für neue Benutzer nicht verfügbar.

* `2023-06-01`  
   * Neues Format für [Streaming](/docs/de/api/streaming) Server-Sent Events (SSE):  
         * Vervollständigungen sind inkrementell. Zum Beispiel `" Hello"`, `" my"`, `" name"`, `" is"`, `" Claude." ` anstatt `" Hello"`, `" Hello my"`, `" Hello my name"`, `" Hello my name is"`, `" Hello my name is Claude."`.  
         * Alle Events sind [benannte Events](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents), anstatt [nur-Daten-Events](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages).  
         * Unnötiges `data: [DONE]` Event entfernt.  
   * Veraltete `exception` und `truncated` Werte in Antworten entfernt.
* `2023-01-01`: Erste Veröffentlichung.