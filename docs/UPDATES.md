# App-Updates (Tauri Updater)

## Funktionen in der App

- **Automatische Prüfung** ~8 Sekunden nach App-Start (nur Produktions-Build)
- **Banner** oben, wenn ein Update verfügbar ist
- **Einstellungen → App-Updates** mit manueller Prüfung und Installation

## Installationsarten

| Art | In-App-Update | Verhalten |
|-----|---------------|-----------|
| **AppImage** | Ja | Download + Install + Neustart |
| **.deb** | Nein | Hinweis + Link zur Release-Seite |
| **Entwicklung** | Nein | Update-Check deaktiviert |

## Signing-Keys generieren

```bash
cd desktop
npm run tauri signer generate -- -w ~/.tauri/open-notebook-desktop.key --ci -p ""
```

Der öffentliche Schlüssel steht in `~/.tauri/open-notebook-desktop.key.pub` und muss in
`desktop/src-tauri/tauri.conf.json` unter `plugins.updater.pubkey` eingetragen werden.

## Release bauen (mit Signatur)

```bash
export TAURI_SIGNING_PRIVATE_KEY="$(cat ~/.tauri/open-notebook-desktop.key)"
cd desktop
npm run tauri build
```

Erzeugt zusätzlich:
- `Open Notebook Desktop_X.Y.Z_amd64.AppImage.sig`
- Signierte AppImage für den Updater

## latest.json veröffentlichen

1. `updates/latest.json.example` als Vorlage nutzen
2. Version, URL und Signatur-Inhalt aus der `.sig`-Datei eintragen
3. Als `latest.json` auf dem Update-Server oder GitHub Release hosten

Endpoint in `tauri.conf.json`:
```json
"endpoints": [
  "https://github.com/3ddruck12/open-notebook-launcher/releases/latest/download/latest.json"
]
```

## GitHub Actions

Das Repository enthält zwei Workflows:

| Workflow | Trigger | Zweck |
|----------|---------|-------|
| `ci.yml` | Push/PR auf `main` | Frontend-Build + `cargo check` |
| `release.yml` | Tag `v*` (z. B. `v0.1.0`) | AppImage + `.deb` + `latest.json` |

### Secret einrichten

Unter **GitHub → Settings → Secrets and variables → Actions**:

- `TAURI_SIGNING_PRIVATE_KEY` — Inhalt von `~/.tauri/open-notebook-desktop.key`

Unter **Settings → Actions → General → Workflow permissions**:
- **Read and write permissions** aktivieren (für Release-Upload)

### Release auslösen

```bash
git tag v0.1.0
git push origin v0.1.0
```

Der Workflow lädt automatisch hoch:
- AppImage + `.sig`
- `.deb`
- `latest.json` (für den In-App-Updater)

## Hinweis

Ohne veröffentlichte `latest.json` schlägt die Update-Prüfung fehl — das ist normal während der Entwicklung.
