# Sichere Bereitstellung von KI-Agenten

Ein Leitfaden zur Sicherung von Claude Code und Agent SDK-Bereitstellungen mit Isolation, Verwaltung von Anmeldedaten und Netzwerkkontrollen

---

Claude Code und das Agent SDK sind leistungsstarke Tools, die Code ausführen, auf Dateien zugreifen und externe Dienste in Ihrem Namen nutzen können. Wie jedes Tool mit diesen Fähigkeiten stellt eine durchdachte Bereitstellung sicher, dass Sie die Vorteile nutzen und gleichzeitig angemessene Kontrollen beibehalten.

Im Gegensatz zu traditioneller Software, die vordefinierten Code-Pfaden folgt, generieren diese Tools ihre Aktionen dynamisch basierend auf Kontext und Zielen. Diese Flexibilität macht sie nützlich, aber es bedeutet auch, dass ihr Verhalten durch den Inhalt beeinflusst werden kann, den sie verarbeiten: Dateien, Webseiten oder Benutzereingaben. Dies wird manchmal als Prompt Injection bezeichnet. Wenn beispielsweise die README eines Repositorys ungewöhnliche Anweisungen enthält, könnte Claude Code diese auf Weise in seine Aktionen einbeziehen, die der Operator nicht erwartet hat. Dieser Leitfaden behandelt praktische Wege, um dieses Risiko zu verringern.

Die gute Nachricht ist, dass die Sicherung einer Agent-Bereitstellung keine exotische Infrastruktur erfordert. Die gleichen Prinzipien, die für die Ausführung von halbvertrautem Code gelten, gelten hier: Isolation, Prinzip der geringsten Berechtigung und Defense in Depth. Claude Code enthält mehrere Sicherheitsfunktionen, die bei häufigen Problemen helfen, und dieser Leitfaden behandelt diese zusammen mit zusätzlichen Härtungsoptionen für diejenigen, die sie benötigen.

Nicht jede Bereitstellung benötigt maximale Sicherheit. Ein Entwickler, der Claude Code auf seinem Laptop ausführt, hat andere Anforderungen als ein Unternehmen, das Kundendaten in einer Multi-Tenant-Umgebung verarbeitet. Dieser Leitfaden präsentiert Optionen, die von Claude Codes integrierten Sicherheitsfunktionen bis zu gehärteten Produktionsarchitekturen reichen, damit Sie wählen können, was zu Ihrer Situation passt.

## Wogegen schützen wir uns?

Agenten können unbeabsichtigte Aktionen aufgrund von Prompt Injection (Anweisungen, die in Inhalte eingebettet sind, die sie verarbeiten) oder Modellfehler durchführen. Claude-Modelle sind so konzipiert, dass sie dagegen resistent sind, und wie wir in unserer [Modellkarte](https://assets.anthropic.com/m/64823ba7485345a7/Claude-Opus-4-5-System-Card.pdf) analysiert haben, glauben wir, dass Claude Opus 4.5 das robusteste Frontier-Modell ist, das verfügbar ist.

Defense in Depth ist dennoch eine gute Praxis. Wenn beispielsweise ein Agent eine bösartige Datei verarbeitet, die ihn anweist, Kundendaten an einen externen Server zu senden, können Netzwerkkontrollen diese Anfrage vollständig blockieren.

## Integrierte Sicherheitsfunktionen

Claude Code enthält mehrere Sicherheitsfunktionen, die häufige Bedenken adressieren. Siehe die [Sicherheitsdokumentation](https://code.claude.com/docs/de/security) für vollständige Details.

- **Berechtigungssystem**: Jedes Tool und jeder Bash-Befehl kann so konfiguriert werden, dass er erlaubt, blockiert oder der Benutzer zur Genehmigung aufgefordert wird. Verwenden Sie Glob-Muster, um Regeln wie „alle npm-Befehle erlauben" oder „jeden Befehl mit sudo blockieren" zu erstellen. Organisationen können Richtlinien festlegen, die für alle Benutzer gelten. Siehe [Zugriffskontrolle und Berechtigungen](https://code.claude.com/docs/de/iam#access-control-and-permissions).
- **Statische Analyse**: Vor der Ausführung von Bash-Befehlen führt Claude Code eine statische Analyse durch, um potenziell riskante Operationen zu identifizieren. Befehle, die Systemdateien ändern oder auf sensible Verzeichnisse zugreifen, werden gekennzeichnet und erfordern explizite Benutzerbestätigung.
- **Web-Suchzusammenfassung**: Suchergebnisse werden zusammengefasst, anstatt rohe Inhalte direkt in den Kontext zu übergeben, was das Risiko von Prompt Injection aus bösartigen Webinhalten verringert.
- **Sandbox-Modus**: Bash-Befehle können in einer Sandbox-Umgebung ausgeführt werden, die Dateisystem- und Netzwerkzugriff einschränkt. Siehe die [Sandboxing-Dokumentation](https://code.claude.com/docs/de/sandboxing) für Details.

## Sicherheitsprinzipien

Für Bereitstellungen, die zusätzliche Härtung über Claude Codes Standardeinstellungen hinaus erfordern, leiten diese Prinzipien die verfügbaren Optionen.

### Sicherheitsgrenzen

Eine Sicherheitsgrenze trennt Komponenten mit unterschiedlichen Vertrauensstufen. Für hochsichere Bereitstellungen können Sie sensible Ressourcen (wie Anmeldedaten) außerhalb der Grenze platzieren, die den Agenten enthält. Wenn etwas in der Umgebung des Agenten schiefgeht, bleiben Ressourcen außerhalb dieser Grenze geschützt.

Anstatt einem Agenten direkten Zugriff auf einen API-Schlüssel zu geben, könnten Sie beispielsweise einen Proxy außerhalb der Umgebung des Agenten ausführen, der den Schlüssel in Anfragen einfügt. Der Agent kann API-Aufrufe tätigen, sieht aber niemals die Anmeldedaten selbst. Dieses Muster ist nützlich für Multi-Tenant-Bereitstellungen oder bei der Verarbeitung nicht vertrauenswürdiger Inhalte.

### Prinzip der geringsten Berechtigung

Bei Bedarf können Sie den Agenten auf nur die Fähigkeiten beschränken, die für seine spezifische Aufgabe erforderlich sind:

| Ressource | Einschränkungsoptionen |
|----------|---------------------|
| Dateisystem | Nur benötigte Verzeichnisse bereitstellen, schreibgeschützt bevorzugen |
| Netzwerk | Auf spezifische Endpunkte über Proxy beschränken |
| Anmeldedaten | Über Proxy einfügen anstatt direkt freizulegen |
| Systemfähigkeiten | Linux-Fähigkeiten in Containern deaktivieren |

### Defense in Depth

Für hochsichere Umgebungen bietet das Schichten mehrerer Kontrollen zusätzlichen Schutz. Optionen umfassen:

- Container-Isolation
- Netzwerkbeschränkungen
- Dateisystemkontrollen
- Anforderungsvalidierung bei einem Proxy

Die richtige Kombination hängt von Ihrem Bedrohungsmodell und Ihren operativen Anforderungen ab.

## Isolationstechnologien

Verschiedene Isolationstechnologien bieten unterschiedliche Kompromisse zwischen Sicherheitsstärke, Leistung und operativer Komplexität.

<Info>
In all diesen Konfigurationen läuft Claude Code (oder Ihre Agent SDK-Anwendung) innerhalb der Isolationsgrenze—der Sandbox, dem Container oder der VM. Die unten beschriebenen Sicherheitskontrollen beschränken, worauf der Agent von innerhalb dieser Grenze zugreifen kann.
</Info>

| Technologie | Isolationsstärke | Leistungsaufwand | Komplexität |
|------------|-------------------|---------------------|------------|
| Sandbox-Laufzeit | Gut (sichere Standardeinstellungen) | Sehr niedrig | Niedrig |
| Container (Docker) | Abhängig von Setup | Niedrig | Mittel |
| gVisor | Ausgezeichnet (mit korrektem Setup) | Mittel/Hoch | Mittel |
| VMs (Firecracker, QEMU) | Ausgezeichnet (mit korrektem Setup) | Hoch | Mittel/Hoch |

### Sandbox-Laufzeit

Für leichte Isolation ohne Container erzwingt [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) Dateisystem- und Netzwerkbeschränkungen auf OS-Ebene.

Der Hauptvorteil ist Einfachheit: keine Docker-Konfiguration, Container-Images oder Netzwerk-Setup erforderlich. Der Proxy und die Dateisystembeschränkungen sind integriert. Sie stellen eine Einstellungsdatei bereit, die zulässige Domänen und Pfade angibt.

**Wie es funktioniert:**
- **Dateisystem**: Verwendet OS-Primitive (`bubblewrap` auf Linux, `sandbox-exec` auf macOS), um Lese-/Schreibzugriff auf konfigurierte Pfade zu beschränken
- **Netzwerk**: Entfernt Netzwerk-Namespace (Linux) oder verwendet Seatbelt-Profile (macOS), um Netzwerkverkehr durch einen integrierten Proxy zu leiten
- **Konfiguration**: JSON-basierte Zulassungslisten für Domänen und Dateisystempfade

**Setup:**
```bash
npm install @anthropic-ai/sandbox-runtime
```

Erstellen Sie dann eine Konfigurationsdatei, die zulässige Pfade und Domänen angibt.

**Sicherheitsüberlegungen:**

1. **Gleicher Host-Kernel**: Im Gegensatz zu VMs teilen sich Sandbox-Prozesse den Host-Kernel. Eine Kernel-Sicherheitslücke könnte theoretisch einen Escape ermöglichen. Für einige Bedrohungsmodelle ist dies akzeptabel, aber wenn Sie Kernel-Ebenen-Isolation benötigen, verwenden Sie gVisor oder eine separate VM.

2. **Keine TLS-Inspektion**: Der Proxy erstellt eine Zulassungsliste für Domänen, inspiziert aber keinen verschlüsselten Verkehr. Wenn der Agent permissive Anmeldedaten für eine zulässige Domäne hat, stellen Sie sicher, dass es nicht möglich ist, diese Domäne zu verwenden, um andere Netzwerkanfragen auszulösen oder Daten zu exfiltrieren.

Für viele Single-Developer- und CI/CD-Anwendungsfälle erhöht sandbox-runtime die Messlatte erheblich mit minimalem Setup. Die folgenden Abschnitte behandeln Container und VMs für Bereitstellungen, die stärkere Isolation erfordern.

### Container

Container bieten Isolation durch Linux-Namespaces. Jeder Container hat seine eigene Ansicht des Dateisystems, des Prozessbaums und des Netzwerk-Stacks, während er den Host-Kernel teilt.

Eine sicherheitshärtete Container-Konfiguration könnte so aussehen:

```bash
docker run \
  --cap-drop ALL \
  --security-opt no-new-privileges \
  --security-opt seccomp=/path/to/seccomp-profile.json \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /home/agent:rw,noexec,nosuid,size=500m \
  --network none \
  --memory 2g \
  --cpus 2 \
  --pids-limit 100 \
  --user 1000:1000 \
  -v /path/to/code:/workspace:ro \
  -v /var/run/proxy.sock:/var/run/proxy.sock:ro \
  agent-image
```

Hier ist, was jede Option bewirkt:

| Option | Zweck |
|--------|---------|
| `--cap-drop ALL` | Entfernt Linux-Fähigkeiten wie `NET_ADMIN` und `SYS_ADMIN`, die Privilege Escalation ermöglichen könnten |
| `--security-opt no-new-privileges` | Verhindert, dass Prozesse Berechtigungen durch setuid-Binärdateien erhalten |
| `--security-opt seccomp=...` | Beschränkt verfügbare Systemaufrufe; Dockers Standard blockiert ~44, benutzerdefinierte Profile können mehr blockieren |
| `--read-only` | Macht das Root-Dateisystem des Containers unveränderlich und verhindert, dass der Agent Änderungen beibehält |
| `--tmpfs /tmp:...` | Stellt ein beschreibbares temporäres Verzeichnis bereit, das gelöscht wird, wenn der Container stoppt |
| `--network none` | Entfernt alle Netzwerkschnittstellen; der Agent kommuniziert über den unten bereitgestellten Unix-Socket |
| `--memory 2g` | Begrenzt die Speichernutzung, um Ressourcenerschöpfung zu verhindern |
| `--pids-limit 100` | Begrenzt die Prozessanzahl, um Fork-Bomben zu verhindern |
| `--user 1000:1000` | Läuft als nicht-root-Benutzer |
| `-v ...:/workspace:ro` | Stellt Code schreibgeschützt bereit, damit der Agent ihn analysieren, aber nicht ändern kann. **Vermeiden Sie das Bereitstellen sensibler Host-Verzeichnisse wie `~/.ssh`, `~/.aws` oder `~/.config`** |
| `-v .../proxy.sock:...` | Stellt einen Unix-Socket bereit, der mit einem Proxy verbunden ist, der außerhalb des Containers läuft (siehe unten) |

**Unix-Socket-Architektur:**

Mit `--network none` hat der Container überhaupt keine Netzwerkschnittstellen. Die einzige Möglichkeit für den Agenten, die Außenwelt zu erreichen, ist über den bereitgestellten Unix-Socket, der sich mit einem Proxy auf dem Host verbindet. Dieser Proxy kann Domänen-Zulassungslisten durchsetzen, Anmeldedaten einfügen und den gesamten Verkehr protokollieren.

Dies ist die gleiche Architektur, die von [sandbox-runtime](https://github.com/anthropic-experimental/sandbox-runtime) verwendet wird. Selbst wenn der Agent durch Prompt Injection kompromittiert wird, kann er Daten nicht zu beliebigen Servern exfiltrieren—er kann nur über den Proxy kommunizieren, der kontrolliert, welche Domänen erreichbar sind. Für weitere Details siehe den [Claude Code Sandboxing Blog-Beitrag](https://www.anthropic.com/engineering/claude-code-sandboxing).

**Zusätzliche Härtungsoptionen:**

| Option | Zweck |
|--------|---------|
| `--userns-remap` | Ordnet Container-Root einem nicht-privilegierten Host-Benutzer zu; erfordert Daemon-Konfiguration, begrenzt aber Schäden durch Container-Escape |
| `--ipc private` | Isoliert Inter-Process-Kommunikation, um Container-übergreifende Angriffe zu verhindern |

### gVisor

Standard-Container teilen den Host-Kernel: Wenn Code in einem Container einen Systemaufruf tätigt, geht er direkt zum gleichen Kernel, der den Host ausführt. Dies bedeutet, dass eine Kernel-Sicherheitslücke einen Container-Escape ermöglichen könnte. gVisor adressiert dies, indem es Systemaufrufe im Userspace abfängt, bevor sie den Host-Kernel erreichen, und seine eigene Kompatibilitätsschicht implementiert, die die meisten Systemaufrufe ohne Beteiligung des echten Kernels handhabt.

Wenn ein Agent bösartigen Code ausführt (vielleicht aufgrund von Prompt Injection), läuft dieser Code im Container und könnte Kernel-Exploits versuchen. Mit gVisor ist die Angriffsfläche viel kleiner: Der bösartige Code müsste zuerst gVisors Userspace-Implementierung ausnutzen und hätte begrenzten Zugriff auf den echten Kernel.

Um gVisor mit Docker zu verwenden, installieren Sie die `runsc`-Laufzeit und konfigurieren Sie den Daemon:

```json
// /etc/docker/daemon.json
{
  "runtimes": {
    "runsc": {
      "path": "/usr/local/bin/runsc"
    }
  }
}
```

Führen Sie dann Container mit aus:

```bash
docker run --runtime=runsc agent-image
```

**Leistungsüberlegungen:**

| Workload | Aufwand |
|----------|----------|
| CPU-gebundene Berechnung | ~0% (keine Systemaufruf-Abfangung) |
| Einfache Systemaufrufe | ~2× langsamer |
| Dateisystem-intensive E/A | Bis zu 10-200× langsamer für schwere Open/Close-Muster |

Für Multi-Tenant-Umgebungen oder bei der Verarbeitung nicht vertrauenswürdiger Inhalte ist die zusätzliche Isolation oft den Aufwand wert.

### Virtuelle Maschinen

VMs bieten Hardware-Ebenen-Isolation durch CPU-Virtualisierungserweiterungen. Jede VM führt ihren eigenen Kernel aus und schafft eine starke Grenze—eine Sicherheitslücke im Gast-Kernel kompromittiert nicht direkt den Host. VMs sind jedoch nicht automatisch „sicherer" als Alternativen wie gVisor. VM-Sicherheit hängt stark vom Hypervisor und dem Device-Emulationscode ab.

Firecracker ist für leichte MicroVM-Isolation konzipiert—es kann VMs in unter 125ms mit weniger als 5 MiB Speicheraufwand starten und entfernt unnötige Device-Emulation, um die Angriffsfläche zu reduzieren.

Mit diesem Ansatz hat die Agent-VM keine externe Netzwerkschnittstelle. Stattdessen kommuniziert sie über `vsock` (virtuelle Sockets). Der gesamte Verkehr leitet über vsock zu einem Proxy auf dem Host, der Zulassungslisten durchsetzt und Anmeldedaten einfügt, bevor Anfragen weitergeleitet werden.

### Cloud-Bereitstellungen

Für Cloud-Bereitstellungen können Sie jede der oben genannten Isolationstechnologien mit Cloud-nativen Netzwerkkontrollen kombinieren:

1. Führen Sie Agent-Container in einem privaten Subnetz ohne Internet-Gateway aus
2. Konfigurieren Sie Cloud-Firewall-Regeln (AWS Security Groups, GCP VPC Firewall), um den gesamten Ausgangsverkehr außer zu Ihrem Proxy zu blockieren
3. Führen Sie einen Proxy aus (wie [Envoy](https://www.envoyproxy.io/) mit seinem `credential_injector`-Filter), der Anfragen validiert, Domänen-Zulassungslisten durchsetzt, Anmeldedaten einfügt und an externe APIs weiterleitet
4. Weisen Sie dem Service-Konto des Agenten minimale IAM-Berechtigungen zu und leiten Sie sensiblen Zugriff wenn möglich über den Proxy
5. Protokollieren Sie den gesamten Verkehr beim Proxy für Audit-Zwecke

## Verwaltung von Anmeldedaten

Agenten benötigen oft Anmeldedaten, um APIs aufzurufen, auf Repositorys zuzugreifen oder mit Cloud-Diensten zu interagieren. Die Herausforderung besteht darin, diesen Zugriff bereitzustellen, ohne die Anmeldedaten selbst freizulegen.

### Das Proxy-Muster

Der empfohlene Ansatz ist, einen Proxy außerhalb der Sicherheitsgrenze des Agenten auszuführen, der Anmeldedaten in ausgehende Anfragen einfügt. Der Agent sendet Anfragen ohne Anmeldedaten, der Proxy fügt sie hinzu und leitet die Anfrage an ihr Ziel weiter.

Dieses Muster hat mehrere Vorteile:

1. Der Agent sieht niemals die tatsächlichen Anmeldedaten
2. Der Proxy kann eine Zulassungsliste zulässiger Endpunkte durchsetzen
3. Der Proxy kann alle Anfragen für Audit-Zwecke protokollieren
4. Anmeldedaten werden an einem sicheren Ort gespeichert, anstatt an jeden Agenten verteilt zu werden

### Konfigurieren von Claude Code zur Verwendung eines Proxy

Claude Code unterstützt zwei Methoden zum Leiten von Sampling-Anfragen durch einen Proxy:

**Option 1: ANTHROPIC_BASE_URL (einfach, aber nur für Sampling-API-Anfragen)**

```bash
export ANTHROPIC_BASE_URL="http://localhost:8080"
```

Dies teilt Claude Code und dem Agent SDK mit, Sampling-Anfragen an Ihren Proxy anstatt direkt an die Anthropic API zu senden. Ihr Proxy empfängt Klartext-HTTP-Anfragen, kann diese inspizieren und ändern (einschließlich Einfügen von Anmeldedaten) und leitet dann an die echte API weiter.

**Option 2: HTTP_PROXY / HTTPS_PROXY (systemweit)**

```bash
export HTTP_PROXY="http://localhost:8080"
export HTTPS_PROXY="http://localhost:8080"
```

Claude Code und das Agent SDK respektieren diese Standard-Umgebungsvariablen und leiten den gesamten HTTP-Verkehr durch den Proxy. Für HTTPS erstellt der Proxy einen verschlüsselten CONNECT-Tunnel: Er kann Anfrageinhalte ohne TLS-Inspektion nicht sehen oder ändern.

### Implementieren eines Proxy

Sie können Ihren eigenen Proxy erstellen oder einen vorhandenen verwenden:

- [Envoy Proxy](https://www.envoyproxy.io/) — produktionsreifer Proxy mit `credential_injector`-Filter zum Hinzufügen von Auth-Headern
- [mitmproxy](https://mitmproxy.org/) — TLS-terminierender Proxy zum Inspizieren und Ändern von HTTPS-Verkehr
- [Squid](http://www.squid-cache.org/) — Caching-Proxy mit Zugriffskontrolllisten
- [LiteLLM](https://github.com/BerriAI/litellm) — LLM-Gateway mit Anmeldedaten-Einfügung und Rate Limiting

### Anmeldedaten für andere Dienste

Neben dem Sampling von der Anthropic API benötigen Agenten oft authentifizierten Zugriff auf andere Dienste—Git-Repositorys, Datenbanken, interne APIs. Es gibt zwei Hauptansätze:

#### Benutzerdefinierte Tools

Stellen Sie Zugriff über einen MCP-Server oder ein benutzerdefiniertes Tool bereit, das Anfragen an einen Dienst leitet, der außerhalb der Sicherheitsgrenze des Agenten läuft. Der Agent ruft das Tool auf, aber die tatsächliche authentifizierte Anfrage erfolgt außerhalb—das Tool ruft einen Proxy auf, der die Anmeldedaten einfügt.

Beispielsweise könnte ein Git-MCP-Server Befehle vom Agenten akzeptieren, diese aber an einen Git-Proxy auf dem Host weiterleiten, der Authentifizierung hinzufügt, bevor er das Remote-Repository kontaktiert. Der Agent sieht niemals die Anmeldedaten.

Vorteile:
- **Keine TLS-Inspektion**: Der externe Dienst tätigt authentifizierte Anfragen direkt
- **Anmeldedaten bleiben außerhalb**: Der Agent sieht nur die Tool-Schnittstelle, nicht die zugrunde liegenden Anmeldedaten

#### Verkehrsweiterleitungen

Für Anthropic-API-Aufrufe ermöglicht `ANTHROPIC_BASE_URL` das Leiten von Anfragen an einen Proxy, der diese im Klartext inspizieren und ändern kann. Aber für andere HTTPS-Dienste (GitHub, npm-Registrys, interne APIs) ist der Verkehr oft Ende-zu-Ende verschlüsselt—selbst wenn Sie ihn über einen Proxy über `HTTP_PROXY` leiten, sieht der Proxy nur einen undurchsichtigen TLS-Tunnel und kann keine Anmeldedaten einfügen.

Um HTTPS-Verkehr zu beliebigen Diensten zu ändern, ohne ein benutzerdefiniertes Tool zu verwenden, benötigen Sie einen TLS-terminierenden Proxy, der Verkehr entschlüsselt, inspiziert oder ändert und dann vor der Weiterleitung erneut verschlüsselt. Dies erfordert:

1. Ausführen des Proxy außerhalb des Containers des Agenten
2. Installieren des CA-Zertifikats des Proxy im Vertrauensspeicher des Agenten (damit der Agent den Zertifikaten des Proxy vertraut)
3. Konfigurieren von `HTTP_PROXY`/`HTTPS_PROXY` zum Leiten von Verkehr durch den Proxy

Dieser Ansatz handhabt jeden HTTP-basierten Dienst ohne das Schreiben benutzerdefinierter Tools, fügt aber Komplexität um die Zertifikatverwaltung hinzu.

Beachten Sie, dass nicht alle Programme `HTTP_PROXY`/`HTTPS_PROXY` respektieren. Die meisten Tools (curl, pip, npm, git) tun dies, aber einige könnten diese Variablen umgehen und direkt verbinden. Beispielsweise ignoriert Node.js `fetch()` diese Variablen standardmäßig; in Node 24+ können Sie `NODE_USE_ENV_PROXY=1` setzen, um Unterstützung zu aktivieren. Für umfassende Abdeckung können Sie [proxychains](https://github.com/haad/proxychains) verwenden, um Netzwerkaufrufe abzufangen, oder iptables konfigurieren, um ausgehenden Verkehr zu einem transparenten Proxy umzuleiten.

<Info>
Ein **transparenter Proxy** fängt Verkehr auf Netzwerk-Ebene ab, daher muss der Client nicht konfiguriert werden, um ihn zu verwenden. Reguläre Proxies erfordern, dass Clients explizit verbinden und HTTP CONNECT oder SOCKS sprechen. Transparente Proxies (wie Squid oder mitmproxy im transparenten Modus) können rohe umgeleitete TCP-Verbindungen handhaben.
</Info>

Beide Ansätze erfordern immer noch den TLS-terminierenden Proxy und das vertraute CA-Zertifikat—sie stellen nur sicher, dass Verkehr tatsächlich den Proxy erreicht.

## Dateisystem-Konfiguration

Dateisystem-Kontrollen bestimmen, welche Dateien der Agent lesen und schreiben kann.

### Schreibgeschützte Code-Bereitstellung

Wenn der Agent Code analysieren, aber nicht ändern muss, stellen Sie das Verzeichnis schreibgeschützt bereit:

```bash
docker run -v /path/to/code:/workspace:ro agent-image
```

<Warning>
Selbst schreibgeschützter Zugriff auf ein Code-Verzeichnis kann Anmeldedaten freigeben. Häufige Dateien zum Ausschließen oder Bereinigen vor der Bereitstellung:

| Datei | Risiko |
|------|------|
| `.env`, `.env.local` | API-Schlüssel, Datenbankpasswörter, Geheimnisse |
| `~/.git-credentials` | Git-Passwörter/Tokens im Klartext |
| `~/.aws/credentials` | AWS-Zugriffschlüssel |
| `~/.config/gcloud/application_default_credentials.json` | Google Cloud ADC-Tokens |
| `~/.azure/` | Azure CLI-Anmeldedaten |
| `~/.docker/config.json` | Docker-Registry-Auth-Tokens |
| `~/.kube/config` | Kubernetes-Cluster-Anmeldedaten |
| `.npmrc`, `.pypirc` | Package-Registry-Tokens |
| `*-service-account.json` | GCP-Service-Account-Schlüssel |
| `*.pem`, `*.key` | Private Schlüssel |

Erwägen Sie, nur die benötigten Quelldateien zu kopieren oder `.dockerignore`-ähnliche Filterung zu verwenden.
</Warning>

### Beschreibbare Orte

Wenn der Agent Dateien schreiben muss, haben Sie je nachdem, ob Sie möchten, dass Änderungen bestehen bleiben, einige Optionen:

Für ephemere Workspaces in Containern verwenden Sie `tmpfs`-Bereitstellungen, die nur im Speicher vorhanden sind und gelöscht werden, wenn der Container stoppt:

```bash
docker run \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,size=100m \
  --tmpfs /workspace:rw,noexec,size=500m \
  agent-image
```

Wenn Sie Änderungen vor dem Beibehalten überprüfen möchten, ermöglicht ein Overlay-Dateisystem dem Agenten zu schreiben, ohne zugrunde liegende Dateien zu ändern—Änderungen werden in einer separaten Schicht gespeichert, die Sie inspizieren, anwenden oder verwerfen können. Für vollständig persistente Ausgabe stellen Sie ein dediziertes Volume bereit, halten es aber getrennt von sensiblen Verzeichnissen.

## Weitere Lektüre

- [Claude Code Sicherheitsdokumentation](https://code.claude.com/docs/de/security)
- [Hosting des Agent SDK](/docs/de/agent-sdk/hosting)
- [Umgang mit Berechtigungen](/docs/de/agent-sdk/permissions)
- [Sandbox-Laufzeit](https://github.com/anthropic-experimental/sandbox-runtime)
- [The Lethal Trifecta for AI Agents](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
- [OWASP Top 10 for LLM Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [gVisor Documentation](https://gvisor.dev/docs/)
- [Firecracker Documentation](https://firecracker-microvm.github.io/)