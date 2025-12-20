# Hosting des Agent SDK

Bereitstellung und Hosting des Claude Agent SDK in Produktionsumgebungen

---

Das Claude Agent SDK unterscheidet sich von traditionellen zustandslosen LLM-APIs dadurch, dass es den Konversationszustand beibehält und Befehle in einer persistenten Umgebung ausführt. Dieser Leitfaden behandelt die Architektur, Hosting-Überlegungen und Best Practices für die Bereitstellung von SDK-basierten Agenten in der Produktion.

<Info>
Für Sicherheitshärtung über grundlegende Sandboxing hinaus – einschließlich Netzwerkkontrollen, Credential-Management und Isolationsoptionen – siehe [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment).
</Info>

## Hosting-Anforderungen

### Container-basiertes Sandboxing

Aus Sicherheits- und Isolationsgründen sollte das SDK in einer Sandbox-Container-Umgebung ausgeführt werden. Dies bietet Prozessisolation, Ressourcenlimits, Netzwerkkontrolle und kurzlebige Dateisysteme.

Das SDK unterstützt auch [programmgesteuerte Sandbox-Konfiguration](/docs/de/agent-sdk/typescript#sandbox-settings) für die Befehlsausführung.

### Systemanforderungen

Jede SDK-Instanz erfordert:

- **Laufzeit-Abhängigkeiten**
  - Python 3.10+ (für Python SDK) oder Node.js 18+ (für TypeScript SDK)
  - Node.js (erforderlich von Claude Code CLI)
  - Claude Code CLI: `npm install -g @anthropic-ai/claude-code`

- **Ressourcenzuteilung**
  - Empfohlen: 1GiB RAM, 5GiB Festplatte und 1 CPU (variieren Sie dies je nach Aufgabe nach Bedarf)

- **Netzwerkzugriff**
  - Ausgehend HTTPS zu `api.anthropic.com`
  - Optional: Zugriff auf MCP-Server oder externe Tools

## Verständnis der SDK-Architektur

Im Gegensatz zu zustandslosen API-Aufrufen funktioniert das Claude Agent SDK als ein **langfristiger Prozess**, der:
- **Befehle ausführt** in einer persistenten Shell-Umgebung
- **Dateivorgänge verwaltet** innerhalb eines Arbeitsverzeichnisses
- **Tool-Ausführung handhabt** mit Kontext aus vorherigen Interaktionen

## Sandbox-Provider-Optionen

Mehrere Provider spezialisieren sich auf sichere Container-Umgebungen für die KI-Code-Ausführung:

- **[Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)**
- **[Modal Sandboxes](https://modal.com/docs/guide/sandbox)**
- **[Daytona](https://www.daytona.io/)**
- **[E2B](https://e2b.dev/)**
- **[Fly Machines](https://fly.io/docs/machines/)**
- **[Vercel Sandbox](https://vercel.com/docs/functions/sandbox)**

Für selbst gehostete Optionen (Docker, gVisor, Firecracker) und detaillierte Isolationskonfiguration siehe [Isolationstechnologien](/docs/de/agent-sdk/secure-deployment#isolation-technologies).

## Produktions-Bereitstellungsmuster

### Muster 1: Kurzlebige Sitzungen

Erstellen Sie einen neuen Container für jede Benutzeraufgabe und zerstören Sie ihn nach Abschluss.

Am besten für einmalige Aufgaben, der Benutzer kann weiterhin mit der KI interagieren, während die Aufgabe abgeschlossen wird, aber nach Abschluss wird der Container zerstört.

**Beispiele:**
- Bug-Untersuchung & Behebung: Debuggen und Beheben eines spezifischen Problems mit relevantem Kontext
- Rechnungsverarbeitung: Extrahieren und Strukturieren von Daten aus Quittungen/Rechnungen für Buchhaltungssysteme
- Übersetzungsaufgaben: Übersetzen von Dokumenten oder Inhaltschargen zwischen Sprachen
- Bild-/Videobearbeitung: Anwenden von Transformationen, Optimierungen oder Extrahieren von Metadaten aus Mediendateien

### Muster 2: Langfristige Sitzungen

Behalten Sie persistente Container-Instanzen für langfristige Aufgaben bei. Oft laufen _mehrere_ Claude Agent-Prozesse im Container basierend auf Bedarf.

Am besten für proaktive Agenten, die ohne Benutzereingabe handeln, Agenten, die Inhalte bereitstellen, oder Agenten, die große Mengen an Nachrichten verarbeiten.

**Beispiele:**
- E-Mail-Agent: Überwacht eingehende E-Mails und sortiert, antwortet oder ergreift automatisch Maßnahmen basierend auf dem Inhalt
- Website-Builder: Hostet benutzerdefinierte Websites pro Benutzer mit Live-Bearbeitungsfunktionen, die über Container-Ports bereitgestellt werden
- Hochfrequenz-Chat-Bots: Handhabt kontinuierliche Nachrichtenströme von Plattformen wie Slack, wo schnelle Reaktionszeiten entscheidend sind

### Muster 3: Hybrid-Sitzungen

Kurzlebige Container, die mit Verlauf und Zustand hydratisiert werden, möglicherweise aus einer Datenbank oder aus den Session-Wiederaufnahmefunktionen des SDK.

Am besten für Container mit intermittierender Benutzerinteraktion, die Arbeit startet und herunterfährt, wenn die Arbeit abgeschlossen ist, aber fortgesetzt werden kann.

**Beispiele:**
- Persönlicher Projekt-Manager: Hilft bei der Verwaltung laufender Projekte mit intermittierenden Check-ins, behält den Kontext von Aufgaben, Entscheidungen und Fortschritt
- Tiefe Recherche: Führt mehrstündige Recherchaufgaben durch, speichert Erkenntnisse und setzt die Untersuchung fort, wenn der Benutzer zurückkehrt
- Kundenservice-Agent: Handhabt Support-Tickets, die mehrere Interaktionen umfassen, lädt Ticket-Verlauf und Kundenkontext

### Muster 4: Einzelne Container

Führen Sie mehrere Claude Agent SDK-Prozesse in einem globalen Container aus.

Am besten für Agenten, die eng zusammenarbeiten müssen. Dies ist wahrscheinlich das am wenigsten beliebte Muster, da Sie verhindern müssen, dass Agenten sich gegenseitig überschreiben.

**Beispiele:**
- **Simulationen**: Agenten, die in Simulationen wie Videospielen miteinander interagieren.

# Häufig gestellte Fragen

### Wie kommuniziere ich mit meinen Sandboxes?
Beim Hosting in Containern müssen Sie Ports freigeben, um mit Ihren SDK-Instanzen zu kommunizieren. Ihre Anwendung kann HTTP/WebSocket-Endpunkte für externe Clients freigeben, während das SDK intern im Container ausgeführt wird.

### Was kostet das Hosting eines Containers?
Wir haben festgestellt, dass die dominanten Kosten für die Bereitstellung von Agenten die Token sind, Container variieren je nachdem, was Sie bereitstellen, aber die Mindestkosten liegen bei etwa 5 Cent pro Stunde Betrieb.

### Wann sollte ich untätige Container herunterfahren und wann sollte ich sie warm halten?
Dies ist wahrscheinlich anbieterabhängig, verschiedene Sandbox-Provider ermöglichen es Ihnen, unterschiedliche Kriterien für Leerlauf-Timeouts festzulegen, nach denen eine Sandbox möglicherweise heruntergefahren wird.
Sie sollten diesen Timeout basierend darauf abstimmen, wie häufig Sie denken, dass eine Benutzerantwort sein könnte.

### Wie oft sollte ich die Claude Code CLI aktualisieren?
Die Claude Code CLI wird mit Semver versioniert, daher werden alle Breaking Changes versioniert.

### Wie überwache ich die Container-Integrität und die Agent-Leistung?
Da Container nur Server sind, funktioniert die gleiche Logging-Infrastruktur, die Sie für das Backend verwenden, auch für Container.

### Wie lange kann eine Agent-Sitzung laufen, bevor sie ausfällt?
Eine Agent-Sitzung wird nicht ausfallen, aber wir empfehlen, eine 'maxTurns'-Eigenschaft festzulegen, um zu verhindern, dass Claude in einer Schleife steckenbleibt.

## Nächste Schritte

- [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment) - Netzwerkkontrollen, Credential-Management und Isolationshärtung
- [TypeScript SDK - Sandbox-Einstellungen](/docs/de/agent-sdk/typescript#sandbox-settings) - Konfigurieren Sie die Sandbox programmgesteuert
- [Sessions-Leitfaden](/docs/de/agent-sdk/sessions) - Erfahren Sie mehr über Session-Management
- [Berechtigungen](/docs/de/agent-sdk/permissions) - Konfigurieren Sie Tool-Berechtigungen
- [Kostenverfolgung](/docs/de/agent-sdk/cost-tracking) - Überwachen Sie die API-Nutzung
- [MCP-Integration](/docs/de/agent-sdk/mcp) - Erweitern Sie mit benutzerdefinierten Tools