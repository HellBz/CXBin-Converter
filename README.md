# 🧊 CXBin-Converter

A simple CLI tool to convert `.cxbin` files (Creality Model Format) into common 3D formats like `.stl`, `.obj`, `.ply`, `.gltf`, and more. Supports both Python environments and standalone EXE for Windows.

---

## 📦 Features

- Supports `.cxbin` mesh files from Creality
- Exports to: `STL`, `OBJ`, `PLY`, `GLB`, `GLTF`, `OFF`, `DAE`, `3MF`
- Auto fallback to `.stl` if no output is specified
- CLI with ASCII header and detailed debug output
- Drag and drop support on Windows
- Optional standalone build via PyInstaller for Windows/Linux
- No third-party libraries from Creality required

---

## 🚀 Installation (Python Environment)

```bash
git clone https://github.com/your-user-name/cxbin_converter.git
cd cxbin_converter
pip install -r requirements.txt
```

---

## ⚙️ Usage

### ➤ Using Python

```bash
# Minimal usage
python cxbin_converter.py model.cxbin

# With desired export format
python cxbin_converter.py model.cxbin output.obj

# Or e.g. output.glb, output.ply, output.3mf ...
```

Supported formats:
```
stl, ply, obj, glb, gltf, off, dae, 3mf
```

### ➤ As Executable (Windows/Linux)

#### Windows:
```bash
build.bat
# Result: dist\cxbin_converter.exe
```

#### Linux:
```bash
chmod +x build.sh
./build.sh
# Result: dist/cxbin_converter
```

---

## 🧪 Sample Output

```bash
   ______  ______  _              ____                          _            
  / ___\ \/ / __ )(_)_ __        / ___|___  _ ____   _____ _ __| |_ ___ _ __ 
 | |    \  /|  _ \| | '_ \ _____| |   / _ \| '_ \ \ / / _ \ '__| __/ _ \ '__|
 | |___ /  \| |_) | | | | |_____| |__| (_) | | | \ V /  __/ |  | ||  __/ |   
  \____/_/\_\____/|_|_| |_|      \____\___/|_| |_|\_/ \___|_|   \__\___|_|   

✅ Successfully exported:
   🔸 Format:        STL
   🔸 Target:        cube.stl
   🔸 Vertices:      8
   🔸 Faces:         12
   🔸 Compressed:    428 Bytes
   🔸 Decompressed:  192 Bytes
```

---

## 🔧 Build Requirements

- Python ≥ 3.7
- Optional: `pyinstaller` for standalone builds

### Manual requirements:
```bash
pip install numpy trimesh meshio networkx lxml
```

---

## 🔒 License

MIT License – Free to use for personal and commercial purposes.

---

## 👤 Author

**Stefan**, Dresden → Karlsruhe  
2025 – Open Source Enthusiast 🛠️

## 🧠 Note

This tool is based on the official implementation from [CrealityOfficial/cxbin](https://github.com/CrealityOfficial/cxbin/tree/version-2.0.0) (version 2.0.0), reimplemented in Python for easier cross-platform use.

