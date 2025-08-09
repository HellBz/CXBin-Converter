# 🧊 CXBin-Converter

A powerful CLI tool to convert `.cxbin` files (Creality Model Format) into many common 3D formats like

`.stl`, `.obj`, `.ply`, `.gltf`, `.glb`, `.dae`, `.3mf` and more.  

Supports batch processing, ZIP packaging for multi-file formats, JSON API output, 

and runs both in Python environments and as standalone EXE.

---

## 📦 Features

- Reads `.cxbin` mesh files from Creality slicers & devices
- Exports via **trimesh** and **meshio** to a wide range of formats
- Supports multi-file formats (`OBJ`, `GLTF`) with optional ZIP output
- Batch mode for folders (with optional recursion)
- JSON output for API-based integration (`--json`)
- Optional inclusion of raw geometry data in JSON (`--json-geometry`)
- Custom output directory and naming templates
- Drag-and-drop support on Windows
- Buildable to standalone EXE or Linux binary with **PyInstaller**

---

## 🚀 Installation (Python Environment)

```bash
git clone https://github.com/HellBz/cxbin_converter.git
cd cxbin_converter
pip install -r requirements.txt
```

---

## ⚙️ Usage

### ➤ Minimal Example

```bash
python cxbin_converter.py model.cxbin
```

### ➤ Export to specific format

```bash
python cxbin_converter.py model.cxbin --format stl
python cxbin_converter.py model.cxbin --format obj --zip
python cxbin_converter.py model.cxbin --format gltf --zip-only
```

### ➤ Custom output directory and name

```bash
python cxbin_converter.py model.cxbin --format stl -o ./exports --output-name export_{stem}
```
`{stem}` = input filename without extension  
`{fmt}` = target format

### ➤ Batch convert all `.cxbin` in folder

```bash
python cxbin_converter.py ./input_folder --format stl --recursive
```

### ➤ JSON mode (API integration)

```bash
python cxbin_converter.py model.cxbin --format obj --json
```

Include geometry arrays in JSON:
```bash
python cxbin_converter.py model.cxbin --format obj --json --json-geometry
```

---

## 📜 Parameters

| Parameter                 | Short | Description |
|---------------------------|-------|-------------|
| `input`                   |       | Input file or folder |
| `--format`                | `-f`  | Output format (e.g. `stl`, `obj`, `ply`, `gltf`, `glb`, `off`, `dae`, `3mf`, …) |
| `--zip`                   |       | Zip multi-file outputs after export |
| `--zip-only`              |       | Zip multi-file outputs and remove folder |
| `--recursive`             | `-r`  | Search subfolders (only in folder input mode) |
| `--list-formats`           |       | Show all supported export formats |
| `--json`                  |       | Output results as JSON (no ASCII banner) |
| `--json-geometry`         |       | Include vertices, faces, UVs in JSON |
| `--output-dir`            | `-o`  | Custom output folder |
| `--output-name`           |       | Custom base name, supports `{stem}` and `{fmt}` |

---

## 📂 Supported Formats

### Trimesh exporters:
`3mf, amf, dae, glb, gltf, obj, off, ply, stl, vrml, x3d`

### Meshio fallback:
`amf, med, msh, obj, off, ply, stl, vtk, vtp, vtu, x3d, xdmf`

Multi-file formats: `gltf, obj`  
Single-file formats: `3mf, amf, dae, glb, off, ply, stl, vrml, x3d`

---

## 🧪 Sample CLI Output

```bash
   ______  ______  _              ____                          _            
  / ___\ \/ / __ )(_)_ __        / ___|___  _ ____   _____ _ __| |_ ___ _ __ 
 | |    \  /|  _ \| | '_ \ _____| |   / _ \| '_ \ \ / / _ \ '__| __/ _ \ '__|
 | |___ /  \| |_) | | | | |_____| |__| (_) | | | \ V /  __/ |  | ||  __/ |   
  \____/_/\_\____/|_|_| |_|      \____\___/|_| |_|\_/ \___|_|   \__\___|_|   

✅ Successfully exported:
   🔸 Format:        STL
   🔸 Target:        exports/cube.stl
   🔸 Vertices:      8
   🔸 Faces:         12
   🔸 Compressed:    428 Bytes
   🔸 Decompressed:  192 Bytes
```

---

## 🔧 Build Requirements

- Python ≥ 3.8
- Required:  
  ```bash
  pip install numpy trimesh Pillow
  ```
- Optional (for more formats):  
  ```bash
  pip install meshio lxml pygltflib h5py networkx
  ```

---

## 🛠 Build Standalone

### Windows:
```bash
pyinstaller --onefile cxbin_converter.py
```
or use
```bash
build.bat
```
Result: `dist/cxbin_converter.exe`

### Linux:
```bash
pyinstaller --onefile cxbin_converter.py
```
or use
```bash
build.sh
```
Result: `dist/cxbin_converter`

---

## 🔒 License

MIT License – Free for personal and commercial use.

---

## 👤 Author

**Stefan** – Dresden → Karlsruhe  
2025 – Open Source Enthusiast 🛠️
