# GitHub: Zwei Repositories

## Übersicht

| Repository | Zweck |
|------------|--------|
| [3ddruck12/open-notebook-desktop](https://github.com/3ddruck12/open-notebook-desktop) | Fork / Verweis-README auf den Launcher |
| [3ddruck12/open-notebook-launcher](https://github.com/3ddruck12/open-notebook-launcher) | **Quellcode**, CI, Releases (AppImage, .deb) |

## 1. Neues leeres Repo anlegen

Auf GitHub: **New repository**

- Name: `open-notebook-launcher`
- **Leer** anlegen (ohne README, ohne .gitignore)
- Nicht als Fork von `open-notebook` erstellen

## 2. Fork-README setzen

Im Fork `open-notebook-desktop` die `README.md` ersetzen durch den Inhalt aus [FORK_README.md](./FORK_README.md) (ohne die erste Überschrift-Zeile „README für den Fork…“).

Optional: Upstream-Dateien im Fork löschen, sodass nur die README übrig bleibt.

## 3. Launcher-Code pushen

```bash
cd "/home/jens/Dokumente/Software Projekte/opennotebook"
git init
git add .
git commit -m "Initial Open Notebook Desktop launcher"
git branch -M main
git remote add origin https://github.com/3ddruck12/open-notebook-launcher.git
git push -u origin main
```

## 4. GitHub Secret (für Releases)

Im Repo **open-notebook-launcher**:

**Settings → Secrets → Actions →** `TAURI_SIGNING_PRIVATE_KEY`

**Settings → Actions → General →** Read and write permissions

## 5. Erstes Release

```bash
git tag v0.1.0
git push origin v0.1.0
```
