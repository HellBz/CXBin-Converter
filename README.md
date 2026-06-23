# 🧊 CXBin Converter – Tauri Rewrite

Ein leistungsstarker Tauri-Desktop-Rewrite des [CXBin-Converter](https://github.com/HellBz/cxbin_converter). Liest `.cxbin`-Dateien (Creality Model Format) und konvertiert sie in gängige 3D-Formate. Das CXBin-Parsing basiert auf der offiziellen C++-Referenzimplementierung von [CrealityOfficial/cxbin](https://github.com/CrealityOfficial/cxbin/tree/version-2.0.0/cxbin).

---

## 📦 Features

- Eigenes Rust-Backend, kompatibel mit `cxbin::loadCXBin` der Version 2.0.0
- Export nach **STL**, **PLY**, **OBJ**, **OFF**, **3MF**, **AMF**, **VRML**, **X3D**
- Integrierter **3D-Viewer** mit Three.js für schnelle Vorschau
- **CLI-Modus** für Batch-Verarbeitung, Drag & Drop auf die EXE und API-Integration
- **JSON-Output** mit optionalen Geometrie-Arrays
- 3MF-Dateien enthalten ein eingebettetes Vorschaubild
- Multi-file Formate (OBJ) werden automatisch in einen Ordner exportiert

---

## 🛠 Tech-Stack

- **Backend:** Rust + Tauri 2.0
- **Frontend:** React 18 + TypeScript + Vite + TailwindCSS + shadcn/ui
- **3D-Viewer:** Three.js
- **Parser:** Eigenes Rust-Modul basierend auf der Creality C++-Referenz

---

## 🚀 Installation & Start

```bash
git clone https://github.com/HellBz/cxbin_converter.git
cd cxbin_converter
git checkout tauri-rewrite
npm install
npm run tauri:dev
```

---

## ⚙️ Usage

### ➤ GUI-Modus

```bash
npm run tauri:dev
```

Öffnet die Desktop-Anwendung mit Drag & Drop, Format-Auswahl und integriertem 3D-Viewer.

### ➤ CLI-Modus

```bash
# Minimal
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin

# Format wählen
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin --format stl
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin --format 3mf
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin --format obj

# Ausgabeordner und Name
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin --format ply -o ./exports
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin --format ply --output-name export_{stem}

# Batch (optional rekursiv)
./src-tauri/target/release/cxbin-converter-tauri.exe ./input_folder --format stl --recursive

# JSON API
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin --format obj --json
./src-tauri/target/release/cxbin-converter-tauri.exe model.cxbin --format obj --json --json-geometry
```

Platzhalter:
- `{stem}` = Dateiname ohne Erweiterung
- `{fmt}` = Zielformat

### ➤ Drag & Drop auf die EXE

Eine `.cxbin`-Datei auf die fertige EXE ziehen konvertiert sie direkt im Standardformat `stl`.

---

## 📜 CLI-Parameter

| Parameter                 | Short | Description |
|---------------------------|-------|-------------|
| `input`                   |       | Eingabedatei oder Ordner |
| `--format`                | `-f`  | Zielformat (`stl`, `ply`, `obj`, `off`, `3mf`, `amf`, `vrml`, `x3d`) |
| `--output`                | `-o`  | Ausgabeordner |
| `--output-name`           |       | Ausgabename, unterstützt `{stem}` und `{fmt}` |
| `--recursive`             | `-r`  | Unterordner durchsuchen (nur bei Ordner-Eingabe) |
| `--json`                  |       | Ergebnisse als JSON ausgeben |
| `--json-geometry`         |       | Geometrie-Arrays in JSON einfügen |

---

## 📂 Unterstützte Formate

| Format | Typ | Besonderheit |
|--------|-----|--------------|
| `stl`  | Einzeldatei | Binär |
| `ply`  | Einzeldatei | ASCII |
| `obj`  | Multi-file | + MTL + Textur, falls vorhanden |
| `off`  | Einzeldatei | ASCII |
| `3mf`  | Einzeldatei | Mit eingebettetem Vorschaubild |
| `amf`  | Einzeldatei | XML-basiert |
| `vrml` | Einzeldatei | Text-basiert |
| `x3d`  | Einzeldatei | XML-basiert |

Weitere Formate wie `dae`, `glb`, `gltf`, `vtk`, `vtp`, `vtu`, `msh`, `med`, `xdmf` können über die `src-tauri/src/export/`-Module ergänzt werden.

---

## 🔧 Build

### Development

```bash
npm run tauri:dev
```

### Release

```bash
npm run tauri:build
```

Das fertige Windows-Installationsprogramm liegt unter:

```
src-tauri/target/release/bundle/
```

---

## 🧪 Projektstruktur

```
cxbin-converter/
├── src/                          # React-Frontend
│   ├── App.tsx
│   ├── components/
│   │   ├── ui/                   # shadcn/ui Komponenten
│   │   └── Viewer.tsx            # Three.js 3D-Viewer
│   ├── lib/utils.ts
│   └── main.tsx
├── src-tauri/                    # Rust-Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── cli.rs                # CLI-Modus
│   │   ├── commands.rs           # Tauri-Kommandos
│   │   ├── cxbin/                # CXBin-Parser
│   │   │   ├── mesh.rs
│   │   │   └── reader.rs
│   │   └── export/               # Export-Module
│   │       ├── stl.rs
│   │       ├── ply.rs
│   │       ├── obj.rs
│   │       ├── off.rs
│   │       ├── threemf.rs
│   │       ├── amf.rs
│   │       ├── vrml.rs
│   │       └── x3d.rs
│   └── tauri.conf.json
└── README.md
```

---

## 🖼 Icons generieren

Für ein vollständiges Icon-Set:

```bash
npm run tauri icon /pfad/zu/logo.svg
```

Momentan wird `src-tauri/icons/icon.ico` verwendet.

---

## 🔒 Lizenz

MIT License – Free for personal and commercial use.

---

## 👤 Author

**Stefan** – Dresden → Karlsruhe  
2025 – Open Source Enthusiast 🛠️
