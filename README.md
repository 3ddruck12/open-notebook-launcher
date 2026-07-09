# Open Notebook Desktop

Native Linux-Desktop-App für [Open Notebook](https://github.com/lfnovo/open-notebook). Die Anwendung ist ein Tauri-Launcher, der den Docker-Compose-Stack verwaltet und die Web-Oberfläche in einem eigenen Fenster öffnet.

**Launcher-Repository:** [github.com/3ddruck12/open-notebook-launcher](https://github.com/3ddruck12/open-notebook-launcher)

**Fork-Verweis:** [github.com/3ddruck12/open-notebook-desktop](https://github.com/3ddruck12/open-notebook-desktop) (README verweist hierher)

## Funktionen

- Geführtes Onboarding für Docker und Verschlüsselungsschlüssel
- Stack startet beim App-Öffnen automatisch (einstellbar)
- Open Notebook öffnet sich direkt — Verwaltung über Menüleiste
- Start, Stopp, Neustart und Logs des Open-Notebook-Stacks
- Automatische Docker-Installation über pkexec (Engine oder Docker Desktop)
- WebView-Fenster für die Open-Notebook-Oberfläche unter `http://127.0.0.1:8502`
- Datenverzeichnis: `~/.local/share/open-notebook-desktop`
- In-App-Updates für AppImage-Nutzer

## Voraussetzungen

### Build-Abhängigkeiten (Ubuntu/Debian/Linux Mint)

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget \
  libssl-dev libayatana-appindicator3-dev librsvg2-dev pkg-config
```

### Laufzeit

- Docker Engine oder Docker Desktop
- Docker Compose Plugin
- Benutzer in der `docker`-Gruppe (nach Installation ggf. neu anmelden)

## Sprache

Die Launcher-UI unterstützt **Deutsch** und **Englisch**:

- Beim ersten Start wird die Systemsprache automatisch erkannt (`LANG` / `LC_ALL`)
- Auf dem Welcome-Screen kann die Sprache geändert werden
- In den Einstellungen ist die Sprache jederzeit änderbar

Die Open-Notebook-Web-UI hat eine eigene Spracheinstellung innerhalb der Anwendung.

## App-Updates

- Automatische Update-Prüfung beim Start (nach ~8 Sekunden)
- Banner oben, wenn eine neue Version verfügbar ist
- **Einstellungen → App-Updates** für manuelle Prüfung
- **AppImage**: In-App-Update mit einem Klick
- **.deb**: Hinweis + Link zur Release-Seite

Details zum Veröffentlichen von Updates: [docs/UPDATES.md](docs/UPDATES.md)

## Entwicklung

```bash
cd desktop
npm install
npm run tauri dev
```

## Produktions-Build

```bash
export TAURI_SIGNING_PRIVATE_KEY="$(cat ~/.tauri/open-notebook-desktop.key)"
cd desktop
npm run tauri build
```

Erzeugte Pakete liegen unter:

- `desktop/src-tauri/target/release/bundle/deb/`
- `desktop/src-tauri/target/release/bundle/appimage/`

### AppImage ausführbar machen

```bash
chmod +x Open\ Notebook\ Desktop_*.AppImage
```

## GitHub Releases

Releases werden per GitHub Actions gebaut, sobald ein Tag wie `v0.1.0` gepusht wird.

**Voraussetzungen im GitHub-Repo:**

1. Secret `TAURI_SIGNING_PRIVATE_KEY` unter Settings → Secrets → Actions
2. Workflow-Berechtigung: **Read and write permissions** für `GITHUB_TOKEN`

**Release auslösen:**

```bash
git tag v0.1.0
git push origin v0.1.0
```

Downloads erscheinen unter [Releases](https://github.com/3ddruck12/open-notebook-launcher/releases).

GitHub-Einrichtung (Fork + leeres Launcher-Repo): [docs/GITHUB_SETUP.md](docs/GITHUB_SETUP.md)

Details: [docs/UPDATES.md](docs/UPDATES.md)

## Projektstruktur

```
opennotebook/
├── desktop/                 # Tauri 2 + React Launcher
│   ├── src/                 # Onboarding, Dashboard, Logs, Settings
│   └── src-tauri/           # Rust-Backend (Docker, Install, Config)
└── resources/
    └── docker-compose.yml   # Desktop-Compose-Datei
```

## Erststart

1. App starten
2. Docker einrichten (automatisch oder manuell)
3. Verschlüsselungsschlüssel generieren und speichern
4. Open Notebook startet automatisch (Stack + Web-UI)

Verwaltung bei Bedarf über die **Menüleiste** → „Verwaltung…“

## Hinweise

- Container laufen außerhalb von Tauri; die App steuert nur den Lifecycle.
- Docker-Images werden beim ersten Start heruntergeladen (nicht im Paket enthalten).
- Ports sind auf `127.0.0.1` gebunden (8502 UI, 5055 API, 8000 SurrealDB).

## Lizenz

MIT — Open Notebook upstream ist ebenfalls MIT-lizenziert.
