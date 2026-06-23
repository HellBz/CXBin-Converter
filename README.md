# CXBin Converter – Tauri Rewrite

Dies ist der Tauri-basierte Desktop-Rewrite des [CXBin-Converter](https://github.com/HellBz/cxbin_converter). Das CXBin-Parsing basiert auf der offiziellen C++-Referenzimplementierung von [CrealityOfficial/cxbin](https://github.com/CrealityOfficial/cxbin/tree/version-2.0.0/cxbin).

## Tech-Stack

- **Backend:** Rust + Tauri 2.0
- **Frontend:** React 18 + TypeScript + Vite + TailwindCSS + shadcn/ui
- **Parser:** Eigenes Rust-Modul, kompatibel mit `cxbin::loadCXBin` der Version 2.0.0

## Unterstützte Formate (aktuell)

- STL (binär)
- PLY (ASCII)
- OBJ (+ MTL + Textur, falls vorhanden)
- OFF

Weitere Formate (GLTF/GLB, DAE, 3MF) können über die `export/`-Module erweitert werden.

## Voraussetzungen

- [Node.js](https://nodejs.org/) (>= 18)
- [Rust](https://www.rust-lang.org/) (inkl. Cargo)
- (Optional) Tauri-CLI: `npm install -g @tauri-apps/cli@latest`

## Installation & Start

```bash
cd cxbin-converter-tauri
npm install
npm run tauri:dev
```

## Build

```bash
npm run tauri:build
```

Das fertige Windows-Installationsprogramm liegt unter `src-tauri/target/release/bundle`.

## Projektstruktur

```
cxbin-converter-tauri/
├── src/                  # React-Frontend
│   ├── App.tsx
│   ├── components/ui/    # shadcn/ui Komponenten
│   └── lib/utils.ts
├── src-tauri/            # Rust-Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs   # Tauri-Kommandos
│   │   ├── cxbin/        # CXBin-Parser
│   │   │   ├── mesh.rs
│   │   │   └── reader.rs
│   │   └── export/       # Export-Module
│   │       ├── stl.rs
│   │       ├── ply.rs
│   │       ├── obj.rs
│   │       └── off.rs
│   └── tauri.conf.json
```

## Icons generieren

Für ein vollständiges Icon-Set:

```bash
npm run tauri icon /pfad/zu/logo.svg
```

Momentan wird nur `src-tauri/icons/icon.ico` verwendet (kopiert aus dem Python-Projekt).

## Lizenz

MIT – siehe Hauptprojekt.
