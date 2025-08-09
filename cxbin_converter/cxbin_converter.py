# cxbin_converter.py
# -*- coding: utf-8 -*-
"""
CLI converter for CXBIN files:
- Uses cxbin_reader to parse geometry, UVs, materials, textures.
- Exports to many formats via trimesh and optionally meshio (fallback).
- Handles multi-file formats (OBJ, GLTF) by creating a folder named after the input (stem),
  and can zip that folder (--zip or --zip-only).
- Batch mode: input path can be a directory; use --recursive to scan subdirs.

Author: Stefan's helper (comments strictly in English as requested)
"""

import argparse
import os
import sys
import shutil
from pathlib import Path
import json
import io
import base64

import numpy as np
import trimesh

try:
    import meshio
except Exception:
    meshio = None

from PIL import Image

from cxbin_reader import load_cxbin, CxbinMesh

ASCII_HEADER = r"""
   ______  ______  _              ____                          _            
  / ___\ \/ / __ )(_)_ __        / ___|___  _ ____   _____ _ __| |_ ___ _ __ 
 | |    \  /|  _ \| | '_ \ _____| |   / _ \| '_ \ \ / / _ \ '__| __/ _ \ '__|
 | |___ /  \| |_) | | | | |_____| |__| (_) | | | \ V /  __/ |  | ||  __/ |   
  \____/_/\_\____/|_|_| |_|      \____\___/|_| |_|\_/ \___|_|   \__\___|_|   
"""

SINGLE_FILE_FORMATS = {
    "stl", "ply", "glb", "off", "dae", "3mf", "amf", "x3d", "vrml"
}

MULTI_FILE_FORMATS = {"obj", "gltf"}

# ---------------- Format helpers ----------------

def _trimesh_supported_formats() -> set:
    from trimesh.exchange import export as _export_mod
    exporters = getattr(_export_mod, "exporters", {}) or {}
    fmts = set(k.lower() for k in exporters.keys())
    if not fmts:
        dummy = trimesh.Trimesh(vertices=[[0,0,0],[1,0,0],[0,1,0]], faces=[[0,1,2]], process=False)
        for fmt in ["stl","ply","obj","glb","gltf","off","dae","3mf","amf","x3d","vrml"]:
            try:
                dummy.export(file_obj=io.BytesIO(), file_type=fmt)
            except Exception:
                pass
        exporters = getattr(_export_mod, "exporters", {}) or {}
        fmts = set(k.lower() for k in exporters.keys())
    if not fmts:
        fmts = {"stl","ply","obj","glb","gltf","off","dae","3mf","amf","x3d","vrml"}
    return fmts

def _meshio_supported_formats() -> set:
    if meshio is None:
        return set()
    return {"stl","ply","off","vtk","vtu","vtp","xdmf","msh","med","obj","x3d","amf"}

def list_formats():
    tri = _trimesh_supported_formats()
    mio = _meshio_supported_formats()
    print("Available export formats:")
    print("  trimesh:", ", ".join(sorted(tri)) if tri else "(none)")
    if meshio:
        print("  meshio (fallback):", ", ".join(sorted(mio)) if mio else "(none)")
    else:
        print("  meshio (fallback): not installed")
    print("\nMulti-file (handled with folder + optional ZIP):", ", ".join(sorted(MULTI_FILE_FORMATS)))
    print("Single-file:", ", ".join(sorted(SINGLE_FILE_FORMATS)))
    if not tri:
        print("\n⚠️  trimesh reported no exporters.")
        print("   Install/upgrade optional deps for more formats:")
        print("     pip install --upgrade numpy trimesh Pillow lxml pygltflib")
        print("   Then re-run with: --list-formats")

# ---------------- Texture helpers ----------------

def _save_texture_images(out_dir: Path, textures) -> list:
    out_dir.mkdir(parents=True, exist_ok=True)
    saved = []
    for idx, tex in enumerate(textures or []):
        data = tex.get("data") or b""
        is_png = bool(tex.get("is_png"))
        name = f"texture_{idx}.png"
        target = out_dir / name
        try:
            if is_png:
                target.write_bytes(data)
            else:
                size_hint = tex.get("size_hint")
                if size_hint and len(data) == size_hint[0] * size_hint[1] * 4:
                    Image.frombytes("RGBA", size_hint, data).save(target, format="PNG")
                else:
                    Image.open(io.BytesIO(data)).save(target, format="PNG")
            saved.append(name)
        except Exception:
            pass
    return saved

def _encode_textures_base64(textures) -> list:
    out = []
    for tex in textures or []:
        data = tex.get("data") or b""
        try:
            out.append({
                "is_png": bool(tex.get("is_png")),
                "size_hint": tex.get("size_hint"),
                "bytes_base64": base64.b64encode(data).decode("ascii") if data else ""
            })
        except Exception:
            out.append({"is_png": bool(tex.get("is_png")), "size_hint": tex.get("size_hint"), "bytes_base64": ""})
    return out

# ---------------- Output planning ----------------

def _render_name(template: str | None, stem: str, fmt: str) -> str:
    if not template:
        return stem
    return template.format(stem=stem, fmt=fmt)

def _plan_outputs(input_path: Path, fmt: str, output_dir: str | None,
                  output_name: str | None, is_multi: bool, zip_mode: str):
    stem = input_path.stem
    fmt = fmt.lower()
    base_dir = Path(output_dir) if output_dir else input_path.parent
    base_name = _render_name(output_name, stem, fmt)

    if is_multi:
        bundle_dir = base_dir / base_name
        bundle_dir.mkdir(parents=True, exist_ok=True)
        zip_file = (base_dir / f"{base_name}.zip") if zip_mode in ("zip","zip-only") else None
        return {"out_dir": base_dir, "bundle_dir": bundle_dir, "out_file": None, "zip_file": zip_file, "base_name": base_name}
    else:
        if output_name and Path(base_name).suffix.lower() == f".{fmt}":
            out_file = base_dir / base_name
        else:
            out_file = base_dir / f"{base_name}.{fmt}"
        out_file.parent.mkdir(parents=True, exist_ok=True)
        return {"out_dir": base_dir, "bundle_dir": None, "out_file": out_file, "zip_file": None, "base_name": base_name}

# ---------------- Mesh conversion helpers ----------------

def _mesh_from_cx(mesh: CxbinMesh) -> trimesh.Trimesh:
    tm = trimesh.Trimesh(vertices=mesh.vertices, faces=mesh.faces, process=False)
    uvs = mesh.uvs if mesh.uvs is not None else None
    if uvs is not None and len(uvs) == len(mesh.vertices):
        tm.visual = trimesh.visual.TextureVisuals(uv=uvs)
    return tm

def _export_with_trimesh(tm: trimesh.Trimesh, out_path: Path, fmt: str, image: Image.Image | None = None):
    if fmt == "obj":
        tm.export(out_path, file_type="obj"); return
    if fmt == "gltf":
        tm.export(out_path, file_type="gltf"); return
    if fmt == "glb":
        tm.export(out_path, file_type="glb"); return
    tm.export(out_path, file_type=fmt)

def _export_with_meshio(tm: trimesh.Trimesh, out_path: Path, fmt: str):
    if meshio is None:
        raise RuntimeError("meshio not installed; cannot export via meshio fallback.")
    points = np.array(tm.vertices, dtype=float)
    cells = [("triangle", np.array(tm.faces, dtype=int))]
    meshio.write(out_path.as_posix(), meshio.Mesh(points, cells))

def _export_obj_bundle(base_dir: Path, base_name: str, tm: trimesh.Trimesh, textures_saved: list) -> list[str]:
    obj_path = base_dir / f"{base_name}.obj"
    mtl_path = base_dir / f"{base_name}.mtl"
    if isinstance(tm.visual, trimesh.visual.texture.TextureVisuals) and textures_saved:
        try:
            tm.visual.material.image = Image.open(base_dir / textures_saved[0])
        except Exception:
            pass
    tm.export(obj_path, file_type="obj")
    if textures_saved and mtl_path.exists():
        try:
            mtl_txt = mtl_path.read_text(encoding="utf-8", errors="ignore")
            if "map_Kd" not in mtl_txt:
                mtl_txt += f"\nmap_Kd {textures_saved[0]}\n"
                mtl_path.write_text(mtl_txt, encoding="utf-8")
        except Exception:
            pass
    files = [str(obj_path)]
    if mtl_path.exists():
        files.append(str(mtl_path))
    for n in textures_saved:
        p = base_dir / n
        if p.exists():
            files.append(str(p))
    return files

def _export_gltf_bundle(base_dir: Path, base_name: str, tm: trimesh.Trimesh, textures_saved: list) -> list[str]:
    gltf_path = base_dir / f"{base_name}.gltf"
    if isinstance(tm.visual, trimesh.visual.texture.TextureVisuals) and textures_saved:
        try:
            tm.visual.material.image = Image.open(base_dir / textures_saved[0])
        except Exception:
            pass
    tm.export(gltf_path, file_type="gltf")
    files = [str(gltf_path)]
    bin_path = base_dir / f"{base_name}.bin"
    if bin_path.exists():
        files.append(str(bin_path))
    for n in textures_saved:
        p = base_dir / n
        if p.exists():
            files.append(str(p))
    return files

def _zip_dir(folder: Path, zip_only: bool):
    zip_base = folder.with_suffix("")
    zip_file = shutil.make_archive(zip_base.as_posix(), "zip", root_dir=folder.parent.as_posix(), base_dir=folder.name)
    if zip_only:
        shutil.rmtree(folder, ignore_errors=True)
    return zip_file

# ---------------- Conversion core ----------------

def convert_one(input_path: Path, out_format: str, zip_mode: str = "none",
                json_mode: bool = False, include_geometry: bool = False,
                output_dir: str | None = None, output_name: str | None = None) -> dict:
    mesh = load_cxbin(input_path.as_posix())
    tm = _mesh_from_cx(mesh)

    fmt = out_format.lower()
    is_multi = fmt in MULTI_FILE_FORMATS
    plan = _plan_outputs(input_path, fmt, output_dir, output_name, is_multi, zip_mode)

    result = {
        "input": str(input_path),
        "format": fmt,
        "zip_mode": zip_mode,
        "success": False,
        "outputs": [],
        "bundle_files": [],  # list concrete files for multi-file if not zipped-away
        "stats": {
            "vertices": int(len(mesh.vertices)) if mesh.vertices is not None else 0,
            "faces": int(len(mesh.faces)) if mesh.faces is not None else 0,
            "compressed_bytes": mesh.compressed_bytes,
            "uncompressed_bytes": mesh.uncompressed_bytes,
        },
        "materials": {
            "name": mesh.materials.material_name if getattr(mesh, "materials", None) else None,
            "texture_count": len(mesh.materials.textures) if getattr(mesh, "materials", None) and mesh.materials.textures else 0,
            "textures_base64": _encode_textures_base64(mesh.materials.textures) if getattr(mesh, "materials", None) and mesh.materials.textures else []
        },
        "error": None,
    }

    if json_mode and include_geometry:
        try:
            result["geometry"] = {
                "vertices": mesh.vertices.tolist() if mesh.vertices is not None else [],
                "faces": mesh.faces.tolist() if mesh.faces is not None else [],
                "uvs": mesh.uvs.tolist() if mesh.uvs is not None else None,
                "face_uvs": mesh.face_uvs.tolist() if mesh.face_uvs is not None else None,
            }
        except Exception as e:
            result["geometry_error"] = str(e)

    textures_saved = []
    if mesh.materials and mesh.materials.textures:
        pass

    try:
        if is_multi:
            bundle_dir = plan["bundle_dir"]
            base_name = plan["base_name"]

            if mesh.materials and mesh.materials.textures:
                textures_saved = _save_texture_images(bundle_dir, mesh.materials.textures)

            if fmt == "obj":
                files = _export_obj_bundle(bundle_dir, base_name, tm, textures_saved)
            elif fmt == "gltf":
                files = _export_gltf_bundle(bundle_dir, base_name, tm, textures_saved)
            else:
                raise ValueError(f"Unexpected multi-file format: {fmt}")

            if zip_mode in ("zip", "zip-only"):
                zip_path = _zip_dir(bundle_dir, zip_only=(zip_mode == "zip-only"))
                result["outputs"] = [str(zip_path)]
                # keep list of bundle files for reference (even if deleted)
                result["bundle_files"] = files
                if not json_mode:
                    print(f"📦 Zipped: {zip_path}")
            else:
                result["outputs"] = [str(bundle_dir)]
                result["bundle_files"] = files
                if not json_mode:
                    print(f"✅ Exported folder: {bundle_dir}")

        else:
            out_file = plan["out_file"]

            if fmt == "glb" and mesh.materials and mesh.materials.textures:
                first_tex = mesh.materials.textures[0]
                if first_tex.get("data"):
                    try:
                        img = Image.open(io.BytesIO(first_tex["data"]))
                        if mesh.uvs is not None:
                            tm.visual = trimesh.visual.TextureVisuals(uv=mesh.uvs, image=img)
                    except Exception:
                        pass

            tri_formats = _trimesh_supported_formats()
            try_trimesh = fmt in tri_formats
            exported = False
            if try_trimesh:
                try:
                    _export_with_trimesh(tm, out_file, fmt)
                    exported = True
                except Exception as e:
                    if meshio is not None and fmt in _meshio_supported_formats():
                        try:
                            _export_with_meshio(tm, out_file, fmt)
                            exported = True
                        except Exception as e2:
                            raise RuntimeError(f"trimesh export failed for {fmt} ({e}); meshio fallback failed ({e2})")
                    else:
                        raise
            if not exported and (meshio is not None) and fmt in _meshio_supported_formats():
                _export_with_meshio(tm, out_file, fmt)
                exported = True

            result["outputs"].append(str(out_file))

        if not json_mode:
            print("✅ Successfully exported:")
            print(f"   🔸 Format:        {fmt.upper()}")
            print(f"   🔸 Target:        {', '.join(result['outputs'])}")
            if result["bundle_files"]:
                print(f"   🔸 Files:         {', '.join(result['bundle_files'])}")
            print(f"   🔸 Vertices:      {result['stats']['vertices']}")
            print(f"   🔸 Faces:         {result['stats']['faces']}")
            if mesh.materials:
                print(f"   🔸 Textures:      {len(mesh.materials.textures)}")
            if mesh.compressed_bytes is not None:
                print(f"   🔸 Compressed:    {mesh.compressed_bytes} Bytes")
            if mesh.uncompressed_bytes is not None:
                print(f"   🔸 Decompressed:  {mesh.uncompressed_bytes} Bytes")

        result["success"] = True
        return result

    except Exception as e:
        if not json_mode:
            print(f"❌ Error: {e}")
        result["error"] = str(e)
        result["success"] = False
        return result

# ---------------- Batch ----------------

def _gather_inputs(path: Path, recursive: bool) -> list:
    if path.is_file() and path.suffix.lower() == ".cxbin":
        return [path]
    if path.is_dir():
        return [p for p in (path.rglob("*.cxbin") if recursive else path.glob("*.cxbin"))]
    return []

# ---------------- CLI ----------------

def main():
    parser = argparse.ArgumentParser(description="CXBIN → Mesh converter (geometry + UVs + materials + textures).")
    parser.add_argument("input", nargs="?", help="Input .cxbin file or directory.")
    parser.add_argument("--format", "-f", default="stl",
                        help="Output format (e.g., stl, ply, obj, gltf, glb, off, dae, 3mf, ...)")
    parser.add_argument("--zip", action="store_true", help="Zip multi-file outputs (OBJ/GLTF) after export.")
    parser.add_argument("--zip-only", action="store_true",
                        help="Zip multi-file outputs and remove the folder afterwards.")
    parser.add_argument("--recursive", "-r", action="store_true",
                        help="When input is a directory: search subfolders recursively.")
    parser.add_argument("--list-formats", action="store_true", help="List supported export formats and exit.")
    # JSON mode
    parser.add_argument("--json", action="store_true",
                        help="Output a JSON object to stdout (suppresses normal prints).")
    parser.add_argument("--json-geometry", action="store_true",
                        help="Include vertices/faces/uvs in the JSON output (can be large).")
    # Custom output planning
    parser.add_argument("--output-dir", "-o", default=None,
                        help="Custom output directory. Defaults to the input's folder.")
    parser.add_argument("--output-name", default=None,
                        help="Custom base name (file or folder). Supports {stem} and {fmt}.")

    args = parser.parse_args()

    if not args.json:
        print(ASCII_HEADER)

    if args.list_formats:
        list_formats()
        return

    if not args.input:
        if not args.json:
            parser.print_help()
        else:
            print(json.dumps({"results": [], "error": "No input provided."}, ensure_ascii=False))
        return

    zip_mode = "zip-only" if args.zip_only else ("zip" if args.zip else "none")

    in_path = Path(args.input)
    inputs = _gather_inputs(in_path, recursive=args.recursive)
    if not inputs:
        if args.json:
            print(json.dumps({"results": [], "error": "No .cxbin files found for the given input."}, ensure_ascii=False))
        else:
            print("❌ No .cxbin files found for the given input.")
        sys.exit(1)

    fmt = args.format.lower()
    results = []
    for i, cx in enumerate(inputs, 1):
        if not args.json:
            print(f"\n[{i}/{len(inputs)}] Converting: {cx}")
        info = convert_one(
            cx, fmt, zip_mode=zip_mode,
            json_mode=args.json, include_geometry=args.json_geometry,
            output_dir=args.output_dir, output_name=args.output_name
        )
        results.append(info)

    if args.json:
        print(json.dumps({"results": results}, ensure_ascii=False))
    else:
        print("\n✨ Done.")

if __name__ == "__main__":
    main()
