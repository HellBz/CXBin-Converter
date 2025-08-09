# cxbin_reader.py
# -*- coding: utf-8 -*-
"""
CXBIN reader: Python port of the C++ logic (geometry + UVs + materials + textures + version handling).

Notes:
- This reader does NOT write files. It only parses CXBIN and returns an in-memory structure.
- Texture buffers are returned as PNG bytes when possible; otherwise raw bytes are returned.
- Some CXBIN material fields are proprietary (e.g., materials[i].decode in C++).
  We keep raw material blocks so higher layers can inspect or ignore them.

Author: Stefan's helper (comments strictly in English as requested)
"""

import io
import struct
import zlib
from dataclasses import dataclass, field
from typing import List, Optional, Dict, Any, Tuple

import numpy as np
from PIL import Image


def _read_exact(f, n: int) -> bytes:
    """Read exactly n bytes or raise EOFError."""
    b = f.read(n)
    if len(b) != n:
        raise EOFError("Unexpected end of file while reading CXBIN.")
    return b


def _read_int_le(f) -> int:
    """Read little-endian 32-bit int."""
    return struct.unpack("<i", _read_exact(f, 4))[0]


def _read_bytes(f, n: int) -> bytes:
    return _read_exact(f, n)


def _zlib_decompress(buf: bytes, expected_size: Optional[int] = None) -> bytes:
    """Decompress zlib buffer, optionally checking expected size."""
    raw = zlib.decompress(buf)
    if expected_size is not None and len(raw) != expected_size:
        # We don't fail hard; we warn by raising ValueError so caller can decide.
        raise ValueError(f"Decompressed size mismatch: got {len(raw)}, expected {expected_size}")
    return raw


@dataclass
class CxbinMaterials:
    """Container for material-related data."""
    # Raw decoded material blocks (opaque to us, we don't know their schema)
    material_blocks: List[bytes] = field(default_factory=list)

    # Optional material name (mtlName)
    material_name: Optional[str] = None

    # Texture buffers (prefer PNG bytes). If not PNG, raw buffer stored.
    # Each entry: dict with keys {'data': bytes, 'is_png': bool, 'size_hint': (w,h) or None}
    textures: List[Dict[str, Any]] = field(default_factory=list)

    # Texture IDs (per-face or per-vertex id mapping, depending on format)
    texture_ids: Optional[np.ndarray] = None


@dataclass
class CxbinMesh:
    """All data parsed from a CXBIN file."""
    # Geometry
    vertices: np.ndarray  # (N, 3) float32
    faces: np.ndarray     # (M, 3) int32

    # UVs and face UVs
    uvs: Optional[np.ndarray] = None        # (K, 2) float32
    face_uvs: Optional[np.ndarray] = None   # (M, 3) int32

    # Materials & textures
    materials: Optional[CxbinMaterials] = None

    # Optional version info
    cxbin_version: Optional[int] = None

    # Debug/metadata
    compressed_bytes: Optional[int] = None
    uncompressed_bytes: Optional[int] = None


def _parse_mesh_block(f) -> Tuple[np.ndarray, np.ndarray, Dict[str, int]]:
    """
    Port of C++ loadMesh():
      totalNum, vertNum, faceNum, compressNum
      compressed payload: [vertices(float32[vertNum*3]), faces(int32[faceNum*3])]
    """
    total_num = _read_int_le(f)
    vert_num = _read_int_le(f)
    face_num = _read_int_le(f)
    compress_num = _read_int_le(f)

    cdata = _read_bytes(f, compress_num)
    raw = _zlib_decompress(cdata)

    expected = vert_num * 3 * 4 + face_num * 3 * 4  # float32 + int32
    if len(raw) != expected:
        # Keep going but raise for caller to potentially handle.
        raise ValueError(f"Geometry block size mismatch (got {len(raw)}, expected {expected}).")

    v_buf = raw[: vert_num * 12]
    f_buf = raw[vert_num * 12 :]

    vertices = np.frombuffer(v_buf, dtype=np.float32).reshape((vert_num, 3))
    faces = np.frombuffer(f_buf, dtype=np.int32).reshape((face_num, 3))

    meta = {
        "compressed_bytes": compress_num,
        "uncompressed_bytes": len(raw),
    }
    return vertices, faces, meta


def _try_png_bytes_from_rgba(width: int, height: int, rgba_bytes: bytes) -> bytes:
    """Convert raw RGBA bytes (width*height*4) into PNG bytes."""
    img = Image.frombytes("RGBA", (width, height), rgba_bytes)
    out = io.BytesIO()
    img.save(out, format="PNG")
    return out.getvalue()


def _parse_material_block_v1_new(f) -> CxbinMaterials:
    """
    Port of C++ loadMaterial() (newer path):
      Reads a header of counts, then a compressed chunk which contains:
      - UVs (vec2)
      - faceUVs (ivec3)
      - textureIDs (int)
      - materials[i] opaque blobs
      - mtlName (char[mtlNameLen])
      - map buffers: for i in mapTypeCount: read int bufferSize + bufferData
    We attempt to keep texture buffers as PNG bytes where possible.
    """
    # Header (all int32 LE)
    total_num = _read_int_le(f)
    uv_num = _read_int_le(f)
    face_uv_num = _read_int_le(f)
    texture_id_num = _read_int_le(f)
    material_num = _read_int_le(f)

    material_sizes = []
    for _ in range(material_num):
        material_sizes.append(_read_int_le(f))

    mtl_name_len = _read_int_le(f)
    map_type_count = _read_int_le(f)
    compress_num = _read_int_le(f)

    cdata = _read_bytes(f, compress_num)
    raw = _zlib_decompress(cdata, expected_size=total_num)

    ptr = 0
    def take(n: int) -> bytes:
        nonlocal ptr
        chunk = raw[ptr:ptr+n]
        if len(chunk) != n:
            raise ValueError("Material block truncated.")
        ptr += n
        return chunk

    mats = CxbinMaterials()

    # UVs (float32[uv_num*2])
    if uv_num > 0:
        uv_bytes = take(uv_num * 2 * 4)
        mats._uvs_internal = np.frombuffer(uv_bytes, dtype=np.float32).reshape((uv_num, 2))  # stored temporarily

    # Face UVs (int32[face_uv_num*3])
    if face_uv_num > 0:
        fuv_bytes = take(face_uv_num * 3 * 4)
        mats._face_uvs_internal = np.frombuffer(fuv_bytes, dtype=np.int32).reshape((face_uv_num, 3))

    # Texture IDs (int32[texture_id_num])
    if texture_id_num > 0:
        tid_bytes = take(texture_id_num * 4)
        mats.texture_ids = np.frombuffer(tid_bytes, dtype=np.int32)

    # Material blocks (opaque)
    mats.material_blocks = []
    for size in material_sizes:
        if size > 0:
            mats.material_blocks.append(take(size))
        else:
            mats.material_blocks.append(b"")

    # Material name (char[mtl_name_len]) — may include trailing NUL
    if mtl_name_len > 0:
        name_bytes = take(mtl_name_len)
        # Strip trailing NULs
        mats.material_name = name_bytes.split(b"\x00", 1)[0].decode("utf-8", errors="replace")

    # Map buffers (textures)
    mats.textures = []
    for _ in range(max(0, map_type_count)):
        if ptr + 4 > len(raw):
            break
        buf_size = struct.unpack_from("<i", raw, ptr)[0]
        ptr += 4
        if buf_size < 0:
            buf_size = 0
        if ptr + buf_size > len(raw):
            # Truncated texture buffer; stop cleanly
            buf_size = max(0, len(raw) - ptr)
        buf = take(buf_size) if buf_size else b""

        is_png = buf.startswith(b"\x89PNG\r\n\x1a\n")
        mats.textures.append({
            "data": buf,
            "is_png": is_png,
            "size_hint": None,  # unknown in this variant
        })

    return mats


def _parse_material_block_v1_old(f) -> Tuple[CxbinMaterials, np.ndarray, np.ndarray]:
    """
    Port of C++ loadCXBin1_old():
      - compressed block containing geometry + uvs + texture ids + materials + mtlName
      - final textures provided as raw RGBA with width/height per map
    Returns (materials, vertices, faces). UVs/faceUVs placed in materials._uvs_internal/_face_uvs_internal.
    """
    mats = CxbinMaterials()

    # Read header (ints)
    total_num = _read_int_le(f)
    vert_num = _read_int_le(f)
    face_num = _read_int_le(f)
    uv_num = _read_int_le(f)
    face_uv_num = _read_int_le(f)
    texture_id_num = _read_int_le(f)
    material_num = _read_int_le(f)

    material_sizes = []
    for _ in range(material_num):
        material_sizes.append(_read_int_le(f))

    mtl_name_len = _read_int_le(f)
    map_type_count = _read_int_le(f)
    compress_num = _read_int_le(f)

    cdata = _read_bytes(f, compress_num)
    raw = _zlib_decompress(cdata, expected_size=total_num)

    ptr = 0
    def take(n: int) -> bytes:
        nonlocal ptr
        chunk = raw[ptr:ptr+n]
        if len(chunk) != n:
            raise ValueError("Old v1 block truncated.")
        ptr += n
        return chunk

    # vertices (float32[vert_num*3])
    v_bytes = take(vert_num * 3 * 4) if vert_num > 0 else b""
    vertices = np.frombuffer(v_bytes, dtype=np.float32).reshape((vert_num, 3)) if vert_num > 0 else np.zeros((0, 3), np.float32)

    # faces (int32[face_num*3])
    f_bytes = take(face_num * 3 * 4) if face_num > 0 else b""
    faces = np.frombuffer(f_bytes, dtype=np.int32).reshape((face_num, 3)) if face_num > 0 else np.zeros((0, 3), np.int32)

    # UVs
    if uv_num > 0:
        uv_bytes = take(uv_num * 2 * 4)
        mats._uvs_internal = np.frombuffer(uv_bytes, dtype=np.float32).reshape((uv_num, 2))

    # face UVs
    if face_uv_num > 0:
        fuv_bytes = take(face_uv_num * 3 * 4)
        mats._face_uvs_internal = np.frombuffer(fuv_bytes, dtype=np.int32).reshape((face_uv_num, 3))

    # texture IDs
    if texture_id_num > 0:
        tid_bytes = take(texture_id_num * 4)
        mats.texture_ids = np.frombuffer(tid_bytes, dtype=np.int32)

    # materials (opaque)
    mats.material_blocks = []
    for size in material_sizes:
        if size > 0:
            mats.material_blocks.append(take(size))
        else:
            mats.material_blocks.append(b"")

    # material name
    if mtl_name_len > 0:
        name_bytes = take(mtl_name_len)
        mats.material_name = name_bytes.split(b"\x00", 1)[0].decode("utf-8", errors="replace")

    # map buffers: width, height, then raw RGBA (we encode to PNG)
    mats.textures = []
    for _ in range(max(0, map_type_count)):
        if ptr + 8 > len(raw):
            break
        width = struct.unpack_from("<i", raw, ptr)[0]; ptr += 4
        height = struct.unpack_from("<i", raw, ptr)[0]; ptr += 4
        if width <= 0 or height <= 0:
            mats.textures.append({"data": b"", "is_png": False, "size_hint": (width, height)})
            continue
        # C++ code implies FORMAT_RGBA_8888; we assume width*height*4 bytes follow
        byte_count = width * height * 4
        if ptr + byte_count > len(raw):
            # Truncated texture; clamp
            byte_count = max(0, len(raw) - ptr)
        rgba = take(byte_count) if byte_count > 0 else b""
        try:
            png = _try_png_bytes_from_rgba(width, height, rgba)
            mats.textures.append({"data": png, "is_png": True, "size_hint": (width, height)})
        except Exception:
            mats.textures.append({"data": rgba, "is_png": False, "size_hint": (width, height)})

    return mats, vertices, faces


def load_cxbin(fp) -> CxbinMesh:
    """
    Main entry: load a CXBIN file and return a CxbinMesh.
    Implements the C++ logic of loadCXBin():
      - reads headCode + 12 bytes
      - if headCode != 1: loadMesh(); then read version and optional material block
      - else: legacy v1 old path (geometry+materials in one big block)
    """
    close_after = False
    if isinstance(fp, (str, bytes, bytearray)):
        f = open(fp, "rb")
        close_after = True
    else:
        f = fp

    try:
        head_code = _read_int_le(f)
        _ = _read_bytes(f, 12)  # skip 12 bytes like C++ does

        if head_code != 1:
            # Newer path: geometry first, then versioned material block
            vertices, faces, meta = _parse_mesh_block(f)

            version = None
            # Try to read version; if EOF, geometry-only file
            try:
                version = _read_int_le(f)
            except EOFError:
                return CxbinMesh(vertices=vertices, faces=faces,
                                 uvs=None, face_uvs=None, materials=None,
                                 cxbin_version=None,
                                 compressed_bytes=meta.get("compressed_bytes"),
                                 uncompressed_bytes=meta.get("uncompressed_bytes"))

            mats = None
            if version == 0:
                # No-op path in C++
                pass
            elif version == 1:
                mats = _parse_material_block_v1_new(f)
            else:
                # Unknown => treat like 0 (no extra material block)
                pass

            # Map temporary UV storage into top-level fields
            uvs = getattr(mats, "_uvs_internal", None) if mats else None
            face_uvs = getattr(mats, "_face_uvs_internal", None) if mats else None

            return CxbinMesh(
                vertices=vertices, faces=faces,
                uvs=uvs, face_uvs=face_uvs,
                materials=mats, cxbin_version=version,
                compressed_bytes=meta.get("compressed_bytes"),
                uncompressed_bytes=meta.get("uncompressed_bytes"),
            )

        else:
            # Legacy v1 "old" path: everything is inside a single compressed block
            mats, vertices, faces = _parse_material_block_v1_old(f)
            uvs = getattr(mats, "_uvs_internal", None)
            face_uvs = getattr(mats, "_face_uvs_internal", None)
            return CxbinMesh(
                vertices=vertices, faces=faces,
                uvs=uvs, face_uvs=face_uvs,
                materials=mats, cxbin_version=1,
                compressed_bytes=None, uncompressed_bytes=None
            )

    finally:
        if close_after:
            f.close()
